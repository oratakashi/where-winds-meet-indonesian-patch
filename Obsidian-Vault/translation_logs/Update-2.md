# Update-2 Translation Log (idx 461,704–534,256)

Update-2 is the batch of strings added by the **game v2.3** update
(2026-10-15) — not a continuation of the original `unique_strings.jsonl`.
See [[Phase-Roadmap]] for how it was created and [[Session-History]] for the
session timeline. All standard [[Glossary]] / [[Quick-Reference]]
conventions apply; this file records Update-2-specific decisions.

## How it was created (session 126)

| v2.3 file | Entries | Tombstones | Unique texts not yet in the dictionary |
| --- | ---: | ---: | ---: |
| `translate_words_map_en` | 1,122,784 (v2.2: 826,388) | 0 | 70,626 |
| `translate_words_map_en_diff` | 8,422 (v2.2: ~213k) | 1,647 | 2,169 |
| **Combined (deduplicated)** | | | **72,553** |

- `__small` / `__small_diff` no longer exist in v2.3 (Steam and the official
  launcher) — dropped from tooling and the release.
- `python tools/rebuild_unique_strings.py --extra strings_diff.jsonl`
  appended idx 461,704–534,256, ordered by frequency in the v2.3
  `strings.jsonl`. Existing idx 0–461,703 untouched.
- 29,022 older strings are absent from v2.3; left in place (matching is by
  source text, so they cost nothing).
- Before any Update-2 work, the existing dictionary already covers 90.65% of
  the v2.3 base file's entries and 29.96% of the live `_diff` entries.

## Sub-blocks

Split per ~30,000 strings at the user's request. Letters (not `Update-3`,
`Update-4`) because all three come from the same game update.

| Block | idx range | Count | Output file |
| --- | --- | ---: | --- |
| Update-2A | 461,704–491,703 | 30,000 | `locale/update2a.jsonl` |
| Update-2B | 491,704–521,703 | 30,000 | `locale/update2b.jsonl` |
| Update-2C | 521,704–534,256 | 12,553 | `locale/update2c.jsonl` |

## Batches

(none yet — next idx 461,704)
