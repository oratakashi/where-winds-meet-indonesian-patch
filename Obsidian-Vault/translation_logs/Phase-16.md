# Phase 16 Translation Log (idx 370,000–399,999)

**Completed 2026-09-27 (session 50).** `locale/phase16.jsonl` holds the full
30,000 rows (idx 370,000–399,999) with 0 QA mismatches outstanding — see
[[Current-Status]] for the overall progress line. Check [[Quick-Reference]]
and [[Glossary]] for the standing conventions applied throughout.

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
- idx 391,000–399,999 (2026-09-27, session 50, 9,000 rows) — **Phase 16
  completed.** Translated in thirty-six scratch passes of 250 rows each,
  each validated with `tools/qa_check.py --locale` before being merged and
  appended to `locale/phase16.jsonl`, per the user's request ("Aku ingin
  menyelesaikan phase 16") to finish the phase in full this session. Notable
  content: the Sixteen Lanes "1.5 billion Youxia-er" anniversary letter (idx
  395764), the Zhu En Nuo Rite ritualist meditation (idx 396050), the Wen
  Wuque "Black and White Gods of Wealth" Weiyang City saga (idx 396809), the
  Su Yuandao Mohist apprenticeship biography (idx 397721), the "Ran"
  child's-eye-view tragedy vignette (idx 397897), the Jiang Jin Jiu-style
  "Eternal Whispers" drinking poem (idx 399049), the Kaifeng tourist diary
  (idx 399170), the Feng Yi/River Master "was it worth it" epilogue (idx
  399991), and a full Where Winds Meet Mobile Invitational tournament-rules
  block (idx 399102). Fixed 5 mismatches across the 36 batches, all the same
  class (Quick-Reference §6 rule 1/3 — a `<...>`-wrapped stage direction or
  full-sentence aside translated instead of kept 100% verbatim in English):
  idx 396555 (`<Inner voice>`), idx 396635 (a cat's aside), idx 396662
  (`<spots a wine jug under the camel>`), idx 397496 (`<whispers>`), and idx
  399972 (`<You tell him about your encounter with Big Smart>`). One
  additional non-QA issue was caught and fixed before validation: the idx
  397721 batch file had an unescaped `"` inside a JSON string value
  (breaking that line's JSON syntax), caught by a `json.loads` pass over the
  scratch file before `qa_check.py` ran, fixed by escaping the quote. Two
  raw non-English strings kept verbatim per convention: idx 396172
  (raw Chinese dev-label) and idx 398744 (raw Chinese dev-label). No new
  terminology decisions — all cases matched existing Quick-Reference
  entries. Final full-file check: `locale/phase16.jsonl` is 30,000 lines,
  idx unique and sequential 370,000–399,999 with no gaps, 0 duplicates, 0
  token mismatches. See [[Current-Status]] session 50 write-up for the full
  overall-progress line.

### 2026-09-27 — Inner Way name revert

- idx 388247 `Insightful Strike: Tome` → `Insightful Strike: Kitab` (was `Serangan
  Berwawasan: Kitab`). Inner Way names are now locked as not-translated — see
  [[Names-Not-Translated]] § "Inner Way names" (same fix applied to Phase 2 idx 2027–3295, see
  [[Phase-2]]).
- Second pass (full game8 list): 379827 `Inner Way: Blossom Barrage`, 387627 `Seasonal Edge:
  Catatan`, 387691 `Sword Horizon: Catatan`.
