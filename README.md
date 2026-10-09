# Where Winds Meet — Localization Toolkit & Translation Pipeline

[![Python Version](https://img.shields.io/badge/python-3.8%2B-blue.svg)](https://www.python.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Engine: Messiah](https://img.shields.io/badge/Engine-NetEase%20Messiah-orange.svg)]()
[![Codec: Zstandard](https://img.shields.io/badge/Codec-Zstandard%20(zstd)-green.svg)]()
[![Round-Trip: Byte-Identical](https://img.shields.io/badge/Round--Trip-Byte--Identical-brightgreen.svg)]()

A high-performance reverse-engineering toolkit, binary format specification, and localization pipeline for **Where Winds Meet** (NetEase / Everstone Studio, built on the Messiah Engine).

This repository provides tools and documentation to unpack, inspect, translate, quality-check, and repack the game's binary localization containers (`translate_words_map_*`). It also hosts the tooling and dictionary database for the ongoing **Indonesian Community Translation Project**.

> [!NOTE]
> This repository distributes **reverse-engineering tools and format specifications only**. It does not distribute copyrighted raw game assets or commercial packs. Review the [Legal & Anti-Tamper Notice](#legal--anti-tamper-notice) before using this toolkit.

```
translate_words_map_en  ──dump──>  strings.jsonl  ──edit / translate──>  strings.jsonl
                                                                               │
translate_words_map_en  <──patch───────────────────────────────────────────────┘
```

NetEase frequently ships incremental updates via companion files (`_diff` for delta changes, and `__small` / `__small_diff` for isolated sub-tables). All variants share the exact same binary container and shard codec. Refer to [Handling Game Updates & File Variants](#handling-game-updates--file-variants) to manage upstream updates without losing translation progress.

---

## Table of Contents

- [Project Status](#project-status)
- [Player Installation Guide](#player-installation-guide)
  - [PC — Steam](#pc--steam)
  - [PC — Official WWM Launcher](#pc--official-wwm-launcher)
  - [Android](#android)
    - [Method A — File manager on the phone](#method-a--file-manager-on-the-phone-no-pc-needed)
    - [Method B — ADB from a PC](#method-b--adb-from-a-pc-step-by-step-for-beginners)
  - [Reverting to English / Handling Game Updates](#reverting-to-english--handling-game-updates)
- [Prerequisites & Installation](#prerequisites--installation)
- [Quickstart Guide](#quickstart-guide)
  - [1. Locate Game Files](#1-locate-game-files)
  - [2. Inspect Binary Header](#2-inspect-binary-header)
  - [3. Dump to JSON Lines](#3-dump-to-json-lines)
  - [4. Translate & Edit](#4-translate--edit)
  - [5. Automated QA Verification](#5-automated-qa-verification)
  - [6. Repack into Binary](#6-repack-into-binary)
- [Daily Translation Workflow (Cheat Sheet)](#daily-translation-workflow-cheat-sheet)
- [Handling Game Updates & File Variants](#handling-game-updates--file-variants)
- [Technical Architecture & Binary Format](#technical-architecture--binary-format)
  - [Why Runtime Requires English Language](#why-runtime-requires-english-language)
  - [In-Place Mutation Without Hash Reversal](#in-place-mutation-without-hash-reversal)
  - [Binary Format Summary](#binary-format-summary)
  - [Critical Gotcha: Relative Pointer Arithmetic](#critical-gotcha-relative-pointer-arithmetic)
- [Translation Guidelines & Engine Syntax](#translation-guidelines--engine-syntax)
  - [Messiah Engine Format Tokens](#messiah-engine-format-tokens)
  - [Proper Nouns & Pinyin Retention](#proper-nouns--pinyin-retention)
  - [Machine Translation & LLM Quality Control](#machine-translation--llm-quality-control)
  - [Oversized Lore Entries](#oversized-lore-entries)
- [Known Limitations & Deployment Gotchas](#known-limitations--deployment-gotchas)
- [Legal & Anti-Tamper Notice](#legal--anti-tamper-notice)
- [Repository Structure](#repository-structure)
- [Contributing](#contributing)
- [License](#license)

---

## Project Status

| Capability | Status | Notes |
|---|---|---|
| **Binary Format Specification** | Complete | Fully reverse-engineered; see [`Format-Spec.md`](Obsidian-Vault/knowladge/Format-Spec.md) |
| **Round-Trip Integrity** | Verified | Byte-identical verification across base, `_diff`, and `__small` variants |
| **Container Decoding** | Supported | Decompresses zstd payloads and parses SwissTable shards into JSONL |
| **Container Encoding** | Supported | Rebuilds shards and zstd blocks with configurable compression levels |
| **In-Place Value Editing** | Supported | Preserves control bytes, slot indices, and hash structures |
| **Arbitrary Key Insertion** | Unsupported | Requires reversing the proprietary 64-bit hash derivation |

**Snapshot Reference**: Tested against global game builds (NetEase Messiah Engine). Localization entry counts vary per game update as keys and shards fluctuate. For reference, the September 2026 global release snapshot contains **826,388 entries across 3,230 blocks** in the primary map, accompanied by a sparse `_diff` file (212,117 active entries) and an isolated `__small` table (4,009 entries; see [§4.6 of Format-Spec.md](Obsidian-Vault/knowladge/Format-Spec.md#46-the-__small--__small_diff-variant-a-separate-table-not-part-of-_diff)).

---

## Player Installation Guide

This section is for players who just want **to install the patch**, without running any Python
tooling. The translated files are rebuilt automatically by CI on every change to `locale/` and
published as a single rolling GitHub Release (tag `latest`):

**[⬇ Download the Latest Patch (`wwm-indonesian-patch.zip`)](../../releases/tag/latest)**

> [!WARNING]
> **Always back up the original files before overwriting anything.** Copy the original
> `translate_words_map_en`, `translate_words_map_en_diff`, `translate_words_map_en__small`, and
> `translate_words_map_en__small_diff` to a separate folder first. If a game update breaks the
> mod or causes crashes, restore these originals.

> [!NOTE]
> Translation progress is still ongoing (see [`Current-Status.md`](Obsidian-Vault/progress/Current-Status.md)
> for current completion percentages) — some text may still appear in English or Chinese; this is expected.

### PC — Steam

1. Open Steam → right-click **Where Winds Meet** → **Manage → Browse Local Files**. This opens
   the game's installation folder (typically
   `<SteamLibrary>\steamapps\common\Where Winds Meet\`).
2. Navigate to the `Package\HD\oversea\locale\` subfolder.
3. Back up the existing files there (see warning above), then extract the contents of
   `wwm-indonesian-patch.zip` into this folder, overwriting the files with matching names:
   `translate_words_map_en`, `translate_words_map_en_diff`, `translate_words_map_en__small`,
   `translate_words_map_en__small_diff`.
4. Launch the game through Steam as usual.
5. In-game: **Settings → Language → Game Language = English** (required — see
   [Why Runtime Requires English Language](#why-runtime-requires-english-language)).

> [!IMPORTANT]
> **Steam only:** on the Steam client, NetEase's patcher **silently restores
> `translate_words_map_en_diff` to its original version** on every game launch (the
> `StagePatchList`/`StageCheck` process), so text that lives in the `_diff` layer stays in English.
> This is outside this mod's control. The base `translate_words_map_en` and `__small` files remain
> permanent and are unaffected. The official WWM launcher does **not** do this — see
> [Known Limitations & Deployment Gotchas](#known-limitations--deployment-gotchas).

### PC — Official WWM Launcher

1. Find the launcher's installation folder (right-click the launcher shortcut → **Open file
   location** if you're unsure). Inside it, the game lives in a subfolder named after the
   **variant you downloaded** — `wwm_lite` for the Lite download, or the matching standard folder
   for the Standard download. Below, `<InstallDir>\<variant>` stands for that folder, e.g.
   `<InstallDir>\wwm_lite`.
2. Open the locale folder inside the variant folder:

   ```
   <InstallDir>\<variant>\LocalData\Patch\HD\oversea\locale\
   ```

   For example: `INSTALLATION_FOLDER\wwm_lite\LocalData\Patch\HD\oversea\locale\`.
3. Back up the original `translate_words_map_*` files in that `locale\` folder.
4. Extract the contents of `wwm-indonesian-patch.zip` into this `locale\` folder, overwriting
   files with matching names (`translate_words_map_en_diff` and any other `translate_words_map_en*`
   files that exist there).
5. The base files also exist under `<InstallDir>\<variant>\Package\HD\oversea\locale\`. Back up
   and overwrite `translate_words_map_en`, `translate_words_map_en__small` and
   `translate_words_map_en__small_diff` there too, as in the Steam steps above.
6. Launch the game through the launcher as usual.
7. Set **Settings → Language → Game Language = English** in-game.

> [!TIP]
> Unlike the Steam client, the official launcher does **not** restore the patched `_diff` file on
> launch (tested 2026-09-27), so all four files — including the `_diff` layer — stay translated.
> This is the most complete way to play with the patch on PC. See
> [Known Limitations & Deployment Gotchas](#known-limitations--deployment-gotchas).

### Android

The Android build of Where Winds Meet stores its locale files under the shared
`Android/data/<package>/` folder. **Root is not required** — `Android/data` is part of normal
internal storage, not the app's private `/data/data` sandbox. The only catch is that newer Android
versions hide this folder from ordinary file managers.

The target folder is typically:

```
Internal Storage/Android/data/com.netease.yysls/files/LocalData/Patch/HD/oversea/locale/
```

(the exact package name and sub-path can vary by release region — look for the folder containing
files named `translate_words_map_en*`.)

> [!IMPORTANT]
> **Why `Android/data` is tricky on Android 10 and newer:** scoped storage (introduced in
> Android 10, enforced from Android 11) stops regular file-manager apps from seeing or writing inside
> `Android/data`, and on Android 13+ even the system "allow access to this folder" prompt is blocked
> for it. This is an OS restriction, not a need for root — pick one of the two methods below.
> Behavior also differs between OEM skins (MIUI/HyperOS, One UI, ColorOS, etc.), so if one method is
> blocked on your device, try the other.

> [!TIP]
> Quick thing to try first: connect the phone to a PC with a USB cable in **File Transfer (MTP)**
> mode and open it in Windows Explorer. On many devices `Android/data` is still visible and writable
> this way — if it is, just back up and copy the files there and skip the methods below.

#### Method A — File manager on the phone (no PC needed)

The easiest route is a file manager that can open `Android/data` on its own, including on
Android 13 and newer. **[Files by Marc apps & software](https://play.google.com/store/apps/details?id=com.marc.files)**
(`com.marc.files`, free on the Play Store) has been tested to work for this — no root, no PC, and
no extra helper apps needed.

1. Install **[Files](https://play.google.com/store/apps/details?id=com.marc.files)** from the Play
   Store and open it. Grant the storage permission it asks for.
2. Download `wwm-indonesian-patch.zip` on the phone and extract it (the app can open ZIP files, or
   use any archive app). Note where the extracted `translate_words_map_en*` files are, e.g.
   `Download/wwm-indonesian-patch/`.
3. In the app, go to **Internal Storage → Android → data** and open the game's folder, then the
   locale folder shown above. If the app shows a prompt asking for access to the folder, allow it.
4. **Back up** the existing `translate_words_map_en*` files by copying them to another folder, e.g.
   `Download/wwm-backup/`.
5. Copy the extracted patch files into the locale folder, overwriting the files with matching names.
6. Force-close the game, relaunch it, and set **Settings → Language → Game Language = English**.

If this doesn't work on your device (some OEM skins are stricter), use Method B below.

#### Method B — ADB from a PC (step by step, for beginners)

ADB (Android Debug Bridge) is Google's official tool for sending files to a phone over USB. It
sounds technical, but it's just a few copy-paste commands. The steps below are for Windows.

**One-time setup on the PC**

1. Download **SDK Platform-Tools for Windows** from Google's official page:
   <https://developer.android.com/tools/releases/platform-tools>
2. Extract the ZIP somewhere easy, e.g. `C:\platform-tools\`. Inside it you'll see `adb.exe`.
3. Extract `wwm-indonesian-patch.zip` **into that same `C:\platform-tools\` folder**, so the
   `translate_words_map_en*` files sit next to `adb.exe`. (This keeps the commands short.)

**One-time setup on the phone**

4. Open **Settings → About phone** and tap **Build number** 7 times until it says
   *"You are now a developer"*. (On Xiaomi it's **MIUI/OS version**; on Samsung it's under
   **Software information**.)
5. Go to **Settings → System → Developer options** (location varies by brand) and turn on
   **USB debugging**.

**Copying the files**

6. Connect the phone to the PC with a USB cable (a data cable, not a charge-only one).
7. Open the `C:\platform-tools\` folder in Windows Explorer, click the address bar, type `cmd`,
   and press **Enter**. A black Command Prompt window opens already inside that folder.
8. Type the following and press **Enter**:
   ```bat
   adb devices
   ```
   A prompt **"Allow USB debugging?"** appears on the phone — tick *Always allow* and tap
   **Allow**. Run `adb devices` again; you should see a line ending in `device`. If it says
   `unauthorized`, check the phone screen for the prompt. If nothing is listed, try another
   cable/USB port or set the phone's USB mode to *File Transfer*.
9. **Back up** the original files to the PC (this creates a `backup` folder next to `adb.exe`):
   ```bat
   mkdir backup
   adb pull /sdcard/Android/data/com.netease.yysls/files/LocalData/Patch/HD/oversea/locale/ backup
   ```
10. Push the patched files to the phone — run one line per file (copy-paste each and press
    **Enter**):
    ```bat
    adb push translate_words_map_en /sdcard/Android/data/com.netease.yysls/files/LocalData/Patch/HD/oversea/locale/
    adb push translate_words_map_en_diff /sdcard/Android/data/com.netease.yysls/files/LocalData/Patch/HD/oversea/locale/
    adb push translate_words_map_en__small /sdcard/Android/data/com.netease.yysls/files/LocalData/Patch/HD/oversea/locale/
    adb push translate_words_map_en__small_diff /sdcard/Android/data/com.netease.yysls/files/LocalData/Patch/HD/oversea/locale/
    ```
    Each line should report `1 file pushed`. If you get *"No such file or directory"*, the package
    name or path differs on your device — run
    `adb shell ls /sdcard/Android/data/ | findstr netease` to find the right package folder and
    adjust the path.
11. Force-close the game, relaunch it, and set **Settings → Language → Game Language = English**.
12. (Optional) Turn **USB debugging** back off when you're done.

To restore the originals later, repeat step 10 using the files from the `backup` folder
(`adb push backup\locale\translate_words_map_en ...` etc.).

> [!NOTE]
> The game's own patcher may re-verify and silently restore modified files on launch, similar to
> the `_diff` behavior on the Steam PC client (see [Known Limitations](#known-limitations--deployment-gotchas)). If
> the Indonesian text reverts to English after a launch or update, re-copy the patched files.

### Reverting to English / Handling Game Updates

To revert to the original English text, or after a game update (which typically overwrites the
locale files with new versions from NetEase), simply overwrite the `translate_words_map_en*` files
with the original backups you saved in step one of the relevant platform section above.

---

## Support & Donation

This Indonesian patch is, and will always remain, **free for everyone**. Keeping it accurate and playable, however, comes with real ongoing costs:

- **AI translation costs** — Translating several hundred thousand strings with AI assistance consumes a substantial volume of tokens, and quality AI is not inexpensive.
- **Mod maintenance** — Every NetEase update changes the locale files. Each release means re-extracting the data, translating new and changed text, running quality checks, and republishing the patch so it continues to work in-game.
- **Time and dedication** — All of this work is carried out by a single maintainer.

If this project has made Where Winds Meet more enjoyable for you, please consider supporting its development. Every contribution, regardless of size, goes directly toward AI costs and toward keeping the patch current after each game update. Your support is what keeps this project alive, and it is sincerely appreciated.

Unable to donate? You can still make a meaningful difference by starring the repository, sharing it with fellow players, or reporting inaccurate translations through Issues.

![QR](.donation/saweria.png)

**Donate via Saweria:** [https://saweria.co/oratakashi](https://saweria.co/oratakashi)

---

## Prerequisites & Installation

*The section below is for contributors/developers who want to run the toolkit itself
(dump/patch/QA), not for players who just want to install the patch — see the
[Player Installation Guide](#player-installation-guide) above for that.*

- **Python 3.8+**
- **pip**

Clone the repository and install the single runtime dependency:

```bash
git clone git@github.com:oratakashi/where-winds-meet-indonesian-patch.git
cd where-winds-meet-indonesian-patch
pip install -r requirements.txt
```

*Note: The only third-party dependency is [`zstandard`](https://pypi.org/project/zstandard/).*

---

## Quickstart Guide

### 1. Locate Game Files

Locate your game installation path:

- **Steam Installation**:
  ```
  <SteamLibrary>\steamapps\common\Where Winds Meet\Package\HD\oversea\locale\translate_words_map_en
  ```
- **Official NetEase Launcher**:
  ```
  <InstallDir>\<variant>\Package\HD\oversea\locale\translate_words_map_en
  <InstallDir>\<variant>\LocalData\Patch\HD\oversea\locale\
  ```
  (`<variant>` is the downloaded game variant folder, e.g. `wwm_lite`.)

> [!WARNING]
> **Always create a backup before modifying game files.** Keep an untouched copy of `translate_words_map_en` in a safe location. If you previously used GearUP Booster, their automatic backup files have a `.gubackup` extension in the same directory and preserve pristine original English text.

### 2. Inspect Binary Header

Sanity-check the file and ensure parser compatibility:

```bash
python wwm_locmap.py info translate_words_map_en
```

Expected output:
```text
version        : 1
blocks         : 3230 (1 index + 3229 shard)
entries (index): 826388
entries parsed : 826388
```

If `entries parsed` matches `entries (index)`, the container is valid. When opening incremental `*_diff` files, entry counts intentionally diverge due to sparse shards (see [§4.5 of Format-Spec.md](Obsidian-Vault/knowladge/Format-Spec.md#45-the-_diff-variant-incremental-file-between-game-updates)).

### 3. Dump to JSON Lines

Export all localization strings into a JSON Lines (`.jsonl`) file (~90 MB for ~826k entries):

```bash
python wwm_locmap.py dump translate_words_map_en strings.jsonl
```

Each line in `strings.jsonl` represents an atomic localization entry:

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

### 4. Translate & Edit

Modify the text inside the `v` field. Review the [Translation Guidelines](#translation-guidelines--engine-syntax) to avoid breaking engine formatting tags.

> [!TIP]
> The composite key `(b, s)` uniquely identifies the slot address. **Sparse overlays are fully supported**: you can delete unchanged lines from your JSONL file. When executing `patch`, unmodified entries are retained directly from the source binary, making localized diffs lightweight.

### 5. Automated QA Verification

Before repacking, run the automated quality assurance suite to catch syntax defects:

```bash
python tools/qa_check.py strings_original.jsonl strings_translated.jsonl --report qa_report.jsonl
```

Example audit output:
```text
inspected   : 963050 entries
PROMPT_LEAK :      0  (0.000%)
MARKUP      :      0  (0.000%)
EMPTY       :      0  (0.000%)
```

The script exits with code `1` if defects are detected, making it ideal for continuous integration (CI) or pre-commit hooks.

### 6. Repack into Binary

Overlay your modified strings onto the original binary container:

```bash
python wwm_locmap.py patch translate_words_map_en strings.jsonl translate_words_map_en.new
```

- Use `--level N` to set the Zstandard compression level (default is `19`). Use `10`–`12` for rapid development iterations; use `19` for distribution builds.
- Rename or copy the repacked file to `translate_words_map_en` inside your game's `Package\HD\oversea\locale\` folder.
- Launch the game with **Settings → Language → Game Language = English**.

---

## Daily Translation Workflow (Cheat Sheet)

When working with translation batches in `translation_work/unique_strings.jsonl` and `locale/phase*.jsonl`, execute these three commands to generate a clean, validated build:

```bash
# 0. Validate the per-idx translation files themselves (tokens, empty values, idx gaps/duplicates)
python tools/qa_check.py --locale "locale/*.jsonl"

# 1. Expand translation dictionary against the original dump
python tools/expand_locale.py

# 2. Run QA validation (prompt leaks, corrupted markup, empty values)
python tools/qa_check.py strings.jsonl strings.translated.jsonl --report qa_report.jsonl

# 3. If QA passes (exit code 0), repack into a distribution binary
python wwm_locmap.py patch translate_words_map_en strings.translated.jsonl translate_words_map_en.id
```

Notes:
- Step 1 expects `strings.jsonl` (generated via `wwm_locmap.py dump`) to be present in the workspace root.
- If Step 2 fails with exit code `1`, inspect `qa_report.jsonl` and rectify the flagged strings before proceeding.
- Copy `translate_words_map_en.id` to the game's locale directory as `translate_words_map_en`.
- To patch all file variants (`base`, `_diff`, `__small`, `__small_diff`, `_mobile`) at once, use `tools/patch_all.py`.
- `wwm_locmap.py patch` refuses a JSONL whose `h` values don't match the target file (i.e. one dumped from a different game version) — re-dump after every game update.

---

## Handling Game Updates & File Variants

Upstream game patches frequently ship new or modified localization files alongside the primary table:
- `translate_words_map_en_diff`: Sparse delta table between major releases.
- `translate_words_map_en__small`: Separate isolated table for UI / system modules.
- `translate_words_map_en__small_diff`: Delta table for the small variant.
- `translate_words_map_en_mobile`: Currently byte-identical to the base table; patched independently anyway.

Because the underlying container and shard layout are identical, use the following sequence after a patch:

```bash
# Dump the updated variant
python wwm_locmap.py dump translate_words_map_en__small strings_small.jsonl

# Append new unique strings without breaking existing translation indices
python tools/rebuild_unique_strings.py

# Apply translations across all variants simultaneously
python tools/patch_all.py --outdir patched
```

See [`Obsidian-Vault/knowladge/Format-Spec.md`](Obsidian-Vault/knowladge/Format-Spec.md) §4.6 for technical findings regarding `__small` tables and `CLAUDE.md` for helper script internals.

---

## Technical Architecture & Binary Format

### Why Runtime Requires English Language

Localization files in Where Winds Meet **do not store source English strings**. The binary contains only a unidirectional map:

$$\text{u64 keyHash} \longrightarrow \text{localized string}$$

At runtime, the Messiah Engine reads the base English string from game code or data assets, computes its 64-bit hash, and performs a lookup in `translate_words_map_en`. 
- If Game Language is set to English, the engine looks up hashes computed from English text.
- If Game Language is changed to German or French, the engine computes hashes from German/French strings, misses every entry in this map, and falls back to raw strings.

Setting **Game Language = English** is a fundamental architectural requirement dictated by the engine's lookup design.

### In-Place Mutation Without Hash Reversal

Because keys are not stored, replacing strings simply requires mutating the **values** linked to existing keys. The search architecture remains untouched:

```text
Preserve: Header + Control Bytes (ctrl) + Sentinel + Padding + Slot Array (keyHash)
Rebuild:  New String Blob (UTF-8)
Update:   relOffset + byteLen in each active slot
```

Hash table probing remains 100% valid. This eliminates the need to reverse-engineer the proprietary 64-bit hash function for translation purposes.

### Binary Format Summary

```text
File Container:
  u32 magic = 0xDEADBEEF
  u32 version = 1
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

### Critical Gotcha: Relative Pointer Arithmetic

A critical pitfall in Messiah's SwissTable implementation is that `relOffset` is a **self-relative pointer**, calculated from the address of the `relOffset` field itself:

$$\text{valueStart} = \text{address}(\text{slot}[i].\text{relOffset}) + \text{relOffset}$$

$$\text{value} = \text{block}[\text{valueStart} : \text{valueStart} + \text{byteLen}]$$

This C++ idiom enables memory-mapping (`mmap`) without pointer relocation. Assuming `relOffset` is an absolute offset from the block start or string blob start results in corrupted, overlapping string fragments.

For the exhaustive specification, see [`Obsidian-Vault/knowladge/Format-Spec.md`](Obsidian-Vault/knowladge/Format-Spec.md).

---

## Translation Guidelines & Engine Syntax

### Messiah Engine Format Tokens

Strings contain proprietary Messiah Engine formatting and color tags. Modifying, reordering, or dropping these tags can crash the UI or corrupt text rendering:

| Token | Function | Rule |
|---|---|---|
| `#Y` `#N` `#R` `#J` `#G` `#H` | Format / color style opening | Keep unaltered |
| `#aee5ae` | 6-character hexadecimal color tag | Keep unaltered |
| `#E` | Format closing tag | **Must** pair exactly with opening tags |
| `%s`, `%d` | Standard printf specifiers | Dynamic runtime values; keep order and count |
| `{0}`, `{1}` | Indexed positional placeholders | Retain indices |
| `\n` | Literal newline | Preserve line break flow |

**Valid Example**:
- **Source**: `Mystic Skill #YMeridian Touch#E`
- **Target**: `Keterampilan Mistik #YSentuhan Meridian#E`

*The text enclosed inside `#Y...#E` may be freely translated, but the tags `#Y` and `#E` must remain intact.*

### Proper Nouns & Pinyin Retention

Character names, martial arts sects, and historical locations are typically rendered in Pinyin (e.g., `Su Jiangyun`, `Gu Zhouyue`, `Zhang Tiemeng`). Retain original Pinyin names so that players can cross-reference guides, community wikis, and player chat. In the reference dataset, approximately 12.5% of strings intentionally leave proper nouns untranslated.

### Machine Translation & LLM Quality Control

When utilizing Machine Translation (MT) or Large Language Models (LLMs), strict output validation is mandatory. In an audit of a commercial translation pack, **1,817 entries (0.189%) were discovered where LLM system prompts leaked directly into the game text**:

```text
Formatting instructions:
  (newline tags) and brackets [] must remain unchanged
- Do not translate content inside {}
- Several special characters must not be translated, such as: %d, %s, #G, #E...
[Actual translated text followed here]
```

These errors typically affect long lore entries, breaking player immersion. `tools/qa_check.py` automatically prevents this by asserting:
1. Zero occurrences of registered prompt leak signatures (`LEAK_SIGNATURES`).
2. Exact matching multisets of engine markup tokens between source and translated entries.
3. Absence of zero-length or whitespace-only values.

### Oversized Lore Entries

While the average string length is ~56 bytes, certain encyclopedia and narrative entries exceed **8 KB**. When integrating with translation APIs, implement robust payload chunking to prevent silent truncation.

---

## Known Limitations & Deployment Gotchas

### 1. The `_diff` CDN Auto-Restore Mechanism (Steam client only)
- **Base `translate_words_map_en` and `__small` are persistent and safe to patch** in `Package\HD\oversea\locale\` on every client.
- The game maintains two separate copies of `translate_words_map_en_diff`:
  - `Package\HD\oversea\locale\` (patched by this tool; safe, never re-verified).
  - `LocalData\Patch\HD\oversea\locale\` (active runtime overlay).
- **Steam client**: the `LocalData` copy is **checksum-verified against NetEase CDN manifests on every game launch** (`StagePatchList` / `StageCheck` / `StageDownload`). Modified files in `LocalData` are detected and restored to original bytes within minutes. DNS blocking of CDN endpoints fails because the client falls back to cached manifests and secondary hosts. The sparse `_diff` table (~213k entries in late 2026 builds) therefore displays in English on Steam until NetEase merges delta patches back into the base package during major game updates.
- **Official WWM launcher**: this restore does **not** happen (tested 2026-09-27). A patched `_diff` in `LocalData\Patch\HD\oversea\locale\` (on the official launcher the full path is `<InstallDir>\<variant>\LocalData\Patch\HD\oversea\locale\`, where `<variant>` is e.g. `wwm_lite`) stays in place across launches, so the whole patch — including the `_diff` layer — is effective. Why the two clients differ has not been investigated.
- Earlier versions of this README described the restore as affecting every PC client; that conclusion came from testing on Steam only.

### 2. Upstream Desynchronization
New strings introduced in game patches default to English until extracted, translated, and patched. Use `tools/rebuild_unique_strings.py` to reconcile deltas.

### 3. Key Insertion Constraint
Creating brand-new keys requires deriving the proprietary 64-bit hashing function, which is currently unsolved.

### 4. Codec Exclusivity
Only `codec = 4` (Zstandard) is implemented. If NetEase introduces alternative codecs, the parser rejects the block with an explicit error rather than outputting corrupted data.

### 5. Text-Only Scope
This toolkit processes text containers. Audio files, voice-over assets, and UI bitmap fonts are governed by separate archive packages.

---

## Legal & Anti-Tamper Notice

Please review the following disclaimers before utilizing this toolkit:

- **Affiliation**: This project is an independent research and translation effort. It is not affiliated with, endorsed by, or connected to NetEase Inc., Everstone Studio, or any related subsidiaries.
- **Online Game Environment**: *Where Winds Meet* is an online title featuring server-side authority and multiplayer functionality. Modifying client files carries inherent risks under the game's Terms of Service (ToS) and may trigger anti-tamper or integrity-check heuristics.
- **Usage Disclaimer**: This toolkit is provided for educational and community localization purposes. The authors accept no liability for account penalties, bans, or software instability resulting from file modification. Do not test experimental modifications on critical accounts.
- **Scope**: This software modifies display text strings exclusively; it contains no gameplay cheats, exploits, or competitive advantages.

---

## Repository Structure

```text
.
├── wwm_locmap.py                    # Primary CLI: info / dump / patch container codec
├── tools/
│   ├── qa_check.py                  # QA validation: catches prompt leaks, broken tags, empty values
│   ├── expand_locale.py             # Expands unique dictionary entries into a full patch JSONL
│   ├── rebuild_unique_strings.py    # Merges upstream patch deltas without invalidating existing indices
│   └── patch_all.py                 # In-memory batch patcher for all translate_words_map_* variants
├── locale/                          # Phase-based translation batches (phase0..phase6, phase17, update1)
├── translation_work/                # Master deduplicated index (unique_strings.jsonl)
├── Obsidian-Vault/                  # Full project documentation & translation memory
│   ├── knowladge/
│   │   ├── Format-Spec.md           # Exhaustive reverse-engineered binary specification
│   │   ├── Quick-Reference.md       # Condensed terminology and translation style guide
│   │   └── Glossary.md              # Wuxia terms, titles, and proper noun references
│   └── progress/                    # Project milestone tracking and session resumption guides
├── requirements.txt                 # Python dependencies (zstandard)
├── LICENSE                          # MIT License
└── CLAUDE.md                        # Developer and AI agent workflow reference
```

---

## Contributing

Contributions are welcomed. Primary research and development priorities include:

1. **Hash Derivation Research**: Investigating the H1/H2 hash calculation derived from `keyHash` and `seed` (see [`Obsidian-Vault/knowladge/Format-Spec.md`](Obsidian-Vault/knowladge/Format-Spec.md) §4.4) to unlock arbitrary key insertion.
2. **QA Signatures**: Reporting new LLM system prompt patterns for inclusion in `tools/qa_check.py` (`LEAK_SIGNATURES`).
3. **Format Verification Across Locales**: Validating parser compatibility against non-English locale packages (`_de`, `_fr`, `_ja`, etc.).
4. **Glossary & Translation Polish**: Refining Wuxia terminology and martial arts lore definitions in `Obsidian-Vault/knowladge/glossary/`.

---

## License

This project is licensed under the [MIT License](LICENSE). The license applies solely to the source code and documentation in this repository; it does not grant rights to any proprietary game content or assets.
