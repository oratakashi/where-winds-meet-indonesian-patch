# Current Status

**Last updated: 2026-09-21.** This is the single source of truth for "how
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
| 6     | 50,000–99,999   | **active**  | **65,000** (15,000/50,000 rows done)           |
| 7     | 100,000–199,999 | not started | 100,000                                        |
| 8     | 200,000–429,886 | not started | 200,000                                        |
| 9     | 429,887–461,703 | **active**  | **445,737** (15,850/31,817 rows done, ~49.81%) |


Phases 0–8 draw from the original `unique_strings.jsonl` (idx 0–429,886).
Phase 9 is the addition from the 2026-09-16 game update (idx 429,887–461,703
— see [[Phase-Roadmap]]).

**Phase 6 and Phase 9 are both in progress in parallel.** If the user
doesn't say which one to continue, ask before starting — see
[[Resume-Procedure]].

## Most recent session (2026-09-21)

Phase 6, batch 8: idx 63,000–64,999 (2,000 rows) translated and appended to
`locale/phase6.jsonl`, at the user's requested 2,000-strings/iteration size
(total now 15,000/50,000 rows done for Phase 6, 30%). Same mix as prior
batches: heavy `freq: 2` random player-username entries (kept verbatim),
Chinese-style NPC names, gear/skill UI labels, stat labels, and casual
gue/lo dialogue, plus several longer narrative pieces (the Chai-family
loyalty stele, the Guo Xin/Chen Hu Anxi-army elegy, the Zhu Youjia Nine
Mortal Ways escape story, the Yan Ying chess-strategist tale, the
age-reversal/disguise recipe, and the Deepwave Vessel legend). No new
terminology decisions — every case matched an existing [[Glossary]] entry,
though 6 honorific mistranslations ("Master Moonstream" kept-English
instead of "Guru Moonstream", "Young Master" ×3 instead of "Tuan Muda",
"Granny" ×3 instead of "Nenek") were caught and fixed by a manual
consistency grep before merging — see [[Phase-6]] for the full list.
Caught and fixed 2 token issues (idx 63835, 64825 — a stat-formatted
`<Name|id|#C|...>` tag's apostrophe/plural was dropped while restructuring
the sentence around it), then 0 mismatches, 0 EMPTY hits, 0 real hits on
a separate untagged-placeholder check. Full merged `locale/phase6.jsonl`
(15,000 lines) re-validated: idx sequential 50,000–64,999, no duplicates.

Phase 9 was not touched this session (still at next idx 445,737).

## Batch size

Current default: **2,000 strings/session** (as of 2026-09-21, reconfirmed
this session). This has changed several times over the project — see
[[Session-History]] for the full change log before assuming it's still
2,000 in a future session.
