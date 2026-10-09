# Phase 11 Translation Log (idx 220,000–249,999)

**Starter batch reconstructed retroactively on 2026-09-26** — that part of this file
was not written during the session that did the work, so it only records what the
commit history and `locale/phase11.jsonl` show, not session-by-session reasoning.

Phase 11 started with a single starter batch, added in `2a503a5` ("Update
progress phase 11", 2026-09-25) alongside a Session-History entry ("Add
Session 32 entry to Session-History for Phase 11 start"), and was continued on
2026-10-09 (sessions 99 and 100). `locale/phase11.jsonl` now has 10,000 rows
(idx 220,000–229,999); the phase is **not complete** — see [[Current-Status]]
for the full idx range still outstanding (next idx 230,000). No terminology
decisions were recorded for the starter batch; check [[Quick-Reference]] and
[[Glossary]] for standing conventions before continuing it.

## Batches

- [[idx-220000-221999]] — idx 220,000–221,999 (`2a503a5`, 2026-09-25)
- [[idx-222000-225999]] — idx 222,000–225,999 (session 99, 2026-10-09, 4,000 rows) — brought the phase to 6,000/30,000 (20.00%). No new core terminology decisions; reaffirmed honorifics (`Elder X`→`Tetua X`, `Master X`→`Guru X`, `Big Feng/Zhao`/`Brother X`→`Kak X`, `Aunt/Granny/Grandpa X`→`Bibi/Nenek/Kakek X`, `Madam/Miss X`→`Nyonya/Nona X`, `Young Master`→`Tuan Muda`, `Hall Master`→`Ketua Hall`), duration shorthand (`Cooldown: 60s`→`60d`, `#Y12#Es`→`#Y12#Ed`, `(3d)`→`(3h)`), decimal comma in prose vs dot inside colour spans, dotted thousands; `<LINK>`/`<IMAGE>` tags and whole-sentence `<…>` rows kept verbatim; `#8c583F`+`ellowship` colour-token edge case (223679); one off-by-one label (224384/224385) caught by the post-merge sweep.
- [[idx-226000-229999]] — idx 226,000–229,999 (session 100, 2026-10-09, 4,000 rows) — brought the phase to 10,000/30,000 (33.33%). No new core terminology decisions; reaffirmed honorifics, duration shorthand (`60 seconds`→`60 detik`, `30s`→`30d`, `{1}d{2}h`→`{1}h{2}j`), decimal comma in prose vs dot inside colour spans, `(Beast Tongue)`→`(Bahasa Binatang)`, `<…>` whole-sentence rows kept verbatim, developer mails in kamu/-mu register.
