---
name: wwm-game-update
description: Migrate this repo to a new Where Winds Meet game version or hotfix — compare the new translate_words_map_* files against their *.before copies, append new strings to unique_strings.jsonl as a new Update block, and update tools, CI workflows and the Obsidian vault docs. Use whenever the user says NetEase/WWM shipped an update, patch, or hotfix, drops new translate_words_map files in the repo root, or asks to "update ke versi baru", "cek patch terbaru", or retarget the release to a new game version.
---

# WWM game-update migration

The repeatable procedure for a new game version (sessions 126 = v2.3, 148 = v2.3 hotfix).
It is a **setup-only** migration: no translation happens here; translating the new block is
a normal translation session afterwards (see `Obsidian-Vault/progress/Resume-Procedure.md`).
Follow the repo rules in `AGENTS.md`: no subagents, never run `translation_work/build_unique.py`,
never hand-edit the generated progress tables, and don't commit/push unless the user asks.

## 0. Preconditions

- The new files are in the repo root under their normal names; the previous version's copies
  are next to them as `*.before` (gitignored). If `.before` copies are missing, ask the user
  for them before going on — they're needed to tell what changed.
- `git status` shows only the game binaries as modified (otherwise ask what the other changes are).

## 1. Measure (read-only)

```bash
python .Codex/skills/wwm-game-update/scripts/compare_update.py
```

It reports, per `translate_words_map_*` file: byte-identical or not, block/entry/tombstone
counts vs `.before`, keys added/removed, values changed, how many texts aren't in the
dictionary yet, the combined new-string total with its future idx range, variants that
appeared or disappeared, and the projected `unique_strings.jsonl` size (GitHub warns at 50 MB,
**rejects at 100 MB**). Use `PYTHONIOENCODING=utf-8` if the Windows console chokes.

Read the result carefully:
- **New variant** (no `.before`): confirm `python wwm_locmap.py info <file>` parses it, then it
  must be added to `FILES` in `tools/patch_all.py`, the release `files:` list, and possibly
  `TRACKED_DUMPS` in `tools/progress.py`.
- **Variant gone** (only `.before`): check on the user's client(s) whether NetEase really
  dropped it (v2.3 dropped `__small`/`__small_diff`); then remove it from `FILES`,
  `rebuild_unique_strings.py` defaults, `progress.py` dumps, the release and the README.
- **Parse error**: stop — the container/shard format changed; that's reverse-engineering
  work, not this procedure (see `Obsidian-Vault/knowladge/Format-Spec.md`).
- **Projected size > 100 MB**: stop and discuss with the user before appending.

## 2. Ask the user (one AskUserQuestion call)

Show the numbers from step 1, then ask:
- **Block name**: a new game version gets `Update-N` (next number); a hotfix of the current
  version may continue its letters (`Update-2D` after `2A/2B/2C`). It's the user's call.
- **Splitting**: blocks of ~30,000 strings (`Update-NA/B/C…`) if the total is large.
- **Scope**: setup only (recommended) vs. setup + translate in the same session.
- Anything structural the measurement raised (a new/removed variant, `_diff`-only change →
  whether to report `_diff` coverage, release title wording like "v2.3 Hotfix").

## 3. Apply

1. Re-dump the untracked dumps from the new binaries:
   `python wwm_locmap.py dump translate_words_map_en strings.jsonl` and
   `python wwm_locmap.py dump translate_words_map_en_diff strings_diff.jsonl` (plus any other
   variant, passed via `--extra`).
2. `python tools/rebuild_unique_strings.py --dry-run` → the count must equal step 1's total.
   Then run it for real. Confirm the appended idx range and that old rows are untouched
   (`git diff --stat translation_work/unique_strings.jsonl` shows only insertions).
3. `tools/progress.py`: add the block(s) to `PHASES` with exact inclusive idx ranges and a
   dated comment; adjust `TRACKED_DUMPS`/`LOCAL_DUMPS` if variants changed (and the docstring).
   Don't create the empty `locale/updateN.jsonl` — the first translation session does that.
4. CI — keep both workflows in step with `progress.py`:
   - `.github/workflows/translation-qa.yml`: one `wwm_locmap.py dump` step per tracked dump
     before `progress.py --check`; `paths:` filters list every tracked binary.
   - `.github/workflows/release-patch.yml`: same dump steps before "Compute progress summary";
     the release `name:`, header comment and body text name the new game version; `files:`
     lists exactly the shipped variants; no `paths:` entry for gitignored files.
5. `python tools/progress.py --write` (regenerates the tables in `Current-Status.md`).

## 4. Docs (all in the same session)

- `Obsidian-Vault/progress/Current-Status.md`: "Last updated" header (date, session number,
  target version); the intro paragraph under the generated block (previous version done, new
  active block, next idx); a new top session entry with the file stats, idx range, tool/CI
  changes and the overall line.
- `Obsidian-Vault/progress/Phase-Roadmap.md`: mark finished blocks `(done)`/`done`, add the new
  row(s) as **active**, update the unique-string total in the intro, add a paragraph for the
  update, and add the new coverage figures to the footnote.
- `Obsidian-Vault/translation_logs/`: a "How it was created" section in the parent
  `Update-N.md` (or a new `Update-N.md` for a new version), plus a stub log per new block
  (copy the style of `Update-2D.md`: range, output file, empty `## Batches`).
- `Obsidian-Vault/progress/Session-History.md`: append the session entry (check the previous
  session is there too — 147 was missed once).
- `README.md`: the snapshot line (entry/block/tombstone counts per file) and the
  "current release targets" note. `AGENTS.md`: only if variants, dumps or format facts changed.

## 5. Verify, then report

```bash
python tools/rebuild_unique_strings.py --dry-run      # must report 0 new strings
python tools/qa_check.py --locale "locale/*.jsonl"    # 0 findings
python tools/progress.py --check                      # block up to date
python tools/patch_all.py --outdir <scratchpad>/patched --level 10   # every variant repacks
```

`patch_all.py`'s match percentages should equal the coverage rows in the progress block.
Write its output to the scratchpad, never to the repo. Report to the user: what changed per
file, the new block(s) and idx range, the overall progress line (unique % and coverage per
tracked dump), anything skipped, and offer to commit. If the user asks to commit, stage the
files explicitly (binaries, `unique_strings.jsonl`, `tools/`, workflows, vault docs, README),
let the pre-commit hook run, and include the push warning about file size if GitHub prints it.
Then propose the translation of the new block as a follow-up session.
