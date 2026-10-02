# Phase 8 Translation Log (idx 130,000–139,999)

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
- idx 136,000–139,999 (session 63, 2026-09-30, 4,000 rows — user raised the cadence to 4,000 per iteration) —
  brought the phase to 10,000/30,000 (33.33%). No new terminology decisions. Reaffirmed: `Old X` without "Man"
  stays English (Old Lee / Old Yang / Old Jin); `Trial: <Name>` titles stay English (idx 137502); `Nd`/`Ns`
  durations → `d` (`{1}s`→`{1}d`, `5s`→`5d`, `8s`→`8d`); a `#Current …#E` label must keep its `#C` token
  (written `#Claju perolehan saat ini: {}%#E`, idx 138546); a `#R…` line whose source has no closing `#E` gets
  none in the translation (idx 137224). QA: the only mismatch was that stray `#E`. Process: pass 1 used bare
  idx labels and drifted by 1–2 rows, so it was realigned with a monotonic matching script; passes 2–8 carried a
  source-prefix guard per row and the merge script auto-corrected mislabelled idx (up to 289 per pass) —
  see [[Current-Status]]'s session 63 write-up.
- [[idx-140000-143999]] — - idx 140,000–143,999 (session 64, 2026-09-30, 4,000 rows) — brought the phase to 14,000/30,000 (46.67%). No new terminology decisions. Reaffirmed: `Heavy Attack`/`Light Attack` labels and `Tier N X: Piece` gear names stay English; `Vendetta Bounty` → `Bounty Dendam`; durations `Ns`→`Nd` and `d`/`h` day/hour shorthand → `h`/`j` (`{total_day:d}d{diff_hour:d}h`→`{total_day:d}h{diff_hour:d}j`, `XXd Xh`→`XXh Xj`, idx 140297/141565); "Master Jin" (teacher) → "Guru Jin"; formal in-world letters (idx 142946, 143324, 143434) use aku/Anda rather than gue/lo. QA: 0 mismatches on the first pass (all eight 500-row passes and the merged batch). Process: source-prefix guard + ±4 idx auto-correct from session 63 reused with 0 unresolved rows; the merge script now also decodes literal `
` (idx 142187, 142831 contain real `
`); the appended file was normalised back to LF line endings — see [[Current-Status]]'s session 64 write-up.
- [[idx-144000-147999]] — idx 144,000–147,999 (session 65, 2026-10-01, 4,000 rows) — brought the phase to 18,000/30,000 (60.00%). No new terminology decisions. Reaffirmed: durations `Ns`→`Nd` / `%dd%dh`→`%dh%dj`; "Red Packet" → "Angpao"; plain `<...>`, `<TEXT>`, `<LINK>` tags kept verbatim with the text between tags translated; hex-prefixed `#8C5823NN Poin#E` keeps its colour token. QA: 0 mismatches on the first pass (all eight passes and the merged batch) — see [[Current-Status]]'s session 65 write-up.
- [[idx-148000-151999]] — idx 148,000–151,999 (session 66, 2026-10-01, 4,000 rows) — brought the phase to 22,000/30,000 (73.33%). No new terminology decisions. Reaffirmed: durations `Ns`→`Nd` (incl. `#Y5#Es`→`#Y5#Ed`, `{status5}s`→`{status5}d`) and day shorthand `%sd`→`%sh`, `%dh ago`→`%dj yang lalu`; "Red Packet" → "Angpao"; `Old X` stays English; `Common: Tier N X Set` / `Weakness (Low): X` labels stay English; `#EAffects` = colour `#EAffec` + `ts` (kept glued). QA: 0 mismatches (all eight passes and the merged batch) — see [[Current-Status]]'s session 66 write-up.
- [[idx-152000-155999]] — idx 152,000–155,999 (session 67, 2026-10-02, 4,000 rows) — brought the phase to 26,000/30,000 (86.67%). No new terminology decisions. Reaffirmed: duration shorthand `Nd`(days)→`Nh` incl. `(90d)`→`(90h)`, `%ss`→`%sd`, `{0}d`→`{0}h`; `Master X` as teacher → `Guru X`, `Elder X` → `Tetua X`, `Big Zhao/Feng` → `Kak Zhao/Feng`; `<IMAGE>`/`<LINK>` composite tags kept byte-for-byte; formal letters/notices use Anda.
- [[idx-156000-159999]] — idx 156,000–159,999 (session 68, 2026-10-02, 4,000 rows) — **completed the phase**: 30,000/30,000 (100.00%). No new terminology decisions. Reaffirmed: durations `Ns`→`Nd`, `(3d)`→`(3h)`, `{}m{}s`→`{}m{}d`, `#Y3#Es`→`#Y3#Ed`; "Red Packet" → "Angpao"; `Master X`/`Elder X` → `Guru X`/`Tetua X`; `<LINK>`/`<IMAGE>`/`<Level 10 - Unbound>` tags kept verbatim. QA: one mismatch (idx 156332, stray `#M`), fixed before appending — see [[Current-Status]]'s session 68 write-up.
