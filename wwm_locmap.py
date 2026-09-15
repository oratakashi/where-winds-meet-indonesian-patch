#!/usr/bin/env python3
"""
wwm_locmap.py — reader/writer untuk file `translate_words_map_*`
(Where Winds Meet / NetEase Messiah Engine).

    pip install zstandard

    python wwm_locmap.py info   translate_words_map_en
    python wwm_locmap.py dump   translate_words_map_en strings.jsonl
    python wwm_locmap.py patch  translate_words_map_en strings.jsonl out_map_en

FORMAT (hasil reverse-engineering, terverifikasi round-trip 963.050 entri)
-------------------------------------------------------------------------
Container
  u32 magic        = 0xDEADBEEF
  u32 version      = 1
  u32 blockCount
  u32 reserved     = 0
  u32[blockCount] endOffset      # relatif terhadap akhir tabel ini

Block
  u8  codec        = 4  (zstd)
  u32 compressedSize
  u32 uncompressedSize
  u8[compressedSize] payload

Block 0  = index  : u64 totalEntries, u64 dataBlockCount, u32[dataBlockCount] id
Block 1..n = shard: hash map gaya SwissTable (absl::flat_hash_map)
  u64 capacity, u64 size, u64 seed
  u8[capacity] ctrl      (0x80 = kosong, <0x80 = terisi / H2)
  u8 0xFF sentinel + u8[15] cloned ctrl + padding ke kelipatan 8
  slot[capacity] { u64 keyHash; u32 relOffset; u32 byteLen; }   # 16 byte
  <string blob>

  nilai = blok[ alamat_field_relOffset + relOffset : + byteLen ]
          ^ relative pointer, BUKAN offset absolut

  shardIndex = (keyHash % dataBlockCount) + 1

CATATAN PENTING
  Kunci (teks sumber bahasa Inggris) TIDAK disimpan — hanya hash 64-bit-nya.
  Jadi map ini satu arah: hash(teks_inggris) -> teks_terjemahan.
  Karena itu patch bahasa apa pun WAJIB dimainkan dengan Game Language = English:
  runtime menghitung hash dari string Inggris, lalu menukarnya dengan nilai di sini.

  Konsekuensinya untuk modding: kita TIDAK perlu (dan tidak bisa) membangun ulang
  tabel hash. Cukup pertahankan ctrl + keyHash + posisi slot apa adanya, lalu
  tulis ulang string blob dan perbarui relOffset/byteLen tiap slot.
"""
import argparse, json, struct, sys

try:
    import zstandard as zstd
except ImportError:
    sys.exit("butuh: pip install zstandard")

MAGIC = 0xDEADBEEF
CODEC_ZSTD = 4

# `translate_words_map_*_diff` (incremental patch shipped between game updates)
# uses this exact same container/shard format, but only carries entries that
# changed since the base map. A key removed since the base version is marked
# with this single-byte sentinel instead of being absent from the shard.
TOMBSTONE = b'\xff'


# ---------------------------------------------------------------- container
def read_container(path):
    d = open(path, 'rb').read()
    magic, ver, cnt, _ = struct.unpack_from('<IIII', d, 0)
    if magic != MAGIC:
        raise ValueError("magic tidak cocok: %08x" % magic)
    offs = struct.unpack_from('<%dI' % cnt, d, 16)
    base = 16 + cnt * 4
    if base + offs[-1] != len(d):
        raise ValueError("ukuran file tidak konsisten dengan tabel offset")
    dctx = zstd.ZstdDecompressor()
    blocks = []
    for i in range(cnt):
        s = base + (offs[i - 1] if i else 0)
        b = d[s:base + offs[i]]
        codec, cs, us = struct.unpack_from('<BII', b, 0)
        if codec != CODEC_ZSTD:
            raise ValueError("codec tak dikenal: %d" % codec)
        raw = dctx.decompress(b[9:9 + cs])
        assert len(raw) == us
        blocks.append(raw)
    return ver, blocks


def write_container(path, ver, blocks, level=19, threads=-1):
    cctx = zstd.ZstdCompressor(level=level, threads=threads)
    payloads = [struct.pack('<BII', CODEC_ZSTD, len(c), len(r)) + c
                for r, c in ((r, cctx.compress(r)) for r in blocks)]
    out = bytearray(struct.pack('<IIII', MAGIC, ver, len(blocks), 0))
    acc, table = 0, []
    for p in payloads:
        acc += len(p)
        table.append(acc)
    out += struct.pack('<%dI' % len(table), *table)
    for p in payloads:
        out += p
    open(path, 'wb').write(bytes(out))


# -------------------------------------------------------------------- shard
def _slots_off(cap):
    o = 24 + cap + 1 + 15
    return o + (-o) % 8


def parse_index(blk):
    total, nblocks = struct.unpack_from('<QQ', blk, 0)
    return total, nblocks


def parse_shard(blk):
    cap, size, seed = struct.unpack_from('<QQQ', blk, 0)
    ctrl = blk[24:24 + cap]
    if blk[24 + cap] != 0xFF:
        raise ValueError("sentinel hilang — layout tidak cocok")
    S = _slots_off(cap)
    out = []
    for i, c in enumerate(ctrl):
        if c >= 0x80:
            continue
        p = S + i * 16
        h, rel, ln = struct.unpack_from('<QII', blk, p)
        a = p + 8 + rel
        out.append((i, h, blk[a:a + ln]))
    if len(out) != size:
        raise ValueError("jumlah slot terisi != size")
    return cap, seed, out


def rebuild_shard(orig, values):
    """values: dict slotIndex -> bytes. ctrl/keyHash/posisi slot dipertahankan."""
    cap, size, seed = struct.unpack_from('<QQQ', orig, 0)
    S = _slots_off(cap)
    head = bytearray(orig[:S + cap * 16])
    ctrl = orig[24:24 + cap]
    blob, pool = bytearray(), {}
    for i, c in enumerate(ctrl):
        if c >= 0x80:
            continue
        v = values[i]
        if v not in pool:                       # dedup string identik
            pool[v] = len(blob)
            blob += v
        field = S + i * 16 + 8                  # alamat field relOffset
        struct.pack_into('<II', head, field,
                         len(head) + pool[v] - field, len(v))
    return bytes(head) + bytes(blob)


# --------------------------------------------------------------------- CLI
def cmd_info(a):
    ver, blocks = read_container(a.src)
    total, nblocks = parse_index(blocks[0])
    print("version        :", ver)
    print("blocks         : %d (1 index + %d shard)" % (len(blocks), nblocks))
    print("entries (index):", total)
    n, deleted = 0, 0
    for b in blocks[1:]:
        _, _, ents = parse_shard(b)
        n += len(ents)
        deleted += sum(1 for _, _, v in ents if v == TOMBSTONE)
    print("entries parsed :", n)
    if deleted:
        print("  deleted (tombstone 0xFF):", deleted)
    if n != total:
        print("note: parsed count != index total -- file terlihat seperti "
              "*_diff (hanya berisi key yang berubah/terhapus sejak versi dasar)")


def cmd_dump(a):
    ver, blocks = read_container(a.src)
    deleted = 0
    with open(a.out, 'w', encoding='utf-8') as f:
        for bi, blk in enumerate(blocks[1:], 1):
            _, _, ents = parse_shard(blk)
            for slot, h, v in ents:
                rec = {"b": bi, "s": slot, "h": "%016x" % h}
                if v == TOMBSTONE:
                    rec["deleted"] = True
                    deleted += 1
                else:
                    try:
                        rec["v"] = v.decode('utf-8')
                    except UnicodeDecodeError as e:
                        raise ValueError(
                            "nilai biner tak terduga di blok %d slot %d "
                            "(bukan UTF-8, bukan tombstone 0xFF): %r"
                            % (bi, slot, v[:32])) from e
                f.write(json.dumps(rec, ensure_ascii=False) + "\n")
    print("ditulis:", a.out)
    if deleted:
        print("entri terhapus (tombstone):", deleted)


def cmd_patch(a):
    ver, blocks = read_container(a.src)
    edits = {}
    with open(a.jsonl, encoding='utf-8') as f:
        for line in f:
            if not line.strip():
                continue
            r = json.loads(line)
            v = TOMBSTONE if r.get("deleted") else \
                r["v"].encode('utf-8', 'surrogateescape')
            edits.setdefault(r["b"], {})[r["s"]] = v
    out = [blocks[0]]
    for bi, blk in enumerate(blocks[1:], 1):
        _, _, ents = parse_shard(blk)
        vals = {s: v for s, _, v in ents}
        vals.update(edits.get(bi, {}))
        out.append(rebuild_shard(blk, vals))
    write_container(a.out, ver, out, level=a.level)
    print("ditulis:", a.out)


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__,
                                formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = p.add_subparsers(dest='cmd', required=True)
    i = sub.add_parser('info');  i.add_argument('src');  i.set_defaults(f=cmd_info)
    d = sub.add_parser('dump');  d.add_argument('src');  d.add_argument('out')
    d.set_defaults(f=cmd_dump)
    q = sub.add_parser('patch'); q.add_argument('src');  q.add_argument('jsonl')
    q.add_argument('out'); q.add_argument('--level', type=int, default=19)
    q.set_defaults(f=cmd_patch)
    a = p.parse_args()
    a.f(a)
