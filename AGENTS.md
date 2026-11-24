# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in this repository.

`CLAUDE.md` is the same document for Claude Code; this file adds two sections of its own
(scratch files, no system assessment).
**Change both files together** so they never drift apart again.

## What this is

A reverse-engineering toolkit for the `translate_words_map_*` localization file format used by
**Where Winds Meet** (NetEase / Everstone Studio, Messiah Engine), plus an in-progress
Indonesian translation built with it. Note the repo **does** track game-derived data: the
`translate_words_map_en*` binaries, `translation_work/unique_strings.jsonl`
(extracted source text) and `locale/*.jsonl` (translations). `.gitignore` lists some of these, but
they were committed before that and remain tracked — keeping them tracked is the owner's decision.
Don't add new extracted dumps (`strings_diff.jsonl`, `strings.translated.jsonl`, `patched/`, ...);
those stay ignored. `strings.jsonl` was tracked too until game v2.3, when it grew past GitHub's
100 MB limit (114 MB); it is now untracked and regenerated with
`sh tools/wwm.sh dump translate_words_map_en strings.jsonl`. Since the v2.3 hotfix (Update-2D)
`progress` also reports `_diff` coverage from `strings_diff.jsonl`
(`sh tools/wwm.sh dump translate_words_map_en_diff strings_diff.jsonl`). The Rust tools decode
the tracked binary in memory when a dump is missing; the Python `progress.py` exits with that
hint instead, so CI dumps both files only on its Python path.

## Scratch and temporary files

Keep the project free of scratch and temporary files. Create all temporary scripts,
intermediate dumps, QA reports, logs, and other disposable outputs outside the repository,
using the system temporary directory (for example, `%TEMP%` on Windows or `/tmp`).
This applies even to ignored files: do not create `scratch/`, `tmp/`, or other temporary
directories inside the project. Override tool output paths when their defaults would write
temporary files into the repository. Only intended project deliverables belong here.

## No system assessment or environment validation

Do not run system assessments, environment diagnostic checks, Python/tooling sanity checks, or dependency verifications. All environments and tools for this project were already running perfectly before Antigravity or Gemini was installed. Assume the environment is completely ready and proceed straight to the requested tasks.

## Toolchain: Rust `wwm` first, Python as failsafe

Every tool exists twice with the **same arguments, output and exit codes**: the Rust binary
`wwm` (workspace `rust/`, much faster — see `docs/Rust-Toolchain.md` for timings) and the
original Python scripts, which are kept as the failsafe and must not be deleted. Always call
them through the dispatcher **`sh tools/wwm.sh <subcommand> ...`**: it runs
`rust/target/release/wwm[.exe]` when it is built (rebuilding it with cargo when `rust/` sources
are newer) and the matching Python script otherwise. `WWM_TOOLCHAIN=python` forces Python,
`WWM_TOOLCHAIN=rust` forces Rust. From PowerShell/cmd (no `sh` on PATH) use `tools\wwm.cmd`
with the same arguments. One-time setup per machine:

```bash
pip install -r requirements.txt                       # Python fallback (only dependency: zstandard)
cargo build --release --manifest-path rust/Cargo.toml  # Rust binary (optional but recommended)
sh tools/wwm.sh toolchain                             # prints which toolchain will run
```

Profile data (phases, regions, file variants, dumps) lives in **`tools/profiles.json`**, read by
both `tools/profiles.py` and the Rust binary — a new phase / `Update-N` block is one edit there.

## Commands

`sh tools/wwm.sh <subcommand>` → Python fallback in brackets.

```bash
sh tools/wwm.sh info  <src>                      # [wwm_locmap.py info] header/entry counts, sanity-check parser
sh tools/wwm.sh dump  <src> <out.jsonl>          # [wwm_locmap.py dump] container -> one JSON object per line
sh tools/wwm.sh patch <src> <edits.jsonl> <out>  # [wwm_locmap.py patch] overlay edits.jsonl onto src
                                                 #   --level N zstd level (default 19; 10-12 for fast iteration)
                                                 #   refuses records whose `h` doesn't match the slot's keyHash

sh tools/wwm.sh qa <original.jsonl> <translated.jsonl> [--report qa.jsonl]
                                                 # [tools/qa_check.py] validate a translated JSONL vs the source;
                                                 # exit code 1 if findings exist (CI/pre-commit friendly)
sh tools/wwm.sh qa --locale "locale/*.jsonl"     # same checks on per-idx translation files vs.
                                                 # unique_strings.jsonl, plus idx gap/duplicate checks —
                                                 # the per-batch validator for translation sessions

sh tools/wwm.sh expand [--strings X.jsonl] [--out Y.jsonl]
                                                 # [tools/expand_locale.py] patch JSONL for ANY single dump by
                                                 # matching its `v` text against unique_strings.jsonl +
                                                 # locale/phase*.jsonl + locale/update*.jsonl (default strings.jsonl)

sh tools/wwm.sh rebuild-unique [--dry-run]       # [tools/rebuild_unique_strings.py] after a game update: append
                                                 # newly-seen strings to unique_strings.jsonl WITHOUT touching
                                                 # existing idx, scanning strings.jsonl + strings_diff.jsonl

sh tools/wwm.sh progress [--write | --check]     # [tools/progress.py] overall/per-phase translation progress;
                                                 # --write regenerates the block in Current-Status.md,
                                                 # --check exits 1 if that block is stale (CI)

sh tools/wwm.sh patch-all [--outdir patched] [--level N]
                                                 # [tools/patch_all.py] apply the dictionary to every
                                                 # translate_words_map_* variant (base, _diff, _mobile) at once,
                                                 # writing repacked copies under --outdir

sh tools/wwm.sh manifest [--outdir patched] --archive wwm-indonesian-patch.zip
                                                 # [tools/make_manifest.py] after patch-all: write the release zip
                                                 # (patched files, flat) and <outdir>/manifest.json (schema 2) with
                                                 # sha256/size of each ORIGINAL (repo binary) and PATCHED file plus
                                                 # the zip's; published with the release, read by Patch Guard

sh tools/wwm.sh compare-update                   # [wwm-game-update skill script] new game files vs *.before
sh tools/wwm.sh hook stop                        # [tools/hooks/claude_stop_check.py] Claude Stop hook

sh tools/parity_check.sh                         # run Rust AND Python on the real data, fail on any difference
cd rust && cargo test                            # Rust unit + integration tests
```

### WWM China (`--profile cn`)

> [!WARNING]
> **Maintenance ended (2026-11):** the China profile is no longer maintained — few Indonesian CN
> players, limited scope (no cutscene subtitles) and CN updates too fast for the phases to keep
> up. The `cn-latest` release is frozen and `release-patch-cn.yml` is manual-only. Don't start new
> CN translation sessions or CN game-update migrations unless the owner says maintenance resumes.
> The commands below are kept for reference only.


Every tool above takes `--profile {global,cn}` (default `global`, behaviour unchanged); paths,
variants, dumps and phases per profile live in `tools/profiles.json`. The `cn` profile is the
Indonesian translation of **WWM China (燕云十六声)**, sourced from Mandarin:

```bash
# once per machine: dictionary keystream -> cn/work/keystream_v2.bin (gitignored). Either decrypt the
# committed copy (passphrase = repo secret CN_KEYSTREAM_PASSPHRASE; the owner also keeps it in
# cn/work/keystream_passphrase.txt) ...
gpg --batch --pinentry-mode loopback --passphrase-file cn/work/keystream_passphrase.txt -o cn/work/keystream_v2.bin -d cn/keystream_v2.bin.gpg
# ... or recover it from the running client (login screen, ADMIN terminal)
sh tools/wwm.sh cn find-dict --pid <yysls.exe PID> --v2-dir "D:\yysls\yysls_fast\LocalData\Patch\HD\v2"
sh tools/wwm.sh dump cn/translate_words_map_zh_cn cn/work/strings_zh_cn.jsonl
sh tools/wwm.sh dump cn/translate_words_map_zh_cn__small cn/work/strings_zh_cn__small.jsonl
# (+ the global strings.jsonl / strings_diff.jsonl dumps — the Python bridge needs them; the Rust
#  bridge decodes any missing dump from its binary)

sh tools/wwm.sh rebuild-unique --profile cn      # find unique strings -> cn/translation_work/
sh tools/wwm.sh qa --profile cn --locale "cn/locale/*.jsonl"
sh tools/wwm.sh progress --profile cn --write    # -> Obsidian-Vault/cn/progress/CN-Current-Status.md
sh tools/wwm.sh expand --profile cn              # -> cn/work/strings.translated.jsonl
sh tools/wwm.sh patch-all --profile cn           # -> cn/work/patched/translate_words_map_zh_cn*
sh tools/wwm.sh cn term-lookup 止戈定音技 …       # [cn/tools/term_lookup.py] how global translated a Mandarin term
sh tools/wwm.sh cn name-check [--idx A-B] [--fix]
                                                 # [cn/tools/name_check.py] CN rows missing a global name
                                                 # (place/NPC/Inner Way); --fix rewrites Pinyin/known variants
                                                 # (Rust uses the `pinyin` crate instead of pypinyin)
```

- CN files are **container v2** (zstd dictionary encrypted with one keystream shared by every
  file in the client; `Format-Spec.md` §8). `wwm_locmap.py` and the Rust crate `wwm-locmap` handle
  v1 and v2 transparently (`read_container_ex`/`write_container_ex`); v2 needs `cn/work/keystream_v2.bin` (or
  `WWM_KEYSTREAM`). Patching copies the encrypted dictionary verbatim — no re-encryption.
- **Bridge** (`tools/bridge.py`): CN keyHashes are the same per-entry IDs as global. Each CN
  entry gets the global translation of its key → else `cn/locale` (by Mandarin text) → else
  the global translation of the same Mandarin text on a shared key. So
  `cn/translation_work/unique_strings.jsonl` holds only the **difference** — 159,176 texts never
  translated in global (Phases 1–6 of 30k). Its `freq` column drives the CN coverage row, so CI
  checks CN without dumps or keystream. `cn/overrides.jsonl` (`{"h","src","v","note"}`) holds
  per-key fixes that win over everything — for inherited global text that breaks a CN layout.
  Never rebuild it from scratch once `cn/locale/` has rows
  (same rule as `build_unique.py`); `rebuild-unique --profile cn` only appends.
- CN translation sessions follow `Obsidian-Vault/cn/` (`CN-Current-Status`, `CN-Resume-Procedure`,
  `CN-Quick-Reference`, `CN-Phase-Roadmap`, logs in `cn/translation_logs/CN-Phase-N.md`) under the
  same rules as global: end with `sh tools/wwm.sh progress --profile cn --write` and update the
  phase log.
  The Stop hook and `.githooks/pre-commit` cover `cn/` too.
- **CI and releases are split per profile.** Global: `translation-qa.yml` + `release-patch.yml`
  (rolling release, tag `latest`). CN: `translation-qa-cn.yml` (QA + progress check without
  secrets; with the secret also decrypts and parses the CN binaries) + `release-patch-cn.yml`
  (rolling release, tag `cn-latest`; decrypts `cn/keystream_v2.bin.gpg` with the
  `CN_KEYSTREAM_PASSPHRASE` secret, runs `patch-all --profile cn`). A global
  `locale/` change also rebuilds `cn-latest`, because CN reuses it through the bridge.
- The CN binaries `cn/translate_words_map_zh_cn*` are tracked (like the global ones) so CI can
  build them; the keystream is only ever committed encrypted. After a CN game update, re-encrypt
  only if the keystream changed: `gpg --batch --pinentry-mode loopback --passphrase-file
  cn/work/keystream_passphrase.txt --symmetric --cipher-algo AES256 -o cn/keystream_v2.bin.gpg
  cn/work/keystream_v2.bin` (the passphrase file must have no trailing newline).
- The CN client runs elevated and keeps its locale files in the patcher overlay
  `LocalData\Patch\HD\v2\locale\`. Tested 2026-10-20: the client loads repacked files and the CN
  launcher's `StageCheck` left them patched. Player install steps: README "PC — China Client".

`translation_work/build_unique.py` is the one-time bootstrap that created `unique_strings.jsonl`.
Never re-run it: it renumbers every idx and silently breaks all `locale/*.jsonl` progress (it now
refuses to run when the file exists; so does `wwm build-unique`). Use `rebuild-unique` instead.

Tests: the Rust workspace has unit + integration tests (`cd rust && cargo test`; synthetic
containers, no game data needed), and `sh tools/parity_check.sh` runs Rust and Python side by side
on the real repo data (dump, qa, progress, expand, rebuild-unique, patch-all, manifest) and fails
on any difference — run it after touching either toolchain; CI does (`rust-ci.yml`). The Python
scripts have no test suite of their own beyond that parity run; the codec is also verified by
round-tripping a real `translate_words_map_en` through `info` → `dump` → `patch` → `info`.

## Translation session workflow lives in the Obsidian vault

This repo also hosts a multi-session Indonesian translation effort (see
`translation_work/unique_strings.jsonl` and `locale/phase*.jsonl`). All of its
process documentation — current progress, session history, the resume
procedure, and the terminology glossary — lives under `Obsidian-Vault/`, not
in this file. Before starting or continuing a translation session, read:

- `Obsidian-Vault/progress/Current-Status.md` — where things stand right now.
- `Obsidian-Vault/progress/Resume-Procedure.md` — how to pick up a session, validate a batch,
  and keep tool-call round-trips low (token cost driver).
- `Obsidian-Vault/knowladge/Quick-Reference.md` — condensed terminology/tone rules; read this
  instead of every file under `knowladge/glossary/` for a routine session.
- `Obsidian-Vault/knowladge/Glossary.md` — the full topic-file index (only needed for a case
  Quick-Reference doesn't cover, or its reasoning/history).
- `Obsidian-Vault/translation_logs/Phase-N.md` — the historical reasoning behind past decisions, per phase.

**Every translation session must end by running `sh tools/wwm.sh progress --write`** and quoting
its overall line (unique strings done/total/% and in-game coverage %) in the session write-ups and
the final message to the user. The "Overall progress" / "Phase status" tables in
`Current-Status.md` are generated by that script — never edit them by hand (if the Rust binary is
not built, `tools/wwm.sh` runs `python tools/progress.py --write` — same result). CI
(`.github/workflows/translation-qa.yml`) runs `qa --locale` and `progress --check` on every PR
touching `locale/`. A new `locale/updateN.jsonl` or phase file must be added to the profile's
`phases` in `tools/profiles.json` (both toolchains refuse to run otherwise).

**Every translation session must also update `Obsidian-Vault/translation_logs/Phase-N.md`** (or
`Update-N.md`) for whichever phase it touched, before ending the turn — not just
`Current-Status.md`. Append a `## Batches` entry (idx range, commit if known, date) and any
terminology/context decisions made during the session, following the existing style in that
phase's file (see e.g. `Phase-6.md`). If the phase has no log file yet, create one. This has been
skipped before (Phase 7–17 logs were missing and had to be reconstructed retroactively from git
history on 2026-09-26) — don't let it happen again.

Local sessions are enforced by two hooks as well:
- `.Codex/settings.json` registers a **Stop hook** (`sh tools/wwm.sh hook stop` → Rust `wwm hook stop`, or
  `tools/hooks/claude_stop_check.py` when the binary is missing/stale; it never builds): if
  `locale/` has uncommitted changes and the progress block is stale, Codex is blocked from ending
  the turn until it runs `progress --write` and reports the overall line.
- `.githooks/pre-commit` runs `tools/wwm.sh qa --locale` + `progress --check` on any commit that
  touches translation data, `tools/` or `rust/`. Enable it once per clone: `git config core.hooksPath .githooks`.

## Do not use subagents

Do all work in this repo directly (Read/Edit/Bash/Grep/Glob etc.) instead of delegating to the
Agent tool. The codebase is small enough that spawning subagents just adds overhead and loses
context — only use one if the user explicitly asks for it by name.

## Architecture

- `rust/` — Cargo workspace of the Rust toolchain (`docs/Rust-Toolchain.md`): `crates/wwm-locmap`
  (codec, port of `wwm_locmap.py`; parallel shard parse/rebuild/compress), `crates/wwm-tools` (one
  module per Python script: `qa`, `dict` (expand), `bridge`, `progress`, `patch_all`, `manifest`,
  `unique`, `compare`, `hook`, `cn/*`; `py.rs` reproduces Python's `json.dumps`/`repr`/number
  formats so text output stays byte-identical) and `crates/wwm-cli` (the `wwm` binary, clap).
  Separate from `app/` (the Patch Guard workspace).
- `tools/wwm.sh` — the dispatcher (Rust or Python, see Toolchain above); `tools/parity_check.sh` —
  Rust-vs-Python check on the real data; `.github/actions/wwm-toolchain` — the CI side: picks the
  toolchain (dispatch input → repo variable `WWM_TOOLCHAIN` → `rust`), caches the binary per
  `rust/` source hash, and falls back to Python with a warning when the Rust build fails.
- `wwm_locmap.py` — the container/shard codec (`read_container`/`write_container`,
  `parse_shard`/`rebuild_shard`) plus a thin CLI (`info`/`dump`/`patch`) over it.
- `tools/qa_check.py` — a standalone validator that diffs an original JSONL dump against a
  translated one and flags three defect classes before a patch is repacked.
- `tools/expand_locale.py` — turns the per-idx translation work (`translation_work/unique_strings.jsonl`
  + `locale/phase*.jsonl` + `locale/update*.jsonl` — the latter holds game-update string batches,
  named `Update-N` rather than `Phase N` to keep them out of the main phase sequence) into a
  `patch`-ready JSONL for one dumped file, matching by literal source text (not by address), so it
  works unmodified on any `translate_words_map_*` variant.
- `tools/rebuild_unique_strings.py` — after a game update, appends newly-seen strings to
  `unique_strings.jsonl` at new idx values without touching existing ones, so already-translated
  `locale/phase*.jsonl` progress never needs remapping.
- `tools/patch_all.py` — imports `expand_locale`'s dictionary builder and applies it to all known
  `translate_words_map_*` files in one pass, in memory (no intermediate per-file JSONL needed).
- `tools/make_manifest.py` — writes `manifest.json` (original + patched sha256 per file, plus the
  release zip's sha256/size under `archive`) next to `patch_all.py`'s output. Each release carries
  only that zip (patched files, flat) + `manifest.json` (+ on `latest`, the Patch Guard installer);
  the workflows delete any other asset. The
  Patch Guard app (`app/`) uses `original_sha256` as the only proof that a build matches the files
  installed in a game folder, so it must always be computed from the binary `patch_all.py` read.
- `app/` — **WWM Patch Guard**, a Windows tray app that reinstalls the patch when NetEase's
  patcher restores the originals. Since v0.4 it is a native Rust app on **GPUI** (`gpui-pre`,
  pinned exactly — pre-1.0 API) with no WebView2; v0.3 and earlier were Tauri + Leptos
  (`docs/Patch-Guard-GPUI-Plan.md` has the migration). It needs no codec: it compares sha256
  hashes against the release `manifest.json` and copies files. Rules live in
  `app/src/core/guard.rs` (unit-tested: `cargo test -p wwm-patch-guard --lib` in `app/`);
  `core/engine.rs` is the watcher/poll loop; `core/` has no UI code. `shell/` is the OS side
  (tray via `tray-icon`, single instance via a named mutex + event, autostart via `auto-launch`
  under the v0.3 Run-key name, accent colour/Mica/data folder in `system.rs`), `fluent/` a small
  WinUI 3 look-alike (theme tokens, Button, Toggle, badge, InfoBar; Segoe UI Variable + Segoe
  Fluent Icons glyphs, Segoe UI / MDL2 Assets on Windows 10, Mica only from build 22621),
  `state.rs` the window state + actions, `app.rs` the window/NavigationView/ContentDialog and the
  main-thread command loop, `pages/` one file per page (Home, Settings, History), `i18n.rs` the
  UI language (Indonesian default, English; all texts in `app/locales/{id,en}.json`, used by key
  as `t!("home.restart_now")`, checked by `app/build.rs`). GPUI quits on
  the last window by default on Windows — `app.rs` sets `QuitMode::Explicit` so the guard keeps
  running in the tray. Data stays in `%LOCALAPPDATA%\com.oratakashi.wwm` (Tauri's folder) so
  config/history carry over.
  `locations.rs` finds installed games (Steam: registry → `libraryfolders.vdf` →
  `appmanifest_3564740.acf`; NetEase launchers: their uninstall registry entries) and picks the
  folders to guard — on Steam only `Package\HD\oversea\locale`, never the `LocalData\Patch`
  overlay the patcher re-downloads every launch. It never installs over a file whose hash matches no known original (a force install is
  user-confirmed). Build: `cargo build --release` in `app/`; installer: `cargo packager --release
  --formats nsis` (`cargo install cargo-packager --locked`; config in `[package.metadata.packager]`
  of `app/Cargo.toml`). CI: `app-ci.yml` tests/builds PRs; on main
  `release-patch.yml` rebuilds the installer only when `app/` changed (or dispatch `build_app`) and
  attaches it to the `latest` release as `wwm-patch-guard-setup.exe` — no separate app release.
  The app downloads the release zip once per check pass and extracts files by name (`remote.rs`).
  Since v0.5 every release file it trusts is **minisign-signed** (`manifest.json.sig` is required;
  trusted public keys `app/signing/patch-guard.pub` (active) + `patch-guard-backup.pub` (backup,
  private key kept offline — rotation/revocation in the vault page); since v0.6.1 a signature's
  trusted comment must name the file it was fetched as and a manifest older than one already used
  is refused (anti-rollback); CI secrets `PATCH_GUARD_SIGN_KEY` /
  `PATCH_GUARD_SIGN_KEY_PASSWORD` (optional, unset for a key without a password) live in the
  environment `release` and are used only by the `sign` job of `release-patch.yml` (no checkout,
  pinned `cargo-packager@0.11.8`); `release-patch-cn.yml` signs `cn-latest` too) and it **updates
  itself** (`core/update.rs`): `patch-guard-update.json(.sig)` + `wwm-patch-guard-setup.exe(.sig)`
  on `latest` are downloaded and verified in the background into `updates\<version>\` (installer
  + the signed JSON and `.sig`, no unsigned `pending.json`), then installed by a copy of the exe in
  a fresh `<data>\updater\<id>\` folder (`--apply-update --wait-pid N [--relaunch [--minimized]]`,
  silent NSIS `/S`) on "Restart sekarang" (tray quit / next start only with the opt-in
  `auto_install_app`, default off). The helper takes **no paths**
  from its command line: it re-verifies both signatures, hash and version, holds the installer
  locked while it runs, and relaunches only the installed exe (uninstall entry `InstallLocation`).
  Full write-up: `Obsidian-Vault/knowladge/Patch-Guard.md`.
  In **debug builds only**, `WWM_PATCH_GUARD_BASE_URL` / `WWM_PATCH_GUARD_DATA_DIR` /
  `WWM_PATCH_GUARD_PUBKEY` point it at a local release server / data folder / test key; release
  builds ignore them and download over https only.
- `tools/profiles.json` — per-profile paths/variants/dumps/phases (`global`, `cn`), loaded by
  `tools/profiles.py` and by `wwm`; `--profile` on
  every tool. `tools/bridge.py` — the global→CN keyHash/text bridge.
- `cn/tools/find_dict_in_dump.py` (recover the v2 dictionary keystream from the running CN client),
  `cn/tools/term_lookup.py` (Mandarin term → global English/Indonesian via shared keys),
  `cn/tools/name_check.py` (flags/fixes CN rows that don't use the global name of a place/NPC/
  Inner Way found in their source).

Pipeline: `translate_words_map_en` --dump--> `strings.jsonl` --(edit `v` field)--> validate with
`qa_check.py` --patch--> new `translate_words_map_en`. For an update with several file variants
(base + `_diff`; `__small` + `__small_diff` up to v2.2), the recommended path is `rebuild_unique_strings.py`
once, then `patch-all` to apply the dictionary everywhere at once.

`translate_words_map_en_diff` (the incremental file shipped between game updates) uses the exact
same container/shard format and is handled by the same `info`/`dump`/`patch` commands. The
difference is purely semantic: most shards are sparse (only changed keys are present), and a key
removed since the base version is stored as a single `0xFF` byte instead of being absent — see
`TOMBSTONE` in `wwm_locmap.py` and the `deleted` JSONL field below.

### The `__small` / `__small_diff` variant (v2.2 and earlier only)

**Game v2.3 (2026-10-15) no longer ships `__small` / `__small_diff`** (checked on Steam and the
official launcher); they were removed from the repo, the global profile's `files` (`tools/profiles.json`) and the release. The
notes below are kept in case NetEase reintroduces them.

A game update introduced `translate_words_map_en__small` (and its own `__small_diff`) alongside
the existing pair. **Same container/shard codec, zero format changes** — confirmed by a full
byte-identical dump→patch→re-parse round trip. It is its own independent hash table with its own
`b`/`s` address space and shard count (16 shards vs. thousands in the main file) — likely a
small "hot"/fast-load subset (loading screen, splash, etc.), not a delta of the main file.

Measured on the 2026-09 update: ~98% of its ~4,000 keys share a `keyHash` with the main file, but
**193 of those have a different value than the main file's copy** (stale/context-specific
duplicate — translate it as its own text, don't assume it matches the main file), and **27 keys
exist only in `__small`**, nowhere else. Always treat every `translate_words_map_*` file as an
independent thing to dump/patch — never assume identical hashes imply identical current text.

Because the translation dictionary (`unique_strings.jsonl` + `locale/phase*.jsonl`) is keyed by
literal source text, not by file/block/slot/hash, applying it to a new variant needs no format
work: dump the file, match its `v` text against the dictionary (`tools/expand_locale.py` or
`tools/patch_all.py`), patch it back. Same recipe for `__small_diff`, main `_diff`, and any future
variant NetEase adds — add its name to the profile's `files` in `tools/profiles.json`.

`translate_words_map_en_mobile` (2026-09-22) is byte-identical to `translate_words_map_en`; it is
in the global profile's `files` anyway so it keeps working if the two ever diverge.

### Deployment gotcha: NetEase's patcher re-verifies installed locale files (Steam `_diff` every launch, every client on updates)

This is not a tool bug — it's a fact about the live game client that affects anyone trying to
actually play with a patched file, so it's recorded here. **It depends on which client runs the
game**: the revert below was only ever observed on the **Steam** build. On 2026-09-27 the owner
retested with NetEase's own standalone launcher (the official WWM launcher, not Steam) and a
patched `_diff` was **not** reverted — it stayed patched across launches. All earlier testing had
been done on Steam only, which is why this was originally written up as a universal limitation.

The game install has **two separate copies** of `translate_words_map_en_diff`:

- `Package\HD\oversea\locale\translate_words_map_en_diff` — the one this repo's tools read/write.
  Patches here are **stable**: nothing in-game re-verifies or overwrites it.
- `LocalData\Patch\HD\oversea\locale\translate_words_map_en_diff` — a **separate hot-patch overlay**
  copy that the running game actually loads for the `_diff` layer. `LocalData\Patch\...` only
  contains `_diff` variants (no `en`/`__small`/`__small_diff`) on Steam, mirroring the `Package`
  structure; on the official launcher's `wwm_lite` it holds the base `translate_words_map_en` too.

On the Steam client, every game launch runs NetEase's patcher (visible in
`LocalData\patch_log\patch_log_*.txt`) through a `StagePatchList` → `StageCheck` → `StageDownload`
sequence: it fetches a checksum manifest from its update CDN (host seen in logs:
`*.update.easebar.com`), compares every file's hash against it, and **silently re-downloads and
overwrites any file that doesn't match** — including a manually patched
`LocalData\Patch\...\translate_words_map_en_diff`. This was confirmed directly: after patching
that file, it was found reverted (byte-identical to the pristine original, confirmed via file
size and creation-time) within minutes, with the log showing `StageCheck:finish_submit
bytes=9333638 #task=1` (the exact pristine file size) as the one file re-fetched. The manifest
fetch also falls back to a locally cached copy (`fetch_patchlist res=ok from=cached`) when the
CDN host is unreachable, and the actual file download appears to use a different host than the
manifest-fetch host, so blocking a single CDN hostname does not stop the repair.

Why the official launcher behaves differently on a normal launch hasn't been investigated — only
the observed outcome is known. **But every server update re-verifies it** (observed 2026-10-28):

**Server updates revert the patch on the official launcher and on Android.** When NetEase pushes
a new build (even a hotfix with byte-identical locale files — `20261017191021` →
`20261026193908` on lite, `20261019214618` → `20261026194403` on Android), the patcher downloads
the new patchlists, diffs them against the local ones
(`StagePatchList:collect_check_list diff=[…, pkg_English, …, pkg_en, …]`) and hash-checks
**every file of each changed package** (~339k files on lite, ~453k on Android), re-fetching any
mismatch (`StageCheck:finish_check … #fetch=219` / `#fetch=222`). The locale files are in
`pkg_en`, so a patched `translate_words_map_en` **and** `_diff` in `LocalData\Patch\...` come back
pristine. On Android it also deleted unmanifested files in that folder (the owner's `.ori`
backups), so never keep backups inside the game folder. On a normal launch there is no patchlist
diff and the check runs from cache (`#fetch=0`), which is why the 2026-09-27 retest saw the patch
hold. The patchlists are encoded (`HEXB_` + base64), so editing them is not a practical
workaround. `locale\patch_version.json` (one number, untouched by updates) is unrelated.

On **Steam** the same hotfix (`20261026193944`) arrived through the in-game patcher
(`IsLauncher: false`), which only manages the small `LocalData` overlay (~1.9k files, 84
fetched). `Package\HD\oversea\locale\translate_words_map_en` belongs to the Steam depot and
survived; it is only replaced by a Steam depot update.

**Practical implication**:

- **Official NetEase launcher** (v2.3: base + `_diff` only; on `wwm_lite` both live in
  `LocalData\Patch\HD\oversea\locale\`): all files hold between updates — base, (≤v2.2) `__small`,
  `__small_diff` and the `_diff` overlay. The `_diff` layer (~213k entries in the 2026-09
  update, ~26% of total entries, representing text changed/added since the base package was last
  rebuilt) is therefore fully usable there, so translating `_diff`-only strings is worthwhile.
  **After every game update (hotfixes included) the patch must be reinstalled.**
- **Android** (`/sdcard/Android/data/com.netease.yysls/files/LocalData/Patch/HD/oversea/locale/`):
  same as the official launcher — holds between updates, reverted by every update.
- **Steam**: only `Package\HD\oversea\locale\translate_words_map_en` (base) and
  `translate_words_map_en__small` hold, and they survive NetEase hot-patches (only a Steam depot
  update replaces them); the `_diff` overlay cannot currently be made to
  stick via a simple file replacement. If a future game update merges `_diff` content back into
  the base package (which NetEase does periodically), whatever fraction of it is already covered
  by the translation dictionary becomes permanent automatically at that point.

### File format (see `Obsidian-Vault/knowladge/Format-Spec.md` for the full spec)

- Container: `magic(0xDEADBEEF) | version | blockCount | reserved`, followed by a
  `u32[blockCount]` table of block **end offsets**, relative to the end of that offset table.
- Each block is independently zstd-compressed (`codec=4` is the only one handled; anything else
  raises rather than silently producing corrupt data).
- Block 0 is an index (`totalEntries`, `dataBlockCount`, an `ids` array). Blocks 1..n are shards:
  a serialized `absl::flat_hash_map` (SwissTable) with `capacity/size/seed`, a control byte array
  (`ctrl[i] >= 0x80` means empty), a sentinel + 15-byte clone region for SIMD probing, then a
  16-byte slot array (`keyHash`, `relOffset`, `byteLen`), then a raw UTF-8 string blob.
- `shardIndex = (keyHash % dataBlockCount) + 1` determines which shard a key lives in.

**The critical gotcha**: `relOffset` is a *relative* pointer computed from the address of the
`relOffset` field itself (`slotArrayStart + i*16 + 8`), not from the start of the block or the
string blob. Treating it as absolute produces overlapping/garbled decoded strings — that's the
signature of this specific mistake. See `_slots_off()` and the `field = ...; a = p + 8 + rel`
line in `parse_shard()`.

**Why the game must be set to English**: the file only stores `u64 keyHash -> translated string`;
it never stores the English source text. The runtime hashes the current-language string and
looks up a match — so translation packs only work when `Game Language = English`, and this is
an architectural fact, not a tool limitation.

**Why no hash function is needed**: since keys aren't stored, changing language means overwriting
*values* for existing keys — `ctrl`, `keyHash`, and slot positions are preserved byte-for-byte in
`rebuild_shard()`; only `relOffset`/`byteLen` and the string blob are rewritten. This is also why
adding genuinely new keys isn't currently possible (see `Obsidian-Vault/knowladge/Format-Spec.md` §4.4 — the H1/H2
derivation from `keyHash`+`seed` hasn't been solved).

### QA checks (`tools/qa_check.py`)

Three checks run per entry when comparing original vs. translated JSONL:

1. `PROMPT_LEAK` — translated value contains a known LLM/MT system-prompt fragment
   (`LEAK_SIGNATURES` tuple — extend this when a new prompt-leak variant is found).
2. `MARKUP` — the multiset of format tokens (`TOKEN` regex: `#aabbcc` colors, `#X` format codes
   incl. `#E`, `%s`/`%d`, `{...}` placeholders, `<...>` tags) differs between source and
   translation. Counts are compared, not order. These tokens must never be translated, added, or
   dropped, or the game UI renders incorrectly. The 6-digit hex alternative must stay before the
   1-letter one, or `#e9a35f` tokenizes as `#e` and a damaged color passes. `TOKEN` is the single
   source of truth — the vault docs point to it instead of carrying their own regex.
3. `EMPTY` — source is non-empty but translation is blank.

Entries missing from the translated file are treated as intentionally left untranslated (not an
error) — proper names (Pinyin character/sect names) are commonly left as-is by convention.
Tombstone entries (`"deleted": true`, from `*_diff` files) are skipped by both loaders and never
flagged — there is no source text to check.

## JSONL record shape

```json
{"b": 1, "s": 3, "h": "c5cadbb857eee8b4", "v": "The leaf?"}
```

`b`/`s` (block/slot) form the entry's address and are required by `patch`. `h` (keyHash) is
never modified; when present, `patch` checks it against the target slot and aborts on a mismatch
(the JSONL was dumped from a different game version). `v` is the only field to edit. `patch` only overwrites entries
present in the given JSONL; everything else is carried over from the source file, so partial
patches are supported.

A key removed since the base version (only seen in `*_diff` files) dumps as a tombstone instead,
with no `v` field:

```json
{"b": 5, "s": 14, "h": "924f075a96b1ccf6", "deleted": true}
```

Never invent a `v` for these or turn them back into text — `patch` re-encodes `"deleted": true`
as the raw `0xFF` sentinel byte regardless of what `v` would otherwise be.
