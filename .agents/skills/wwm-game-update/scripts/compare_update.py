#!/usr/bin/env python3
"""
compare_update.py — laporan read-only untuk update game baru: bandingkan setiap
translate_words_map_* di root repo dengan salinan `.before`-nya, lalu hitung
berapa string yang belum ada di translation_work/unique_strings.jsonl.

    python .claude/skills/wwm-game-update/scripts/compare_update.py

Tidak menulis apa pun ke repo (decode langsung di memori, tanpa dump JSONL).
"""
import hashlib, glob, json, os, sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", ".."))
sys.path.insert(0, ROOT)
sys.path.insert(0, os.path.join(ROOT, "tools"))
from wwm_locmap import read_container, parse_index, parse_shard, TOMBSTONE  # noqa: E402

UNIQUE = os.path.join(ROOT, "translation_work", "unique_strings.jsonl")
GITHUB_WARN, GITHUB_LIMIT = 50 * 2**20, 100 * 2**20


def sha(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def load(path):
    """-> (index total, {keyHash: text atau None untuk tombstone}, jumlah block)."""
    _, blocks = read_container(path)
    total = parse_index(blocks[0])[0]
    ents = {}
    for blk in blocks[1:]:
        for _, h, v in parse_shard(blk)[2]:
            ents[h] = None if v == TOMBSTONE else v.decode("utf-8")
    return total, ents, len(blocks)


def main():
    seen = set()
    with open(UNIQUE, encoding="utf-8") as f:
        for line in f:
            if line.strip():
                seen.add(json.loads(line)["v"])
    print("unique_strings.jsonl: %d baris" % len(seen))

    names = sorted(os.path.basename(p) for p in glob.glob(os.path.join(ROOT, "translate_words_map_*"))
                   if not p.endswith(".before"))
    befores = sorted(os.path.basename(p)[:-7] for p in glob.glob(os.path.join(ROOT, "translate_words_map_*.before")))
    for gone in sorted(set(befores) - set(names)):
        print("\n!! %s ada di .before tapi TIDAK ada di versi baru (variant dihapus?)" % gone)

    new_all = set()
    for name in names:
        path = os.path.join(ROOT, name)
        before = path + ".before"
        print("\n== %s (%s bytes)" % (name, format(os.path.getsize(path), ",")))
        if not os.path.exists(before):
            print("   !! tidak ada .before — variant BARU (cek format, tambahkan ke FILES di tools/patch_all.py)")
            old = None
        elif sha(path) == sha(before):
            print("   byte-identik dengan .before — tidak berubah")
            continue
        else:
            old = load(before)
        total, ents, nblk = load(path)
        live = [v for v in ents.values() if v is not None]
        print("   blocks %d, index total %s, parsed %s, tombstone %s"
              % (nblk, format(total, ","), format(len(ents), ","),
                 format(len(ents) - len(live), ",")))
        if old:
            o_total, o_ents, o_nblk = old
            added = ents.keys() - o_ents.keys()
            removed = o_ents.keys() - ents.keys()
            changed = sum(1 for h in ents.keys() & o_ents.keys() if ents[h] != o_ents[h])
            print("   sebelumnya: blocks %d, index total %s, parsed %s"
                  % (o_nblk, format(o_total, ","), format(len(o_ents), ",")))
            print("   key +%s / -%s, nilai berubah %s"
                  % (format(len(added), ","), format(len(removed), ","), format(changed, ",")))
        new = {v for v in live if v not in seen}
        new_all |= new
        print("   teks unik belum ada di dictionary: %s (%s karakter)"
              % (format(len(new), ","), format(sum(map(len, new)), ",")))

    print("\n== TOTAL string baru (gabungan, dedup): %s (%s karakter)"
          % (format(len(new_all), ","), format(sum(map(len, new_all)), ",")))
    print("   akan jadi idx %d..%d bila di-append" % (len(seen), len(seen) + len(new_all) - 1)
          if new_all else "   tidak ada yang perlu di-append")

    size = os.path.getsize(UNIQUE)
    est = size * (len(seen) + len(new_all)) / max(len(seen), 1)
    flag = "!! MELEWATI 100 MB" if est > GITHUB_LIMIT else ("peringatan GitHub >50 MB" if est > GITHUB_WARN else "ok")
    print("\nukuran unique_strings.jsonl: %.1f MB -> perkiraan %.1f MB (%s)"
          % (size / 2**20, est / 2**20, flag))


if __name__ == "__main__":
    main()
