# Phase 16 Translation Log (idx 370,000–371,999)

**Reconstructed retroactively on 2026-09-26** — this file was not written during
the session that did the work, so it only records what the commit history and
`locale/phase16.jsonl` show, not session-by-session reasoning.

Phase 16 currently holds a single starter batch, committed 2026-09-26
(`ba9ba57` — "Start Phase 16 translation (idx 370000-371999, 2000 rows)").
`locale/phase16.jsonl` has 2,000 rows; the phase is **not complete** — see
[[Current-Status]] for the full idx range still outstanding. No terminology
decisions were recorded for this batch; check [[Quick-Reference]] and
[[Glossary]] for standing conventions before continuing it.

## Batches

- [[idx-370000-371999]] — idx 370,000–371,999 (`ba9ba57`, 2026-09-26)
- idx 372,000–376,999 (2026-09-26, session 44, 5,000 rows) — translated in
  twenty scratch passes of 250 rows each, validated with `tools/qa_check.py`,
  merged and appended to `locale/phase16.jsonl`, per the user's standing
  5,000-row-per-iteration request. Fixed 11 `<...>`-tag mismatches on the
  first QA pass (plain non-`|id|#C|n>` tags like `<Beast Tongue>`,
  `<thinking>`, `<barking>` had their bracketed content translated instead
  of kept 100% in English per Quick-Reference §6 rule 1) before appending.
  No new terminology decisions — all cases matched existing Quick-Reference
  entries. See [[Current-Status]] session 44 write-up for full content
  notes.
