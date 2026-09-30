# Phase 8 Translation Log (idx 130,000–133,999)

**Reconstructed retroactively on 2026-09-26** — this file was not written during
the session that did the work, so it only records what the commit history and
`locale/phase8.jsonl` show, not session-by-session reasoning.

Phase 8 currently holds a single starter batch, committed 2026-09-25
(`f909a1e` — "Start Phase 8 translation (idx 130000-131999, 2000 rows)").
`locale/phase8.jsonl` has 2,000 rows; the phase is **not complete** — see
[[Current-Status]] for the full idx range still outstanding. No terminology
decisions were recorded for this batch; check [[Quick-Reference]] and
[[Glossary]] for standing conventions before continuing it.

## Batches

- [[idx-130000-131999]] — idx 130,000–131,999 (`f909a1e`, 2026-09-25)
- idx 132,000–133,999 (session 61, 2026-09-29, 2,000 rows) — brought the phase to 4,000/30,000
  (13.33%). One decision: a gender-switch string `#handsome#or#beautiful#` (idx 132353) keeps its
  English words, because the QA `TOKEN` regex reads the letter after each `#` as a token (`#h`, `#o`,
  `#b`) and translating them changes the token set. Duration shorthand converted per
  [[Quick-Reference]] §6 (`80s`→`80d`, `%d d %d h`→`%d h %d j`, `{}s`→`{}d`). `qa_check.py --locale`
  came back clean on the first pass for all four 500-row passes — see [[Current-Status]]'s session 61
  write-up for detail.
- idx 134,000–135,999 (session 62, 2026-09-30, 2,000 rows) — brought the phase to 6,000/30,000 (20.00%).
  No new terminology decisions. Reaffirmed: `Inner Way: Tome` keeps "Tome" in English (precedent idx 251943),
  "Red Packet" → "Angpao", duration `Ns`→`Nd` incl. inside `#Y..#E` wraps (`#Y600#Es`→`#Y600#Ed`). `qa_check.py
  --locale` clean on the first pass — see [[Current-Status]]'s session 62 write-up.
