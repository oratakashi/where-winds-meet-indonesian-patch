# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A reverse-engineering toolkit for the `translate_words_map_*` localization file format used by
**Where Winds Meet** (NetEase / Everstone Studio, Messiah Engine). This repo ships only the
tool and format documentation (Indonesian-language) — not any game text, translation packs, or
extracted content. See "Never commit game content" below before touching working files.

## Commands

```bash
pip install -r requirements.txt      # only dependency: zstandard

python wwm_locmap.py info  <src>                     # print header/entry counts, sanity-check parser
python wwm_locmap.py dump  <src> <out.jsonl>          # decode container -> one JSON object per line
python wwm_locmap.py patch <src> <edits.jsonl> <out>  # re-encode: overlay edits.jsonl onto src, write out
                                                       #   --level Nsets zstd level (default 19; use 10-12 for fast iteration)

python tools/qa_check.py <original.jsonl> <translated.jsonl> [--report qa.jsonl]
                                                       # validates a translated JSONL against the source JSONL
                                                       # exit code 1 if findings exist (CI/pre-commit friendly)

python tools/expand_locale.py [--strings X.jsonl] [--out Y.jsonl]
                                                       # build a patch JSONL for ANY single dump by matching its
                                                       # `v` text against translation_work/unique_strings.jsonl +
                                                       # locale/phase*.jsonl (defaults to strings.jsonl)

python tools/rebuild_unique_strings.py [--dry-run]    # after a game update: append newly-seen strings to
                                                       # unique_strings.jsonl WITHOUT touching existing idx
                                                       # (keeps locale/phase*.jsonl valid), scanning
                                                       # strings.jsonl + strings_diff.jsonl + strings_small.jsonl +
                                                       # strings_small_diff.jsonl

python tools/patch_all.py [--outdir patched] [--level N]
                                                       # apply the translation dictionary to all four
                                                       # translate_words_map_* variants at once, writing
                                                       # repacked copies under --outdir
```

There is no test suite; correctness is verified by round-tripping a real
`translate_words_map_en` file through `info` → `dump` → `patch` → `info` and diffing entries.

## Never commit game content

`.gitignore` already excludes `*.gubackup`, `*.map`, `*.cleaned`. Do not add `strings.jsonl`,
`translate_words_map_*`, or any dump/patch output to git — all such content is NetEase's
copyrighted game text. Working files like `strings.jsonl` and `translate_words_map_en` present
in the working tree are local scratch data, not repo assets.

## Architecture

- `wwm_locmap.py` — the container/shard codec (`read_container`/`write_container`,
  `parse_shard`/`rebuild_shard`) plus a thin CLI (`info`/`dump`/`patch`) over it.
- `tools/qa_check.py` — a standalone validator that diffs an original JSONL dump against a
  translated one and flags three defect classes before a patch is repacked.
- `tools/expand_locale.py` — turns the per-idx translation work (`translation_work/unique_strings.jsonl`
  + `locale/phase*.jsonl`) into a `patch`-ready JSONL for one dumped file, matching by literal
  source text (not by address), so it works unmodified on any `translate_words_map_*` variant.
- `tools/rebuild_unique_strings.py` — after a game update, appends newly-seen strings to
  `unique_strings.jsonl` at new idx values without touching existing ones, so already-translated
  `locale/phase*.jsonl` progress never needs remapping.
- `tools/patch_all.py` — imports `expand_locale`'s dictionary builder and applies it to all known
  `translate_words_map_*` files in one pass, in memory (no intermediate per-file JSONL needed).

Pipeline: `translate_words_map_en` --dump--> `strings.jsonl` --(edit `v` field)--> validate with
`qa_check.py` --patch--> new `translate_words_map_en`. For an update with several file variants
(base + `_diff` + `__small` + `__small_diff`), the recommended path is `rebuild_unique_strings.py`
once, then `patch_all.py` to apply the dictionary everywhere at once.

`translate_words_map_en_diff` (the incremental file shipped between game updates) uses the exact
same container/shard format and is handled by the same `info`/`dump`/`patch` commands. The
difference is purely semantic: most shards are sparse (only changed keys are present), and a key
removed since the base version is stored as a single `0xFF` byte instead of being absent — see
`TOMBSTONE` in `wwm_locmap.py` and the `deleted` JSONL field below.

### The `__small` / `__small_diff` variant

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
variant NetEase adds — add its name to `FILES` in `tools/patch_all.py`.

### Deployment gotcha: the installed `_diff` file gets re-verified by the game's own CDN patcher

This is not a tool bug — it's a fact about the live game client that affects anyone trying to
actually play with a patched file, so it's recorded here.

The game install has **two separate copies** of `translate_words_map_en_diff`:

- `Package\HD\oversea\locale\translate_words_map_en_diff` — the one this repo's tools read/write.
  Patches here are **stable**: nothing in-game re-verifies or overwrites it.
- `LocalData\Patch\HD\oversea\locale\translate_words_map_en_diff` — a **separate hot-patch overlay**
  copy that the running game actually loads for the `_diff` layer. `LocalData\Patch\...` only
  contains `_diff` variants (no `en`/`__small`/`__small_diff`), mirroring the `Package` structure.

Every game launch, NetEase's own launcher/patcher (visible in
`LocalData\patch_log\patch_log_*.txt`) runs a `StagePatchList` → `StageCheck` → `StageDownload`
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

**Practical implication**: only `Package\HD\oversea\locale\translate_words_map_en` (base) and
`translate_words_map_en__small` hold permanently — patch and deploy those. The `_diff` layer
(~213k entries in the 2026-09 update, ~26% of total entries, representing text changed/added
since the base package was last rebuilt) cannot currently be made to stick via a simple file
replacement in `LocalData\Patch\...`. If a future game update merges `_diff` content back into
the base package (which NetEase does periodically), whatever fraction of it is already covered
by the translation dictionary becomes permanent automatically at that point, no extra work needed.

### File format (see `docs/FORMAT.md` for the full spec)

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
adding genuinely new keys isn't currently possible (see `docs/FORMAT.md` §4.4 — the H1/H2
derivation from `keyHash`+`seed` hasn't been solved).

### QA checks (`tools/qa_check.py`)

Three checks run per entry when comparing original vs. translated JSONL:

1. `PROMPT_LEAK` — translated value contains a known LLM/MT system-prompt fragment
   (`LEAK_SIGNATURES` tuple — extend this when a new prompt-leak variant is found).
2. `MARKUP` — the multiset of format tokens (`TOKEN` regex: `#E`, `#aabbcc`, `#X` format codes,
   `%s`/`%d`, `{0}`-style placeholders) differs between source and translation. These tokens must
   never be translated, reordered-in-count, or dropped, or the game UI renders incorrectly.
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
reference-only — never modify it. `v` is the only field to edit. `patch` only overwrites entries
present in the given JSONL; everything else is carried over from the source file, so partial
patches are supported.

A key removed since the base version (only seen in `*_diff` files) dumps as a tombstone instead,
with no `v` field:

```json
{"b": 5, "s": 14, "h": "924f075a96b1ccf6", "deleted": true}
```

Never invent a `v` for these or turn them back into text — `patch` re-encodes `"deleted": true`
as the raw `0xFF` sentinel byte regardless of what `v` would otherwise be.
