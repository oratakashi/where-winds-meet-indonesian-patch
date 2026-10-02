# Phase 15 Translation Log (idx 340,000–349,999)

The starter batch (idx 340,000–341,999) was **reconstructed retroactively on 2026-09-26** — it was not logged during
the session that did the work, so for it only the commit history and `locale/phase15.jsonl` are known. Later batches
are logged as they are done.

`locale/phase15.jsonl` has 10,000 rows (idx 340,000–349,999); the phase is **not complete** — see
[[Current-Status]] for the idx range still outstanding (next idx 350,000). Check [[Quick-Reference]] and
[[Glossary]] for standing conventions before continuing.

## Batches

- [[idx-340000-341999]] — idx 340,000–341,999 (`384f262`, 2026-09-26)
- [[idx-342000-345999]] — idx 342,000–345,999 (session 69, 2026-10-02, 4,000 rows) — brought the phase to 6,000/30,000 (20.00%). No new terminology decisions. Reaffirmed: durations `#Y5#Es`→`#Y5#Ed` / `{}h`→`{}j` / `{diff_hour:02d}h{diff_minute:02d}m`→`…j…m`; "Red Packet" → "Angpao"; `Big Zhao/Feng` → `Kak Zhao/Feng`; `Master X`/`Elder X` → `Guru X`/`Tetua X`; hex-prefixed colour tokens and `<LINK>`/`<i>` composite tags kept byte-for-byte; formal letters/mail use Anda. QA: 0 mismatches on the first pass — see [[Current-Status]]'s session 69 write-up.
- [[idx-346000-349999]] — idx 346,000–349,999 (session 70, 2026-10-02, 4,000 rows) — brought the phase to 10,000/30,000 (33.33%). No new terminology decisions. Reaffirmed: durations `#Y8.4s#E`→`#Y8.4d#E`, `1.5s`/`15s`→`1.5d`/`15d`, `Unlocks in {}d`→`{}h`, `(3d)`→`(3h)`, spelled-out units → detik/menit with decimal comma; "Red Packet" → "Angpao"; `Big Zhao/Feng` → `Kak Zhao/Feng`; `Master X`/`Elder X` → `Guru X`/`Tetua X`; plain `<Tag>`/whole-sentence `<...>` and `<LINK …>` tags kept byte-for-byte; formal mail/legal text uses Anda. QA: 0 mismatches on the first pass — see [[Current-Status]]'s session 70 write-up.
