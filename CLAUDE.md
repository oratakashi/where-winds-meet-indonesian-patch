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
```

There is no test suite; correctness is verified by round-tripping a real
`translate_words_map_en` file through `info` → `dump` → `patch` → `info` and diffing entries.

## Never commit game content

`.gitignore` already excludes `*.gubackup`, `*.map`, `*.cleaned`. Do not add `strings.jsonl`,
`translate_words_map_*`, or any dump/patch output to git — all such content is NetEase's
copyrighted game text. Working files like `strings.jsonl` and `translate_words_map_en` present
in the working tree are local scratch data, not repo assets.

## Architecture

Two scripts implement one pipeline:

- `wwm_locmap.py` — the container/shard codec (`read_container`/`write_container`,
  `parse_shard`/`rebuild_shard`) plus a thin CLI (`info`/`dump`/`patch`) over it.
- `tools/qa_check.py` — a standalone validator that diffs an original JSONL dump against a
  translated one and flags three defect classes before a patch is repacked.

Pipeline: `translate_words_map_en` --dump--> `strings.jsonl` --(edit `v` field)--> validate with
`qa_check.py` --patch--> new `translate_words_map_en`.

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

## JSONL record shape

```json
{"b": 1, "s": 3, "h": "c5cadbb857eee8b4", "v": "The leaf?"}
```

`b`/`s` (block/slot) form the entry's address and are required by `patch`. `h` (keyHash) is
reference-only — never modify it. `v` is the only field to edit. `patch` only overwrites entries
present in the given JSONL; everything else is carried over from the source file, so partial
patches are supported.
