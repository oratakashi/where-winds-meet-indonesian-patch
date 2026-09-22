# Session History

A chronological, append-only log of what happened each session: which phase
was worked, what idx range was completed, batch-size changes, and process
notes (sub-batch counts, mismatches found and fixed). Terminology decisions
made during these sessions are **not** repeated here — see the matching
`translation_logs/Phase-N` file for those. For the current resting state,
see [[Current-Status]].

## Before 2026-09-15

Phases 0–3 completed (idx 0–9,999). No session dates were recorded for this
period in the original working notes.

## 2026-09-15 — Phase 4 continuation

Batch size fixed at **1,000 strings/session** at the user's request — the
default until further notice.

## 2026-09-16 — Game update: new file variants discovered

The user updated the game client and found two new locale files:
`translate_words_map_en__small` and `__small_diff`, alongside the existing
`translate_words_map_en`/`_diff`. This triggered a tooling pass:

- Confirmed the binary format is unchanged (byte-identical dump→patch→parse
  round trip on all four files) — no changes needed to `wwm_locmap.py`. See
  [[Format-Spec]] §4.6 for the technical findings (98% key overlap between
  `__small` and the main file, but 193 values differ and 27 keys are
  `__small`-exclusive).
- The main file shrank from 963,050 → 826,388 entries (138,445 keys
  removed, 1,783 added, 32,397 changed) and was re-sharded (3,762 → 3,229
  shards) — no impact on the tool, since the parser never recomputes shard
  placement itself.
- Fixed a bug in `tools/expand_locale.py`: it crashed with `KeyError: 'v'`
  when run against a dump containing tombstones (`"deleted": true`, from
  `_diff` files) — now skipped, same as `qa_check.py` already did.
- Added two new scripts: `tools/rebuild_unique_strings.py` (appends newly
  observed strings after a game update to `unique_strings.jsonl` **without
  touching old idx values** — this is how Update-1 was created) and
  `tools/patch_all.py` (applies the existing translation dictionary to all
  known `translate_words_map_*` variants in one pass).
- Re-dumped `strings.jsonl` from the updated main file. Expanded
  `unique_strings.jsonl` to idx 461,703 (+31,817 new strings — this became
  Update-1). Applied the existing dictionary (Phases 0–4, 20,000 translated
  strings) to all four file variants via `patch_all.py`:

  | file                                  | entries | matched (already translated) |
  | -------------------------------------- | ------- | ------------------------------ |
  | `translate_words_map_en`               | 826,388 | 358,162 (43.34%)               |
  | `translate_words_map_en_diff`          | 212,117 | 38,094 (17.96%)                |
  | `translate_words_map_en__small`        | 4,009   | 2,148 (53.58%)                 |
  | `translate_words_map_en__small_diff`   | 0       | 0                               |

  All patched output passed `qa_check.py` with 0 findings.
- Translating the 31,817 new Update-1 strings was deferred to later sessions
  (out of scope for this tooling-focused session).

## 2026-09-17 — Update-1 opened; `_diff` deployment limitation found; Phase 5 continues

- **Update-1 opened.** At the user's explicit request, the session jumped to
  Update-1 first (batch 1, idx 429,887–431,886) even though Phase 5 was still
  unfinished (stopped at idx 37,000). From this point, **Phase 5 and Update-1
  run in parallel** — a session must ask the user which phase to continue if
  not stated explicitly. Processed in 4 sub-batches of 500, validated
  together; 2 token mismatches found and fixed before appending.
- **Update-1 batch 2** (idx 431,887–433,886): processed in 8 sub-batches of
  250, validated together; 4 token mismatches found and fixed.
- **Update-1 batch 3** (idx 433,887–435,886): processed in 8 sub-batches of
  250; 5 token mismatches found via validation, plus 2 non-token
  inconsistencies ("steward") found via a separate manual audit.
- User then explicitly asked to switch back to **Phase 5** instead of Phase
  9. Batch size **re-confirmed at 2,000 strings/session** (doubled from the
  1,000/session default set on 2026-09-15) — this becomes the new default
  until further notice.
- **Phase 5** idx 37,000–38,999 completed. Next idx Phase 5 = 39,000; Phase
  9 unaffected this batch (next idx 435,887).
- **Phase 5** idx 39,000–40,999 completed (batch 5). Next idx Phase 5 =
  41,000.
- **Phase 5** idx 41,000–42,999 completed (batch 6). Next idx Phase 5 =
  43,000. Processed in 4 sub-batches of 500; 0 mismatches on the first
  validation pass.
- **Finding: the installed `_diff` file cannot be patched permanently.**
  Investigating the game's install folder and `LocalData\patch_log\`
  revealed that the game keeps **two separate copies** of
  `translate_words_map_en_diff` — the one under `Package\HD\oversea\locale\`
  that this repo's tools read/write (stable), and a second one under
  `LocalData\Patch\HD\oversea\locale\` that the running game actually loads
  for the `_diff` layer. The second copy is **checksum-verified and silently
  restored from NetEase's CDN on every game launch**
  (`StagePatchList`/`StageCheck`/`StageDownload` in the launcher log) —
  confirmed directly: a patched copy reverted to the pristine file within
  minutes. Full technical detail is kept in `CLAUDE.md` ("Deployment gotcha:
  the installed `_diff` file gets re-verified by the game's own CDN
  patcher"), since that file remains the technical source of truth for this
  gotcha. **Practical implication**: only the base
  `translate_words_map_en` and `__small` files hold permanently — focus
  translation effort there; `_diff` content (~213k entries, ~26% of the
  2026-09 update's total entries) will read as English in-game until
  NetEase eventually merges `_diff` back into the base package, at which
  point whatever fraction is already translated becomes permanent
  automatically.

## 2026-09-18 — Alternating Phase 5 and Update-1 batches (dense session day)

A high-throughput day covering both active phases:

1. **Phase 5** idx 43,000–44,999 completed (batch 7). Next idx Phase 5 =
   45,000.
2. **Update-1** idx 435,887–437,136 completed (batch 4, 1,250 of a planned
   2,000 rows) — translation was delegated to a background subagent; the
   user asked to stop it early after 1,250/2,000 strings ("looks like
   enough"), so this batch is smaller than the usual 2,000. Validated (token
   check + manual fixes) before appending. Next idx Update-1 = 437,137.
3. **Update-1** idx 437,137–439,136 completed (batch 5, full 2,000 rows, done
   directly by the main session rather than delegated). Next idx Update-1 =
   439,137.
4. **Phase 5** idx 45,000–46,999 completed (batch 8). Next idx Phase 5 =
   47,000 (12.5 of the original 15 sessions-at-2,000 estimate for Phase 5
   now done).
5. **Phase 5** idx 47,000–48,999 completed (batch 9). Next idx Phase 5 =
   49,000 (only 1,000 rows of Phase 5 remained after this).
6. **Phase 5 closes; Phase 6 opens.** idx 49,000–49,999 (1,000 rows)
   completed Phase 5 in full (idx 20,000–49,999, 30,000/30,000 rows) —
   `locale/phase5.jsonl` is now final. The remaining 1,000 rows of the
   2,000/session target rolled over into Phase 6 per the standard
   "cut the batch at the phase boundary" procedure (see
   [[Resume-Procedure]]), opening `locale/phase6.jsonl` for the first time
   (1,000/50,000 rows of this phase done). Next idx Phase 6 = 51,000.
7. **Phase 6** idx 51,000–52,999 completed (batch 2, 2,000 rows;
   3,000/50,000 rows of Phase 6 done). Next idx Phase 6 = 53,000.
8. **Update-1** idx 439,137–441,136 completed (batch 6, 2,000 rows;
   11,250/31,817 rows of Update-1 done, ~35.36%). Next idx Update-1 =
   441,137.

## 2026-09-19 — Update-1 continues, batch size lowered

**Update-1** idx 441,137–442,136 completed (batch 7, 1,000 rows;
12,250/31,817 rows of Update-1 done, ~38.50%). The user **lowered the batch
size to 1,000 strings/session** (down from 2,000) — this becomes the new
default until further notice, superseding the 2026-09-17 preference. Next
idx Update-1 = 442,137. Phase 6 untouched this session (still at next idx
53,000).

## 2026-09-20 — Update-1 continues, batch size raised again

**Update-1** idx 442,137–444,136 completed (batch 8, 2,000 rows). The user
**raised the batch size back to 2,000 strings/session** (reconfirmed at the
start of the session) — current default. Processed in 8 sub-batches of 250,
merged and validated together: 0 token mismatches across all 2,000 rows
after merging, including a full re-validation of the combined
`update1.jsonl` (14,250 rows at the time) and a cross-check for 0
duplicate/gap idx across every `locale/phase*.jsonl` file (67,250 total
rows/idx recorded with no collisions). Next idx Update-1 = 444,137
(14,250/31,817 rows of Update-1 done, ~44.78%). Phase 6 untouched this
session (still at next idx 53,000).

## 2026-09-20 (continued) — Update-1 batch 9, session paused mid-batch then resumed

**Update-1** idx 444,137–445,736 completed (batch 9, 1,600 rows — a shortened
batch: the user asked to stop after 1,400 rows/7 sub-batches, then, since
sub-batch 8 (200 rows) had already been drafted and validated, asked to
include it anyway rather than discard it). Translated and validated in 8
sub-batches of 200 each (0 token mismatches per sub-batch), then combined
and re-validated as one file before appending: 0 mismatches across all
1,600 rows, idx sequential with no gaps/duplicates, and a full re-validation
of the resulting `update1.jsonl` (15,850 rows) confirmed clean. Next idx
Update-1 = 445,737 (15,850/31,817 rows done, ~49.81%). Phase 6 untouched
this session (still at next idx 53,000).

## 2026-09-21 — Phase 6 batch 3

**Phase 6** idx 53,000–54,999 completed (batch 3, 2,000 rows), at the
user's requested 2,000-strings/iteration size — reconfirming the 2,000
default. Translated and validated in 4 sub-batches of 500 each (0 token
mismatches per sub-batch), then re-validated the full merged
`locale/phase6.jsonl` (5,000 rows): 0 mismatches, idx sequential
50,000–54,999, no duplicates. No new terminology decisions — see
[[Phase-6]] for the one process note (internal mechanic/animation
debug-style labels left untranslated, not yet promoted to a locked
[[Glossary]] rule). Next idx Phase 6 = 55,000 (5,000/50,000 rows done, 10%).
Update-1 untouched this session (still at next idx 445,737).

## 2026-09-21 — Phase 6 batch 4

**Phase 6** idx 55,000–56,999 completed (batch 4, 2,000 rows), at the
user's requested 2,000-strings/iteration size. Translated and validated
directly against `unique_strings.jsonl` for the full 2,000-row batch: 0
token mismatches, 0 EMPTY hits, then appended and re-validated the full
merged `locale/phase6.jsonl` (7,000 rows): idx sequential 50,000–56,999, no
duplicates. No new terminology decisions — every case matched an existing
[[Glossary]] entry; see [[Phase-6]] for the one flagged-but-unresolved
ambiguous case ("Master Qi"/"Master Pu" as a non-teacher, non-org-leader
honorific, kept English pending a clearer rule if it recurs). Next idx
Phase 6 = 57,000 (7,000/50,000 rows done, 14%). Update-1 untouched this
session (still at next idx 445,737).

## 2026-09-21 — Phase 6 batch 5

**Phase 6** idx 57,000–58,999 completed (batch 5, 2,000 rows), at the
user's requested 2,000-strings/iteration size. Translated and validated
directly against `unique_strings.jsonl` for the full 2,000-row batch: found
and fixed 1 token mismatch (idx 57636, `<Light Attacks|...>` accidentally
singularized to `<Light Attack|...>` inside a stat-formatted tag), then 0
mismatches, 0 EMPTY hits, and 0 mismatches on a separate untagged-
placeholder check (`$VAR$`, `@T[...]`, `$link<...>^ID^$`). Appended and
re-validated the full merged `locale/phase6.jsonl` (9,000 rows): idx
sequential 50,000–58,999, no duplicates. No new terminology decisions —
every case matched an existing [[Glossary]] entry; see [[Phase-6]] for
the full batch notes. Next idx Phase 6 = 59,000 (9,000/50,000 rows done,
18%). Update-1 untouched this session (still at next idx 445,737).

## 2026-09-21 — Phase 6 batch 6

**Phase 6** idx 59,000–60,999 completed (batch 6, 2,000 rows), at the
user's requested 2,000-strings/iteration size. Translated and validated
directly against `unique_strings.jsonl` for the full 2,000-row batch:
found and fixed 2 issues (idx 60957, a `<...>`-tag long-sentence violation
where the bracketed English content had been translated by mistake; idx
60068, a garbled-number typo introduced while restructuring a sentence
around a `#e9a358`-style color tag), then 0 mismatches, 0 EMPTY hits. Also
caught and fixed a drafting error mid-batch: one source line (idx 59441)
was skipped while translating, which shifted ~17 subsequent idx labels
down by one and produced one fabricated line — found by re-reading the
exact source range and comparing line-by-line, fixed before merging. No
new terminology decisions — every case matched an existing [[Glossary]]
entry; see [[Phase-6]] for the full batch notes. Appended and re-validated
the full merged `locale/phase6.jsonl` (11,000 rows): idx sequential
50,000–60,999, no duplicates. Next idx Phase 6 = 61,000 (11,000/50,000
rows done, 22%). Update-1 untouched this session (still at next idx
445,737).

## 2026-09-21 — Phase 6 batch 7

**Phase 6** idx 61,000–62,999 completed (batch 7, 2,000 rows), at the
user's requested 2,000-strings/iteration size. Translated and validated
directly against `unique_strings.jsonl` for the full 2,000-row batch:
found and fixed 1 issue (idx 62806, a source-side malformed tag —
`#YFledgling Appearance Chests#` is missing its closing `E`, i.e. it's
`#` not `#E` — the draft translation had "fixed" it to a proper `#E`,
which broke the token-count match; corrected to reproduce the source's
typo exactly rather than fixing it), then 0 mismatches, 0 EMPTY hits. No
new terminology decisions — every case matched an existing [[Glossary]]
entry; see [[Phase-6]] for the full batch notes. Appended and
re-validated the full merged `locale/phase6.jsonl` (13,000 rows): idx
sequential 50,000–62,999, no duplicates. Next idx Phase 6 = 63,000
(13,000/50,000 rows done, 26%). Update-1 untouched this session (still at
next idx 445,737).

## 2026-09-21 — Phase 6 batch 8

**Phase 6** idx 63,000–64,999 completed (batch 8, 2,000 rows), at the
user's requested 2,000-strings/iteration size. Translated and validated
directly against `unique_strings.jsonl` for the full 2,000-row batch:
found and fixed 2 token mismatches (idx 63835 and 64825, both a
`<Name|id|#C|...>` stat-tag's apostrophe/plural dropped while
restructuring the surrounding sentence), then 0 mismatches, 0 EMPTY hits,
and 0 real hits on a separate untagged-placeholder check (`$VAR$`,
`@T[...]`/`@t[...]`, `$link<...>^ID^$`, `$S...$E`). A manual
honorific-consistency grep across the full batch also caught 6
mistranslations before merging — "Master Moonstream", "Young Master"
(×3), and "Granny" (×3, one NPC name appears twice) had been left
kept-English by over-generalizing from other "Master"/proper-noun
patterns instead of applying the [[Honorifics-And-Titles]] table
directly; all fixed to Guru/Tuan Muda/Nenek. See [[Phase-6]] for the
full batch notes, including the mid-batch bash-heredoc issue (very long
single `cat >> file <<EOF` commands were silently truncated by the tool
at very large sizes, producing a 10-line idx gap that was caught and
refilled by a completeness check rather than lost — kept batches to
~50 lines per shell call afterward). Appended and re-validated the full
merged `locale/phase6.jsonl` (15,000 rows): idx sequential 50,000–64,999,
no duplicates. Next idx Phase 6 = 65,000 (15,000/50,000 rows done, 30%).
Update-1 untouched this session (still at next idx 445,737).

## 2026-09-22 — Phase 6 continuation, batch size dropped to 1,000

User requested the per-iteration batch size be reduced from 2,000 to
**1,000 strings/session** going forward — the new default until further
notice (see [[Current-Status]] Batch size).

**Phase 6** idx 65,000–65,999 completed (batch 9, 1,000 rows) at this new
size. Read the full [[Glossary]] topic-file set before translating (all
nine topic files under `knowladge/glossary/`), then translated the batch
in ten internal sub-batches of ~100 rows each (written to scratch files
and concatenated) to keep each `Write` call a manageable size, rather than
one very long inline block. Same content mix as prior Phase 6 batches:
heavy `freq: 2` random player-username entries with irregular internal
capitalization (kept verbatim), Chinese-style NPC pinyin names, gear/skill/
stat UI labels kept English per [[Kept-In-English-Terms]], casual gue/lo
dialogue, plus several longer narrative/lore pieces — see [[Phase-6]] for
the content list.

One inline consistency catch during drafting: idx 65450 ("Auto Track"
Complete 4 Sect Commands) was initially left with "Sect" untranslated,
corrected to "Command Sekte" to match the locked Sect→Sekte rule (see
[[Wuxia-And-Cultural-Terms]]) before merging — caught by comparing against
the "Sect Rule"→"Aturan Sekte" translation used earlier in the same batch,
not by the automated grep.

Validation after merging the ten sub-batches: 0 token/placeholder
mismatches via the standard `TOKEN` regex script, 0 mismatches on a
separate untagged `$...$`/`@T[...]` placeholder check, 0 EMPTY values, and
a grep sweep for stray untranslated honorifics (Master, Young Master,
Granny, Grandpa, Aunt, Uncle, Elder, Lord, Mr., Sect, Doctor, Wanderer,
Wayfarer) found nothing left un-converted after the one fix above. Full
merged `locale/phase6.jsonl` (16,000 rows) re-validated: idx sequential
50,000–65,999, no duplicates. Next idx Phase 6 = 66,000 (16,000/50,000
rows done, 32%). Update-1 untouched this session (still at next idx
445,737).

## 2026-09-22 — Phase 6 batch 10

**Phase 6** idx 66,000–66,999 completed (batch 10, 1,000 rows) at the
current 1,000-strings/iteration size. Translated in two ~500-row
sub-batches (scratch files via the `Write` tool, no shell heredoc). Same
content mix as prior Phase 6 batches: heavy `freq: 2` random
player-username entries with irregular internal capitalization (kept
verbatim), Chinese-style NPC pinyin names, gear/skill/stat UI labels kept
English per [[Kept-In-English-Terms]], casual gue/lo dialogue, plus several
longer narrative/lore pieces — see [[Phase-6/idx-66000-66999|the batch
note]] for the content list. No new terminology decisions.

Token/placeholder validation via the standard `TOKEN` regex script found
**3 mismatches** on the first pass: idx 66159 (a plain, non-stat `<...>`
tag mistakenly translated instead of kept verbatim), idx 66626 (an
apostrophe-s dropped from inside a stat tag's content, changing the tag
text), and idx 66897 (the second of two `#Yresonance#E` occurrences left
as untagged plain text). All three fixed and re-validated: 0 mismatches, 0
EMPTY values. Full merged `locale/phase6.jsonl` (17,000 rows) re-validated:
idx sequential 50,000–66,999, no duplicates. Next idx Phase 6 = 67,000
(17,000/50,000 rows done, 34%). Update-1 untouched this session (still at
next idx 445,737).

## 2026-09-22 — Phase 6 batch 11, batch size restored to 2,000

Batch size changed back up to **2,000 strings/session** at the user's
request (was reduced to 1,000 for the previous session only) — the default
until further notice.

**Phase 6** idx 67,000–68,999 completed (batch 11, 2,000 rows) at the new
2,000-strings/iteration size. Translated in four 500-row sub-batches
(scratch files via the `Write` tool). Same content mix as prior Phase 6
batches: heavy `freq: 2` random player-username entries with irregular
internal capitalization (kept verbatim), Chinese-style NPC pinyin names,
gear/skill/stat UI labels kept English per [[Kept-In-English-Terms]],
casual gue/lo dialogue, plus several longer narrative/lore pieces (the
Wingfall/Rampage/Argent Oath spear-reserve history note, the Liu Xiaomei
Dragonbend Mountain ode, the Grand Historian's burnt-book fragment on the
benefits/harms of water, the astronomer "Lord of Clouds" star-observation
legend, and the "Righteous and Elusive Red Heroine" letter). No new
terminology decisions — every case matched an existing [[Glossary]] entry.

Token/placeholder validation via the standard `TOKEN` regex script: **0
mismatches** on the first pass across all 2,000 rows. Full merged
`locale/phase6.jsonl` (19,000 rows) re-validated: idx sequential
50,000–68,999, no duplicates. Next idx Phase 6 = 69,000 (19,000/50,000
rows done, 38%). Update-1 untouched this session (still at next idx
445,737).

## 2026-09-22 — Phase 6, batch 12 (2,000-strings/iteration continued)

Phase 6: idx 69,000–70,999 (2,000 rows) translated in one pass and appended
to `locale/phase6.jsonl`. Same content mix as prior batches: heavy `freq: 2`
random player-username entries with irregular internal capitalization (kept
verbatim), Chinese-style NPC pinyin names, gear/skill/stat UI labels kept
English per [[Kept-In-English-Terms]], casual gue/lo dialogue, plus several
longer narrative/lore pieces (the Li Tiegu/Khitan-raider vignette, the Gold
Leaf Case murder-mystery excerpt, the Celestial Spring Station ghost-station
legend, the vajra/Zhang Yichao reflection, the "Ten Sages' Collection" last
testament, the Layla and Majnun folklore note, and the Poet's Soul rhapsody
ghost story). No new terminology decisions — every case matched an existing
[[Glossary]] entry.

Token/placeholder validation via the standard `TOKEN` regex script: **0
mismatches** on the first pass across all 2,000 rows. Full merged
`locale/phase6.jsonl` (21,000 rows) re-validated: idx sequential
50,000–70,999, no duplicates. Next idx Phase 6 = 71,000 (21,000/50,000
rows done, 42%). Update-1 untouched this session (still at next idx
445,737).

## Undated note

At some point before this history was consolidated, `strings.jsonl` and
`translate_words_map_en` were already staged in git; at the user's request
("leave it as is") that staged state wasn't touched. `translation_work/` and
`strings.jsonl` were added to `.gitignore` going forward so they won't be
committed again, but the pre-existing staged files were left alone.
