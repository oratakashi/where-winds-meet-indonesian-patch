# Current Status

**Last updated: 2026-09-23 (session 10).** This is the single source of truth for "how
far are we" — it gets overwritten each session, not appended to. For the
full timeline, see [[Session-History]]; for the phase plan, see
[[Phase-Roadmap]]; for the resume checklist, see [[Resume-Procedure]].

## Totals

- Total lines in `strings.jsonl`: **963,050** (pre-update version; the
  2026-09 update's main file has 826,388 — see [[Session-History]] §2026-09-16).
- Total unique strings: **461,704** (429,887 original + 31,817 added by the
  2026-09 game update, idx 0–461,703).

## Phase status

| Phase | Range           | Status      | Next idx                                       |
| ----- | --------------- | ----------- | ---------------------------------------------- |
| 0     | 0–799           | done        | —                                              |
| 1     | 800–1,999       | done        | —                                              |
| 2     | 2,000–4,999     | done        | —                                              |
| 3     | 5,000–9,999     | done        | —                                              |
| 4     | 10,000–19,999   | done        | —                                              |
| 5     | 20,000–49,999   | done        | —                                              |
| 6     | 50,000–99,999   | **active**  | **76,000** (26,000/50,000 rows done, 52.00%)   |
| 7     | 100,000–129,999 | not started | 100,000                                        |
| 8     | 130,000–159,999 | not started | 130,000                                        |
| 9     | 160,000–189,999 | not started | 160,000                                        |
| 10    | 190,000–219,999 | not started | 190,000                                        |
| 11    | 220,000–249,999 | not started | 220,000                                        |
| 12    | 250,000–279,999 | not started | 250,000                                        |
| 13    | 280,000–309,999 | not started | 280,000                                        |
| 14    | 310,000–339,999 | not started | 310,000                                        |
| 15    | 340,000–369,999 | not started | 340,000                                        |
| 16    | 370,000–399,999 | not started | 370,000                                        |
| 17    | 400,000–429,886 | not started | 400,000                                        |
| Update-1 | 429,887–461,703 | **active** | **449,737** (19,850/31,817 rows done, ~62.39%) |


Phases 0–17 draw from the original `unique_strings.jsonl` (idx 0–429,886).
Update-1 is the addition from the 2026-09-16 game update (idx
429,887–461,703 — see [[Phase-Roadmap]]). Phases 7–17 replace the old
100,000/229,887-string "Phase 7"/"Phase 8" split with eleven ~30,000-string
phases (rebalanced 2026-09-22 — see [[Phase-Roadmap]]).

**Phase 6 and Update-1 are both in progress in parallel.** If the user
doesn't say which one to continue, ask before starting — see
[[Resume-Procedure]].

## Most recent session (2026-09-23, session 10)

Update-1, batch 19: idx 448,737–449,736 (1,000 rows) translated and appended
to `locale/update1.jsonl`. Total now 19,850/31,817 rows done for Update-1
(~62.39%). Mix of content: Mohist City/Hidden Mountain lore (Yi Xieyu's
death scene, Master Jian's Night-of-Falling-Sky backstory, the Mozi/Mohist
Hill founding legend, Mohist Research/thesis-publishing mechanics), several
long emotional letters (An Ya's torn letter, the Qiang prison letter, the
"home safe" wife's diary across dozens of pages, Chunhe's mother's farewell
letter), Tang-dynasty museum-piece flavor text (agate rhyton, crystal cup,
silver flask, bronze horse, calligraphy pieces), gear/skill tooltips
(Stonesplit/Spring Sorrow/Infernal Twinblades Sin-Karma mechanics, Echo
Skill/Divinecraft Water Boost chain), casual gue/lo NPC dialogue (Wuyou and
Di Juan's wine-shop scene, Zhang Niu's mockery, Mingxi's recovery), and
Jiazhong/Huangzhong season-transition patch notes. No new terminology
decisions — every case matched an existing [[Glossary]] entry.

Token/placeholder validation via the standard `TOKEN` regex script passed
with **0 mismatches**, checked against `unique_strings.jsonl` by idx (line
count of the merged 4-chunk scratch batch verified at exactly 1,000 before
validation, per the [[Resume-Procedure]] lesson from session 8). Full-file
re-validation: `locale/update1.jsonl` now 19,850 lines, all idx unique, 0
duplicates, 0 token mismatches across the entire file.

Phase 6 was not touched this session — it was already advanced to
26,000/50,000 rows (batch 26, idx 75,000–75,999) by a parallel session
earlier today (see [[Session-History]] §2026-09-23 session 9); that
progress is reflected in the table above.

## Batch size

Current default: **1,000 strings/session** (as of 2026-09-23, per the
user's request this session — unchanged from the prior session's setting).
This has changed several times over the project — see [[Session-History]]
for the full change log before assuming it's still 1,000 in a future
session.
