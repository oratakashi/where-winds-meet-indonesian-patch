# Current Status

**Last updated: 2026-09-22 (session 8).** This is the single source of truth for "how
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
| Update-1 | 429,887–461,703 | **active** | **448,737** (18,850/31,817 rows done, ~59.24%) |


Phases 0–17 draw from the original `unique_strings.jsonl` (idx 0–429,886).
Update-1 is the addition from the 2026-09-16 game update (idx
429,887–461,703 — see [[Phase-Roadmap]]). Phases 7–17 replace the old
100,000/229,887-string "Phase 7"/"Phase 8" split with eleven ~30,000-string
phases (rebalanced 2026-09-22 — see [[Phase-Roadmap]]).

**Phase 6 and Update-1 are both in progress in parallel.** If the user
doesn't say which one to continue, ask before starting — see
[[Resume-Procedure]].

## Most recent session (2026-09-22, session 8)

Update-1, batch 18: idx 447,737–448,736 (1,000 rows) translated and appended
to `locale/update1.jsonl`. Total now 18,850/31,817 rows done for Update-1
(~59.24%). Mix of content: the Lord of Clouds origin myth (long multi-part
narrative), several Mohist Hill lore pieces (Qiongqi/Golden Colossi history,
the election/scoring process, Elder Toad's backstory), gear/skill tooltip
strings (Bellstrike/Bamboocut/Silkbind attack scaling, Inebriate - Tipsy/
Deepdaze/Bone Corrosion mechanics, Boundvessel, Starweave), NPC dialogue
(Wan Wuyou's backstory, Zhao Tie's letter to General Wang, Reflection
Temple's plague letter), event/UI strings, and casual gue/lo dialogue. No
new terminology decisions — every case matched an existing [[Glossary]]
entry.

**Batch-assembly bug found and fixed before validation**: while merging the
four 250-row read chunks into one scratch file, one entry ("Stop it! Stop
fighting!") got duplicated into the wrong chunk (spurious extra row not in
source at that position) and two adjacent entries ("The army's might
sweeps..." / "Common: Tier 86 Bamboocut Draught") ended up translated in
reversed order — together this desynced every `idx` label after that point
by one for the rest of the batch, though the actual translated text was
always correct and in true source order. Caught by comparing the merged
scratch file's line count (1001) against the expected 1000, root-caused via
binary-search idx spot-checks against `unique_strings.jsonl`, fixed by
removing the spurious duplicate line, restoring source order for the
reversed pair, then reassigning `idx` by strict sequential position
(447,737–448,736) across the whole 1000-row batch — safe here because the
translations were produced by reading `unique_strings.jsonl` linearly with
no skips, so position-based reindexing exactly reconstructs the correct
mapping. Re-validated afterward against source by `idx` lookup (not just
position), confirming every row's tokens match. **Lesson for future
sessions**: when merging multi-chunk batches, verify the merged line count
equals the expected batch size *before* running token validation — token
validation alone won't catch an idx-label/order desync since it also looks
up by `idx`, which was itself wrong here until the recount forced a deeper
check.

Token/placeholder validation via standard `TOKEN` regex script passed with
**0 mismatches** after the fix, checked against `unique_strings.jsonl` by
idx (not position). Full-file re-validation: `locale/update1.jsonl` now
18,850 lines, all idx unique, 0 duplicates, 0 token mismatches across the
entire file (not just this batch).

Phase 6 was not touched this session (still at next idx 75,000).

## Batch size

Current default: **1,000 strings/session** (as of 2026-09-22, changed back
down from 2,000 at the user's request this session). This has changed
several times over the project — see [[Session-History]] for the full
change log before assuming it's still 1,000 in a future session.
