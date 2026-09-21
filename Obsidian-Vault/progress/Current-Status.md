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
| 6     | 50,000–99,999   | **active**  | **61,000** (11,000/50,000 rows done)           |
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

Phase 6, batch 6: idx 59,000–60,999 (2,000 rows) translated and appended to
`locale/phase6.jsonl`, at the user's requested 2,000-strings/iteration size
(total now 11,000/50,000 rows done for Phase 6). Same mix as batch 5: UI
strings, NPC/quest text, gear/skill labels, debug-style strings (e.g.
"No.6 1 Release - Scene 95008"), and scattered random player-username
entries (kept verbatim), plus several longer narrative pieces (the eldest
Zhang son's temple-regret story, the Cui Xiaojin border-guard letter, the
Yuan Xi well-digging passage, the Imperial Archives "Inkbound" vignette,
and the Jiang Dazhen lamp-offering prayer). No new terminology decisions —
every case matched an existing [[Glossary]] entry. Full details are logged
in [[Phase-6]]. Caught and fixed a drafting error mid-batch (one source
line skipped, shifting ~17 idx labels — found and corrected before
merging) plus 2 token issues (a `<...>`-tag long-sentence violation and a
garbled-number typo), then 0 mismatches, 0 EMPTY hits. Full merged
`locale/phase6.jsonl` (11,000 lines) re-validated: idx sequential
50,000–60,999, no duplicates.

Phase 9 was not touched this session (still at next idx 445,737).

## Batch size

Current default: **2,000 strings/session** (as of 2026-09-21, reconfirmed
this session). This has changed several times over the project — see
[[Session-History]] for the full change log before assuming it's still
2,000 in a future session.
