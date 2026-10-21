#!/usr/bin/env python3
"""
rebuild_unique_strings.py — perluas translation_work/unique_strings.jsonl setelah
update game, TANPA mengubah idx yang sudah ada (supaya locale/phase*.jsonl yang
sudah diterjemahkan tetap valid/tidak perlu di-remap).

    python tools/rebuild_unique_strings.py [--strings strings.jsonl]
        [--extra strings_diff.jsonl]
    python tools/rebuild_unique_strings.py --profile cn     # WWM China (cn/)

Default --strings/--extra = dump-dump profil (tools/profiles.py). Kalau
unique_strings.jsonl profil belum ada, file dibuat dari nol (bootstrap: idx 0..N-1).
Profil dengan jembatan (cn) melewati entri yang key-nya ada di global dan teks yang
muncul di key bersama — itu diterjemahkan lewat tools/bridge.py, bukan di profil ini.

Mengumpulkan semua teks (`v`) yang sedang "hidup" di dump-dump saat ini (melewati
entri "deleted" dari file *_diff). Teks yang sudah pernah tercatat di
unique_strings.jsonl dilewati (idx lama dipertahankan apa adanya). Teks baru
diberi idx baru berurutan mulai dari (idx maksimum lama + 1), diurutkan menurun
berdasarkan frekuensi kemunculan di --strings (mengikuti metodologi
translation_work/build_unique.py) — string yang hanya muncul di file --extra (mis. _diff)
diberi freq minimal 1.
"""
import argparse, json, os
from collections import Counter

import profiles
from bridge import global_keys, shared_texts


def load_existing(unique_path):
    seen, max_idx, count = set(), -1, 0
    if not os.path.exists(unique_path):         # bootstrap profil baru
        return seen, max_idx, count
    with open(unique_path, encoding="utf-8") as f:
        for line in f:
            if not line.strip():
                continue
            d = json.loads(line)
            seen.add(d["v"])
            max_idx = max(max_idx, d["idx"])
            count += 1
    return seen, max_idx, count


def iter_live_texts(path, skip_keys=(), skip_texts=()):
    if not os.path.exists(path):
        return
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            d = json.loads(line)
            if d.get("deleted") or d["h"] in skip_keys or d["v"] in skip_texts:
                continue
            yield d["v"]


def main():
    ap = argparse.ArgumentParser()
    profiles.add_argument(ap)
    ap.add_argument("--strings", help="default: dump pertama profil")
    ap.add_argument("--extra", nargs="*", help="default: dump profil lainnya")
    ap.add_argument("--dry-run", action="store_true",
                     help="hitung & laporkan saja, jangan tulis ke unique_strings.jsonl")
    a = ap.parse_args()
    P = profiles.get(a.profile)
    UNIQUE_PATH = P.unique
    dumps = [P.path(d) for _, d in P.dumps]
    if a.strings is None:
        a.strings = dumps[0]
    if a.extra is None:
        a.extra = dumps[1:]
    if not os.path.exists(a.strings):
        raise SystemExit("%s tidak ada — dump dulu binary-nya dengan wwm_locmap.py dump"
                         % a.strings)

    skip, skip_texts = set(), set()
    if P.bridge_from == "global":
        skip = global_keys()
        skip_texts = set(shared_texts(P, skip))
        print("jembatan global: %d key global + %d teks di key bersama dilewati "
              "(diterjemahkan lewat profil global)" % (len(skip), len(skip_texts)))

    seen, max_idx, old_count = load_existing(UNIQUE_PATH)
    print("unique_strings.jsonl lama: %d entri, idx maksimum: %d" % (old_count, max_idx))

    freq = Counter(iter_live_texts(a.strings, skip, skip_texts))

    new_texts, new_seen, first_seen, order = [], set(), {}, 0
    for path in [a.strings] + a.extra:
        for v in iter_live_texts(path, skip, skip_texts):
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

    if not new_texts:
        return

    # Pastikan baris baru tidak menempel ke baris terakhir kalau file tidak
    # diakhiri newline (akan merusak JSON baris itu).
    os.makedirs(os.path.dirname(UNIQUE_PATH), exist_ok=True)
    with open(UNIQUE_PATH, "ab+") as f:
        size = f.seek(0, os.SEEK_END)
        needs_nl = False
        if size:
            f.seek(-1, os.SEEK_END)
            needs_nl = f.read(1) != b"\n"

    next_idx = max_idx + 1
    with open(UNIQUE_PATH, "a", encoding="utf-8", newline="\n") as f:
        if needs_nl:
            f.write("\n")
        for v in new_texts:
            f.write(json.dumps({"idx": next_idx, "freq": freq.get(v, 0) or 1, "v": v},
                                ensure_ascii=False) + "\n")
            next_idx += 1

    print("ditambahkan idx %d..%d ke %s" % (max_idx + 1, next_idx - 1, UNIQUE_PATH))


if __name__ == "__main__":
    main()
