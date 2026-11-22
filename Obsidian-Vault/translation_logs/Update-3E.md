# Update-3E Translation Log (idx 655,919–685,918)

Strings new in **game v2.4 [Across Autumn Waters]** (2026-11-13), documented in [[Update-3]]. Output: `locale/update3e.jsonl` (created by the first translation session). 30,000 strings. Standard [[Quick-Reference]] conventions apply.

## Batches

- 2026-11-22 (session 157): idx 655,919–685,918 (30,000 rows) translated by 30 subagents (1,000 rows each; several resumed after usage-limit interruptions), merged in idx order into `locale/update3e.jsonl`, QA clean (0 PROMPT_LEAK / MARKUP / EMPTY / IDX). **Update-3E complete: 30,000/30,000, which completes all of Update-3 (v2.4).** Overall after session: 733,662 / 733,662 unique strings (100.00%), in-game coverage 100.00% (base) / 100.00% (`_diff`).

## Decisions & notes
- Merge check: 521 rows came back from subagents with literal backslash-n instead of real newlines (newline count differed from source); converted back before append. Two rows (657254, 657859) also had an extra trailing newline, collapsed. Whole-sentence `<...>` tags kept verbatim; U+FFFD placeholders in the source were normalised by some agents (em dash/dropped).
- Conventions as Update-3F/3G. No new glossary rules.
