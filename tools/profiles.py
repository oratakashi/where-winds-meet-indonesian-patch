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
(mis. Update-N setelah update game) WAJIB ditambahkan di tools/profiles.json —
progress.py menolak file locale yang tidak terdaftar. File JSON itu juga dibaca
binary Rust `wwm`, jadi kedua toolchain selalu memakai data fase yang sama.
"""
import json, os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


class Profile:
    def __init__(self, name, base, phases, files, dumps, status, coverage,
                 corpus_label, patched_dir, translated_out, bridge_from=None,
                 overrides=None, regions=None):
        self.name = name
        # Tabel "Phase status" dikelompokkan per region game: [(nama region, tag fase,
        # [(label fase, nama bagian)])]. Region yang selesai semua jadi satu baris; yang
        # belum ditampilkan per bagian. Fase tanpa region tampil sebagai barisnya sendiri.
        self.regions = regions or []
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


# ------------------------------------------------------------- profiles.json
# Data profil (fase, region, varian file, dump, path) ada di tools/profiles.json — satu
# sumber untuk script Python ini DAN binary Rust `wwm` (rust/). Fase/blok Update-N baru
# ditambahkan di sana, bukan di sini.
#
# Riwayat fase global: Phase 0–17 = korpus awal; Update-1 = Hidden Mountain; Update-2A–2C =
# string baru game v2.3 (2026-10-15) per 30.000; Update-2D = hotfix v2.3 (2026-10-17, hanya
# _diff); Update-3A–3G = game v2.4 "Across Autumn Waters" (2026-11-13), 197.743 string per
# 30.000. CN: korpus 159.176 teks khusus CN (client 2026-10-20) per 30.000 -> Phase 1..6.
PROFILES_JSON = os.path.join(ROOT, "tools", "profiles.json")


def _abs(rel):
    return None if rel is None else os.path.normpath(os.path.join(ROOT, rel))


def _load_profiles():
    with open(PROFILES_JSON, encoding="utf-8") as f:
        data = json.load(f)
    out = {}
    for name, d in data.items():
        if name.startswith("_"):
            continue
        out[name] = Profile(
            name=name, base=_abs(d["base"]),
            phases=[tuple(p) for p in d["phases"]],
            files=list(d["files"]),
            dumps=[(b, os.path.join(*dump.split("/"))) for b, dump in d["dumps"]],
            status=_abs(d["status"]),
            coverage=d["coverage"],
            corpus_label=d["corpus_label"],
            patched_dir=_abs(d["patched_dir"]),
            translated_out=_abs(d["translated_out"]),
            bridge_from=d.get("bridge_from"),
            overrides=_abs(d.get("overrides")),
            regions=[(r, tag, [tuple(p) for p in parts]) for r, tag, parts in d.get("regions", [])])
    return out


PROFILES = _load_profiles()
GLOBAL, CN = PROFILES["global"], PROFILES["cn"]
GLOBAL_PHASES, GLOBAL_REGIONS = GLOBAL.phases, GLOBAL.regions
CN_PHASES = CN.phases


def get(name):
    return PROFILES[name]


def add_argument(ap):
    ap.add_argument("--profile", choices=sorted(PROFILES), default="global",
                    help="global (WWM global, default) atau cn (WWM China)")
