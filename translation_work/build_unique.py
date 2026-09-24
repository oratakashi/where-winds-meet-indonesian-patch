"""
build_unique.py — bootstrap SEKALI PAKAI: bangun unique_strings.jsonl dari nol
(urut menurun berdasarkan frekuensi di strings.jsonl).

JANGAN jalankan ulang setelah terjemahan dimulai: idx akan diberi ulang dari 0,
dan setiap locale/*.jsonl (yang dikunci ke idx) jadi menunjuk string yang salah.
Untuk menambah string setelah update game, pakai tools/rebuild_unique_strings.py
(hanya menambah idx baru, tidak mengubah idx lama).
"""
import json, os, sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "..", "strings.jsonl")
OUT = os.path.join(HERE, "unique_strings.jsonl")

if os.path.exists(OUT):
    sys.exit("%s sudah ada — menimpanya akan merusak mapping idx di locale/*.jsonl.\n"
             "Pakai tools/rebuild_unique_strings.py untuk menambah string baru." % OUT)

cnt = Counter()
first_seen = {}
with open(SRC, encoding="utf-8") as f:
    for i, line in enumerate(f):
        if not line.strip():
            continue
        d = json.loads(line)
        if d.get("deleted"):            # tombstone dari *_diff, tidak ada teks
            continue
        v = d["v"]
        cnt[v] += 1
        if v not in first_seen:
            first_seen[v] = i

uniques = sorted(cnt.keys(), key=lambda v: (-cnt[v], first_seen[v]))

with open(OUT, "w", encoding="utf-8") as out:
    for idx, v in enumerate(uniques):
        out.write(json.dumps({"idx": idx, "freq": cnt[v], "v": v}, ensure_ascii=False) + "\n")

print("total unique:", len(uniques))
