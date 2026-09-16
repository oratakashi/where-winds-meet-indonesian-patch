#!/usr/bin/env python3
"""
rebuild_unique_strings.py — perluas translation_work/unique_strings.jsonl setelah
update game, TANPA mengubah idx yang sudah ada (supaya locale/phase*.jsonl yang
sudah diterjemahkan tetap valid/tidak perlu di-remap).

    python tools/rebuild_unique_strings.py [--strings strings.jsonl]
        [--extra strings_diff.jsonl strings_small.jsonl strings_small_diff.jsonl]

Mengumpulkan semua teks (`v`) yang sedang "hidup" di dump-dump saat ini (melewati
entri "deleted" dari file *_diff). Teks yang sudah pernah tercatat di
unique_strings.jsonl dilewati (idx lama dipertahankan apa adanya). Teks baru
diberi idx baru berurutan mulai dari (idx maksimum lama + 1), diurutkan menurun
berdasarkan frekuensi kemunculan di --strings (mengikuti metodologi
translation_work/build_unique.py) — string yang hanya muncul di file _diff/__small
diberi freq minimal 1.
"""
import argparse, json, os
from collections import Counter

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
UNIQUE_PATH = os.path.join(ROOT, "translation_work", "unique_strings.jsonl")


def load_existing():
    seen, max_idx, count = set(), -1, 0
    with open(UNIQUE_PATH, encoding="utf-8") as f:
        for line in f:
            d = json.loads(line)
            seen.add(d["v"])
            max_idx = max(max_idx, d["idx"])
            count += 1
    return seen, max_idx, count


def iter_live_texts(path):
    if not os.path.exists(path):
        return
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            d = json.loads(line)
            if d.get("deleted"):
                continue
            yield d["v"]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--strings", default=os.path.join(ROOT, "strings.jsonl"))
    ap.add_argument("--extra", nargs="*", default=[
        os.path.join(ROOT, "strings_diff.jsonl"),
        os.path.join(ROOT, "strings_small.jsonl"),
        os.path.join(ROOT, "strings_small_diff.jsonl"),
    ])
    ap.add_argument("--dry-run", action="store_true",
                     help="hitung & laporkan saja, jangan tulis ke unique_strings.jsonl")
    a = ap.parse_args()

    seen, max_idx, old_count = load_existing()
    print("unique_strings.jsonl lama: %d entri, idx maksimum: %d" % (old_count, max_idx))

    freq = Counter(iter_live_texts(a.strings))

    new_texts, new_seen, first_seen, order = [], set(), {}, 0
    for path in [a.strings] + a.extra:
        for v in iter_live_texts(path):
            if v in seen or v in new_seen:
                continue
            new_seen.add(v)
            new_texts.append(v)
            first_seen[v] = order
            order += 1

    new_texts.sort(key=lambda v: (-freq.get(v, 0), first_seen[v]))

    print("string baru (belum pernah tercatat):", len(new_texts))
    if a.dry_run:
        print("--dry-run: tidak ada yang ditulis")
        return

    next_idx = max_idx + 1
    with open(UNIQUE_PATH, "a", encoding="utf-8") as f:
        for v in new_texts:
            f.write(json.dumps({"idx": next_idx, "freq": freq.get(v, 0) or 1, "v": v},
                                ensure_ascii=False) + "\n")
            next_idx += 1

    print("ditambahkan idx %d..%d ke %s" % (max_idx + 1, next_idx - 1, UNIQUE_PATH))


if __name__ == "__main__":
    main()
