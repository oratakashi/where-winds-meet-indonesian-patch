# Phase 7 Translation Log (idx 100,000–102,999)

**Reconstructed retroactively on 2026-09-26** — this file was not written during
the session that did the work, so it only records what the commit history and
`locale/phase7.jsonl` show, not session-by-session reasoning.

Phase 7 currently holds a single starter batch, committed 2026-09-25
(`df960e8` — "Start Phase 7 translation (idx 100000-102999, 3000 rows)").
`locale/phase7.jsonl` has 3,000 rows; the phase is **not complete** — see
[[Current-Status]] for the full idx range still outstanding. No terminology
decisions were recorded for this batch; check [[Quick-Reference]] and
[[Glossary]] for standing conventions before continuing it.

## Batches

- [[idx-100000-102999]] — idx 100,000–102,999 (`df960e8`, 2026-09-25)
- idx 103,000–104,999 (session 42, 2026-09-26, 2,000 rows) — brought the phase to 5,000/30,000.
- idx 105,000–109,999 (session 43, 2026-09-26, 5,000 rows) — brought the phase to 10,000/30,000.
- idx 110,000–111,999 (session 51, 2026-09-27, 2,000 rows) — brought the phase to 12,000/30,000.
  No new terminology decisions; two `<...>`-tag content mismatches (idx 110288, idx 111619) were
  caught by `qa_check.py --locale` and fixed before appending — see [[Current-Status]]'s session
  51 write-up for detail.
- idx 112,000–113,999 (session 52, 2026-09-28, 2,000 rows) — brought the phase to 14,000/30,000.
  No new terminology decisions; two mismatches were caught by `qa_check.py --locale` and fixed
  before appending: idx 112741 (a `<TEXT id='text1'>...</TEXT>` UI-composition string had a `%s`
  placeholder duplicated into the wrong tag segment) and idx 113465 (a plain `<Busy>` tag had its
  content translated instead of kept verbatim per Quick-Reference §6 rule 1) — see
  [[Current-Status]]'s session 52 write-up for detail.
- idx 114,000–115,999 (session 53, 2026-09-28, 2,000 rows) — brought the phase to 16,000/30,000.
  No new terminology decisions; one mismatch was caught by `qa_check.py --locale` and fixed before
  appending: idx 115949 (a second `#H` tag in a Partnership-formed string was mistakenly typed as
  `#Y`) — see [[Current-Status]]'s session 53 write-up for detail.
