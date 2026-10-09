# Phase 13 Translation Log (idx 280,000–309,999)

**Reconstructed retroactively on 2026-09-26** — this file was not written during
the session that did the work, so it only records what the commit history and
`locale/phase13.jsonl` show, not session-by-session reasoning.

Phase 13 began with a single starter batch, committed 2026-09-25
(`8c02b00` — "Start Phase 13 translation (idx 280000-281999, 2000 rows)").
`locale/phase13.jsonl` now has 24,000 rows; the phase is **not complete** — see
[[Current-Status]] for the full idx range still outstanding. No terminology
decisions were recorded for this batch; check [[Quick-Reference]] and
[[Glossary]] for standing conventions before continuing it.

## Batches

- [[idx-280000-281999]] — idx 280,000–281,999 (`8c02b00`, 2026-09-25)

- [[idx-282000-285999]] — idx 282,000–285,999 (session 104, 2026-10-09, `ce7fcc9`, 4,000 rows) — brought the phase to 6,000/30,000. No new terminology decisions.

- [[idx-286000-289999]] — idx 286,000–289,999 (session 106, 2026-10-09, 4,000 rows) — brought the phase to 10,000/30,000. No new terminology decisions.

- [[idx-290000-293999]] — idx 290,000–293,999 (session 108, 2026-10-09, 4,000 rows) — brought the phase to 14,000/30,000. No new terminology decisions.

- [[idx-294000-294999]] — idx 294,000–294,999 (session 111, 2026-10-09, 1,000 rows; partial 4,000-row iteration). Phase now 15,000/30,000; next idx 295,000. No new terminology decisions. Overall: 423,704 / 461,704 unique strings (91.77%), in-game coverage 96.52%.

- [[idx-295000-297999]] — idx 295,000–297,999 (session 113, 2026-10-09, 3,000 rows), completing session 111's requested 4,000-row iteration (idx 294,000–297,999). Phase now 18,000/30,000; next idx 298,000. Translator subagents explicitly authorized by user. No new terminology decisions. Overall: 427,704 / 461,704 unique strings (92.64%), in-game coverage 96.88%.

- [[idx-298000-299999]] — idx 298,000–299,999 (session 115, 2026-10-09, 2,000 rows; no commit created). Phase now 20,000/30,000 (66.67%); next idx 300,000. Translator subagents explicitly authorized by user. No new terminology decisions; Retainer, Physiotherapy and Confucius follow existing English/name conventions. Overall: 432,704 / 461,704 unique strings (93.72%), in-game coverage 97.34%.

- [[idx-300000-303999]] — idx 300,000–303,999 (session 118, 2026-10-09, 4,000 rows; no commit created). Phase now 24,000/30,000 (80.00%); next idx 304,000. Primary agent only, no subagents. No new terminology decisions. Overall: 444,704 / 461,704 unique strings (96.32%), in-game coverage 98.44%.

- [[idx-304000-304999]] — idx 304,000–304,999 (session 119, 2026-10-09, 1,000 rows; no commit created). Phase now 25,000/30,000 (83.33%); next idx 305,000. Primary agent only. Batch size reduced to 1,000 at the user's request. No new terminology decisions; stray `#After` token kept intact, plain `<...>` emote tag kept English. Overall: 445,704 / 461,704 unique strings (96.53%), in-game coverage 98.53%.
