# Current Status

**Last updated: 2026-09-20 (session 2).** This is the single source of truth for "how
far are we" — it gets overwritten each session, not appended to. For the
full timeline, see [[Session-History]]; for the phase plan, see
[[Phase-Roadmap]]; for the resume checklist, see [[Resume-Procedure]].

## Totals

- Total lines in `strings.jsonl`: **963,050** (pre-update version; the
  2026-09 update's main file has 826,388 — see [[Session-History]] §2026-09-16).
- Total unique strings: **461,704** (429,887 original + 31,817 added by the
  2026-09 game update, idx 0–461,703).

## Phase status

| Phase | Range | Status | Next idx |
| ----- | ----- | ------ | -------- |
| 0 | 0–799 | done | — |
| 1 | 800–1,999 | done | — |
| 2 | 2,000–4,999 | done | — |
| 3 | 5,000–9,999 | done | — |
| 4 | 10,000–19,999 | done | — |
| 5 | 20,000–49,999 | done | — |
| 6 | 50,000–99,999 | **active** | **53,000** (3,000/50,000 rows done) |
| 7 | 100,000–199,999 | not started | 100,000 |
| 8 | 200,000–429,886 | not started | 200,000 |
| 9 | 429,887–461,703 | **active** | **445,737** (15,850/31,817 rows done, ~49.81%) |

Phases 0–8 draw from the original `unique_strings.jsonl` (idx 0–429,886).
Phase 9 is the addition from the 2026-09-16 game update (idx 429,887–461,703
— see [[Phase-Roadmap]]).

**Phase 6 and Phase 9 are both in progress in parallel.** If the user
doesn't say which one to continue, ask before starting — see
[[Resume-Procedure]].

## Most recent session (2026-09-20, session 2)

Phase 9, batch 9: idx 444,137–445,736 (1,600 rows) translated and appended
to `locale/phase9.jsonl`. The user asked for 2,000 strings/iteration at the
start of this session, but asked to stop after 1,400 rows partway through,
then — since the 8th 200-row sub-batch had already been drafted and
validated — asked to include it anyway rather than throw it away, landing
at 1,600 rows for this batch. The batch covered a wide spread of Hidden
Mountain/Sky Citadel/Dragonbend Academy content: the Qin Fen origin story,
the Fang Bai letter, the Old Mohist City wind-chime lore, the Yutang war
letter (Ragehoof Hamlet/Northern Vow), the Dragonbend Academy fishing
parable ("chum the water, then set the bait"), a Northern Vow New Year
gift-giving letter, a Zhenming-era travel diary (the Wang Qing bandit
gags), and a long letter home about academy life and star-charting. See
[[Phase-9]] for the full terminology notes from this batch.

Phase 6 was not touched this session (still at next idx 53,000).

## Batch size

Current default: **2,000 strings/session** (as of 2026-09-20). This has
changed several times over the project — see [[Session-History]] for the
full change log before assuming it's still 2,000 in a future session. Note
that the 2026-09-20 session 2 batch landed at 1,600 rather than 2,000 for
session-specific reasons (see [[Session-History]]) — this isn't a change to
the standing default.
