#!/usr/bin/env python3
"""
bridge.py — jembatan dari profil global ke profil cn.

Key `translate_words_map_*` adalah ID per entri (bukan hash dari teks), dan WWM
China memakai ruang key yang sama dengan WWM global: 94,5% key global ada di
translate_words_map_zh_cn dengan arti yang sama (diukur 2026-10-20, lihat
Obsidian-Vault/knowladge/Format-Spec.md §8). Dua lapis, dicoba berurutan saat patch:

  1. per key   keyHash CN ada di global -> terjemahan Indonesia global untuk key itu
               (keyHash -> teks Inggris di strings.jsonl/strings_diff.jsonl -> dictionary global)
  2. per teks  teks Mandarin yang sama muncul di key bersama -> terjemahan Indonesia
               yang paling sering dipakai untuk teks itu (menutup mis. __small, yang
               key-nya sudah tidak ada di global sejak v2.3)

Di antara keduanya patch_all/expand_locale memakai dictionary CN sendiri. Profil cn
hanya menerjemahkan SELISIHNYA: rebuild_unique_strings.py --profile cn melewati
entri yang key-nya ada di global DAN teks yang muncul di key bersama, sehingga
daftar kerja CN tidak bergeser saat global maju.

Butuh dump global (strings.jsonl + strings_diff.jsonl) dan dump profil cn
(cn/work/strings_zh_cn*.jsonl) — semuanya tidak ter-track, buat dengan wwm_locmap.py dump.
"""
import json, os
from collections import Counter, defaultdict

import profiles


def _iter_dump(path, hint):
    if not os.path.exists(path):
        raise SystemExit("%s tidak ada — jembatan butuh dump-nya: %s" % (path, hint))
    with open(path, encoding="utf-8") as f:
        for line in f:
            if line.strip():
                yield json.loads(line)


def global_keys():
    """keyHash (hex) -> teks Inggris untuk setiap key hidup di dump global;
    _diff menimpa base, tombstone _diff menghapus key."""
    G = profiles.GLOBAL
    keys = {}
    for name, dump in G.dumps:                  # base dulu, lalu _diff
        for d in _iter_dump(G.path(dump), "python wwm_locmap.py dump %s %s" % (name, dump)):
            if d.get("deleted"):
                keys.pop(d["h"], None)
            else:
                keys[d["h"]] = d["v"]
    return keys


def shared_texts(P, gkeys):
    """Teks profil P -> Counter(teks Inggris global) atas entri yang key-nya ada di global."""
    out = defaultdict(Counter)
    for name, dump in P.dumps:
        for d in _iter_dump(P.path(dump), "python wwm_locmap.py dump %s %s"
                            % (P.rel(P.path(name)), P.rel(P.path(dump)))):
            e = gkeys.get(d["h"])
            if e is not None and "v" in d:
                out[d["v"]][e] += 1
    return out


def load_bridge(profile):
    """(per_key, per_text): keyHash hex -> terjemahan, teks sumber -> terjemahan.
    Dua dict kosong kalau profil ini tidak memakai jembatan."""
    P = profiles.get(profile)
    if P.bridge_from != "global":
        return {}, {}
    from expand_locale import load_translated_map      # hindari import melingkar
    translated, _ = load_translated_map("global")
    gkeys = global_keys()
    per_key = {h: translated[e] for h, e in gkeys.items() if e in translated}
    per_text = {}
    for text, ens in shared_texts(P, gkeys).items():
        ids = Counter()
        for e, n in ens.items():
            if e in translated:
                ids[translated[e]] += n
        if ids:
            per_text[text] = ids.most_common(1)[0][0]
    return per_key, per_text
