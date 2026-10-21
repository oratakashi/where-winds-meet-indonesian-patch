#!/usr/bin/env python3
"""
patch_all.py — terapkan dictionary terjemahan (translation_work/unique_strings.jsonl
+ locale/phase*.jsonl + locale/update*.jsonl) ke SEMUA varian translate_words_map_*
sekaligus (base, _diff, _mobile), lalu repack masing-masing. (__small/__small_diff
sudah tidak dikirim game sejak v2.3; tambahkan lagi ke files profil global di tools/profiles.py kalau muncul kembali.)

    python tools/patch_all.py [--outdir patched] [--level 12] [--files ...]
    python tools/patch_all.py --profile cn     # WWM China: cn/translate_words_map_zh_cn* -> cn/work/patched/

Profil cn, per entri: terjemahan global untuk key-nya (jembatan per key) -> dictionary
CN untuk teks Mandarin-nya -> terjemahan global untuk teks Mandarin yang sama di key
bersama (jembatan per teks). Lihat tools/bridge.py; butuh dump global + dump CN.
Container v2 (CN) ditulis kembali sebagai v2: kamus terenkripsi disalin apa adanya,
blok dikompres ulang dengan kamus yang sama (butuh cn/work/keystream_v2.bin).

Setiap entri dicocokkan berdasarkan teks sumbernya sendiri (bukan block/slot atau
hash) terhadap dictionary source->translated, jadi otomatis benar untuk file mana
pun — termasuk translate_words_map_en__small yang punya address space (b/s) sendiri
dan sebagian isinya beda dari translate_words_map_en meski hash-nya sama.
Entri yang tidak match (belum diterjemahkan) dipertahankan dari file asli, tombstone
(*_diff) dilewati tanpa disentuh.
"""
import argparse, os, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, ROOT)
sys.path.insert(0, os.path.join(ROOT, "tools"))

from wwm_locmap import read_container_ex, write_container_ex, parse_shard, rebuild_shard, TOMBSTONE
from expand_locale import load_translated_map, load_overrides
from bridge import load_bridge
import profiles

FILES = profiles.GLOBAL.files       # varian per profil: tools/profiles.py


def patch_file(src_path, src_to_translated, level, bridge=None, text_bridge=None,
               overrides=None):
    """overrides: keyHash (hex) -> (teks sumber, terjemahan), menang atas semuanya;
    bridge: keyHash (hex) -> terjemahan, dicoba sebelum src_to_translated;
    text_bridge: teks -> terjemahan, dicoba sesudahnya."""
    bridge, text_bridge, overrides = bridge or {}, text_bridge or {}, overrides or {}
    hdr, blocks = read_container_ex(src_path)
    out_blocks = [blocks[0]]
    total = matched = tombstones = bridged = 0
    for blk in blocks[1:]:
        _, _, ents = parse_shard(blk)
        vals = {}
        for slot, h, v in ents:
            total += 1
            if v == TOMBSTONE:
                vals[slot] = v
                tombstones += 1
                continue
            try:
                text = v.decode('utf-8')
            except UnicodeDecodeError:
                vals[slot] = v          # nilai biner tak terduga, biarkan apa adanya
                continue
            hx = "%016x" % h
            o = overrides.get(hx)
            if o and o[0] != text:
                print("PERINGATAN: override %s dilewati, teks sumber sudah berubah" % hx)
                o = None
            if o:
                t = o[1]
            else:
                t = bridge.get(hx) if bridge else None
                if t is not None:
                    bridged += 1
                else:
                    t = src_to_translated.get(text)
                    if t is None:
                        t = text_bridge.get(text)
                        bridged += t is not None
            if t is not None:
                vals[slot] = t.encode('utf-8', 'surrogateescape')
                matched += 1
            else:
                vals[slot] = v
        out_blocks.append(rebuild_shard(blk, vals))
    return hdr, out_blocks, total, matched, tombstones, bridged


def main():
    ap = argparse.ArgumentParser()
    profiles.add_argument(ap)
    ap.add_argument("--outdir", help="default: patched/ (global), cn/work/patched/ (cn)")
    ap.add_argument("--level", type=int, default=12)
    ap.add_argument("--files", nargs="*", help="default: semua varian profil")
    a = ap.parse_args()
    P = profiles.get(a.profile)
    a.outdir = a.outdir or P.patched_dir
    a.files = a.files or P.files

    src_to_translated, phase_files = load_translated_map(a.profile)
    print("dictionary terjemahan: %d string unik, dari %d file fase"
          % (len(src_to_translated), len(phase_files)))
    bridge, text_bridge = load_bridge(a.profile)
    overrides = load_overrides(a.profile)
    if overrides:
        print("override per key: %d" % len(overrides))
    if bridge:
        print("jembatan global: %d key + %d teks sudah diterjemahkan"
              % (len(bridge), len(text_bridge)))

    os.makedirs(a.outdir, exist_ok=True)
    for name in a.files:
        src_path = P.path(name)
        if not os.path.exists(src_path):
            print("skip (tidak ada):", name)
            continue
        hdr, out_blocks, total, matched, tombstones, bridged = patch_file(
            src_path, src_to_translated, a.level, bridge, text_bridge, overrides)
        out_path = os.path.join(a.outdir, name)
        write_container_ex(out_path, hdr, out_blocks, level=a.level)
        live = total - tombstones
        pct = 100 * matched / live if live else 0
        via = " (jembatan=%d, cn=%d)" % (bridged, matched - bridged) if bridge else ""
        print("%-40s entri=%-8d tombstone=%-6d cocok=%-7d (%5.2f%%)%s -> %s"
              % (name, total, tombstones, matched, pct, via, out_path))


if __name__ == "__main__":
    main()
