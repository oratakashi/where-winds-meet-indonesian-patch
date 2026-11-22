# Rust Toolchain

All repo tooling exists twice: the Rust binary **`wwm`** (workspace [`rust/`](../rust)) and the
original Python scripts. The Rust port is the default because it makes CI and releases several
times faster; the Python scripts stay as the **failsafe** and are never deleted. Both take the
same arguments, print the same output and use the same exit codes, and
[`tools/parity_check.sh`](../tools/parity_check.sh) checks on every change that they agree.

- [Running the tools](#running-the-tools)
- [Subcommand map](#subcommand-map)
- [Choosing the toolchain](#choosing-the-toolchain)
- [CI](#ci)
- [Tests and parity](#tests-and-parity)
- [Timings](#timings)
- [Known differences](#known-differences)
- [Workspace layout](#workspace-layout)

Related: [Developer Guide](Developer-Guide.md) · [Technical Overview](Technical-Overview.md)

---

## Running the tools

```bash
cargo build --release --manifest-path rust/Cargo.toml   # once (and after pulling rust/ changes)
pip install -r requirements.txt                         # Python failsafe

sh tools/wwm.sh toolchain                # which toolchain would run ("rust <path>" / "python")
sh tools/wwm.sh qa --locale "locale/*.jsonl"
sh tools/wwm.sh progress --write
```

From **PowerShell or cmd** (where `sh` is not on PATH) use the wrapper `tools\wwm.cmd` with the
same arguments — it finds Git for Windows' `sh.exe` and runs `tools/wwm.sh`:

```powershell
tools\wwm.cmd progress --check
```

Always go through `sh tools/wwm.sh` (POSIX sh — Git Bash on Windows) or `tools\wwm.cmd`. It runs
`rust/target/release/wwm[.exe]` when it exists and the matching Python script otherwise. The
binary can also be called directly (`rust/target/release/wwm progress --check`); it finds the
repo root from the current directory (or `WWM_ROOT`).

## Subcommand map

| `sh tools/wwm.sh …` | Python fallback |
|---|---|
| `info <src>` / `dump <src> <out>` / `patch <src> <edits> <out> [--level N]` | `wwm_locmap.py info/dump/patch` |
| `qa [orig trans] [--locale GLOB…] [--report F] [--profile P]` | `tools/qa_check.py` |
| `expand [--strings F] [--out F] [--profile P]` | `tools/expand_locale.py` |
| `rebuild-unique [--strings F] [--extra F…] [--dry-run] [--profile P]` | `tools/rebuild_unique_strings.py` |
| `progress [--write\|--check] [--profile P]` | `tools/progress.py` |
| `patch-all [--outdir D] [--level N] [--files …] [--profile P]` | `tools/patch_all.py` |
| `manifest --archive Z [--outdir D] [--commit SHA] [--profile P]` | `tools/make_manifest.py` |
| `build-unique` (refuses when `unique_strings.jsonl` exists) | `translation_work/build_unique.py` |
| `compare-update` | `.claude/skills/wwm-game-update/scripts/compare_update.py` |
| `hook stop` | `tools/hooks/claude_stop_check.py` |
| `cn term-lookup TERM… [--contains] [--limit N]` | `cn/tools/term_lookup.py` |
| `cn name-check [--locale G] [--idx A-B] [--report F] [--fix] [--top N]` | `cn/tools/name_check.py` |
| `cn find-dict [DUMP…] [--pid PID] [--v2-dir D] [--out D]` | `cn/tools/find_dict_in_dump.py` |

Profile data (phases, regions, file variants, dumps, paths) lives in
[`tools/profiles.json`](../tools/profiles.json); `tools/profiles.py` and the binary both read it at
run time, so adding an `Update-N` block is one edit and never needs a rebuild.

## Choosing the toolchain

| Setting | Effect |
|---|---|
| *(nothing)* / `WWM_TOOLCHAIN=auto` | Rust if `rust/target/release/wwm[.exe]` exists, else Python. A binary older than any file under `rust/crates/`, `rust/Cargo.toml` or `rust/Cargo.lock` is rebuilt with `cargo build --release` first; if that fails (or cargo is missing) Python runs, with a warning on stderr. |
| `WWM_TOOLCHAIN=python` | Always Python — the failsafe. |
| `WWM_TOOLCHAIN=rust` | Always Rust; exits 1 if the binary is missing or cannot be rebuilt. |
| `WWM_BIN=/path/to/wwm` | Use this binary as is (no staleness check). CI sets it. |
| `WWM_NO_BUILD=1` | Never build; a stale binary means Python. The Claude Stop hook uses this so it stays fast. |

The pre-commit hook (`.githooks/pre-commit`) and the Claude Code Stop hook (`.claude/settings.json`)
both go through `tools/wwm.sh`.

## CI

Every workflow that runs the tooling (`translation-qa`, `release-patch`, `translation-qa-cn`,
`release-patch-cn`) starts with the composite action
[`.github/actions/wwm-toolchain`](../.github/actions/wwm-toolchain/action.yml):

1. **Pick** the toolchain: workflow-dispatch input `toolchain` (`auto`/`rust`/`python`) → repo
   variable `WWM_TOOLCHAIN` → `rust`.
2. **Rust:** restore `rust/target/release/wwm` from the Actions cache (key = hash of the `rust/`
   sources). On a miss, build it (`Swatinem/rust-cache` keeps dependencies warm). The binary is
   only rebuilt when `rust/` changes, so a normal translation push pays nothing for it.
3. **Fallback:** if the build fails or the binary does not run, the action prints a warning and
   switches to **Python** (`setup-python` + `pip install`). A failing QA or progress check is a
   real failure and is never retried with Python.
4. Exports `WWM_TOOLCHAIN` / `WWM_BIN` for every later `sh tools/wwm.sh …` step and writes the
   toolchain used to the job summary.

On the Rust path the `strings.jsonl` / `strings_diff.jsonl` dump steps are skipped: `progress`,
`rebuild-unique` and the CN bridge decode the tracked binaries in memory when a dump is missing.

**Forcing Python for every run** (e.g. if a Rust bug is found): repository *Settings → Secrets and
variables → Actions → Variables* → `WWM_TOOLCHAIN` = `python`. Delete the variable to go back to
Rust. For a single run use *Run workflow* with `toolchain: python`.

`rust-ci.yml` tests the Rust workspace (`cargo fmt --check`, `clippy -D warnings`, `cargo test`
on Linux and Windows) and runs `tools/parity_check.sh` on the real data whenever `rust/`,
`tools/`, `wwm_locmap.py` or `cn/tools/` change — so neither toolchain can drift from the other.

## Tests and parity

```bash
cd rust && cargo test          # unit + integration tests (no game data needed)
sh tools/parity_check.sh       # Rust vs Python on the repo data; exit 1 on any difference
```

Unit tests build synthetic containers in memory (SwissTable shards with ctrl bytes, sentinel,
relative `relOffset`, tombstones; v1 and v2 containers with a trained zstd dictionary and an
XOR keystream) and cover every module: codec errors, `TOKEN` regex edge cases (`#e9a35f` vs
`#e`, `$STAT:.0f$`), QA idx checks, `json.dumps`/`repr`/`{:,}`/`%.2f` compatibility, the
progress table, bridge tie-breaking (`Counter.most_common`), rebuild-unique ordering, the
deterministic release zip, name-check fixes and dictionary recovery. Integration tests run the
`wwm` binary end to end on a throw-away repo (`WWM_ROOT`).

`parity_check.sh` compares: `dump` of base + `_diff` (byte-identical), `qa --locale` and
`progress` output for global and CN, `rebuild-unique --dry-run`, `expand` (byte-identical),
`patch-all` (decoded contents) and `manifest` (schema, original hashes, file list), and checks
that the Rust release zip is readable, stored and timestamped 1980-01-01.

## Timings

Measured 2026-11-22 on the owner's Windows PC (warm file cache, game v2.4 data; release builds).
CI runners have fewer cores, so the parallel steps gain somewhat less there.

| Step | Python | Rust | Speed-up |
|---|---:|---:|---:|
| `patch-all --level 19` (release build, base + `_diff`) | 22.8 s | 2.8 s | 8.2× |
| `progress --check` in CI (Python needs both dumps first) | 3.6 + 1.2 + 6.0 s | 1.7 s | 6.4× |
| `dump translate_words_map_en` | 3.6 s | 0.4 s | 8.3× |
| `qa --locale "locale/*.jsonl"` (733,662 rows) | 4.7 s | 1.0 s | 4.7× |
| `expand` | 7.4 s | 2.4 s | 3.1× |
| `rebuild-unique --dry-run` | 6.0 s | 1.4 s | 4.3× |
| `patch-all --profile cn --level 3` | 16.7 s | 5.9 s | 2.8× |
| `cn name-check` | 18.6 s | 7.9 s | 2.4× |

`patch-all --level 19` with the Rust toolchain writes **byte-identical** files to the Python
script (same libzstd parameters), so a Rust-built release is the same release.

## Known differences

- `cn name-check --fix` gets Pinyin syllables from the `pinyin` crate instead of `pypinyin`; a
  rare heteronym could differ. On the current `cn/locale` both fix the same 348 rows identically.
- When a dump (`strings.jsonl`, `strings_diff.jsonl`, `cn/work/strings_zh_cn*.jsonl`) is missing,
  the Rust tools decode the tracked binary instead of exiting with the "dump it first" hint. The
  numbers are the same, because a dump is a pure function of the binary.
- A binary value that is not UTF-8 is reported with a Rust byte list instead of a Python `repr`.
- Release zips differ by a few bytes (zip64 extra fields); their content, names, timestamps and
  permissions are the same. The manifest records whichever zip was built.

## Workspace layout

```text
rust/
├── Cargo.toml              # workspace (edition 2024), release profile with LTO
├── rust-toolchain.toml     # stable + rustfmt + clippy
└── crates/
    ├── wwm-locmap/         # codec: container v1/v2, shard parse/rebuild (port of wwm_locmap.py)
    ├── wwm-tools/          # one module per Python script; py.rs = Python-compatible formatting
    │   └── src/cn/         # term_lookup, name_check, find_dict
    └── wwm-cli/            # the `wwm` binary (clap) + integration tests
```

When changing a tool, change **both** implementations (or neither) and run
`sh tools/parity_check.sh`; CI enforces it.
