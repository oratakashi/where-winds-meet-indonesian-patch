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
- idx 377,000–378,999 (2026-09-27, session 45, 2,000 rows) — translated in
  eight scratch passes of 250 rows each, validated with a direct `TOKEN`-regex
  comparison, merged and appended to `locale/phase16.jsonl`, per the user's
  new standing request to run Phase 16 at 2,000 rows/iteration going forward
  ("Mulai phase 16, aku ingin tiap iterasi 2000 string"), superseding the
  earlier 5,000-row cadence for this phase. Fixed 2 mismatches on the first
  QA pass: idx 377194 (a stat tag's name was translated instead of kept
  English per Quick-Reference §6 rule 2) and idx 378067 (a plain `<...>` tag
  wrapping a full stage direction was translated instead of kept 100%
  verbatim per rule 1). No new terminology decisions — all cases matched
  existing Quick-Reference entries. See [[Current-Status]] session 45
  write-up for full content notes.
- idx 379,000–382,999 (2026-09-27, session 46, 4,000 rows) — translated in
  sixteen scratch passes of 250 rows each, validated with a direct
  `TOKEN`-regex comparison against the source, merged and appended to
  `locale/phase16.jsonl`, per the user's new standing request to run Phase
  16 at 4,000 rows/iteration going forward ("Mulai phase 16, aku ingin
  tiap iterasi 4000 string"), superseding the earlier 2,000-row cadence
  for this phase. Fixed 2 mismatches on the first QA pass: idx 382772 (a
  plain `<Beast Tongue>` tag was translated instead of kept 100% in
  English per Quick-Reference §6 rule 1) and idx 382910 (a `{0}`
  placeholder was duplicated three times instead of the source's two).
  One raw garbled-encoding non-source-text string (idx 381486,
  `鍏充簬瀵掑喎`) was kept verbatim per the idx 411158 precedent. No new
  terminology decisions — all cases matched existing Quick-Reference
  entries. See [[Current-Status]] session 46 write-up for full content
  notes.
- idx 383,000–386,999 (2026-09-27, session 47, 4,000 rows) — translated in
  sixteen scratch passes of 250 rows each, validated with a direct
  `TOKEN`-regex comparison against the source, merged and appended to
  `locale/phase16.jsonl`, continuing the user's 4,000-row/iteration
  cadence for this phase. Fixed 3 mismatches on the first QA pass: idx
  383926 and idx 386635 (plain `<...>`-tagged stage directions/
  inner-thoughts had their bracketed content translated instead of kept
  100% in English per Quick-Reference §6 rule 1) and idx 385860 (a
  `#R...#E` warning wrap was rewritten as `#Y...#E`, changing the token).
  Two non-English/garbled strings kept verbatim per convention: idx
  383466 (`今年も一緒に騒ごう！`, a leaked Japanese string) and idx 386937
  (`谷子渘, You Are Good`, a raw Chinese label mixed with English). No new
  terminology decisions — all cases matched existing Quick-Reference
  entries. See [[Current-Status]] session 47 write-up for full content
  notes.
- idx 387,000–388,999 (2026-09-27, session 48, 2,000 rows) — translated in
  eight scratch passes of 250 rows each, validated with a direct
  `TOKEN`-regex comparison against the source, merged and appended to
  `locale/phase16.jsonl`, per the user's updated request to run Phase 16
  at 2,000 rows/iteration again ("Mulai phase 16, aku ingin tiap iterasi
  2000 string"), superseding the previous 4,000-row cadence for this
  phase. Fixed 2 mismatches on the first QA pass: idx 387696 (a plain
  `<%s - Boat Owner>` tag had its bracketed content translated instead of
  kept 100% in English per Quick-Reference §6 rule 1) and idx 388329 (a
  plain `<Stunned>` stage-direction tag was translated instead of kept
  in English). One raw non-English string kept verbatim per convention:
  idx 388910 (`新年快樂`, a leaked Chinese New Year greeting string). No new
  terminology decisions — all cases matched existing Quick-Reference
  entries. See [[Current-Status]] session 48 write-up for full content
  notes.
- idx 389,000–390,999 (2026-09-27, session 49, 2,000 rows) — translated in
  eight scratch passes of 250 rows each, validated with a direct
  `TOKEN`-regex comparison against the source, merged and appended to
  `locale/phase16.jsonl`, per the user's standing 2,000-row/iteration
  request. Fixed 1 mismatch on the first QA pass: idx 390884, a long
  "36 tactics for the lovelorn" narrative where all seven `<...>`-wrapped
  inner-monologue quotes (Quick-Reference §6 rule 3 — a `<...>` wrapping
  one entire long sentence) had their bracketed content translated into
  Indonesian instead of kept 100% verbatim in English; corrected so only
  the surrounding prose (intro paragraphs, "Tactic One/Two/…" lines) was
  translated and the seven tags restored to their exact source English
  text. Three raw non-English/placeholder strings kept verbatim per the
  idx 411158 precedent: idx 390233, idx 390703, and idx 390985/390990
  (raw Chinese dev/location labels). No new terminology decisions — all
  cases matched existing Quick-Reference entries. See [[Current-Status]]
  session 49 write-up for full content notes.
