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

u8[capacity]  ctrl   0x80 = empty slot; < 0x80 = occupied (H2, 7 bits)
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
differences, both verified on a real file (2.4 MB, 3762 shards, 31,471
entries present out of the index block's claimed `totalEntries` of
973,284):

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
independently against its source text (see
[[Glossary|translation_logs]]/`tools/expand_locale.py` /
`tools/patch_all.py` — the translation dictionary is keyed to literal
source text, not to hash/address, so this is automatically correct for any
file variant).

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
