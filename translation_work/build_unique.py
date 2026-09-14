import json
from collections import Counter

cnt = Counter()
first_seen = {}
with open("../strings.jsonl", encoding="utf-8") as f:
    for i, line in enumerate(f):
        d = json.loads(line)
        v = d["v"]
        cnt[v] += 1
        if v not in first_seen:
            first_seen[v] = i

uniques = sorted(cnt.keys(), key=lambda v: (-cnt[v], first_seen[v]))

with open("unique_strings.jsonl", "w", encoding="utf-8") as out:
    for idx, v in enumerate(uniques):
        out.write(json.dumps({"idx": idx, "freq": cnt[v], "v": v}, ensure_ascii=False) + "\n")

print("total unique:", len(uniques))
