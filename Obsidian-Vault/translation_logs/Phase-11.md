# Phase 11 Translation Log (idx 220,000–249,999)

**Starter batch reconstructed retroactively on 2026-09-26** — that part of this file
was not written during the session that did the work, so it only records what the
commit history and `locale/phase11.jsonl` show, not session-by-session reasoning.

Phase 11 started with a single starter batch, added in `2a503a5` ("Update
progress phase 11", 2026-09-25) alongside a Session-History entry ("Add
Session 32 entry to Session-History for Phase 11 start"), and was continued on
2026-10-09 (sessions 99, 100, 101, 102, 107 and 109). `locale/phase11.jsonl` now has 26,000 rows
(idx 220,000–245,999); the phase is **not complete** — see [[Current-Status]]
for the full idx range still outstanding (next idx 246,000). No terminology
decisions were recorded for the starter batch; check [[Quick-Reference]] and
[[Glossary]] for standing conventions before continuing it.

## Batches

- [[idx-220000-221999]] — idx 220,000–221,999 (`2a503a5`, 2026-09-25)
- [[idx-222000-225999]] — idx 222,000–225,999 (session 99, 2026-10-09, 4,000 rows) — brought the phase to 6,000/30,000 (20.00%). No new core terminology decisions; reaffirmed honorifics (`Elder X`→`Tetua X`, `Master X`→`Guru X`, `Big Feng/Zhao`/`Brother X`→`Kak X`, `Aunt/Granny/Grandpa X`→`Bibi/Nenek/Kakek X`, `Madam/Miss X`→`Nyonya/Nona X`, `Young Master`→`Tuan Muda`, `Hall Master`→`Ketua Hall`), duration shorthand (`Cooldown: 60s`→`60d`, `#Y12#Es`→`#Y12#Ed`, `(3d)`→`(3h)`), decimal comma in prose vs dot inside colour spans, dotted thousands; `<LINK>`/`<IMAGE>` tags and whole-sentence `<…>` rows kept verbatim; `#8c583F`+`ellowship` colour-token edge case (223679); one off-by-one label (224384/224385) caught by the post-merge sweep.
- [[idx-226000-229999]] — idx 226,000–229,999 (session 100, 2026-10-09, 4,000 rows) — brought the phase to 10,000/30,000 (33.33%). No new core terminology decisions; reaffirmed honorifics, duration shorthand (`60 seconds`→`60 detik`, `30s`→`30d`, `{1}d{2}h`→`{1}h{2}j`), decimal comma in prose vs dot inside colour spans, `(Beast Tongue)`→`(Bahasa Binatang)`, `<…>` whole-sentence rows kept verbatim, developer mails in kamu/-mu register.
- [[idx-230000-233999]] — idx 230,000–233,999 (session 101, 2026-10-09, 4,000 rows) — brought the phase to 14,000/30,000 (46.67%). No new core terminology decisions; reaffirmed honorifics, duration shorthand (`Ns`→`Nd`, `#Y18#Es`→`#Y18#Ed`, `(30d)`→`(30h)`, `{diff_hour:02d}h`→`{diff_hour:02d}j`, `N seconds`→`N detik`), decimal comma in prose vs dot inside colour spans, dotted thousands (`9,000`→`9.000`, `250,000`→`250.000`), `(Beast Tongue)`→`(Bahasa Binatang)`, `#HPassive:#E`→`#HPasif:#E`, `<…>` whole-sentence rows kept verbatim.

- [[idx-234000-237999]] — idx 234,000–237,999 (session 102, 2026-10-09, 4,000 rows; commit pending) — brought the phase to 18,000/30,000 (60.00%). No new core terminology decisions; reaffirmed NPC gue/lo voice, neutral UI/narrative, aku/kamu letters, honorific translations, duration shorthand and English stat/skill names. Format tags and literal escape sequences preserved; source-equality sweep caught four missed dialogue lines before append. Overall: 393,704 / 461,704 unique strings (85.27%), in-game coverage 93.78%.

- [[idx-238000-241999]] — idx 238,000–241,999 (session 107, 2026-10-09, 4,000 rows; commit pending) — brought the phase to 22,000/30,000 (73.33%). No new core terminology decisions; standing voice, honorific, named-skill/stat and duration conventions retained. Locust-leaf noodles refers to foliage, translated as mi daun akasia rather than insect leaves. All markup, engine placeholders and line breaks preserved; full locale QA clean. Overall: 410,704 / 461,704 unique strings (88.95%), in-game coverage 95.32%.

- [[idx-242000-245999]] — idx 242,000–245,999 (session 109, 2026-10-09, 4,000 rows; commit pending) — brought the phase to 26,000/30,000 (86.67%). No new core terminology decisions; standing gue/lo voice for NPCs, neutral UI, honorifics (Elder→Tetua, Master→Guru, Aunt/Granny→Bibi/Nenek), duration shorthand (`60s`→`60d`, `#Y3#Es`→`#Y3#Ed`, `(3d)`→`(3h)`, `{diff_second:d}s`→`{diff_second:d}d`), decimal comma in prose, dotted thousands. Whole-sentence `<…>` rows and `<Beast Tongue>`-style tags kept verbatim. Overall: 418,704 / 461,704 unique strings (90.69%), in-game coverage 96.06%.
