# Technical Overview

How the `translate_words_map_*` localization files work, why the patch is built the way it is, and
its known limitations. This is a summary; the exhaustive reverse-engineered specification is
[`Format-Spec.md`](../Obsidian-Vault/knowladge/Format-Spec.md).

- [Project Status](#project-status)
- [Why the Game Must Be Set to English](#why-the-game-must-be-set-to-english)
- [In-Place Mutation Without Hash Reversal](#in-place-mutation-without-hash-reversal)
- [Binary Format Summary](#binary-format-summary)
- [Critical Gotcha: Relative Pointer Arithmetic](#critical-gotcha-relative-pointer-arithmetic)
- [Known Limitations & Deployment Gotchas](#known-limitations--deployment-gotchas)

---

## Project Status

| Capability | Status | Notes |
|---|---|---|
| **Binary format specification** | Complete | Fully reverse-engineered; see [`Format-Spec.md`](../Obsidian-Vault/knowladge/Format-Spec.md) |
| **Round-trip integrity** | Verified | Byte-identical across base, `_diff` and `__small` variants |
| **Container decoding** | Supported | Decompresses zstd payloads and parses SwissTable shards into JSONL |
| **Container encoding** | Supported | Rebuilds shards and zstd blocks with configurable compression levels |
| **Container v2 (China client)** | Supported | Encrypted zstd dictionary; copied verbatim when patching |
| **In-place value editing** | Supported | Preserves control bytes, slot indices and hash structures |
| **Arbitrary key insertion** | Unsupported | Requires reversing the proprietary 64-bit hash derivation |

**Snapshot.** Entry counts change with every game update. The current target, **game v2.3
hotfix** (October 2026), has **1,122,784 entries across 4,387 blocks** in the primary map plus a
sparse `_diff` (12,934 entries, 1,919 tombstones), and no `__small` tables. For reference, the
September 2026 (v2.2) release had 826,388 entries across 3,230 blocks, a sparse `_diff` (212,117
active entries) and an isolated `__small` table (4,009 entries; see
[Format-Spec §4.6](../Obsidian-Vault/knowladge/Format-Spec.md#46-the-__small--__small_diff-variant-a-separate-table-not-part-of-_diff)).

---

## Why the Game Must Be Set to English

The localization files **do not store the English source strings**. They only hold a one-way map:

$$\text{u64 keyHash} \longrightarrow \text{localized string}$$

At runtime the Messiah Engine takes the English string from game code or data assets, computes its
64-bit hash and looks it up in `translate_words_map_en`.

- With **Game Language = English**, the engine looks up hashes of the English text — which this
  patch has replaced with Indonesian values.
- With any other language, the engine hashes that language's strings, misses every entry in this
  map and falls back to the raw text.

So English is an architectural requirement of the engine's lookup design, not a tool limitation.
(The China client is different: the CN patch replaces the Mandarin values directly, so the game
stays on Chinese.)

---

## In-Place Mutation Without Hash Reversal

Because keys aren't stored, translating only means replacing the **values** of existing keys. The
lookup structure is left untouched:

```text
Preserve: Header + Control Bytes (ctrl) + Sentinel + Padding + Slot Array (keyHash)
Rebuild:  New String Blob (UTF-8)
Update:   relOffset + byteLen in each active slot
```

Hash-table probing stays 100% valid, so the proprietary 64-bit hash function never has to be
reverse-engineered for translation.

---

## Binary Format Summary

```text
File Container:
  u32 magic = 0xDEADBEEF
  u32 version = 1            (2 = China client, encrypted zstd dictionary)
  u32 blockCount
  u32 reserved = 0
  u32[blockCount] endOffset        <- Offsets relative to end of this table

Payload Blocks:
  u8  codec = 4 (zstd)
  u32 compressedSize
  u32 uncompressedSize
  u8[compressedSize] payload

Block Layout:
  Block 0    = Index Block : u64 totalEntries, u64 dataBlockCount, u32[] ids
  Block 1..n = Shard Blocks: Abseil SwissTable (absl::flat_hash_map)
      u64 capacity | u64 size | u64 seed
      u8[capacity] ctrl (0x80 = empty, <0x80 = occupied H2 hash byte)
      u8 0xFF sentinel + u8[15] cloned ctrl + 8-byte alignment padding
      slot[capacity] { u64 keyHash; u32 relOffset; u32 byteLen; }
      <string blob UTF-8>

Shard Routing:
  shardIndex = (keyHash % dataBlockCount) + 1
```

In `*_diff` files, a key removed since the base version is stored as a single `0xFF` byte
(tombstone) and dumps as `"deleted": true`.

---

## Critical Gotcha: Relative Pointer Arithmetic

`relOffset` is a **self-relative pointer**, computed from the address of the `relOffset` field
itself:

$$\text{valueStart} = \text{address}(\text{slot}[i].\text{relOffset}) + \text{relOffset}$$

$$\text{value} = \text{block}[\text{valueStart} : \text{valueStart} + \text{byteLen}]$$

This C++ idiom allows memory-mapping (`mmap`) without pointer relocation. Treating `relOffset` as
an absolute offset from the block or string-blob start produces corrupted, overlapping strings.

---

## Known Limitations & Deployment Gotchas

### 1. NetEase's patcher restores locale files

The game keeps two copies of `translate_words_map_en_diff`:

- `Package\HD\oversea\locale\` — never re-verified on Steam.
- `LocalData\Patch\HD\oversea\locale\` — the overlay the game actually loads for the `_diff` layer.

| Client | Normal launch | Game update (incl. hotfixes) |
|---|---|---|
| **Steam** | `LocalData` `_diff` is hash-checked against NetEase's CDN manifest and re-downloaded (`StagePatchList` → `StageCheck` → `StageDownload`). The base file in `Package\` is untouched. | NetEase hot-patches only touch the small `LocalData` overlay; the base file in `Package\` is replaced only by a Steam depot update. |
| **Official launcher** | Nothing is restored (tested 2026-09-27) — the whole patch, including `_diff`, works. | Every file of each changed package is hash-checked; patched locale files are re-downloaded (observed 2026-10-28). |
| **Android** | Nothing is restored. | Same as the official launcher, and extra files in `locale/` (e.g. backups) are **deleted**. |
| **China client** | Not restored (tested 2026-10-20). | A CN update replaces the locale files even when their content didn't change (2026-10-25). |

Blocking the CDN hostname doesn't help on Steam: the client falls back to a cached manifest and
downloads from other hosts. The patch lists are encoded (`HEXB_` + base64), so editing them isn't
practical. `locale\patch_version.json` is unrelated. If NetEase merges `_diff` content back into
the base package, whatever part of it is already translated becomes permanent on Steam too.

### 2. New strings after updates

Strings added in a game update stay in English until they are extracted, translated and released.
`tools/rebuild_unique_strings.py` reconciles the new text without breaking existing progress.

### 3. Key insertion

Creating brand-new keys requires deriving the H1/H2 hash split from `keyHash` + `seed`, which is
unsolved ([Format-Spec §4.4](../Obsidian-Vault/knowladge/Format-Spec.md)).

### 4. Codec exclusivity

Only `codec = 4` (zstd) is implemented. Any other codec is rejected with an explicit error rather
than producing corrupted output.

### 5. Text-only scope

Only text containers are processed. Audio, voice-over, artwork with baked-in text and fonts live in
separate archive packages. On the China client, cutscene/story subtitles are read from integrity-
checked data packs, not from these files, so they stay in Chinese
([Format-Spec §9](../Obsidian-Vault/knowladge/Format-Spec.md)).
