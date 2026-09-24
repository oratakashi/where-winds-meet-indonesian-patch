# Phase Roadmap — Indonesian Translation

This is the **static roadmap** (strategy, phases, per-phase targets). It
rarely changes — read it once at the start of a session to know which phase
is active and the suggested batch size. For up-to-the-minute state (last
completed idx, exact resume point), see [[Current-Status]]; for the
session-by-session timeline, see [[Session-History]].

## Why the work is split into phases

429,887 unique strings is too much for one session (even several small
ones). To keep each session light and the output careful (not just
haphazard translation), the work is split into **phases** based on `idx`
ranges in `translation_work/unique_strings.jsonl` (already sorted by
descending usage frequency). Batch size per session varies by phase —
earlier phases are smaller (lots of long lore/dialogue text that needs
care), later phases can be larger (mostly short, repetitive-pattern
strings).

## Phases and targets

| Phase             | idx range         | Unique count | Cumulative line coverage*   | Suggested batch/session | Estimated sessions    | Output file            |
| ----------------- | ----------------- | ------------ | --------------------------- | ----------------------- | --------------------- | ---------------------- |
| 0 (done)          | 0 – 799           | 800          | ~16.2%                      | —                       | done                  | `locale/phase0.jsonl`  |
| 1 (done)          | 800 – 1,999       | 1,200        | ~21.28% (measured)          | —                       | done (3 sessions)     | `locale/phase1.jsonl`  |
| 2 (done)          | 2,000 – 4,999     | 3,000        | ~25–28% (measured)          | —                       | done (2 sessions)     | `locale/phase2.jsonl`  |
| 3 (done)          | 5,000 – 9,999     | 5,000        | ~33.5%                      | —                       | done (~6 sessions)    | `locale/phase3.jsonl`  |
| 4 (done)          | 10,000 – 19,999   | 10,000       | ~40.0%                      | —                       | done (7 sessions)     | `locale/phase4.jsonl`  |
| 5 (done)          | 20,000 – 49,999   | 30,000       | ~50.1%                      | —                       | done (15/15 sessions) | `locale/phase5.jsonl`  |
| 6 (active)        | 50,000 – 99,999   | 50,000       | ~60.5%                      | 2,000/session (fixed)   | ~17–25 sessions       | `locale/phase6.jsonl`  |
| 7                 | 100,000 – 129,999 | 30,000       | ~64.6%                      | 2,000/session           | ~10–15 sessions       | `locale/phase7.jsonl`  |
| 8                 | 130,000 – 159,999 | 30,000       | ~68.7%                      | 2,000/session           | ~10–15 sessions       | `locale/phase8.jsonl`  |
| 9                 | 160,000 – 189,999 | 30,000       | ~72.8%                      | 2,000/session           | ~10–15 sessions       | `locale/phase9.jsonl`  |
| 10                | 190,000 – 219,999 | 30,000       | ~76.9%                      | 2,000/session           | ~10–15 sessions       | `locale/phase10.jsonl` |
| 11                | 220,000 – 249,999 | 30,000       | ~81.0%                      | 2,000–3,000/session     | ~10–15 sessions       | `locale/phase11.jsonl` |
| 12                | 250,000 – 279,999 | 30,000       | ~85.1%                      | 2,000–3,000/session     | ~10–15 sessions       | `locale/phase12.jsonl` |
| 13                | 280,000 – 309,999 | 30,000       | ~89.2%                      | 2,000–3,000/session     | ~10–15 sessions       | `locale/phase13.jsonl` |
| 14                | 310,000 – 339,999 | 30,000       | ~93.3%                      | 2,000–3,000/session     | ~10–15 sessions       | `locale/phase14.jsonl` |
| 15                | 340,000 – 369,999 | 30,000       | ~96.3%                      | 3,000–5,000/session     | ~6–10 sessions        | `locale/phase15.jsonl` |
| 16                | 370,000 – 399,999 | 30,000       | ~98.7%                      | 3,000–5,000/session     | ~6–10 sessions        | `locale/phase16.jsonl` |
| 17 (active)       | 400,000 – 429,886 | 29,887       | 100%*                       | 3,000–5,000/session     | ~6–10 sessions        | `locale/phase17.jsonl` |
| Update-1 (active) | 429,887 – 461,703 | 31,817       | additional (see note below) | 2,000/session (fixed)   | ~16 sessions          | `locale/update1.jsonl` |

\* Percentage of the 963,050 total lines in `strings.jsonl`, based on actual
frequency distribution. Phase 2–8 figures are rough interpolations, not
precise per-idx counts — don't treat them as exact.

**Update-1** (formerly labeled "Phase 9") was added on 2026-09-16 after a
game update — it is not part of the original 429,887 unique strings.
`tools/rebuild_unique_strings.py` appended idx 429,887–461,703 (31,817 new
strings) found in the updated `translate_words_map_en`, `_diff`, `__small`,
and `__small_diff` that had **never been recorded** in `unique_strings.jsonl`
before. idx 0–429,886 (Phases 0–17) were **not touched at all** — old
progress stays valid with no remapping needed. See [[Session-History]] and
[[Update-1]] for how `patch_all.py` applies the dictionary across all four
files at once.

**Naming convention going forward:** a game-update string batch (appended by
`rebuild_unique_strings.py`, always a disjoint idx block above the current
max) is always labeled **`Update-N`** (Update-1, Update-2, ...) — never a
`Phase N` number. This keeps the `Phase N` sequence meaning "progress through
the original 429,887-string corpus" and avoids a patch update ever forcing a
renumber of the main phase sequence again. `tools/expand_locale.py` and
`tools/patch_all.py` pick up both `locale/phase*.jsonl` and
`locale/update*.jsonl` automatically.

Live status for each active phase (next idx, most recent batch) lives in
[[Current-Status]]; the full history of which phase was worked when, and
every batch-size change, lives in [[Session-History]].

## Why the output is split into per-phase files

A single giant JSONL file (all 429,887+ lines) isn't practical to
read/diff/open. Each phase gets its own output file under `locale/`,
containing `{"idx": N, "v": "..."}` per line with **absolute** `idx` values
matching `unique_strings.jsonl` (never reset to 0 per file) — so files can
be merged later without any remapping. Resume/validation mechanics are in
[[Resume-Procedure]].

**Realistic total estimate: 125–190+ sessions** to reach 100% of the unique
strings. This is a rough projection and can shift significantly depending on
how much of the distribution's tail (Phases 6–8) turns out to be short,
repetitive-pattern text (faster) vs. unique text needing context (slower).

## Reasonable stopping points (optional)

Coverage isn't linear — the further into the tail of the distribution, the
more effort is needed for smaller gains:

- **~50% of lines** (end of Phase 5, idx ~50,000) is a reasonable midpoint
  for a long pause and a re-evaluation of whether continuing into the tail
  is still worth it.
- Strings deep in the tail (Phases 7–17) are often rare items/skills, debug
  text, or minor variations of already-translated strings — lower gameplay
  value per unit of effort.
- `CLAUDE.md` and `patch` already support **partial translation** (an
  untranslated entry is automatically treated as "intentionally left as
  is", not an error) — so stopping at any point still produces a valid,
  usable patch.

Where to stop is the user's call — this document just gives that decision
some data to work with.

## Process decisions locked early on

- **Deduplicate first** (translate each unique string once), then expand to
  every row — approved by the user; see [[Initial-Brief]].
- **Tone**: character dialogue is casual (gue/lo); UI/system text is
  neutral-casual without gue/lo — approved by the user. Full tone/term
  rules live in [[Glossary]].
- Batch size per session has changed several times as the project went on
  (1,000 → 2,000 → 1,000 → 2,000/session) — the current default and its full
  change history are tracked in [[Session-History]], not here, since it's a
  session-to-session preference rather than a fixed roadmap value.

## One-session procedure (summary — full detail in [[Resume-Procedure]])

1. Read [[Current-Status]] → know the last completed idx and the active
   phase.
2. Read [[Glossary]] → follow the locked conventions.
3. Pull the next batch from `translation_work/unique_strings.jsonl`, sized
   per the active phase's suggested batch size (see the table above).
4. Translate, append to `locale/phase{N}.jsonl` (N = the active phase — see
   the "Output file" column above). Never mix idx from another phase into
   the wrong file.
5. Run the token-validation script (in [[Resume-Procedure]]) — 0 mismatches
   required before continuing.
6. Update [[Current-Status]]: new last-completed idx, active phase, date,
   and any new terminology decisions (also add those to [[Glossary]] and
   the relevant `translation_logs/Phase-N` file).
7. Stop cleanly for the session — don't push until context/tokens run out;
   a session that ends at a clean batch boundary, fully validated and
   logged, beats a longer one that gets cut off mid-batch.

## After all phases (or a chosen stopping point) are done

Merge all `locale/phase*.jsonl` files (per-unique-string) and expand them
into the full `strings.jsonl` (per-line), then run `tools/qa_check.py` and
`wwm_locmap.py patch`. The detailed expansion plan is in
[[Resume-Procedure]] (not yet implemented as a script — to be done once
enough strings are translated, or at the chosen stopping point).
