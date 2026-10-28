# Developer Guide

How to run the toolkit yourself: dump, translate, validate and repack the game's
`translate_words_map_*` localization files. Players who only want to install the patch should
follow the [installation guide in the README](../README.md#player-installation-guide) instead.

- [Prerequisites](#prerequisites)
- [Quickstart](#quickstart)
- [Daily Translation Workflow (Cheat Sheet)](#daily-translation-workflow-cheat-sheet)
- [Handling Game Updates & File Variants](#handling-game-updates--file-variants)
- [China Client (`--profile cn`)](#china-client---profile-cn)
- [Repository Structure](#repository-structure)

Related: [Technical Overview](Technical-Overview.md) · [Translation Guidelines](Translation-Guidelines.md) ·
[Contributing](Contributing.md) · [Full format spec](../Obsidian-Vault/knowladge/Format-Spec.md)

---

## Prerequisites

- **Python 3.8+** and **pip**

```bash
git clone git@github.com:oratakashi/where-winds-meet-indonesian-patch.git
cd where-winds-meet-indonesian-patch
pip install -r requirements.txt
```

The only third-party dependency is [`zstandard`](https://pypi.org/project/zstandard/).

---

## Quickstart

```
translate_words_map_en  ──dump──>  strings.jsonl  ──edit / translate──>  strings.jsonl
                                                                               │
translate_words_map_en  <──patch───────────────────────────────────────────────┘
```

### 1. Locate the game files

- **Steam**
  ```
  <SteamLibrary>\steamapps\common\Where Winds Meet\Package\HD\oversea\locale\translate_words_map_en
  ```
- **Official NetEase launcher** (`<variant>` is the downloaded game folder, e.g. `wwm_lite`)
  ```
  <InstallDir>\<variant>\Package\HD\oversea\locale\translate_words_map_en
  <InstallDir>\<variant>\LocalData\Patch\HD\oversea\locale\
  ```

> [!WARNING]
> **Always back up before modifying game files.** Keep an untouched copy of
> `translate_words_map_en` somewhere safe. If you previously used GearUP Booster, its automatic
> backups (`.gubackup` extension, same folder) also hold the pristine English text.

### 2. Inspect the binary header

```bash
python wwm_locmap.py info translate_words_map_en
```

```text
version        : 1
blocks         : 3230 (1 index + 3229 shard)
entries (index): 826388
entries parsed : 826388
```

If `entries parsed` matches `entries (index)`, the container is valid. For incremental `*_diff`
files the counts intentionally diverge because their shards are sparse (see
[Format-Spec §4.5](../Obsidian-Vault/knowladge/Format-Spec.md#45-the-_diff-variant-incremental-file-between-game-updates)).

### 3. Dump to JSON Lines

```bash
python wwm_locmap.py dump translate_words_map_en strings.jsonl
```

Each line is one localization entry:

```json
{"b": 1, "s": 3, "h": "c5cadbb857eee8b4", "v": "The leaf?"}
{"b": 1, "s": 4, "h": "b6b9b1230c20a990", "v": "Velvet Shade Guest"}
```

| Field | Type | Description |
|---|---|---|
| `b` | integer | Shard block index (1-based) |
| `s` | integer | Slot index inside the shard |
| `h` | string (hex) | 64-bit `keyHash` (reference only — **do not edit**) |
| `v` | string | Display string rendered in-game |
| `deleted` | bool | Only in `*_diff` files: a key removed since the base version (no `v`). Never turn it back into text. |

### 4. Translate & edit

Edit the `v` field only. Follow the [Translation Guidelines](Translation-Guidelines.md) so engine
format tags stay intact.

> [!TIP]
> `(b, s)` uniquely identifies a slot, and **sparse overlays are supported**: you can delete
> unchanged lines from the JSONL. `patch` carries every entry not in the file over from the source
> binary unchanged.

### 5. Run the QA check

```bash
python tools/qa_check.py strings_original.jsonl strings_translated.jsonl --report qa_report.jsonl
```

```text
inspected   : 963050 entries
PROMPT_LEAK :      0  (0.000%)
MARKUP      :      0  (0.000%)
EMPTY       :      0  (0.000%)
```

The script exits with code `1` when it finds defects, so it works in CI and pre-commit hooks.

### 6. Repack into a binary

```bash
python wwm_locmap.py patch translate_words_map_en strings.jsonl translate_words_map_en.new
```

- `--level N` sets the zstd compression level (default `19`). Use `10`–`12` for fast iteration,
  `19` for distribution builds.
- `patch` refuses a JSONL whose `h` values don't match the target slot (i.e. dumped from a
  different game version) — re-dump after every game update.
- Copy the result into the game's locale folder as `translate_words_map_en` and run the game with
  **Game Language = English**.

---

## Daily Translation Workflow (Cheat Sheet)

The translation itself lives in `translation_work/unique_strings.jsonl` (source text, one row per
unique string) and `locale/phase*.jsonl` / `locale/update*.jsonl` (translations by `idx`).

```bash
# 0. Validate the per-idx translation files (tokens, empty values, idx gaps/duplicates)
python tools/qa_check.py --locale "locale/*.jsonl"

# 1. Expand the translation dictionary against the original dump
python tools/expand_locale.py

# 2. QA the expanded result (prompt leaks, broken markup, empty values)
python tools/qa_check.py strings.jsonl strings.translated.jsonl --report qa_report.jsonl

# 3. If QA passes, repack into a distribution binary
python wwm_locmap.py patch translate_words_map_en strings.translated.jsonl translate_words_map_en.id

# 4. Refresh the progress tables in Obsidian-Vault/progress/Current-Status.md
python tools/progress.py --write
```

Notes:
- Step 1 expects `strings.jsonl` (from `wwm_locmap.py dump`) in the repo root.
- If step 2 exits with `1`, fix the strings listed in `qa_report.jsonl` before repacking.
- To patch every file variant (base, `_diff`, `_mobile`) in one pass, use
  `python tools/patch_all.py --outdir patched`.
- Session workflow, terminology and tone rules live in the Obsidian vault:
  [`Current-Status.md`](../Obsidian-Vault/progress/Current-Status.md),
  [`Resume-Procedure.md`](../Obsidian-Vault/progress/Resume-Procedure.md) and
  [`Quick-Reference.md`](../Obsidian-Vault/knowladge/Quick-Reference.md).

---

## Handling Game Updates & File Variants

Game updates can ship several companion files, all using the same container and shard codec:

| File | What it is |
|---|---|
| `translate_words_map_en` | Base table |
| `translate_words_map_en_diff` | Sparse delta table between major releases (contains tombstones) |
| `translate_words_map_en_mobile` | Currently byte-identical to the base table; patched independently anyway |
| `translate_words_map_en__small` / `__small_diff` | Separate isolated table (+ delta) — shipped up to v2.2, **removed in v2.3** |

After an update:

```bash
# Dump the updated files
python wwm_locmap.py dump translate_words_map_en strings.jsonl
python wwm_locmap.py dump translate_words_map_en_diff strings_diff.jsonl

# Append new unique strings WITHOUT renumbering existing idx values
python tools/rebuild_unique_strings.py

# Apply the translations to every variant at once
python tools/patch_all.py --outdir patched
```

> [!CAUTION]
> Never re-run `translation_work/build_unique.py` — it renumbers every `idx` and silently breaks
> all `locale/*.jsonl` progress. Use `rebuild_unique_strings.py`, which only appends.

A new `locale/updateN.jsonl` must also be registered in `PHASES` in `tools/progress.py`. See
[Format-Spec §4.6](../Obsidian-Vault/knowladge/Format-Spec.md) for the `__small` findings and
[`CLAUDE.md`](../CLAUDE.md) for helper-script internals.

---

## China Client (`--profile cn`)

Every tool takes `--profile {global,cn}` (default `global`). The `cn` profile translates
**WWM China (燕云十六声)** from Mandarin. CN files use **container v2** (zstd dictionary encrypted
with a keystream), which `wwm_locmap.py` handles transparently once `cn/work/keystream_v2.bin`
is present. Lines shared with the global game reuse the global translation through
`tools/bridge.py`, so `cn/translation_work/unique_strings.jsonl` holds only CN-only text.

```bash
python tools/rebuild_unique_strings.py --profile cn
python tools/qa_check.py --profile cn --locale "cn/locale/*.jsonl"
python tools/progress.py --profile cn --write
python tools/patch_all.py --profile cn
```

Full setup (keystream decryption, dumps, name/term helpers) is in [`CLAUDE.md`](../CLAUDE.md#wwm-china---profile-cn)
and the CN vault ([`CN-Current-Status.md`](../Obsidian-Vault/cn/progress/CN-Current-Status.md)).

---

## Repository Structure

```text
.
├── wwm_locmap.py                    # Container codec + CLI: info / dump / patch (v1 and v2)
├── tools/
│   ├── qa_check.py                  # QA: prompt leaks, broken markup, empty values, idx gaps
│   ├── expand_locale.py             # Expands the per-idx dictionary into a patch JSONL for one dump
│   ├── rebuild_unique_strings.py    # Appends new strings after a game update (keeps existing idx)
│   ├── patch_all.py                 # Patches every translate_words_map_* variant in one pass
│   ├── progress.py                  # Translation progress tables (--write / --check)
│   ├── profiles.py                  # Per-profile paths/variants (global, cn)
│   └── bridge.py                    # Global → CN keyHash/text bridge
├── locale/                          # Global translations (phase*.jsonl, update*.jsonl)
├── translation_work/                # unique_strings.jsonl — deduplicated source text
├── cn/                              # China client: binaries, locale/, tools/, overrides.jsonl
├── Obsidian-Vault/                  # Documentation, glossary, progress and session logs
├── docs/                            # Developer, technical and contributor docs (this folder)
├── .github/workflows/               # QA + rolling release builds (global and CN)
├── .githooks/pre-commit             # QA + progress check on translation commits
├── requirements.txt                 # zstandard
└── LICENSE                          # GNU GPL v3
```
