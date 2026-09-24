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

## 2026-09-22 — Phase 6, batch 13 (2,000-strings/iteration continued)

Phase 6: idx 71,000–72,999 (2,000 rows) translated in one pass and appended
to `locale/phase6.jsonl`. Same content mix as prior batches: heavy `freq: 2`
random player-username entries with irregular internal capitalization (kept
verbatim), Chinese-style NPC pinyin names, gear/skill/stat UI labels kept
English per [[Kept-In-English-Terms]], casual gue/lo dialogue, plus several
longer narrative/lore pieces (the Well of Heaven/Guo Xie founding legend,
the Sogdian merchant Kang's "homeland" monologue about Chang'an, the Star
Stealer/Zhao Pu assassination legend, the Erguotou ledger-keeping vignette,
and the River Master's Four Seas verse). No new terminology decisions —
every case matched an existing [[Glossary]] entry.

Token/placeholder validation via the standard `TOKEN` regex script: 3
mismatches on the first pass — idx 71099 (dropped a `#Y...#E` wrap around
"puzzle chests"), idx 71923 (a `<shakes head>` stage direction got
translated to `<menggeleng>` instead of being kept literal per the §6.1
plain-tag rule), idx 72432 (typo'd the stat-tag name, dropping the source's
apostrophe-s: `<Nameless Sword|...>` vs. the correct `<Nameless Sword's|...>`).
All three fixed and re-validated to **0 mismatches**. Full merged
`locale/phase6.jsonl` (23,000 rows) re-validated: idx sequential
50,000–72,999, no duplicates. Next idx Phase 6 = 73,000 (23,000/50,000
rows done, 46%). Update-1 untouched this session (still at next idx
445,737).

## 2026-09-22 — Session 4: Phase 6 batch 14, batch size back to 1,000

Batch size changed back down to **1,000 strings/session** at the user's
request this session (was 2,000 as of session 3).

Phase 6, batch 14: idx 73,000–73,999 (1,000 rows) translated and appended
to `locale/phase6.jsonl`. Same recurring mix: `freq: 2` random
player-username strings (kept verbatim), Chinese-style pinyin NPC names,
gear/skill/stat UI labels kept English, casual dialogue register, plus a
handful of longer lore/narrative entries (Surangama Sutra history, the
Fu/Lu/Shou siblings' dictated letter, the Mahjong God Challenge vignette,
the nameless prodigy's swallow elegy, the Shiye wolf-pack Inner Way origin
story). No new terminology decisions — everything matched existing
[[Glossary]] entries.

Token/placeholder validation via the standard `TOKEN` regex script: 2
mismatches on the first pass — idx 73220 (an extra `#E` closed right after
"Komponen Pribadi" instead of only at the string's end, per the source's
single `#Y...#E` span covering both `{}` placeholders), idx 73924 (translated
text inside a stat tag, `<Pertahanan Fisik|780|#C|17>`, instead of keeping
`<Physical Defense|780|#C|17>` literal per the standard stat-tag rule). Both
fixed and re-validated to **0 mismatches**. Full merged `locale/phase6.jsonl`
(24,000 rows) re-validated: idx sequential 50,000–73,999, no duplicates.
Next idx Phase 6 = 74,000 (24,000/50,000 rows done, 48%). Update-1 untouched
this session (still at next idx 445,737).

## 2026-09-22 — Session 5: Phase 6 batch 15 (1,000 strings/iteration)

Phase 6, batch 15: idx 74,000–74,999 (1,000 rows) translated and appended
to `locale/phase6.jsonl`. Total now 25,000/50,000 rows done for Phase 6
(50.00% milestone reached). Mix of content: random player usernames
(kept verbatim, e.g. `cAlmPath`, `AtlasminotauR`, `speCtralachilles`), Chinese-style pinyin NPC names (kept verbatim, e.g. `Lin Xiansheng`, `Wang Qing`, `Dongfang Yangyue`), gear/skill/stat UI labels kept English per [[Kept-In-English-Terms]] (`Battle Will`, `Tenacity`, `Nameless Sword`, `Breaking Army`, `Jade Fish`), casual gue/lo dialogue register, plus several longer narrative/lore entries (spear exile legend after Later Liang fall, Zhang Yichao/Guiyi Army liberation of Hexi lore, Han Dynasty Celestial Spring Station Dunhuang artifact description, Lucky Turtle/Lucky Bag event rules, and Zhao Pu/Wei Zhixi gossip). No new terminology decisions — everything matched existing [[Glossary]] entries.

Token/placeholder validation via standard `TOKEN` regex script: passed with **0 mismatches** after adjusting plain tag `<Beast Tongue>` and stat tag `<Herbal Resonance's|...>` apostrophe precision. Full merged `locale/phase6.jsonl` (25,000 rows) re-validated: idx sequential 50,000–74,999, no duplicates. Next idx Phase 6 = 75,000 (25,000/50,000 rows done, 50.00%). Update-1 untouched this session (still at next idx 445,737).

## 2026-09-22 — Session 6: Update-1 batch 16 (1,000 strings/iteration)

Update-1, batch 16: idx 445,737–446,736 (1,000 rows) translated and
appended to `locale/update1.jsonl`. Total now 16,850/31,817 rows done for
Update-1 (~52.96%, past halfway). Mix of content: many gear/skill tooltip
strings (Bamboocut/Stonesplit Attack scaling formulas, Inebriate/Deepdaze
state effects, Martial Art Mastery objectives), several long narrative/lore
entries (Chi Qingqian's backstory among the Mohist "Five Wonders" disciples,
the Gao Siji wine-gourd legend from Crossblade Manor, Raging Tides' formal
letter to General Wang Qing about supplies during the Hutuo standoff, the
Luancheng invasion/Zhao Tie account), event/UI strings, and casual gue/lo
dialogue. No new terminology decisions — everything matched existing
[[Glossary]] entries.

Token/placeholder validation via standard `TOKEN` regex script: 2
mismatches on the first pass, both idx where the entire source string is
wrapped in a plain `<...>` tag with nothing outside it (§6 case 3 in
[[Quick-Reference]]) — idx 446278 (`<Stops short, then recognizes you>`)
and idx 446732 (`<Sharpens wits, they say...>`) had been translated inside
the tag instead of kept literal. Both fixed and re-validated to **0
mismatches**. Full batch re-validated: idx sequential 445,737–446,736, no
duplicates; `locale/update1.jsonl` now 16,850 lines. Next idx Update-1 =
446,737. Phase 6 untouched this session (still at next idx 75,000).

## Undated note

At some point before this history was consolidated, `strings.jsonl` and
`translate_words_map_en` were already staged in git; at the user's request
("leave it as is") that staged state wasn't touched. `translation_work/` and
`strings.jsonl` were added to `.gitignore` going forward so they won't be
committed again, but the pre-existing staged files were left alone.

## 2026-09-22 — Update-1, batch 17

idx 446,737–447,736 (1,000 rows) translated and appended to
`locale/update1.jsonl`. Total now 17,850/31,817 rows done for Update-1
(~56.11%). Mix of content: extensive gear/skill tooltip strings (Stonesplit
Attack scaling, Cleftpeak/Cognition/Might/Dust/Splendor/Deluge stack
mechanics, Inebriate-Clash-toast, Zenith Sword), several long narrative
pieces (Fu Qianli's thousand-li journey to Well of Heaven, Xiuxiu's river
song leading refugees north, Gao Siji/Crossblade Manor origin, Qiu
Yuehai/Lie Yan campfire duel, the Lord of Clouds star-chart legend), event/UI
strings, and casual gue/lo dialogue. One case caught by validation: idx
447100, `<He considered himself ordinary...>` — a `<...>` wrapping one
entire sentence with no text outside it (Quick-Reference §6.3 case 3),
initially translated in error and corrected back to 100% English. Token/
placeholder validation via the standard `TOKEN` regex script passed with
**0 mismatches** after that fix. Full batch re-validated: idx sequential
446,737–447,736, no duplicates; `locale/update1.jsonl` now 17,850 lines.

Phase 6 was not touched this session (still at next idx 75,000).

## 2026-09-22 — Update-1, batch 18

idx 447,737–448,736 (1,000 rows) translated and appended to
`locale/update1.jsonl`. Total now 18,850/31,817 rows done for Update-1
(~59.24%). Mix of content: the Lord of Clouds origin myth, Mohist Hill/
Qiongqi lore (Golden Colossi history, election process, Elder Toad
backstory), gear/skill tooltips (Inebriate - Tipsy/Deepdaze/Bone Corrosion,
Boundvessel, Starweave), NPC dialogue and letters (Wan Wuyou, Zhao Tie to
General Wang, Reflection Temple plague letter), event/UI strings, casual
gue/lo dialogue. No new terminology decisions.

Process incident: while merging the four 250-row read chunks into one
scratch file before appending, one entry got duplicated into the wrong
chunk and two adjacent entries were translated in reversed order relative
to source, desyncing every subsequent `idx` label in that chunk by one
(translated text itself was unaffected and stayed in correct source order
throughout). Caught by the merged-file line count (1001, not the expected
1000) rather than by token validation, since token validation looks up by
`idx` and the mismatched idx labels initially still resolved to plausible
(wrong) source entries without tripping the regex check. Fixed by removing
the spurious duplicate row, restoring source order for the reversed pair,
then reassigning `idx` by strict sequential position across the full
1000-row batch (valid here because translation proceeded linearly through
`unique_strings.jsonl` with no skips). Re-validated by `idx` lookup against
source afterward: 0 token mismatches. Full-file check after appending:
`locale/update1.jsonl` now 18,850 lines, idx all unique, 0 duplicates, 0
token mismatches file-wide.

**Process note added to [[Resume-Procedure]] workflow going forward**: when
assembling a batch from multiple read chunks, verify the merged line count
equals the expected batch size before running token validation — that catch
is cheaper than relying on token validation alone to surface an idx/order
desync.

Phase 6 was not touched this session (still at next idx 75,000).

## 2026-09-23 — Phase 6, batch 26

idx 75,000–75,999 (1,000 rows) translated and appended to
`locale/phase6.jsonl`. Total now 26,000/50,000 rows done for Phase 6
(52.00%). Mix of content: Mohist Hill/Golden Colossus lore, Sealed Treasury
heist-event flavor text, Zheng E's tragic backstory, Song-dynasty
historical exposition (Zhao Kuangyin/Zhao Guangyi's southern-conquest
strategy, gunpowder-arrow siegecraft), TCM injury/qi descriptions,
gear/skill tooltips (Bellstrike/Bamboocut/Silkbind scaling, Mystic Skill
Vitality mechanics, Flower Burial skill text), casual gue/lo NPC dialogue,
and a large share of gamertag-style random-username strings left
untranslated. No new terminology decisions; a few judgment calls
(Swordsman→Pendekar, Brother \<Name\>→Kak \<Name\>, quack doctor→tabib
gadungan) applied by analogy to already-locked patterns.

Process incident (different from session 8's): idx 75,100 and 75,101 had
their translated `v` values swapped in the scratch batch file — a plain
manual transcription slip while typing the batch out, not a chunk-merge
desync (the merged file's line count matched the expected 1,000 exactly,
so that check passed). Caught directly by the token/placeholder validation
script: idx 75,100's translation had none of the `#Y`/`{prop}` tokens
present in that idx's actual source text (they'd been typed under 75,101
instead), and vice versa — token validation working exactly as intended
here, unlike the batch-18 case where it couldn't catch a coherent
idx-label desync. Fixed by swapping the two `v` values back so each
matched its correct idx, then re-ran validation for 0 mismatches.

Token/placeholder validation via the standard `TOKEN` regex script passed
with 0 mismatches after the fix. Full-file check after appending:
`locale/phase6.jsonl` now 26,000 lines, idx sequential and contiguous
50,000–75,999, 0 duplicates, 0 token mismatches file-wide.

## 2026-09-23 — Update-1, batch 19

idx 448,737–449,736 (1,000 rows) translated and appended to
`locale/update1.jsonl`, run in parallel with session 9's Phase 6 work.
Total now 19,850/31,817 rows done for Update-1 (~62.39%). Read in four
250-row chunks (tool output limits); merged line count verified at exactly
1,000 before running token validation, per the lesson recorded in session
8's entry above. Content mix: Mohist City/Hidden Mountain lore (Yi Xieyu's
death scene, Master Jian's Night-of-Falling-Sky backstory, the Mozi/Mohist
Hill founding legend), several long emotional letters (An Ya's torn letter,
the Qiang prison letter, the wife's "home safe" diary, Chunhe's mother's
farewell letter), Tang-dynasty museum-piece flavor text, gear/skill
tooltips (Stonesplit/Spring Sorrow/Infernal Twinblades Sin-Karma
mechanics), casual gue/lo NPC dialogue, and Jiazhong/Huangzhong
season-transition patch notes. No new terminology decisions — every case
matched an existing [[Glossary]] entry, so no [[Update-1]] translation-log
entry was needed this batch.

Token/placeholder validation via the standard `TOKEN` regex script passed
with 0 mismatches, checked against `unique_strings.jsonl` by idx. Full-file
re-validation: `locale/update1.jsonl` now 19,850 lines, all idx unique, 0
duplicates, 0 token mismatches across the entire file.

Update-1 was not touched this session (still at next idx 448,737).

## 2026-09-24 — Update-1 batch 20, batch size raised to 3,000

**Update-1** idx 449,737–452,736 completed (batch 20, 3,000 rows), at the
user's requested **3,000-strings/iteration** size — raised from 1,000, the
new default until further notice (see [[Current-Status]] for the
process-overhead note this raise implies). Read from `unique_strings.jsonl`
in 10 sequential 300-line chunks (the `Read` tool's ~25k-token cap forces
smaller reads than usual for this densely-packed idx range) and translated
directly into 10 scratch files, one per chunk, to keep per-call context
manageable; merged into one 3,000-line file and verified the merge landed
on exactly 3,000 lines with idx sequential 449,737–452,736 before running
validation.

Token/placeholder validation found **3 mismatches on the first pass**:
idx 451246 and 452491 each had translated text bleed into the interior of
a `<Name|id|#C|slot>`-style tag (a possessive skill-name label and an
"Inebriate-enhanced skills" label respectively) — both fixed by restoring
the tag to byte-identical English and moving the translation entirely
outside it, per the existing §6 Quick-Reference rule. idx 450595 had one
`#Y`/`#E` wrap pair dropped while restructuring a multi-clause sentence
around its four separate tags — fixed by rebuilding the sentence with all
four pairs preserved. Re-ran validation after fixes: 0 mismatches across
the full 3,000-row batch. Appended to `locale/update1.jsonl` and
re-validated the full file: 22,850 lines, idx unique, 0 duplicates, 0 token
mismatches. No new terminology decisions — every case matched an existing
[[Glossary]] entry. Next idx Update-1 = 452,737 (22,850/31,817 rows done,
~71.82%). Phase 6 untouched this session.

## 2026-09-24 — Phase 6, batch 27 (session 12)

User asked for **1,000 strings/session** for this session specifically,
overriding the 3,000 default set earlier the same day in session 11 — batch
size is decided per-session, not carried over automatically. Read idx
76,000–76,999 from `unique_strings.jsonl` in four sequential 250-line reads,
translated each chunk to its own scratch file, merged into one 1,000-line
file and confirmed the line count before validating.

Token/placeholder validation found **1 mismatch on the first pass**: idx
76248 dropped the `#Y`/`#E` color-tag pair that should wrap `[Frigid Fall]`
when the bracketed skill name was moved next to "Sun Sisi's" in the
Indonesian word order — fixed by restoring the `#Y...#E` wrap around the
moved phrase. Re-ran validation after the fix: 0 mismatches across the full
1,000-row batch. Appended to `locale/phase6.jsonl` and re-validated the full
file: 27,000 lines, idx unique, 0 duplicates, 0 token mismatches. No new
terminology decisions — every case matched an existing [[Glossary]] entry.
Next idx Phase 6 = 77,000 (27,000/50,000 rows done, 54.00%). Update-1
untouched this session.

## 2026-09-24 — Phase 6, batch 28 (session 13)

User again asked for **1,000 strings/session** for this session. Read idx
77,000–77,999 from `unique_strings.jsonl` in three sequential ~300-line
reads, translated the full batch in one pass into a single scratch file,
then validated before appending.

Token/placeholder validation found **2 mismatches on the first pass**: idx
77161 had the plain tag `<yawn>` (no `|id|#C|n>` stat format — §6 case 1)
mistakenly translated to `<menguap>` instead of being left untouched in
English; idx 77774 had the internal space in the stat tag
`<Direct Affinity Rate |780|#C|150>` dropped (became `<Direct Affinity
Rate|780|#C|150>`), which the tag-content regex treats as a different
token since it captures the tag's exact text. Both fixed in place —
restored `<yawn>` verbatim and restored the space before `|780|`. Re-ran
validation after the fixes: 0 mismatches across the full 1,000-row batch.
Appended to `locale/phase6.jsonl` and re-validated the full file: 28,000
lines, idx unique, 0 duplicates, 0 token mismatches. No new terminology
decisions — every case matched an existing [[Glossary]] entry. Next idx
Phase 6 = 78,000 (28,000/50,000 rows done, 56.00%). Update-1 untouched this
session.

## 2026-09-24 — Update-1, batch 21 (session 13)

User asked to continue Update-1 with a **3,000 strings/iteration** batch
size going forward. Read idx 452,737–455,736 from `unique_strings.jsonl` in
ten sequential 300-line reads, translating each chunk to its own scratch
file (batches 21–30), then merged all ten into one 3,000-line file before
validating.

Token/placeholder validation found **9 mismatches on the first pass**:
5 were `<...>`-wrapped inner-thought/dialogue lines (idx 453656, 453754,
453776, 453884, 455027 — e.g. `<I wonder when Dai will ever be free...>`)
that had been translated into Indonesian instead of kept byte-identical in
English, per Quick-Reference §6 rule 1 (a plain `<...>` tag without
`|id|#C|n>` format must stay untouched) — reverted to the source text
verbatim. The other 4 (idx 454095, 454164, 454252, 455165) were `#Y...#E`
color-tag wraps dropped or corrupted while restructuring sentences into
Indonesian word order: a missing wrap around "Enhancement"/"Enhancements"
in two nearly-identical Inebriate-buff tooltips, a missing wrap around
"Max-tuned" in a gear-filter tooltip, and a typo (`#Ditumpuk#E` instead of
`#Ytumpuk#E`) in a Tile-combine tutorial string — all fixed by restoring or
correcting the `#Y...#E` pairs. Re-ran validation after fixes: 0 mismatches
across the full 3,000-row batch. Appended to `locale/update1.jsonl` and
re-validated the full file: 25,850 lines, idx unique, 0 duplicates, 0 token
mismatches. No new terminology decisions — every case matched an existing
[[Glossary]] entry. Next idx Update-1 = 455,737 (25,850/31,817 rows done,
~81.25%). Phase 6 untouched this session.

## 2026-09-24 — Phase 6, batch 29 (session 14)

User asked to continue Phase 6 with a **2,000 strings/iteration** batch
size for this session, and to keep the Obsidian vault updated. Read idx
78,000–79,999 from `unique_strings.jsonl` in seven sequential ~300-line
reads, translating each chunk to its own scratch file, then merged all
seven into one 2,000-line file before validating.

Token/placeholder validation found **3 mismatches on the first pass**:
idx 78577 (a `#Y...#E` tag incorrectly split across "Boss" and "menggunakan
Energy" instead of wrapping only the phrase matching the source's single
tag), idx 78715 (a stray `# ` with a space in place of a proper `#Y...#E`
wrap around "combined"), and idx 78740 (the `#Ydeflection#E` tag was
dropped when the sentence was reordered into Indonesian word order). All
three fixed by restoring correct `#Y...#E` placement. Re-ran validation
after fixes: 0 mismatches across the full 2,000-row batch. Appended to
`locale/phase6.jsonl` and re-validated the full file: 30,000 lines, idx
unique, 0 duplicates, 0 token mismatches. No new terminology decisions —
every case matched an existing [[Glossary]] entry. Next idx Phase 6 =
80,000 (30,000/50,000 rows done, 60.00%). Update-1 untouched this session.

## 2026-09-24 — Phase 6, batch 30 (session 15)

User asked to continue Phase 6 with a **2,000 strings/iteration** batch
size again this session, and to keep the Obsidian vault updated. Read idx
80,000–81,999 from `unique_strings.jsonl` in seven sequential ~300-line
reads, translating each chunk to its own scratch file, then merged all
seven into one 2,000-line file before validating.

Token/placeholder validation found **0 mismatches on the first pass** —
the entire 2,000-row batch validated clean. Appended to
`locale/phase6.jsonl` and re-validated the full file: 32,000 lines, idx
unique, 0 duplicates, 0 token mismatches. No new terminology decisions —
every case matched an existing [[Quick-Reference]] entry. Next idx Phase 6
= 82,000 (32,000/50,000 rows done, 64.00%). Update-1 untouched this
session.

## 2026-09-24 — Update-1 completed (session 14)

User asked to finish Update-1 completely and update the Obsidian vault.
Continued from idx 455,737 in ~350-row batches (batches 35–38), reading
each chunk from `unique_strings.jsonl`, translating to a scratch file,
validating with the standard `TOKEN` regex script, and appending to
`locale/update1.jsonl` before moving to the next chunk. Covered idx
455,737–461,703 (5,967 rows) across this session:

- Batch 35 (idx 460,287–460,636, 350 rows, translated in the prior/
  interrupted part of this session): validated clean, 0 mismatches,
  appended.
- Batch 36 (idx 460,637–460,986, 350 rows): 1 mismatch on first pass — idx
  460,865 had a `#Y` color-tag prefix dropped while translating "can be
  #Ycombined#E" (written as `# Digabungkan#E` instead of
  `#YDigabungkan#E`). Fixed with a targeted string replace, re-validated at
  0 mismatches, appended.
- Batch 37 (idx 460,987–461,336, 350 rows): validated clean, 0 mismatches,
  appended. Contains several long lore blocks (Dragonbend Academy founding
  register/roster, Grand Artisan Gull's "raising Xiaoxiao" story, the
  "Ying Ning" wildfire-child origin myth).
- Batch 38 (idx 461,337–461,703, 367 rows — the final batch, sized to
  reach exactly idx 461,703): validated clean, 0 mismatches, appended.
  Contains the Ma Pingshan revenge-to-redemption story, the Fang Bai/
  Qiniang Golden-Peach war letter, and Tang Wenyao's father's
  marriage-pressure letter.

No new terminology decisions across any of these batches — every case
matched an existing [[Glossary]] entry.

After the final append, ran a full-file re-validation of
`locale/update1.jsonl`: **31,817 total rows, all idx unique, range
429,887–461,703 with zero gaps, 0 duplicate idx, 0 token/placeholder
mismatches, 0 empty translations.** Update-1 is now **100% complete** (was
25,850/31,817, ~81.25%, at the start of this session). Updated
[[Current-Status]] to mark Update-1 "done" and recorded this session's
work. Phase 6 was not touched this session.

## 2026-09-24 — Phase 17 started

At the user's request, started Phase 17 (idx 400,000–429,886) ahead of
Phase 6, which remains active/incomplete. Batch 1: idx 400,000–400,400 (401
rows — user asked for 3,000 rows, but the batch was cut short partway
through translation because a single-session 3,000-row batch could not be
translated and validated carefully within the session; the user chose to
bank the completed portion rather than push through the rest at lower
quality) translated and written to new file `locale/phase17.jsonl`. Mix of
content: NPC dialogue and lore blurbs around Kaifeng/Apricot
Village/Mirkvale (Elder Harrier's letter to the Master of Haven, Xiaoba's
Vale water-track repairs, Mohist artisan flavor text), several classical-
style poems, gear/skill tooltips (Etherwrath stacking, Petalwhirl
Immobilize mechanic), and many Pinyin NPC name entries (kept verbatim per
convention). No new terminology decisions — every case matched an existing
[[Quick-Reference]] entry.

Token/placeholder validation via the standard `TOKEN` regex script found
**1 mismatch on the first pass**: idx 400199 had a non-standard token
`# evaded sandstorms$hunt#` (space after the leading `#`, not a real tag)
that was translated as `#menghindari...` without preserving the leading
space — corrected to keep the source's exact odd spacing character-for-
character. Re-validated at **0 mismatches** before saving. Phase 6 was not
touched this session.

Later the same session, the user asked to continue to a round 1,000-row
milestone. Batch 2: idx 400,401–400,999 (599 rows) translated and appended
in-session (merged into `locale/phase17.jsonl` directly, no separate
sub-file kept). Mix of content: more Kaifeng/Mirkvale NPC dialogue and
lore (Wang Fen's lost-daughter story, the Crow Brew lore entry, Doudou the
palace cat's vignette), gear/skill tooltips (Thundercry Blade defensive
riposte mechanic), and many Pinyin NPC name entries. Token/placeholder
validation found **2 mismatches**: idx 400754 and idx 400861, both §6
rule violations where a `<...>` tag (one wrapping a full sentence, one
wrapping a short stage direction `<shakes head>`) had been translated
instead of kept verbatim — corrected to restore the original English tag
content. Re-validated at 0 mismatches. After merging batch 1 + batch 2,
`locale/phase17.jsonl` was re-checked as a whole: **1,000 total rows, idx
400,000–400,999, no gaps, no duplicate idx.** Phase 17 is now 1,000/29,887
rows done (~3.35%); remaining idx 401,000–429,886 to be continued in a
follow-up session. Phase 6 was not touched this session.
