#!/usr/bin/env python3
"""
profiles.py — satu tempat untuk semua path/daftar file per "profil" terjemahan.

  global  WWM global (Steam / launcher NetEase), sumber teks Inggris, container v1.
          Data di root repo: translation_work/, locale/, translate_words_map_en*.
  cn      WWM China (燕云十六声), sumber teks Mandarin, container v2 (kamus zstd
          terenkripsi, lihat Obsidian-Vault/knowladge/Format-Spec.md §8).
          Data di cn/: cn/translation_work/, cn/locale/, cn/translate_words_map_zh_cn*;
          dump & hasil patch di cn/work/ (gitignored). Entri yang key-nya juga ada di
          global memakai terjemahan global lewat keyHash (tools/bridge.py); cn/ hanya
          menerjemahkan selisihnya dari teks Mandarin.

Semua tools (rebuild_unique_strings, expand_locale, patch_all, qa_check,
progress) menerima `--profile {global,cn}`; default `global`, jadi perilaku lama
tidak berubah.

Fase: (label, file di locale_dir, idx awal, idx akhir inklusif). Fase/blok baru
(mis. Update-N setelah update game) WAJIB ditambahkan di sini — progress.py
menolak file locale yang tidak terdaftar.
"""
import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


class Profile:
    def __init__(self, name, base, phases, files, dumps, status, coverage,
                 corpus_label, patched_dir, translated_out, bridge_from=None,
                 overrides=None):
        self.name = name
        self.base = base                                    # folder binary + data
        self.unique = os.path.join(base, "translation_work", "unique_strings.jsonl")
        self.locale_dir = os.path.join(base, "locale")
        self.locale_patterns = ("phase*.jsonl", "update*.jsonl")
        self.phases = phases
        self.files = files                                  # varian yang dipatch
        self.dumps = dumps                                  # [(binary, dump)] relatif base
        self.status = status                                # Current-Status.md profil ini
        # "dump": coverage dihitung dari dump (global; CI men-dump binary dulu).
        # "freq": dari kolom freq unique_strings.jsonl — tanpa dump, jadi CI tidak
        #         butuh keystream kamus CN (yang gitignored).
        self.coverage = coverage
        self.corpus_label = corpus_label
        self.patched_dir = patched_dir
        self.translated_out = translated_out
        # "global": entri yang key-nya ada di global memakai terjemahan global
        # (tools/bridge.py); unique_strings profil ini hanya berisi selisihnya.
        self.bridge_from = bridge_from
        # JSONL {"h", "src", "v"}: terjemahan per key yang menang atas semua sumber lain
        # (mis. teks warisan global yang tidak muat di layout CN). `src` = teks sumber
        # saat override ditulis; patch melewati override yang src-nya sudah berubah.
        self.overrides = overrides

    def path(self, rel):
        return os.path.join(self.base, rel)

    def rel(self, path):
        """Path relatif ke root repo dengan '/' (untuk teks di dokumen)."""
        return os.path.relpath(path, ROOT).replace(os.sep, "/")


# ------------------------------------------------------------------ global
GLOBAL_PHASES = [("0", "phase0.jsonl", 0, 799),
                 ("1", "phase1.jsonl", 800, 1999),
                 ("2", "phase2.jsonl", 2000, 4999),
                 ("3", "phase3.jsonl", 5000, 9999),
                 ("4", "phase4.jsonl", 10000, 19999),
                 ("5", "phase5.jsonl", 20000, 49999),
                 ("6", "phase6.jsonl", 50000, 99999)]
GLOBAL_PHASES += [(str(n), "phase%d.jsonl" % n, 100000 + (n - 7) * 30000, 129999 + (n - 7) * 30000)
                  for n in range(7, 17)]
GLOBAL_PHASES += [("17", "phase17.jsonl", 400000, 429886),
                  ("Update-1", "update1.jsonl", 429887, 461703)]
# Update-2 = string baru dari game v2.3 (2026-10-15), dipecah per 30.000 string.
GLOBAL_PHASES += [("Update-2A", "update2a.jsonl", 461704, 491703),
                  ("Update-2B", "update2b.jsonl", 491704, 521703),
                  ("Update-2C", "update2c.jsonl", 521704, 534256)]
# Update-2D = string baru dari hotfix v2.3 (2026-10-17) — hanya _diff yang berubah.
GLOBAL_PHASES += [("Update-2D", "update2d.jsonl", 534257, 535918)]

GLOBAL = Profile(
    name="global", base=ROOT, phases=GLOBAL_PHASES,
    files=["translate_words_map_en",
           "translate_words_map_en_diff",
           "translate_words_map_en_mobile"],
    # _diff ikut dihitung sejak hotfix v2.3 (string barunya hanya ada di _diff).
    # (__small / __small_diff tidak dikirim lagi sejak game v2.3.)
    dumps=[("translate_words_map_en", "strings.jsonl"),
           ("translate_words_map_en_diff", "strings_diff.jsonl")],
    status=os.path.join(ROOT, "Obsidian-Vault", "progress", "Current-Status.md"),
    coverage="dump",
    corpus_label="original corpus (Phases 0–17)",
    patched_dir=os.path.join(ROOT, "patched"),
    translated_out=os.path.join(ROOT, "strings.translated.jsonl"))

# ---------------------------------------------------------------------- cn
CN_BASE = os.path.join(ROOT, "cn")
# Korpus awal CN (client 2026-10-20): 159.176 teks unik yang belum pernah diterjemahkan
# lewat global (key tidak ada di global DAN teksnya tidak muncul di key bersama — lihat
# tools/bridge.py), urut frekuensi menurun, dipecah rata per 30.000 string -> Phase 1..6.
CN_CORPUS = 159176
CN_PHASES = [(str(n + 1), "phase%d.jsonl" % (n + 1), n * 30000,
              min((n + 1) * 30000, CN_CORPUS) - 1)
             for n in range((CN_CORPUS + 29999) // 30000)]

CN = Profile(
    name="cn", base=CN_BASE, phases=CN_PHASES,
    files=["translate_words_map_zh_cn",
           "translate_words_map_zh_cn__small"],
    dumps=[("translate_words_map_zh_cn", os.path.join("work", "strings_zh_cn.jsonl")),
           ("translate_words_map_zh_cn__small", os.path.join("work", "strings_zh_cn__small.jsonl"))],
    status=os.path.join(ROOT, "Obsidian-Vault", "cn", "progress", "CN-Current-Status.md"),
    coverage="freq",
    corpus_label="CN-only corpus (Phases 1–%d)" % len(CN_PHASES),
    patched_dir=os.path.join(CN_BASE, "work", "patched"),
    translated_out=os.path.join(CN_BASE, "work", "strings.translated.jsonl"),
    bridge_from="global",
    overrides=os.path.join(CN_BASE, "overrides.jsonl"))

PROFILES = {"global": GLOBAL, "cn": CN}


def get(name):
    return PROFILES[name]


def add_argument(ap):
    ap.add_argument("--profile", choices=sorted(PROFILES), default="global",
                    help="global (WWM global, default) atau cn (WWM China)")
