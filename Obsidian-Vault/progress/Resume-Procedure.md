# Resume Procedure

Step-by-step checklist for picking up a translation session, plus the token
validation script that must pass before any batch is appended. See
[[Current-Status]] for where things stand right now and [[Glossary]] for the
rules to apply while translating.

## How these files relate

- `translation_work/unique_strings.jsonl` — 429,887 unique strings, ordered by
  descending frequency (idx 0..N-1), plus 31,817 more appended after the
  2026-09 update (idx up to 461,703 — see [[Phase-9]]).
- `locale/phase*.jsonl` — translation output, append-only,
  `{"idx": N, "v": "..."}` per line, one file per phase (see
  [[Phase-Roadmap]]). The last line in the active phase's file marks the
  most recently completed idx.
- Progress tracking itself lives in [[Current-Status]] (snapshot),
  [[Session-History]] (chronological log), and this file (how to pick a
  session back up, below). See "After all phases" further down for how
  these two files eventually get merged into a deployable patch.

## How to resume a session

1. Read [[Phase-Roadmap]] — check the active phase (number N), its idx
   range, and the suggested batch size.
2. Read [[Glossary]] — it contains every locked convention (names that
   aren't translated, terms kept in English, style per context). **Don't
   translate anything before reading this**, to stay consistent with
   everything already done.
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
6. **Validate before appending — no exceptions**: run the token/placeholder
   check below to make sure `#E`, `#aabbcc`, `#X`, `%s`, `%d`, `{0}`, etc.
   match the source exactly in count and order. This is the same check
   `tools/qa_check.py` runs later.
7. Append the batch to `locale/phase{N}.jsonl` (the currently active
   phase's file — **never** write into another phase's file; if the active
   phase's idx range runs out partway through a batch, cut the batch there
   and let the remainder start a new `locale/phase{N+1}.jsonl`). Confirm idx
   stays strictly sequential with no gaps/duplicates within each file (see
   the validation script below — just change the filename).
8. Update [[Current-Status]] (last completed idx, phase, session date, and
   any new terminology decisions — also add those to [[Glossary]] and the
   matching `translation_logs/Phase-N` file) and append an entry to
   [[Session-History]]. If the active phase's idx range is now exhausted,
   advance to the next phase per [[Phase-Roadmap]].
9. **Stop cleanly at the end of the batch** — don't push until
   context/tokens are nearly exhausted. A short, cleanly logged session
   beats a long one that gets cut off mid-batch without validation or a
   status update.

## Token/placeholder validation script

Run from the repo root (swap `phase4.jsonl` for whichever phase file is
active):

```python
import json, re
from collections import Counter

TOKEN = re.compile(r'#[A-Za-z]|#[0-9a-fA-F]{6}|%s|%d|\{[^}]*\}|<[^>]*>')

src = {}
with open('translation_work/unique_strings.jsonl', encoding='utf-8') as f:
    for line in f:
        d = json.loads(line)
        src[d['idx']] = d['v']

mismatches = []
with open('locale/phase4.jsonl', encoding='utf-8') as f:  # <- change to the active phase
    for line in f:
        d = json.loads(line)
        s = src[d['idx']]
        t = d['v']
        cs = Counter(TOKEN.findall(s))
        ct = Counter(TOKEN.findall(t))
        if cs != ct:
            mismatches.append((d['idx'], s, t))

print('mismatches:', len(mismatches))
for m in mismatches[:20]:
    print(m)
```

For a full cross-phase check (also useful to confirm no idx is duplicated
across two different phase files), glob `locale/phase*.jsonl`, load every
line, and run the same comparison over the combined set.

Some placeholders are **not** caught by the `TOKEN` regex above but must
still be preserved character-for-character: `$VAR$`/`$P`/`$N`-style
variables, `@T[...]`/`@t[...]` date placeholders, `$link<...>^ID^$` link
blocks, and literal duration shorthand outside of tags. See [[Glossary]] and
the relevant `translation_logs/Phase-N` file for the full list — these need
a manual check on top of the automated script.

## After all phases (or a chosen stopping point) are done

Not yet implemented as a script. The plan: read
`translation_work/unique_strings.jsonl` plus every `locale/phase*.jsonl`
(merged) to build a `source text → translated text` dictionary, then stream
the original `strings.jsonl`, replacing the `v` field per that dictionary
(skipping/keeping the original where nothing was translated yet — partial
translation is supported by `patch`, per `CLAUDE.md`), writing the result to
`strings.translated.jsonl`. Then run:

```bash
python tools/qa_check.py strings.jsonl strings.translated.jsonl
python wwm_locmap.py patch translate_words_map_en strings.translated.jsonl <out>
```

See [[Phase-Roadmap]] for the reasoning behind picking a stopping point
before 100% coverage is reached.
