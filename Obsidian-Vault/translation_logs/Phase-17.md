# Phase 17 Translation Log (idx 400,000–429,886)

**Reconstructed retroactively on 2026-09-26** — this file was not written during
the sessions that did the work, so it only records what the commit history
shows (batch boundaries, dates, row counts), not the session-by-session
terminology reasoning that [[Phase-6]] and [[Update-1]] capture for earlier
phases. Check [[Quick-Reference]] and [[Glossary]] for the standing
conventions that applied throughout.

Phase 17 is the first phase since [[Phase-6]] to be carried through to
completion (29,887 rows, idx 400,000–429,886), run across sessions from
2026-09-24 to 2026-09-26.

## Batches

- Batch 1 — idx 400,000–400,400 (`53f6892`, 2026-09-24)
- Batch 2 — idx 400,401–400,999 (`2ee54c1`, 2026-09-24, "Continue Phase 17 translation to idx 400999")
- Batch 3 — idx 401,000–403,999 (`a3f890a`, 2026-09-24)
- Batch 4 — idx 404,000–406,999 (`7f21f2a`, 2026-09-24)
- Batch 5 — idx 407,000–409,999 (`2b26499`, 2026-09-25)
- Batch 6 — idx 410,000–411,999 (`4c215f4`, 2026-09-25)
- Batch 7 — idx 412,000–413,999 (`1e46592`, 2026-09-25)
- Batch 8 — idx 414,000–415,999 (`64b3dd3`, 2026-09-25)
- Batch 9 — idx 416,000–419,999 (`68c253b`, 2026-09-26, 4000 rows)
- Batch 10 — idx 420,000–421,999 (`05387af`, 2026-09-26)
- Batch 11 — idx 422,000–423,999 (`344d4d3`, 2026-09-26)
- Batch 12 — idx 424,000–429,886 (`5becee9`, 2026-09-26, 5887 rows — phase completion)

A merge (`87327d2`, "Merge origin/main (PR #8, Phase 17 batch 3) into tooling
branch") landed alongside batch 3, indicating this phase was worked in
parallel with a separate tooling branch/PR for part of its run.
