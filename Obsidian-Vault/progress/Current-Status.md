# Current Status

**Last updated: 2026-09-22 (session 5).** This is the single source of truth for "how
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
| 6     | 50,000–99,999   | **active**  | **75,000** (25,000/50,000 rows done, 50.00%)   |
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
| Update-1 | 429,887–461,703 | **active** | **445,737** (15,850/31,817 rows done, ~49.81%) |


Phases 0–17 draw from the original `unique_strings.jsonl` (idx 0–429,886).
Update-1 is the addition from the 2026-09-16 game update (idx
429,887–461,703 — see [[Phase-Roadmap]]). Phases 7–17 replace the old
100,000/229,887-string "Phase 7"/"Phase 8" split with eleven ~30,000-string
phases (rebalanced 2026-09-22 — see [[Phase-Roadmap]]).

**Phase 6 and Update-1 are both in progress in parallel.** If the user
doesn't say which one to continue, ask before starting — see
[[Resume-Procedure]].

## Most recent session (2026-09-22, session 5)

Phase 6, batch 15: idx 74,000–74,999 (1,000 rows) translated and appended
to `locale/phase6.jsonl`. Total now 25,000/50,000 rows done for Phase 6
(50.00% milestone reached). Mix of content: random player usernames
(kept verbatim), Chinese-style NPC pinyin names, gear/skill/stat UI labels kept
English per [[Kept-In-English-Terms]], casual gue/lo-register dialogue, plus several
longer narrative/lore pieces (spear exile legend after Later Liang fall, Zhang Yichao/Guiyi Army liberation of Hexi lore, Han Dynasty Celestial Spring Station Dunhuang artifact description, Lucky Turtle/Lucky Bag event rules, and Zhao Pu/Wei Zhixi gossip). No new terminology decisions — every case matched an
existing [[Glossary]] entry. Token/placeholder validation via standard `TOKEN` regex script passed with **0 mismatches** after resolving plain tag `<Beast Tongue>` and stat tag `<Herbal Resonance's|...>` apostrophe precision. Full merged `locale/phase6.jsonl` (25,000 lines) re-validated: idx sequential 50,000–74,999, no duplicates.

Update-1 was not touched this session (still at next idx 445,737).

## Batch size

Current default: **1,000 strings/session** (as of 2026-09-22, changed back
down from 2,000 at the user's request this session). This has changed
several times over the project — see [[Session-History]] for the full
change log before assuming it's still 1,000 in a future session.
