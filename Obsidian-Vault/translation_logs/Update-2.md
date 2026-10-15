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

### 2026-10-15 — Session 135: Update-2C completed, idx 533,704–534,256

- Appended the final 553 rows to `locale/update2c.jsonl`. **Update-2C is 100% (12,553/12,553).** No commit created. Primary agent only. Details: [[Update-2C]].
- 462 changed / 91 retained. No new terminology rule. Long lore, essays, stories, tournament rules and Tang poems translated fully. Batch and full-dictionary QA: 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings.
- Overall: 486,257 / 534,257 unique strings (91.02%), in-game coverage 95.14%.

### 2026-10-15 — Session 134: Update-2A second iteration completed, idx 465,704–469,703

- Appended 4,000 rows to `locale/update2a.jsonl` (ten 400-row chunks). Update-2A **8,000/30,000 (26.67%)**; next idx **469,704**. No commit created. Details: [[Update-2A]].
- 2,758 changed / 1,242 retained. Long classics, poetry and notices translated fully. Batch and full-dictionary QA: 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings.
- Overall: 485,704 / 534,257 unique strings (90.91%), in-game coverage 95.14%.

### 2026-10-15 — Session 131: Update-2B iteration completed, idx 491,704–495,703

- Added the remaining 3,500 rows (492,204–495,703); session 128's checkpoint plus this batch completes the requested **4,000-row iteration**. Update-2B **4,000/30,000 (13.33%)**; next idx **495,704**. No commit created. Details: [[Update-2B]].
- Three user-authorized subagents translated separate 1,000-row ranges; primary translated 500, reviewed and merged. All temporary files outside repository. New batch 3,269 changed / 231 retained; full iteration 3,722 changed / 278 retained. No new glossary rule; Terminal Pivot Tower follows established English-name precedent.
- Long classics, poetry and lore translated fully; ten long entries independently reviewed. Batch and full-dictionary QA: 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings. Exact sequential idx, extra placeholders, newline markers, numeric formatting and long-text completeness checked. `python tools/progress.py --write` refreshed tables.
- Overall: 473,704 / 534,257 unique strings (88.67%), in-game coverage 93.77%.

### 2026-10-15 — Session 129: Update-2A iteration completed, idx 462,704–465,703

- Appended 3,000 rows to `locale/update2a.jsonl`. Together with session 127's checkpoint, **idx 461,704–465,703 (4,000 rows) is complete**. Next idx: **465,704**; Update-2A 4,000/30,000 (13.33%). No commit created.
- User-authorized subagents each translated a disjoint 1,000-row range; primary agent reviewed and merged. Scratch outputs remained outside the repository. The original 1,000-row checkpoint was preserved byte-for-byte.
- Added 2,145 changed translations / 855 retained names, poetic titles, Skill/stat labels and opaque cases. Full iteration: 2,854 changed / 1,146 retained. Dialogue gue/lo, UI neutral; no new terminology lock.
- Long prose translated fully: octopus/mudskipper lore, birthday noodles letter, Inkbound horror introduction, Everbloom Bazaar, Jian Xingzhi's investigation, Jiaqu Houguan sick-leave slip and Analects virtue/friendship passage (465621). Book title Analects kept consistent. Common labels Monyet Emas, Kucing Lynx, Ivi, Klivia, Hiasint follow the existing common-animal/plant rule.
- Review repaired introduced replacement characters at 464357, 464431, 464543 and 464565 before merge. Plain/structured angle tags, dollar/date variables and actual/literal newline counts preserved. Numeric changes reviewed as decimal/thousands localization or a spelled-out four.
- Merged 3,000-row remainder, complete 4,000-row iteration and full 466,204-entry locale dictionary passed QA: 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings. Exact sequential idx, extra placeholders and long-text completeness checks passed. `python tools/progress.py --write` regenerated status tables.
- Overall: 466,204 / 534,257 unique strings (87.26%), in-game coverage 93.10%.

### 2026-10-15 — Session 128: Update-2B checkpoint, idx 491,704–492,203

- Saved 500 rows to `locale/update2b.jsonl`; no commit created. Primary agent only. Details: [[Update-2B]].
- Requested 4,000-row iteration (491,704–495,703) is **incomplete**. Resume **492,204–495,703 (3,500 rows)** before starting another iteration.
- 453 changed translations and 47 retained names, poetic titles, stat/gear labels and opaque tags. Long prose translated in full, including the supplied Mencius excerpt. No new terminology rule.
- Scratch and full-dictionary QA passed with 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings; extra placeholders and newlines checked. `python tools/progress.py --write` regenerated status tables.
- Overall: 463,204 / 534,257 unique strings (86.70%), in-game coverage 92.02%.

### 2026-10-15 — Session 127: Update-2A checkpoint, idx 461,704–462,703

- Saved 1,000 rows to `locale/update2a.jsonl`; no commit created. Primary agent only.
- Requested iteration: 461,704–465,703 (4,000 rows). **Incomplete: resume 462,704–465,703 (3,000 rows)** before beginning another iteration.
- 709 translations differ from source; 291 names, poetic titles, skill/stat and technical labels retained. Dialogue uses gue/lo, UI neutral. No new terminology lock.
- Content: Qingzhou/Jiangnan introductions, Chen family journey, Homestead/Guild Battle, Six Arts, seafood lore, and fan/drum/blade effects. Long text translated in full. Plain angle tags and dollar variables preserved; truncated `<` openings retained while following free text translated. Actual and literal newline counts preserved.
- Initial temporary output was recovered from this chat's own tool-call records after TEMP changed between executions. Scratch files remained outside the repository; recovered output passed QA before saving.
- Scratch and full-dictionary QA: 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings. Exactly 1,000 consecutive idx. `python tools/progress.py --write` regenerated status tables.
- Overall: 462,704 / 534,257 unique strings (86.61%), in-game coverage 91.98%.

### 2026-10-15 — Session 132: Update-2C — second 4,000-row iteration completed, idx 525,704–529,703

- Appended 4,000 rows to `locale/update2c.jsonl`; Update-2C is now 8,000/12,553 (63.73%), next idx **529,704**. Details in [[Update-2C]]. No commit created.
- Overall: 477,704 / 534,257 unique strings (89.41%), in-game coverage 94.13%.

### 2026-10-15 — Session 130: Update-2C — 4,000-row iteration completed, idx 521,704–525,703

- Created `locale/update2c.jsonl` with 4,000 rows; next idx **525,704** (Update-2C 4,000/12,553). Details in [[Update-2C]]. No commit created.
- Overall: 470,204 / 534,257 unique strings (88.01%), in-game coverage 93.46%.
