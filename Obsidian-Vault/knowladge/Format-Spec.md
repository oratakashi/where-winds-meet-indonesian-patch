# Format Spec — `translate_words_map_*`

Reverse-engineered specification of the Where Winds Meet (NetEase, Messiah Engine)
localization file format. All fields are **little-endian**.

Fully verified via round-trip on 963,050 entries: parse → dump → repack → parse
produces `(keyHash, value)` pairs that are 100% identical to the source.

Related: [[Glossary]] (translation conventions), [[Resume-Procedure]] (workflow that
consumes this format).

---

## 1. Container

```
offset  type                 value
0x00    u32                  magic       = 0xDEADBEEF
0x04    u32                  version     = 1
0x08    u32                  blockCount  (e.g. 3763)
0x0C    u32                  reserved    = 0
0x10    u32[blockCount]      endOffset
```

`endOffset[i]` is the **end** offset of block *i*, relative to the end of the
offset table itself:

```
tableEnd   = 0x10 + blockCount * 4
blockStart = tableEnd + (i > 0 ? endOffset[i-1] : 0)
blockEnd   = tableEnd + endOffset[i]
```

Useful invariant for a quick sanity check:
`tableEnd + endOffset[blockCount-1] == filesize`.

## 2. Blocks

Each block has a 9-byte header, then a compressed payload:

```
u8   codec              4 = zstd (the only codec observed)
u32  compressedSize
u32  uncompressedSize
u8[compressedSize]      zstd payload (magic 28 B5 2F FD)
```

Compression is applied **per block**, not per file. This is why the file as a
whole isn't recognized as a zstd archive by `file(1)`.

## 3. Block 0 — index

```
u64  totalEntries        e.g. 963050
u64  dataBlockCount      e.g. 3762  (= blockCount - 1)
u32[dataBlockCount] ids  values 1..dataBlockCount, sequential
```

In practice this is just metadata; the `ids` array is redundant.

## 4. Blocks 1..n — shard hash maps

Each shard is a **SwissTable**-style hash map (`absl::flat_hash_map`) serialized
as-is so it can be `mmap`ed directly.

```
u64  capacity        e.g. 511   (always 2^k - 1)
u64  size            number of occupied slots, e.g. ~256
u64  seed            e.g. 0x595896DC — identical across the entire file

u8[capacity]  ctrl   0x80 = empty, 0xFE = deleted (absl kDeleted);
                     < 0x80 = occupied (H2, 7 bits). Parsers treat every
                     byte >= 0x80 as "no entry here".
u8            0xFF   sentinel
u8[15]        clone  copy of the first 15 bytes of ctrl (for SIMD probing)
u8[...]       pad    padding up to a multiple of 8

slot[capacity] {     // 16 bytes per slot
    u64 keyHash
    u32 relOffset
    u32 byteLen
}

<string blob>        UTF-8, no NUL terminator
```

### 4.1 Relative pointer — the easiest part to get wrong

`relOffset` is **not** an offset from the start of the block, nor from the
start of the string blob. It's computed from **the address of the
`relOffset` field itself**:

```
valueStart = addressOf(slot.relOffset) + slot.relOffset
value      = block[valueStart : valueStart + slot.byteLen]
```

With `S` = the start offset of the slot array and `i` = the slot index:

```
field      = S + i * 16 + 8
valueStart = field + relOffset
```

A classic C++ idiom (similar to `boost::offset_ptr`): the block can be
memory-mapped with zero pointer relocation. Treating this as an absolute
offset produces overlapping/garbled decoded strings — that's the signature
symptom of this exact mistake.

### 4.2 Sharding

```
shardIndex = (keyHash % dataBlockCount) + 1
```

Confirmed consistent across blocks 1, 500, 2000, and 3762.

Implication: if you **add** a new key, it must be placed in the correct
shard. If you only **change the value** of an existing key (the normal case
when building a language pack), sharding doesn't need to be touched at all.

### 4.3 What is NOT stored: the key itself

This file only stores `u64 keyHash → translated string`. The English source
text is never written to disk.

Two major consequences:

**a. The game must be set to English.** The runtime takes the current
English string from its own assets, hashes it (64-bit), and swaps in
whatever this file's lookup returns. Switch the language to German → the
hashes differ → every lookup misses → the text falls back to German. The
"Game Language = English" requirement for any translation pack is therefore
an architectural fact, not a limitation of this tool.

**b. Building a pack is far simpler than it looks.** You don't need to know
the hash function. Just keep `ctrl`, `keyHash`, and slot positions exactly
as they are, and rewrite the string blob plus `relOffset`/`byteLen`. The
lookup structure is never touched, and lookups keep working perfectly.

### 4.4 The hash function: unsolved (and unnecessary to solve)

`ctrl[i]` (H2) is clearly not a simple derivative of `keyHash` — 58 shift
variants were tested, with the highest correlation only ~2.7% (noise level).
There's likely some mixing with `seed` before H1/H2 are derived.

This does **not** block building a language pack, because the approach in
§4.3(b) never recomputes slot placement. What can't currently be done
without solving the hash: **adding a genuinely new key** to the map.

### 4.5 The `_diff` variant: incremental file between game updates

`translate_words_map_en_diff` uses **the exact same container and shard
format** as the full file — no binary structural difference. The only two
differences, both verified on real files:

| Game version       | File size   | Shards | Entries present | Index `totalEntries` | Tombstones |
| ------------------ | ----------- | ------ | --------------- | -------------------- | ---------- |
| pre-2026-09 update | ~2.4 MB     | 3,762  | 31,471          | 973,284              | —          |
| 2026-09 update     | 9,229,275 B | 3,229  | 212,117         | 984,927              | 2,450      |

(The `_diff` shard count equals the base file's of the same version —
3,229 for the 2026-09 base, which has 826,388 entries.)

1. **Sparse.** Most shards are nearly empty — this file only carries keys
   that *changed* since the base version, not the whole map. So
   `entries parsed` (counted from each shard's `size`) will be much lower
   than `totalEntries` in the index block; that's not a parser bug, it's a
   property of diff files.
2. **Tombstone.** A key that was **deleted** since the base version keeps
   its slot (ctrl/keyHash untouched), but its value is replaced by a single
   `0xFF` byte — not valid UTF-8, deliberately used as a sentinel because
   `0xFF` never appears as a UTF-8 lead byte. Detect it with
   `value == b'\xff'`.

`wwm_locmap.py` handles this transparently via the `TOMBSTONE` constant in
`cmd_dump`/`cmd_patch` — no separate CLI flag needed for `_diff` vs. full
files.

### 4.6 The `__small` / `__small_diff` variant: a separate table, not part of `_diff`

A 2026-09 game update added `translate_words_map_en__small` and
`__small_diff`. Both use **the identical container/shard format** as the
main file — confirmed via a byte-identical dump→patch→re-parse round trip,
with zero format changes. The difference isn't binary structure, it's
content:

- This is a **separate** hash table, with its own address space (`b`/`s`)
  and shard count (16 shards in `__small` vs. thousands in the main file) —
  not a delta/diff of the main file despite the similar name. Likely a
  "hot" subset (loading screen, splash, etc.) loaded earlier/faster.
- Measured on the 2026-09 update: of ~4,000 keys in `__small`, **98% share
  a hash** with `translate_words_map_en`, but **193 of those have a
  different value** than the main file's copy (a stale/contextual
  duplicate — treat it as its own text, don't assume it matches the main
  file), and **27 keys exist only in `__small`**, in neither the main file
  nor the pre-update version.
- `__small_diff` follows the same pattern as `_diff` (see §4.5) — in the
  sample examined, it was completely empty (0 data shards; the index is
  metadata-only with no changes).

Practical implication: never assume that a shared `keyHash` between files
means the current text is the same. Always dump and match each file
independently against its source text (see `tools/expand_locale.py` /
`tools/patch_all.py` and [[Resume-Procedure]] — the translation
dictionary is keyed to literal source text, not to hash/address, so this
is automatically correct for any file variant).

### 4.7 The `_mobile` variant

`translate_words_map_en_mobile` (added to the repo 2026-09-22) is
**byte-identical** to `translate_words_map_en` of the same version. It
is in the global profile's `files` list (`tools/profiles.py`) anyway, so if a future update
makes the two diverge, it still gets patched independently by source
text. Re-check with a plain byte compare after each game update.

---

## 5. Decode flow summary

1. Read and validate the container header, get the `endOffset` table.
2. For each block: read the 9-byte header, decompress the zstd payload.
3. Block 0 → metadata. Blocks 1..n → shards.
4. Per shard: read `capacity/size/seed`, get `ctrl[capacity]`, verify the
   `0xFF` sentinel at `24 + capacity`.
5. Compute `S` = the slot array offset (8-byte aligned after the sentinel +
   15-byte clone).
6. For each `i` where `ctrl[i] < 0x80`: read the slot, resolve the relative
   pointer, read `byteLen` bytes as the UTF-8 value.
7. Sanity check: the number of occupied slots must equal `size`.

## 6. Encode flow summary

1. Copy `header + ctrl + sentinel + clone + pad + slot array` **as-is**.
2. Build a new string blob; dedup identical strings to save space (~4% in
   the original file, since it does store duplicates).
3. For each occupied slot, rewrite `relOffset = blobPos - field` and
   `byteLen = len(value)`.
4. Compress each block with zstd, rebuild the `endOffset` table, write the
   container.

`keyHash`, `ctrl`, `capacity`, `size`, and `seed` never change.

`wwm_locmap.py patch` addresses entries by `b`/`s`, which is only valid
for the exact file version that was dumped. It therefore checks each
record's `h` against the slot's `keyHash` and refuses to write if they
differ (a JSONL dumped from another game version), and also refuses a
`b`/`s` that isn't an occupied slot.

## 7. Deployment: which installed copies stay patched

Not a format property, but it decides which patched files are useful:

- `Package\HD\oversea\locale\translate_words_map_en` and
  `translate_words_map_en__small` — **stable**. Nothing re-verifies them;
  a patched copy stays until the next game update replaces it.
- `LocalData\Patch\HD\oversea\locale\translate_words_map_en_diff` — the
  hot-patch overlay copy the running game actually loads for the `_diff`
  layer. Its behaviour **depends on the client**:
  - **Official WWM launcher** (NetEase's standalone launcher) — **stable**.
    Retested 2026-09-27: a patched copy was not reverted across launches.
  - **Steam client** — reverted. On every launch the NetEase patcher
    (`StagePatchList` → `StageCheck` → `StageDownload`, logged in
    `LocalData\patch_log\patch_log_*.txt`) compares it against a CDN
    checksum manifest and silently re-downloads it if it differs — a
    patched copy was observed reverted within minutes. The manifest falls
    back to a cached copy when the CDN is unreachable, so blocking one host
    doesn't help.

All earlier testing (before 2026-09-27) was done on Steam only, which is why
this was first recorded as a limitation for every client. Why the two
clients differ hasn't been investigated.

So on the official launcher all patched files stick, `_diff` included
(212,117 entries in the 2026-09 update). On Steam only the base and
`__small` patches stick; the `_diff` layer there becomes permanent only
when NetEase folds it into the base package in a later update.

## 8. Container v2 — WWM China (燕云十六声), found 2026-10-20

The CN client (`D:\yysls\yysls_fast\LocalData\Patch\HD\v2\locale\`) ships
`translate_words_map_zh_cn` / `__small` (+ near-empty `_diff`s) in a **version 2**
container. Sample files live in `cn/`; `wwm_locmap.py` reads and writes v2 (see below).

```
u32 magic = 0xDEADBEEF
u32 version = 2
u32 mode               # 4 = plain zstd blocks (codec 4), 6 = zstd + encrypted dictionary (codec 5)
-- mode 6 only --
u32 dictSize           # 0x40000 for translate_words_map_*, 0x10000 (or less) for data*/index*
u8[dictSize]           # zstd dictionary (magic 37 A4 30 EC + dictID ...) XOR keystream
-- both modes --
u64 blockCount         # (v1 had u32 blockCount + u32 reserved in the header instead)
u32[blockCount]        # block end offsets, relative to the end of this table (same as v1)
blocks                 # u8 codec | u32 cs | u32 us | payload — codec 5 = zstd frame with dictID
```

- Block 0 is the same index as v1 (`u64 total, u64 dataBlockCount, u32[] ids`); shards are
  expected to be the v1 SwissTable layout (unverified until a dictionary is decrypted).
- `zh_cn`: 8133 blocks, dictID `3fc25256`; `__small`: 87 blocks, dictID `3c8b2d1b`.
- **One keystream for every file.** All 38 mode-6 containers in the client (the two locale maps
  plus `data*`/`index*` in `HD\v2`) decrypt their first 8 bytes to `37 A4 30 EC <own dictID>`
  with the same keystream (`36 89 40 07 d9 d3 72 38 ...`). It is not a repeating XOR key
  (no period ≤ 196 K) and not GF(2)-linear (Berlekamp–Massey ≈ n/2) — a real stream cipher.
- Many-time-pad statistics over the 38 files recover ~95 % of keystream bytes 0..64 K
  (dictionary *content*, e.g. Chinese text at offset 300 of the zh_cn dict), saved as
  `cn/work/keystream_v2_statistical_64k.bin`. They can't recover the ~160-byte entropy-table
  header (high-entropy bitstream) nor bytes 64 K..256 K (only the two locale dicts are that long).
- **Solved 2026-10-20**: scanning the running client (login screen, elevated terminal —
  the CN client runs as admin) with `cn/tools/find_dict_in_dump.py --pid <yysls.exe>` found 34
  plaintext dictionaries, including the 256 KB `zh_cn` one → full 262,144-byte keystream in
  `cn/work/keystream_v2.bin` (gitignored). The `__small` dictionary was not in memory but
  decrypts correctly with it, confirming the single-keystream model. (The statistical guess
  above turned out 99.9 % correct on the first 64 K.)
- Shards are byte-for-byte the v1 SwissTable layout: `wwm_locmap.py info/dump/patch` handle v2
  via `read_container_ex`/`write_container_ex`; `patch` copies the encrypted dictionary
  verbatim and recompresses blocks as codec 5 with the same dictID. Round trip (50 edits)
  verified on both files: 0 mismatches, header + encrypted dict byte-identical.
- Sizes (CN, 2026-10-20): `zh_cn` 2,081,685 entries / 8132 shards, `__small` 21,932 entries /
  86 shards (every `__small` text also occurs in `zh_cn`); 627,412 unique texts, ~10.9 M
  characters; a CN `unique_strings.jsonl` would be ≈ 47 MB. Markup is the global set
  (`#E #Y #G #R`, `#rrggbb`, `%s`, `{0}`, `<LINK…>`/`<IMAGE…>`) plus `$STEADY_…`-style
  variables and `#C|151>` forms that `qa_check.TOKEN` doesn't cover yet.
- Deployment caveat: the CN locale files live in the client's `LocalData\Patch\HD\v2\locale\`
  overlay — the folder the Steam patcher re-verifies on the global client. **Tested 2026-10-20:**
  the CN client loads repacked v2 files (Latin text renders fine) and the CN launcher's
  `StageCheck` (4,852 files passed) did not revert them. Main-menu `退出游戏` is artwork, and
  the account login dialog is the NetEase SDK UI — neither comes from these files.
- **Game updates rewrite them anyway** (2026-10-25, client `20261025072716`): the update left the
  locale files' content unchanged (byte-identical to the previous version) but rewrote
  `zh_cn` and `__small` with the originals, which removed the installed patch. After every CN
  update, hash the client files against `cn/*.before`. If they're identical, just rebuild
  (`patch_all.py --profile cn`) and reinstall. No migration is needed.

## 9. WWM China: story/cutscene subtitles don't use `translate_words_map` (found 2026-10-25)

On the CN client (PC, `language = zh_cn`, no language option in game), the patch translates
the UI, but **story and cutscene subtitles stay in Mandarin**. This is not a translation gap:
the subtitle `怎么睡着了呀` occurs on 7 keys of `translate_words_map_zh_cn`, and all 7 are
translated in the installed patch.

**Where the displayed text comes from.** The data packs `LocalData\Patch\HD\v2\index*` /
`data*` are the same v2 container, **mode 6**, with the same keystream as the locale files
(§8). `wwm_locmap.read_container_ex` decodes all 46 of them (~4.5 GB raw). A cutscene timeline
entry (`EntityTextSpan`, e.g. `data205` block 1934) stores, in a tagged serialization
(`u16 nameLen | name | u8 type | value`):

```
text           04 <u64 LE keyHash>          e.g. 0x647cdf16558b6253 — the translate_words_map key
formatted_text 08 <u16 len> <UTF-8>        "快点起来，快点起来！怎么睡着了呀！" (pre-baked Mandarin)
format_result  08 ... "GoodFormat"
```

The global client (`HD\oversea`, container v1) carries the **same Mandarin `formatted_text`**
and translates through the `text` key. The CN client shows `formatted_text` directly, which
makes sense because Mandarin is its source language. `data3` also holds voice-over tables
(`vo_*`, `Event/Chinese/*.bnk`, inline text as `20 <u16 len> <UTF-8>`, no key). The manifest
(`HD\v2\MANIFEST`, mode 4, one block) names the related tables:
`cutscene_localization.{text_2_text_no, md5_2_text_no, ukey_2_tid}`, `dialog_timelines.*`,
`video_locale`, `entry_collect_movie_subtitle_data`.

**Why it can't be patched (dead end, unless NetEase changes it).** The launcher verifies the
**content** of every `HD\v2` data/index pack on every launch, *before* the game starts:

```
StageLoadMpk:verify_2 res=fail filename=HD/v2/data205
Patcher:clear_local_patchmd5_and_list reason=corrupt_non_mpk_files   -> deleted + re-downloaded
```

Two PoCs failed this way: one with a 1-block change (+17 bytes), and one with the same size and
mtime and a single block (768 bytes) recompressed to its exact original length. The reference
lists (`essential.txt`, `patchmd5_*.txt`, `patchlist_*.txt`) are encrypted (`HEX_` + base64).
`.mpk` files are checked as well, per 2 MB chunk against the `pak` table of `Patch.mpkdb`
(`v0` path id, `v1` chunk no., `v2` chunk size, `v3` 64-bit checksum; not xxh64/xxh3/city/
farm/murmur3/md5/sha1). Getting past this means forging NetEase's integrity data on an online
game with anti-cheat. That puts accounts at risk, so it was not pursued. The locale folder is
**not** part of this check, which is why the UI patch holds.

**Open:** whether the Android CN client stores and verifies subtitles the same way (the owner is
checking, 2026-10-25). Players who want translated story should use the global client, where
subtitles go through the key.
