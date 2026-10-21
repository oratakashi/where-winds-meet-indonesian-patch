#!/usr/bin/env python3
"""
expand_locale.py — gabungkan translation_work/unique_strings.jsonl + semua
locale/phase*.jsonl DAN locale/update*.jsonl (per-idx-unik) menjadi patch per-baris
di atas strings.jsonl (format {"b","s","h","v"} yang dipakai wwm_locmap.py patch).

    python tools/expand_locale.py [--strings strings.jsonl] [--out strings.translated.jsonl]
    python tools/expand_locale.py --profile cn      # cn/work/strings_zh_cn.jsonl -> cn/work/strings.translated.jsonl

Profil cn juga memasang terjemahan global lewat tools/bridge.py: key yang ada di
global dulu, lalu dictionary CN, lalu teks Mandarin yang sama di key bersama.

Entri yang belum diterjemahkan dilewati (tidak ditulis) — patch bersifat parsial,
sesuai dukungan wwm_locmap.py patch (baris yang tidak ada di jsonl dipertahankan
dari file source).
"""
import argparse, glob, json, os

import profiles


def load_translated_map(profile="global"):
    P = profiles.get(profile)
    idx_to_src = {}
    with open(P.unique, encoding="utf-8") as f:
        for line in f:
            if not line.strip():
                continue
            d = json.loads(line)
            idx_to_src[d["idx"]] = d["v"]

    src_to_translated, idx_file = {}, {}
    phase_files = sorted(p for pat in P.locale_patterns
                         for p in glob.glob(os.path.join(P.locale_dir, pat)))
    for path in phase_files:
        name = os.path.basename(path)
        with open(path, encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                d = json.loads(line)
                i = d["idx"]
                if i not in idx_to_src:
                    raise ValueError("%s: idx %d tidak ada di unique_strings.jsonl" % (name, i))
                if i in idx_file:       # jangan biarkan last-wins diam-diam
                    raise ValueError("idx %d diterjemahkan dua kali: %s dan %s"
                                     % (i, idx_file[i], name))
                idx_file[i] = name
                src_to_translated[idx_to_src[i]] = d["v"]
    return src_to_translated, phase_files


def load_overrides(profile="global"):
    """keyHash (hex) -> (teks sumber saat ditulis, terjemahan) dari overrides profil."""
    P = profiles.get(profile)
    out = {}
    if P.overrides and os.path.exists(P.overrides):
        with open(P.overrides, encoding="utf-8") as f:
            for ln, line in enumerate(f, 1):
                if not line.strip():
                    continue
                d = json.loads(line)
                if d["h"] in out:
                    raise ValueError("%s baris %d: key %s dua kali" % (P.overrides, ln, d["h"]))
                out[d["h"]] = (d["src"], d["v"])
    return out


def main():
    ap = argparse.ArgumentParser()
    profiles.add_argument(ap)
    ap.add_argument("--strings", help="default: dump pertama profil")
    ap.add_argument("--out", help="default: strings.translated.jsonl profil")
    a = ap.parse_args()
    P = profiles.get(a.profile)
    a.strings = a.strings or P.path(P.dumps[0][1])
    a.out = a.out or P.translated_out

    src_to_translated, phase_files = load_translated_map(a.profile)
    from bridge import load_bridge
    bridge, text_bridge = load_bridge(a.profile)
    overrides = load_overrides(a.profile)
    print("string unik diterjemahkan:", len(src_to_translated))
    if overrides:
        print("override per key:", len(overrides))
    if bridge:
        print("jembatan global: %d key + %d teks" % (len(bridge), len(text_bridge)))
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
            o = overrides.get(d["h"])
            if o and o[0] != d["v"]:
                print("PERINGATAN: override %s dilewati, teks sumber sudah berubah" % d["h"])
                o = None
            t = (o and o[1]) or (bridge.get(d["h"]) or src_to_translated.get(d["v"])
                                 or text_bridge.get(d["v"]))
            if t is None:
                continue
            fout.write(json.dumps({"b": d["b"], "s": d["s"], "h": d["h"], "v": t},
                                   ensure_ascii=False) + "\n")
            written += 1
    print("baris ditulis ke %s: %d" % (a.out, written))


if __name__ == "__main__":
    main()
