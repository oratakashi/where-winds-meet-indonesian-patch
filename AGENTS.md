# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in this repository.

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
`python wwm_locmap.py dump translate_words_map_en strings.jsonl` (CI does this before
`progress.py`, which exits with that hint if the file is missing). Since the v2.3 hotfix
(Update-2D) `progress.py` also reports `_diff` coverage, so `strings_diff.jsonl` must be dumped
the same way (`python wwm_locmap.py dump translate_words_map_en_diff strings_diff.jsonl`).

## Scratch and temporary files

Keep the project free of scratch and temporary files. Create all temporary scripts,
intermediate dumps, QA reports, logs, and other disposable outputs outside the repository,
using the system temporary directory (for example, `%TEMP%` on Windows or `/tmp`).
This applies even to ignored files: do not create `scratch/`, `tmp/`, or other temporary
directories inside the project. Override tool output paths when their defaults would write
temporary files into the repository. Only intended project deliverables belong here.

## No system assessment or environment validation

Do not run system assessments, environment diagnostic checks, Python/tooling sanity checks, or dependency verifications. All environments and tools for this project were already running perfectly before Antigravity or Gemini was installed. Assume the environment is completely ready and proceed straight to the requested tasks.

## Commands

```bash
pip install -r requirements.txt      # only dependency: zstandard

python wwm_locmap.py info  <src>                     # print header/entry counts, sanity-check parser
python wwm_locmap.py dump  <src> <out.jsonl>          # decode container -> one JSON object per line
python wwm_locmap.py patch <src> <edits.jsonl> <out>  # re-encode: overlay edits.jsonl onto src, write out
                                                       #   --level N sets zstd level (default 19; use 10-12 for fast iteration)
                                                       #   refuses records whose `h` doesn't match the slot's keyHash

python tools/qa_check.py <original.jsonl> <translated.jsonl> [--report qa.jsonl]
                                                       # validates a translated JSONL against the source JSONL
                                                       # exit code 1 if findings exist (CI/pre-commit friendly)
python tools/qa_check.py --locale "locale/*.jsonl"     # same checks on per-idx translation files vs.
                                                       # unique_strings.jsonl, plus idx gap/duplicate checks —
                                                       # the per-batch validator for translation sessions

python tools/expand_locale.py [--strings X.jsonl] [--out Y.jsonl]
                                                       # build a patch JSONL for ANY single dump by matching its
                                                       # `v` text against translation_work/unique_strings.jsonl +
                                                       # locale/phase*.jsonl + locale/update*.jsonl (defaults to strings.jsonl)

python tools/rebuild_unique_strings.py [--dry-run]    # after a game update: append newly-seen strings to
                                                       # unique_strings.jsonl WITHOUT touching existing idx
                                                       # (keeps locale/phase*.jsonl valid), scanning
                                                       # strings.jsonl + strings_diff.jsonl

python tools/progress.py [--write | --check]          # compute overall/per-phase translation progress;
                                                       # --write regenerates the block in Current-Status.md,
                                                       # --check exits 1 if that block is stale (CI)

python tools/patch_all.py [--outdir patched] [--level N]
                                                       # apply the translation dictionary to every
                                                       # translate_words_map_* variant (base, _diff, _mobile)
                                                       # at once, writing
                                                       # repacked copies under --outdir
```

### WWM China (`--profile cn`)

Every tool above takes `--profile {global,cn}` (default `global`, behaviour unchanged); paths,
variants, dumps and phases per profile live in `tools/profiles.py`. The `cn` profile is the
Indonesian translation of **WWM China (燕云十六声)**, sourced from Mandarin:

```bash
# once per machine: dictionary keystream -> cn/work/keystream_v2.bin (gitignored). Either decrypt the
# committed copy (passphrase = repo secret CN_KEYSTREAM_PASSPHRASE; the owner also keeps it in
# cn/work/keystream_passphrase.txt) ...
gpg --batch --pinentry-mode loopback --passphrase-file cn/work/keystream_passphrase.txt -o cn/work/keystream_v2.bin -d cn/keystream_v2.bin.gpg
# ... or recover it from the running client (login screen, ADMIN terminal)
python cn/tools/find_dict_in_dump.py --pid <yysls.exe PID> --v2-dir "D:\yysls\yysls_fast\LocalData\Patch\HD\v2"
python wwm_locmap.py dump cn/translate_words_map_zh_cn cn/work/strings_zh_cn.jsonl
python wwm_locmap.py dump cn/translate_words_map_zh_cn__small cn/work/strings_zh_cn__small.jsonl
# (+ the global strings.jsonl / strings_diff.jsonl dumps — the bridge needs them)

python tools/rebuild_unique_strings.py --profile cn   # find unique strings -> cn/translation_work/
python tools/qa_check.py --profile cn --locale "cn/locale/*.jsonl"
python tools/progress.py --profile cn --write         # -> Obsidian-Vault/cn/progress/CN-Current-Status.md
python tools/expand_locale.py --profile cn            # -> cn/work/strings.translated.jsonl
python tools/patch_all.py --profile cn                # -> cn/work/patched/translate_words_map_zh_cn*
python cn/tools/term_lookup.py 止戈定音技 …            # how global translated a Mandarin term
```

- CN files are **container v2** (zstd dictionary encrypted with one keystream shared by every
  file in the client; `Format-Spec.md` §8). `wwm_locmap.py` handles v1 and v2 transparently
  (`read_container_ex`/`write_container_ex`); v2 needs `cn/work/keystream_v2.bin` (or
  `WWM_KEYSTREAM`). Patching copies the encrypted dictionary verbatim — no re-encryption.
- **Bridge** (`tools/bridge.py`): CN keyHashes are the same per-entry IDs as global. Each CN
  entry gets the global translation of its key → else `cn/locale` (by Mandarin text) → else
  the global translation of the same Mandarin text on a shared key. So
  `cn/translation_work/unique_strings.jsonl` holds only the **difference** — 159,176 texts never
  translated in global (Phases 1–6 of 30k). Its `freq` column drives the CN coverage row, so CI
  checks CN without dumps or keystream. `cn/overrides.jsonl` (`{"h","src","v","note"}`) holds
  per-key fixes that win over everything — for inherited global text that breaks a CN layout.
  Never rebuild it from scratch once `cn/locale/` has rows
  (same rule as `build_unique.py`); `rebuild_unique_strings.py --profile cn` only appends.
- CN translation sessions follow `Obsidian-Vault/cn/` (`CN-Current-Status`, `CN-Resume-Procedure`,
  `CN-Quick-Reference`, `CN-Phase-Roadmap`, logs in `cn/translation_logs/CN-Phase-N.md`) under the
  same rules as global: end with `progress.py --profile cn --write` and update the phase log.
  The Stop hook and `.githooks/pre-commit` cover `cn/` too.
- **CI and releases are split per profile.** Global: `translation-qa.yml` + `release-patch.yml`
  (rolling release, tag `latest`). CN: `translation-qa-cn.yml` (QA + progress check without
  secrets; with the secret also decrypts and parses the CN binaries) + `release-patch-cn.yml`
  (rolling release, tag `cn-latest`; decrypts `cn/keystream_v2.bin.gpg` with the
  `CN_KEYSTREAM_PASSPHRASE` secret, dumps global + CN, runs `patch_all --profile cn`). A global
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
refuses to run when the file exists). Use `rebuild_unique_strings.py` instead.

There is no test suite; correctness is verified by round-tripping a real
`translate_words_map_en` file through `info` → `dump` → `patch` → `info` and diffing entries.

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

**Every translation session must end by running `python tools/progress.py --write`** and quoting
its overall line (unique strings done/total/% and in-game coverage %) in the session write-ups and
the final message to the user. The "Overall progress" / "Phase status" tables in
`Current-Status.md` are generated by that script — never edit them by hand. CI
(`.github/workflows/translation-qa.yml`) runs `qa_check.py --locale` and `progress.py --check` on
every PR touching `locale/`. A new `locale/updateN.jsonl` or phase file must be added to `PHASES`
in `tools/progress.py` (the script refuses to run otherwise).

**Every translation session must also update `Obsidian-Vault/translation_logs/Phase-N.md`** (or
`Update-N.md`) for whichever phase it touched, before ending the turn — not just
`Current-Status.md`. Append a `## Batches` entry (idx range, commit if known, date) and any
terminology/context decisions made during the session, following the existing style in that
phase's file (see e.g. `Phase-6.md`). If the phase has no log file yet, create one. This has been
skipped before (Phase 7–17 logs were missing and had to be reconstructed retroactively from git
history on 2026-09-26) — don't let it happen again.

Local sessions are enforced by two hooks as well:
- `.Codex/settings.json` registers a **Stop hook** (`tools/hooks/claude_stop_check.py`): if
  `locale/` has uncommitted changes and the progress block is stale, Codex is blocked from ending
  the turn until it runs `progress.py --write` and reports the overall line.
- `.githooks/pre-commit` runs `qa_check.py --locale` + `progress.py --check` on any commit that
  touches translation data. Enable it once per clone: `git config core.hooksPath .githooks`.

## Do not use subagents

Do all work in this repo directly (Read/Edit/Bash/Grep/Glob etc.) instead of delegating to the
Agent tool. The codebase is small enough that spawning subagents just adds overhead and loses
context — only use one if the user explicitly asks for it by name.

## Architecture

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
- `tools/profiles.py` — per-profile paths/variants/dumps/phases (`global`, `cn`); `--profile` on
  every tool. `tools/bridge.py` — the global→CN keyHash/text bridge.
- `cn/tools/find_dict_in_dump.py` (recover the v2 dictionary keystream from the running CN client),
  `cn/tools/term_lookup.py` (Mandarin term → global English/Indonesian via shared keys).

Pipeline: `translate_words_map_en` --dump--> `strings.jsonl` --(edit `v` field)--> validate with
`qa_check.py` --patch--> new `translate_words_map_en`. For an update with several file variants
(base + `_diff`; `__small` + `__small_diff` up to v2.2), the recommended path is `rebuild_unique_strings.py`
once, then `patch_all.py` to apply the dictionary everywhere at once.

`translate_words_map_en_diff` (the incremental file shipped between game updates) uses the exact
same container/shard format and is handled by the same `info`/`dump`/`patch` commands. The
difference is purely semantic: most shards are sparse (only changed keys are present), and a key
removed since the base version is stored as a single `0xFF` byte instead of being absent — see
`TOMBSTONE` in `wwm_locmap.py` and the `deleted` JSONL field below.

### The `__small` / `__small_diff` variant (v2.2 and earlier only)

**Game v2.3 (2026-10-15) no longer ships `__small` / `__small_diff`** (checked on Steam and the
official launcher); they were removed from the repo, the global profile's `files` (`tools/profiles.py`) and the release. The
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
variant NetEase adds — add its name to the profile's `files` in `tools/profiles.py`.

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
