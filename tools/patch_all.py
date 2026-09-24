#!/usr/bin/env python3
"""
patch_all.py — terapkan dictionary terjemahan (translation_work/unique_strings.jsonl
+ locale/phase*.jsonl + locale/update*.jsonl) ke SEMUA varian translate_words_map_*
sekaligus (base, _diff, __small, __small_diff, _mobile), lalu repack masing-masing.

    python tools/patch_all.py [--outdir patched] [--level 12] [--files ...]

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

from wwm_locmap import read_container, write_container, parse_shard, rebuild_shard, TOMBSTONE
from expand_locale import load_translated_map

FILES = [
    "translate_words_map_en",
    "translate_words_map_en_diff",
    "translate_words_map_en__small",
    "translate_words_map_en__small_diff",
    "translate_words_map_en_mobile",
]


def patch_file(src_path, src_to_translated, level):
    ver, blocks = read_container(src_path)
    out_blocks = [blocks[0]]
    total = matched = tombstones = 0
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
            t = src_to_translated.get(text)
            if t is not None:
                vals[slot] = t.encode('utf-8', 'surrogateescape')
                matched += 1
            else:
                vals[slot] = v
        out_blocks.append(rebuild_shard(blk, vals))
    return ver, out_blocks, total, matched, tombstones


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--outdir", default=os.path.join(ROOT, "patched"))
    ap.add_argument("--level", type=int, default=12)
    ap.add_argument("--files", nargs="*", default=FILES)
    a = ap.parse_args()

    src_to_translated, phase_files = load_translated_map()
    print("dictionary terjemahan: %d string unik, dari %d file fase"
          % (len(src_to_translated), len(phase_files)))

    os.makedirs(a.outdir, exist_ok=True)
    for name in a.files:
        src_path = os.path.join(ROOT, name)
        if not os.path.exists(src_path):
            print("skip (tidak ada):", name)
            continue
        ver, out_blocks, total, matched, tombstones = patch_file(src_path, src_to_translated, a.level)
        out_path = os.path.join(a.outdir, name)
        write_container(out_path, ver, out_blocks, level=a.level)
        live = total - tombstones
        pct = 100 * matched / live if live else 0
        print("%-40s entri=%-8d tombstone=%-6d cocok=%-7d (%5.2f%%) -> %s"
              % (name, total, tombstones, matched, pct, out_path))


if __name__ == "__main__":
    main()
