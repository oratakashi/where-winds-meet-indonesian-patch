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

Related: [Rust Toolchain](Rust-Toolchain.md) · [Technical Overview](Technical-Overview.md) · [Translation Guidelines](Translation-Guidelines.md) ·
[Contributing](Contributing.md) · [Full format spec](../Obsidian-Vault/knowladge/Format-Spec.md)

---

## Prerequisites

The toolkit exists twice, with identical arguments and output: a **Rust** binary `wwm` (fast,
the default) and the original **Python** scripts (the failsafe). You need at least one of them;
having both is best.

- **Rust (stable)** via [rustup](https://rustup.rs) — builds `rust/target/release/wwm`
- **Python 3.8+** and **pip** — the only third-party dependency is
  [`zstandard`](https://pypi.org/project/zstandard/)
- A POSIX `sh` (Linux/macOS, or Git Bash on Windows) for the `tools/wwm.sh` dispatcher — from
  PowerShell/cmd use `tools\wwm.cmd <subcommand>` instead (it runs Git for Windows' `sh.exe`)

```bash
git clone git@github.com:oratakashi/where-winds-meet-indonesian-patch.git
cd where-winds-meet-indonesian-patch
cargo build --release --manifest-path rust/Cargo.toml   # Rust toolchain
pip install -r requirements.txt                         # Python failsafe
sh tools/wwm.sh toolchain                               # -> "rust <path>" or "python"
```

Every command below goes through `sh tools/wwm.sh <subcommand>`, which runs the Rust binary when
it is built and the matching Python script otherwise (`WWM_TOOLCHAIN=python` forces Python). The
mapping, timings and CI switch are in [Rust Toolchain](Rust-Toolchain.md).

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
sh tools/wwm.sh info translate_words_map_en
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
sh tools/wwm.sh dump translate_words_map_en strings.jsonl
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
sh tools/wwm.sh qa strings_original.jsonl strings_translated.jsonl --report qa_report.jsonl
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
sh tools/wwm.sh patch translate_words_map_en strings.jsonl translate_words_map_en.new
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
sh tools/wwm.sh qa --locale "locale/*.jsonl"

# 1. Expand the translation dictionary against the original dump
sh tools/wwm.sh expand

# 2. QA the expanded result (prompt leaks, broken markup, empty values)
sh tools/wwm.sh qa strings.jsonl strings.translated.jsonl --report qa_report.jsonl

# 3. If QA passes, repack into a distribution binary
sh tools/wwm.sh patch translate_words_map_en strings.translated.jsonl translate_words_map_en.id

# 4. Refresh the progress tables in Obsidian-Vault/progress/Current-Status.md
sh tools/wwm.sh progress --write
```

Notes:
- Step 1 expects `strings.jsonl` (from `sh tools/wwm.sh dump`) in the repo root. Step 4 does not
  need it on the Rust toolchain (the binary is decoded directly); the Python `progress.py` does.
- If step 2 exits with `1`, fix the strings listed in `qa_report.jsonl` before repacking.
- To patch every file variant (base, `_diff`, `_mobile`) in one pass, use
  `sh tools/wwm.sh patch-all --outdir patched`.
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
sh tools/wwm.sh dump translate_words_map_en strings.jsonl
sh tools/wwm.sh dump translate_words_map_en_diff strings_diff.jsonl

# Append new unique strings WITHOUT renumbering existing idx values
sh tools/wwm.sh rebuild-unique

# Apply the translations to every variant at once
sh tools/wwm.sh patch-all --outdir patched
```

> [!CAUTION]
> Never re-run `translation_work/build_unique.py` — it renumbers every `idx` and silently breaks
> all `locale/*.jsonl` progress. Use `rebuild-unique`, which only appends.

A new `locale/updateN.jsonl` must also be registered in the profile's `phases` in
`tools/profiles.json` (read by both toolchains). See
[Format-Spec §4.6](../Obsidian-Vault/knowladge/Format-Spec.md) for the `__small` findings and
[`CLAUDE.md`](../CLAUDE.md) for helper-script internals.

---

## China Client (`--profile cn`)

> [!WARNING]
> **Maintenance ended (2026-11):** the China profile is no longer maintained — few Indonesian CN
> players, limited scope (no cutscene subtitles) and CN updates too fast for the phases to keep
> up. The `cn-latest` release is frozen and `release-patch-cn.yml` is manual-only. Don't start new
> CN translation sessions or CN game-update migrations unless the owner says maintenance resumes.
> The tooling below is kept for reference.

Every tool takes `--profile {global,cn}` (default `global`). The `cn` profile translates
**WWM China (燕云十六声)** from Mandarin. CN files use **container v2** (zstd dictionary encrypted
with a keystream), which both codecs handle transparently once `cn/work/keystream_v2.bin`
is present. Lines shared with the global game reuse the global translation through
`tools/bridge.py`, so `cn/translation_work/unique_strings.jsonl` holds only CN-only text.

```bash
sh tools/wwm.sh rebuild-unique --profile cn
sh tools/wwm.sh qa --profile cn --locale "cn/locale/*.jsonl"
sh tools/wwm.sh progress --profile cn --write
sh tools/wwm.sh patch-all --profile cn
```

Full setup (keystream decryption, dumps, name/term helpers) is in [`CLAUDE.md`](../CLAUDE.md#wwm-china---profile-cn)
and the CN vault ([`CN-Current-Status.md`](../Obsidian-Vault/cn/progress/CN-Current-Status.md)).

---

## WWM Patch Guard (`app/`)

The tray app players run to have the patch reinstalled automatically. It is independent of the
toolkit (and of the `rust/` workspace): it only compares file hashes with the `manifest.json`
published in each patch release (`sh tools/wwm.sh manifest`) and copies files.

A native Rust app on [GPUI](https://gpui.rs) since v0.4 (no WebView2, no JS/WASM build step).

```bash
cargo install cargo-packager --locked    # once, for the installer only

cd app
cargo test -p wwm-patch-guard --lib      # guard rules (core/guard.rs) + folder detection
cargo run                                # debug build with a console
cargo packager --release --formats nsis  # release exe + NSIS installer
```

Installed games are found by `locations.rs`: Steam through the registry (`SteamPath`) →
`steamapps\libraryfolders.vdf` → `appmanifest_3564740.acf`, the NetEase launchers through their
uninstall entries (`wwm_launcher_uninstall`, `yysls_launcher_uninstall`). A Steam install is
guarded in `Package\HD\oversea\locale` only. The guard itself lives in `app/src/core/` (no UI
code); `shell/` is the tray, single instance and autostart; `fluent/` the WinUI-style controls;
the window is a navigation pane plus one file per page in `pages/` (Home, Settings, History). The
history is kept in `history.json`.

Since v0.5 it updates itself (`core/update.rs`): it downloads a newer installer from the `latest`
release in the background, verifies it, and installs it when the user clicks **Restart sekarang**,
quits from the tray, or starts again. Every release file it trusts is signed with minisign:
`manifest.json.sig` (unsigned manifests are rejected), `wwm-patch-guard-setup.exe.sig` and
`patch-guard-update.json(.sig)`. The public key is `app/signing/patch-guard.pub`. CI signs with the
secrets `PATCH_GUARD_SIGN_KEY` / `PATCH_GUARD_SIGN_KEY_PASSWORD` (the latter optional, unset when the key has no password).

To try it without touching a real install, set `WWM_PATCH_GUARD_BASE_URL` to a local server that
serves `<url>/latest/manifest.json` + `manifest.json.sig` plus the release zip it names
(`archive.name`), `WWM_PATCH_GUARD_DATA_DIR` to a scratch folder, and (debug builds only)
`WWM_PATCH_GUARD_PUBKEY` to a test key's public key. Releasing: bump the version in
`app/Cargo.toml` and push to main. When `app/` changed, `release-patch.yml` rebuilds the
installer, signs it, writes `patch-guard-update.json` and attaches them to the `latest` release;
installed copies pick it up within 6 hours (`app-ci.yml` only tests PRs).

The full write-up (folders guarded, release assets, signing and key rotation, the self-update
flow, the end-to-end test recipe) is in
[`Obsidian-Vault/knowladge/Patch-Guard.md`](../Obsidian-Vault/knowladge/Patch-Guard.md).

## Repository Structure

```text
.
├── rust/                            # Rust toolchain: the `wwm` binary (default; see docs/Rust-Toolchain.md)
│   └── crates/                      # wwm-locmap (codec), wwm-tools (one module per script), wwm-cli (`wwm`)
├── wwm_locmap.py                    # Python codec + CLI: info / dump / patch (v1 and v2) — failsafe
├── tools/
│   ├── wwm.sh                       # Dispatcher: `sh tools/wwm.sh <subcommand>` -> Rust or Python
│   ├── parity_check.sh              # Runs both toolchains on the repo data, fails on any difference
│   ├── profiles.json                # Per-profile phases/regions/variants/paths (read by both toolchains)
│   ├── qa_check.py                  # QA: prompt leaks, broken markup, empty values, idx gaps
│   ├── expand_locale.py             # Expands the per-idx dictionary into a patch JSONL for one dump
│   ├── rebuild_unique_strings.py    # Appends new strings after a game update (keeps existing idx)
│   ├── patch_all.py                 # Patches every translate_words_map_* variant in one pass
│   ├── make_manifest.py             # manifest.json (original + patched sha256, release zip hash) for a patch_all.py output
│   ├── progress.py                  # Translation progress tables (--write / --check)
│   ├── profiles.py                  # Loads profiles.json for the Python scripts
│   └── bridge.py                    # Global → CN keyHash/text bridge
├── app/                             # WWM Patch Guard: Windows tray app that reinstalls the patch (Rust, GPUI)
├── locale/                          # Global translations (phase*.jsonl, update*.jsonl)
├── translation_work/                # unique_strings.jsonl — deduplicated source text
├── cn/                              # China client: binaries, locale/, tools/, overrides.jsonl
├── Obsidian-Vault/                  # Documentation, glossary, progress and session logs
├── docs/                            # Developer, technical and contributor docs (this folder)
├── .github/workflows/               # QA + rolling release builds (global and CN), rust-ci (tests + parity)
├── .github/actions/wwm-toolchain/   # CI: pick Rust or Python, cache the `wwm` binary, Python fallback
├── .githooks/pre-commit             # QA + progress check on translation commits
├── requirements.txt                 # zstandard (Python failsafe)
└── LICENSE                          # GNU GPL v3
```
