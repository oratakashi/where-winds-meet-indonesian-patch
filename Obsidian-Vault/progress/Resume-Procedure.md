# Resume Procedure

Step-by-step checklist for picking up a translation session, plus the token
validation that must pass before any batch is appended. See
[[Current-Status]] for where things stand right now and [[Glossary]] for the
rules to apply while translating.

## How these files relate

- `translation_work/unique_strings.jsonl` — 461,704 unique strings (idx
  0–461,703): the original 429,887 ordered by descending frequency (idx
  0–429,886), plus 31,817 appended after the 2026-09 update (idx
  429,887–461,703 — see [[Update-1]]). Never regenerate it with
  `translation_work/build_unique.py` (that would renumber every idx); new
  strings are only ever appended by `tools/rebuild_unique_strings.py`.
- `locale/phase*.jsonl` — translation output, append-only,
  `{"idx": N, "v": "..."}` per line, one file per phase (see
  [[Phase-Roadmap]]). The last line in the active phase's file marks the
  most recently completed idx.
- Progress tracking itself lives in [[Current-Status]] (snapshot),
  [[Session-History]] (chronological log), and this file (how to pick a
  session back up, below). See "Building a playable patch" further down
  for how these files get merged into a deployable patch.

## How to resume a session

1. Read [[Phase-Roadmap]] — check the active phase (number N), its idx
   range, and the suggested batch size.
2. Read [[Quick-Reference]] — a condensed cheat-sheet of the locked
   conventions (names that aren't translated, terms kept in English, style
   per context) used most often. **Don't translate anything before reading
   this**, to stay consistent with everything already done. Only open a
   specific file under `knowladge/glossary/` (see [[Glossary]] for the
   index) when Quick-Reference doesn't cover a case in the batch, you need
   the reasoning/history behind a rule, or the term is on the "not yet
   locked" list — don't open all nine topic files by default, that's the
   overhead Quick-Reference exists to avoid.
3. Check the last completed idx: `wc -l locale/phase{N}.jsonl` (if the file
   doesn't exist yet, this phase hasn't started — next idx = the phase's
   starting idx; if it exists, next idx = phase start idx + number of lines
   in the file).
4. Read the next batch from `translation_work/unique_strings.jsonl` starting
   at line (next_idx + 1) (1-indexed), for as many lines as the active
   phase's suggested batch size (`Read` with `offset`/`limit`).
5. Translate each `v` per [[Glossary]], writing to a scratch file as
   `{"idx": N, "v": "..."}` per line (idx stays the **absolute** idx from
   `unique_strings.jsonl`), in ascending idx order.
6. **Validate before appending — no exceptions**: run
   `python tools/qa_check.py --locale <scratch file>` (see below) to make
   sure `#E`, `#aabbcc`, `#X`, `%s`, `%d`, `{0}`, `<...>` etc. appear the
   same number of times as in the source.
7. Append the batch to `locale/phase{N}.jsonl` (the currently active
   phase's file — **never** write into another phase's file; if the active
   phase's idx range runs out partway through a batch, cut the batch there
   and let the remainder start a new `locale/phase{N+1}.jsonl`). Confirm idx
   stays strictly sequential with no gaps/duplicates within each file
   (`qa_check.py --locale` on the phase file checks this too).
8. **Refresh the overall progress — mandatory, every session:** run
   `python tools/progress.py --write`. It recomputes the "Overall progress"
   and "Phase status" tables in [[Current-Status]] from `locale/*.jsonl`
   and `strings.jsonl` — never type those numbers by hand (CI runs
   `tools/progress.py --check` and fails a PR whose block is stale). Then
   update the prose in [[Current-Status]] (session date/number, what the
   batch contained, any new terminology decisions — also add those to
   [[Glossary]] and the matching `translation_logs/Phase-N` file) and
   append an entry to [[Session-History]]. Both session write-ups and the
   end-of-session message to the user must quote the overall line from the
   script output, e.g. "Overall: 123,817 / 461,704 unique strings (26.82%),
   in-game coverage 64.20%". If a new decision is a high-value/frequently-
   recurring pattern (not a one-off edge case), also add a one-line entry
   to [[Quick-Reference]] — otherwise the cheat-sheet goes stale and future
   sessions fall back to opening the full topic files anyway. Niche
   edge-cases only need the topic file, not Quick-Reference. If the active
   phase's idx range is now exhausted, advance to the next phase per
   [[Phase-Roadmap]].
9. **Stop cleanly at the end of the batch** — don't push until
   context/tokens are nearly exhausted. A short, cleanly logged session
   beats a long one that gets cut off mid-batch without validation or a
   status update.

## Minimize round-trips per session

Raw content isn't what drives token cost up: a 1,000-string batch is only
~13k tokens of source text and ~15k of translated output. What actually
balloons a session's token usage is the **number of tool-call round
trips** — every `Read`/`Write`/`Bash` call resends the entire conversation
so far, so splitting one batch into many small back-and-forths (translate
50 → write → read back to check → translate 50 more → ...) makes the
accumulated context (glossary + batch + prior output) get re-sent over and
over. Keep it to a handful of calls per session:

- One `Read` for the whole batch (`offset`/`limit`), not several partial
  reads.
- Translate the full batch in one pass, into one scratch file.
- One append to `locale/phase{N}.jsonl` for the batch, not one per
  sub-chunk.
- **If the batch had to be read in multiple chunks** (e.g. a 1,000-row batch
  split into 250-row `Read` calls because of tool output limits) and each
  chunk was translated into its own scratch file: after `cat`-merging the
  chunks, check the merged line count equals the expected batch size
  *before* running token validation. A chunk-boundary slip (a duplicated or
  dropped row) desyncs every `idx` label after it, but token validation
  alone won't reliably catch this since it looks up by `idx` — a mismatched
  label can still resolve to a plausible source row without tripping the
  regex check. See [[Session-History]] §2026-09-22 batch 18 for a case
  where this happened.
- Run the token/placeholder validation **once** at the end and
  trust its "0 mismatches" output — don't re-open the full phase file or
  `unique_strings.jsonl` afterward just to double-check by eye.
- If a batch feels too big to translate carefully in one pass, reduce the
  batch size for next session (see [[Current-Status]]) rather than
  splitting this session's batch into many small read/write/verify loops.

## Token/placeholder validation

Run from the repo root on the merged scratch file, then on the phase file
after appending:

```bash
python tools/qa_check.py --locale scratch_batch.jsonl
python tools/qa_check.py --locale locale/phase6.jsonl     # <- the active phase
python tools/qa_check.py --locale "locale/*.jsonl"        # full cross-file check
```

It looks every `idx` up in `translation_work/unique_strings.jsonl` and
reports `MARKUP` (token counts differ), `EMPTY`, `PROMPT_LEAK`, and `IDX`
(unknown idx, gap/out-of-order idx within a file, or an idx duplicated
across files). Exit code 0 = clean. Token order is **not** checked — moving
a token to fit Indonesian word order is fine as long as none is added,
dropped, or altered.

This replaces the inline Python snippet older sessions copy-pasted from
here. That snippet's regex tried `#[A-Za-z]` before `#[0-9a-fA-F]{6}`, so a
color like `#e9a35f` was read as `#e` and a damaged color code passed
(idx 54382 — see [[Placeholders-And-Formatting]]). There is now one regex,
the `TOKEN` constant in `tools/qa_check.py`; don't write a separate one.

Some placeholders are **not** caught by the `TOKEN` regex but must still be
preserved character-for-character: `$VAR$`/`$P`/`$N`-style variables,
`@T[...]`/`@t[...]` date placeholders, `$link<...>^ID^$` link blocks, and
literal duration shorthand outside of tags. See [[Placeholders-And-Formatting]]
and the relevant `translation_logs/Phase-N` file for the full list — these
need a manual check on top of the automated one.

## Building a playable patch (any time — partial translation is fine)

The expansion from per-idx translations to per-entry patches is implemented
and matches by literal source text, so it works on every file variant:

```bash
# after a game update only: append newly seen strings (existing idx untouched)
python tools/rebuild_unique_strings.py --dry-run
python tools/rebuild_unique_strings.py

# validate the whole dictionary, then patch every translate_words_map_* at once
python tools/qa_check.py --locale "locale/*.jsonl"
python tools/patch_all.py --outdir patched            # --level 19 for a release build
```

`patch_all.py` writes repacked copies of `translate_words_map_en`, `_diff`,
`__small`, `__small_diff`, and `_mobile` under `patched/`. For a single
file, `tools/expand_locale.py` + `wwm_locmap.py patch` does the same via an
intermediate JSONL (and `wwm_locmap.py patch` refuses a JSONL whose `h`
doesn't match the target file, i.e. one dumped from another game version).
Only the `Package\...` copies of the base and `__small` files stay patched
in a live install — see [[Format-Spec]] §7.

See [[Phase-Roadmap]] for the reasoning behind picking a stopping point
before 100% coverage is reached.
