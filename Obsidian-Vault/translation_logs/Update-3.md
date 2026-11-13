# Update-3 Translation Log (idx 535,919–733,661)

Update-3 is the batch of strings added by **game v2.4 [Across Autumn Waters]**
(2026-11-13) — not a continuation of the original `unique_strings.jsonl`.
See [[Phase-Roadmap]] for how it fits and [[Session-History]] for the session
timeline. All standard [[Glossary]] / [[Quick-Reference]] conventions apply;
this file records Update-3-specific decisions.

## How it was created (session 150)

| v2.4 file | Entries | Tombstones | Change vs v2.3 hotfix | Unique texts not yet in the dictionary |
| --- | ---: | ---: | --- | ---: |
| `translate_words_map_en` | 1,116,561 (4,363 blocks; v2.3: 1,122,784 / 4,387) | 0 | +1,332 keys, −7,555 keys, **305,768 values changed** | 188,491 |
| `translate_words_map_en_diff` | 334,882 (v2.3: 12,934) | 1,772 (v2.3: 1,919) | +324,160 keys, −2,212 keys, 559 values changed | 9,298 |
| **Combined (deduplicated)** | | | | **197,743** (~17.4M characters) |

- Most of the base change is a **rewrite of existing English text** rather than
  new content — rephrased dialogue and renamed places/items, e.g. "Greenwood
  Mansion Indoor Floor" → "Qingmu Hall Indoor Flooring", "Qianjiang Streets" →
  "Qiantang Marketplace", "Helmet, Chestpiece, Greaves, Bracer" → "Helm,
  Chestplate, Greaves, Bracers", "Imperial Soldier" → "Song Dynasty Soldier".
  When a renamed term recurs, check the glossary / [[Place-Names-Wiki]] and record
  the new English name rather than silently mixing old and new names.
- `python tools/rebuild_unique_strings.py` (defaults: `strings.jsonl` +
  `strings_diff.jsonl`) appended **197,743** strings as idx 535,919–733,661,
  ordered by frequency. Existing idx 0–535,918 untouched; the old v2.3 texts
  stay in the dictionary (matching is by source text, so they cost nothing).
- `unique_strings.jsonl` grew from ~53 MB to ~73 MB (GitHub warns above 50 MB,
  rejects files above 100 MB) — a future update of this size would need a split.
- Before any Update-3 work the dictionary covers 76.86% (858,199 / 1,116,561)
  of the base file and 95.35% (317,613 / 333,110) of the live `_diff` entries.

## Sub-blocks

Split per 30,000 strings at the user's request.

| Block | idx range | Count | Output file |
| --- | --- | ---: | --- |
| Update-3A | 535,919–565,918 | 30,000 | `locale/update3a.jsonl` |
| Update-3B | 565,919–595,918 | 30,000 | `locale/update3b.jsonl` |
| Update-3C | 595,919–625,918 | 30,000 | `locale/update3c.jsonl` |
| Update-3D | 625,919–655,918 | 30,000 | `locale/update3d.jsonl` |
| Update-3E | 655,919–685,918 | 30,000 | `locale/update3e.jsonl` |
| Update-3F | 685,919–715,918 | 30,000 | `locale/update3f.jsonl` |
| Update-3G | 715,919–733,661 | 17,743 | `locale/update3g.jsonl` |
