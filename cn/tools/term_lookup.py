#!/usr/bin/env python3
"""
term_lookup.py — cari bagaimana teks/istilah Mandarin sudah diterjemahkan di
profil global, lewat key yang sama di kedua versi game (jembatan keyHash).

    python cn/tools/term_lookup.py 止戈定音技 告辞            # teks persis
    python cn/tools/term_lookup.py --contains 燕云 --limit 5  # teks yang memuat istilah

Untuk tiap kueri: pasangan (Inggris global -> Indonesia) yang paling sering
dipakai untuk teks Mandarin itu. Pakai untuk menjaga istilah CN konsisten dengan
entri yang diisi jembatan (lihat Obsidian-Vault/cn/knowladge/CN-Quick-Reference.md).
Banyak kueri sekaligus dalam satu panggilan — memuat indeks butuh ~20 detik.

Butuh: cn/work/strings_zh_cn.jsonl (dump CN) + strings.jsonl/strings_diff.jsonl (dump global).
"""
import argparse, json, os, sys
from collections import Counter, defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, os.path.join(ROOT, "tools"))
import profiles
from bridge import global_keys
from expand_locale import load_translated_map


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("terms", nargs="+")
    ap.add_argument("--contains", action="store_true",
                    help="cari teks Mandarin yang MEMUAT istilah (default: teks persis)")
    ap.add_argument("--limit", type=int, default=8)
    a = ap.parse_args()
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8")

    CN = profiles.CN
    dump = CN.path(CN.dumps[0][1])
    if not os.path.exists(dump):
        sys.exit("%s tidak ada — python wwm_locmap.py dump cn/%s %s"
                 % (dump, CN.dumps[0][0], CN.rel(dump)))
    en = global_keys()
    idn, _ = load_translated_map("global")
    by_text = defaultdict(Counter)              # teks Mandarin -> Counter((en, id))
    with open(dump, encoding="utf-8") as f:
        for line in f:
            d = json.loads(line)
            e = en.get(d["h"])
            if e is not None and "v" in d:
                by_text[d["v"]][(e, idn.get(e))] += 1

    for term in a.terms:
        print("== %s" % term)
        if a.contains:
            hits = sorted((t for t in by_text if term in t), key=len)[:a.limit]
            if not hits:
                print("   (tidak ada di key bersama — istilah khusus CN)")
            for t in hits:
                (e, i), n = by_text[t].most_common(1)[0]
                print("   %s\n     EN: %s\n     ID: %s" % (t, e, i or "(belum diterjemahkan)"))
        else:
            c = by_text.get(term)
            if not c:
                print("   (tidak ada di key bersama — coba --contains)")
                continue
            for (e, i), n in c.most_common(a.limit):
                print("   %5dx  EN: %s\n          ID: %s" % (n, e, i or "(belum diterjemahkan)"))


if __name__ == "__main__":
    main()
