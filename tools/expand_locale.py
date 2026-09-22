#!/usr/bin/env python3
"""
expand_locale.py — gabungkan translation_work/unique_strings.jsonl + semua
locale/phase*.jsonl DAN locale/update*.jsonl (per-idx-unik) menjadi patch per-baris
di atas strings.jsonl (format {"b","s","h","v"} yang dipakai wwm_locmap.py patch).

    python tools/expand_locale.py [--strings strings.jsonl] [--out strings.translated.jsonl]

Entri yang belum diterjemahkan dilewati (tidak ditulis) — patch bersifat parsial,
sesuai dukungan wwm_locmap.py patch (baris yang tidak ada di jsonl dipertahankan
dari file source).
"""
import argparse, glob, json, os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def load_translated_map():
    idx_to_src = {}
    with open(os.path.join(ROOT, "translation_work", "unique_strings.jsonl"), encoding="utf-8") as f:
        for line in f:
            d = json.loads(line)
            idx_to_src[d["idx"]] = d["v"]

    src_to_translated = {}
    phase_files = sorted(
        glob.glob(os.path.join(ROOT, "locale", "phase*.jsonl"))
        + glob.glob(os.path.join(ROOT, "locale", "update*.jsonl"))
    )
    for path in phase_files:
        with open(path, encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                d = json.loads(line)
                src = idx_to_src[d["idx"]]
                src_to_translated[src] = d["v"]
    return src_to_translated, phase_files


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--strings", default=os.path.join(ROOT, "strings.jsonl"))
    ap.add_argument("--out", default=os.path.join(ROOT, "strings.translated.jsonl"))
    a = ap.parse_args()

    src_to_translated, phase_files = load_translated_map()
    print("string unik diterjemahkan:", len(src_to_translated))
    print("dari file fase:", [os.path.basename(p) for p in phase_files])

    written = 0
    with open(a.strings, encoding="utf-8") as fin, \
         open(a.out, "w", encoding="utf-8") as fout:
        for line in fin:
            line = line.strip()
            if not line:
                continue
            d = json.loads(line)
            if d.get("deleted"):            # tombstone dari *_diff, tidak ada yang diterjemahkan
                continue
            t = src_to_translated.get(d["v"])
            if t is None:
                continue
            fout.write(json.dumps({"b": d["b"], "s": d["s"], "h": d["h"], "v": t},
                                   ensure_ascii=False) + "\n")
            written += 1
    print("baris ditulis ke %s: %d" % (a.out, written))


if __name__ == "__main__":
    main()
