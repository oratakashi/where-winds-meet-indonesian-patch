# Phase 9 Translation Log (idx 160,000–161,999)

**Reconstructed retroactively on 2026-09-26** — this file was not written during
the session that did the work, so it only records what the commit history and
`locale/phase9.jsonl` show, not session-by-session reasoning.

Note the numbering history: an earlier "Phase 9" batch was renamed to
[[Update-1]] (`fab99ca` — "Rebalance Phase 7/8 into even phases, rename Phase 9
to Update-1"), and this file covers the *new* Phase 9 that was started
afterward, committed 2026-09-25 (`f297e2b` — "Start Phase 9 translation (idx
160000-161999, 2000 rows)"). `locale/phase9.jsonl` has 2,000 rows; the phase is
**not complete** — see [[Current-Status]] for the full idx range still
outstanding. No terminology decisions were recorded for this batch; check
[[Quick-Reference]] and [[Glossary]] for standing conventions before
continuing it.

## Batches

- [[idx-160000-161999]] — idx 160,000–161,999 (`f297e2b`, 2026-09-25)
- [[idx-162000-165999]] — idx 162,000–165,999 (session 76, 2026-10-03, 4,000 rows) — brought the phase to 6,000/30,000 (20.00%). No new terminology decisions; reaffirmed duration shorthand, `Elder`/`Master`/`Big X` honorifics, the `#A…` format-code prefix rule, plain `<Tag>` kept verbatim.
- [[idx-166000-169999]] — idx 166,000–169,999 (session 77, 2026-10-03, 4,000 rows) — brought the phase to 10,000/30,000 (33.33%). No new terminology decisions; reaffirmed duration shorthand, honorifics, composite-tag and plain `<Tag>` rules.
- [[idx-170000-173999]] — idx 170,000–173,999 (session 78, 2026-10-04, 4,000 rows) — brought the phase to 14,000/30,000 (46.67%). No new terminology decisions; reaffirmed duration shorthand (`%dd`→`%dh`, `{diff_hour:d}h`→`j`), honorifics, decimal comma, and byte-for-byte `<#…#E>`/plain `<Tag>` handling.
- [[idx-174000-177999]] — idx 174,000–177,999 (session 79, 2026-10-04, 4,000 rows) — brought the phase to 18,000/30,000 (60.00%). No new terminology decisions; reaffirmed duration shorthand (`(30d)`→`(30h)`, `#Y10#Es`→`#Y10#Ed`, `{diff_day:d}d`→`h`), honorifics, decimal comma, composite `<LINK>`/`<IMAGE>`/`<TEXT>` tags kept byte-for-byte, literal `\\n` preserved.
- [[idx-178000-181999]] — idx 178,000–181,999 (session 80, 2026-10-04, 4,000 rows) — brought the phase to 22,000/30,000 (73.33%). No new terminology decisions; reaffirmed duration shorthand (`%sd`→`%sh`, `#Y5#Es`→`#Y5#Ed`, `{diff_second:02d}s`→`d`), honorifics, `X Hour` → `Jam X`, decimal comma, `#R…#E` token preserved in `#RHadiah Angpao#E`, `$D$F…$E`/`$STEADY_CRI` placeholders and `<Stat|780|#C|n>` composites byte-for-byte.
