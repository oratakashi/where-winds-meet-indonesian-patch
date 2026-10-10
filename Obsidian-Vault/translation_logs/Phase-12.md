# Phase 12 Translation Log (idx 250,000–279,999)

**Reconstructed retroactively on 2026-09-26** — this file was not written during
the session that did the work, so it only records what the commit history and
`locale/phase12.jsonl` show, not session-by-session reasoning.

Phase 12 began with a starter batch, committed 2026-09-25
(`147c8b7` — "Start Phase 12 translation (idx 250000-251999, 2000 rows)").
`locale/phase12.jsonl` now has **30,000 rows; the phase is complete** (2026-10-10, session 125). See [[Current-Status]] for overall progress. The starter batch recorded no terminology decisions; later sessions follow [[Quick-Reference]] and [[Glossary]].

## Batches

- [[idx-277000-279999]] — idx 277,000–279,999 (3,000 rows, 2026-10-10; Phase 12 complete at 30,000/30,000, 100.00%; no commit created)

- [[idx-273500-276999]] — idx 273,500–276,999 (3,500 rows, 2026-10-10; completes the 4,000-row iteration idx 273,000–276,999; Phase 12 at 27,000/30,000, 90.00%; no commit created)

- [[idx-273000-273499]] — idx 273,000–273,499 (500 rows, 2026-10-09; partial 4,000-row iteration; next idx 273,500; no commit created)

- [[idx-269000-272999]] — idx 269,000–272,999 (4,000 rows, 2026-10-09; Phase 12 at 23,000/30,000, 76.67%; no commit created)

- [[idx-265000-268999]] — idx 265,000–268,999 (4,000 rows, 2026-10-09; completes the requested 4,000-row iteration; Phase 12 at 19,000/30,000, 63.33%; commit pending)

- [[idx-261000-264999]] — idx 261,000–264,999 (4,000 rows, 2026-10-09; completes the requested 4,000-row iteration; Phase 12 at 15,000/30,000, 50.00%; commit pending)

- [[idx-258000-260999]] — idx 258,000–260,999 (3,000 rows, 2026-10-09; completes 4,000-row iteration idx 257,000–260,999; commit pending)

- [[idx-257000-257999]] — idx 257,000–257,999 (1,000 rows, 2026-10-09; commit pending; incomplete 4,000-row target)

- [[idx-256000-256999]] — idx 256,000–256,999 (1,000 rows, 2026-10-09; commit pending; partial 4,000-row iteration)

- [[idx-250000-251999]] — idx 250,000–251,999 (`147c8b7`, 2026-09-25)

- [[idx-252000-255999]] — idx 252,000–255,999 (4,000 rows, 2026-10-09; commit pending)

## 2026-10-09 continuation

Phase 12 continued with 4,000 rows (idx 252,000–255,999), bringing the phase from 2,000/30,000 (6.67%) to **6,000/30,000 (20.00%)**. Next idx: **256,000**.

The batch contains 2,971 manually translated rows and 1,029 rows retaining proper names, poetic titles, developer labels, stat labels and opaque tags under the standing conventions. Dialogue uses gue/lo; UI text uses neutral Indonesian. Content includes quest instructions, system notices, skill descriptions, mail and long character lore (Zhang Huaishen, Su Wan and Long Yuntian, Wen Wuque and Yingying, Niu Mang, Oghuz, Woody and Big Smart).

The 4,000-row batch, complete 6,000-row Phase 12 and whole 401,704-entry locale dictionary passed QA with 0 PROMPT_LEAK, MARKUP or EMPTY findings and no idx gaps or duplicates. Additional checks confirmed opaque placeholders and literal backslash-n sequences were preserved, and found no Cyrillic characters. One skipped dialogue line was corrected before append; placeholder order was preserved in idx 254123. The locale file retains LF line endings.

No new core terminology decisions. See [[Phase-12]] and [[idx-252000-255999]]. `python tools/progress.py --write` regenerated the progress tables.

Overall: 401,704 / 461,704 unique strings (87.00%), in-game coverage 94.51%.

### Terminology and context

- Existing proper names, faction/place names, named gear and skills, poetic titles and stat labels retain their established English forms. No new glossary entries were needed.
- Dialogue follows gue/lo, while system messages remain neutral. Oghuz’s confession and the Qixi letter retain their narrative context.
- `<IMAGE ...>` and other whole tags remain verbatim; `$T(...)`, `$N`, format tokens and literal `\n` remain intact. The three `{}` placeholders at idx 254123 retain their source order.

## 2026-10-09 — Session 105: Phase 12 partial iteration (idx 256,000–256,999, 1,000 rows)

The requested 4,000-row iteration (idx 256,000–259,999) is **not complete**. Only the first 1,000 rows (idx 256,000–256,999) have been finished, validated, and appended. The remaining 3,000 rows (idx 257,000–259,999) are still outstanding; the requested cadence remains 4,000 per iteration.

Phase 12 now contains 7,000/30,000 rows (23.33%). Next idx: **257,000**. The partial batch has 722 manually translated rows and 278 rows preserving names, poetic titles, developer labels, and stat labels under standing conventions. Content includes Content and Li Xiaogou's lore, the dialect lesson, Fu Luwa's letter, the village-school letter, and Feng's debate encounter. Dialogue follows gue/lo; UI remains neutral. No new terminology decisions.

The 1,000-row batch and complete 402,704-entry locale dictionary passed `qa_check.py --locale` with 0 PROMPT_LEAK, MARKUP, EMPTY, or IDX findings. Additional checks confirmed sequential idx, opaque placeholders, paragraph breaks, literal backslash-n sequences, and no Cyrillic characters. Image tags were retained byte-for-byte from the source. The appended batch uses LF line endings. `python tools/progress.py --write` regenerated the progress tables.

See [[Phase-12]] and [[idx-256000-256999]].

Overall: 402,704 / 461,704 unique strings (87.22%), in-game coverage 94.60%.

## 2026-10-09 — Session 112: Phase 12 continuation checkpoint (idx 257,000–257,999, 1,000 rows)

The user requested 4,000 rows per iteration. This session saved 1,000 rows (idx 257,000–257,999); the requested 4,000-row target remains incomplete, with 3,000 rows still required (idx 258,000–260,999). The earlier partial iteration (idx 256,000–259,999) now has 2,000 rows completed and 2,000 remaining. Phase 12 contains 8,000/30,000 rows (26.67%). Next idx: **258,000**. No commit was created.

768 rows differ from source; 232 retain names, short poetic titles, stat labels and other established verbatim cases. Content includes Xiao Yu and Wang Xun'er's long escape story, Halcyon's training incident, Shi Zhen's family shame, the Mohist City study rules, Lucky Seventeen's backstory, and Zong Qi's naming story. Long text is translated in full. Dialogue follows gue/lo; UI is neutral. No new terminology rules.

The batch and full 424,704-entry locale dictionary passed `qa_check.py --locale`: 0 PROMPT_LEAK, MARKUP, EMPTY and IDX findings. Additional validation confirmed contiguous idx, opaque placeholders, paragraph breaks, literal backslash-n sequences and no Cyrillic characters. Nonstandard `#D` and `$O` markers in idx 257429 were preserved. Locale output uses LF endings. `python tools/progress.py --write` regenerated the status tables.

Overall: 424,704 / 461,704 unique strings (91.99%), in-game coverage 96.61%.

See [[idx-257000-257999]] and [[Phase-12]].

## 2026-10-09 — Session 114: Phase 12 — 4,000-row target completed (idx 257,000–260,999)

This continuation translated and appended the remaining **3,000 rows, idx 258,000–260,999**. Together with the previously saved 1,000 rows (idx 257,000–257,999), the user's requested **4,000-row iteration is complete**. Phase 12 now contains **11,000/30,000 rows (36.67%)**. Next idx: **261,000**. No commit was created by this session.

The new 3,000-row batch contains 2,297 changed translations and 703 unchanged names, poetic titles, stat labels and opaque tag cases. The full iteration has 3,065 changed rows and 935 retained rows. All long prose was translated in full. Content includes Xiaobai, Maomao, Xixi, Han Tong and Shouliang, Li Zhenzhen, the Palace of Annals origin, Brant's Mohist research, letters, music-class notices, game-system instructions and combat effects. Dialogue follows gue/lo; UI stays neutral. No new terminology rule was introduced.

The user explicitly authorized multiple agents during this continuation. The primary agent translated idx 258,000–259,499; three agents independently translated disjoint 500-row ranges 259,500–259,999, 260,000–260,499 and 260,500–260,999 into scratch files outside the repository. The primary agent merged and checked the full batch before appending.

Batch and whole-dictionary `qa_check.py --locale` passed with 0 PROMPT_LEAK, MARKUP, EMPTY and IDX findings. Additional checks confirmed all 3,000 sequential idx, opaque dollar/date placeholders, paragraph counts, literal backslash-n counts and no Cyrillic characters. Long-text length checks found no suspicious shortening; sampled lore and LINK markup were reviewed. All locale rows use LF endings. `python tools/progress.py --write` regenerated the status tables.

Overall: 430,704 / 461,704 unique strings (93.29%), in-game coverage 97.15%.

See [[idx-258000-260999]], [[idx-257000-257999]] and [[Phase-12]].

## 2026-10-09 — Session 116: idx 261,000–264,999

This iteration translated and appended **4,000 rows, idx 261,000–264,999**, the requested 4,000-row cadence for Phase 12. Phase 12 now contains **15,000/30,000 rows (50.00%)** — the phase is at its halfway point. Next idx: **265,000**. No commit was created by this session.

The batch contains 3,003 changed translations and 997 retained names, poetic titles, developer/location labels, stat labels and opaque tag cases under standing conventions. Dialogue uses gue/lo; UI and system text remain neutral. Long prose was translated in full, including Yingying's palace-escape storybook, the Mohist City elders' paper critiques, the Sogdian caravan/princess poem story, Niu Mang's Forsaken Quarter story, the Twin Lions' backstory, Ye Wanshan's Pagoda of Karma lore, the Boneling cat tale, and the Wang Qing / Hutuo River cutscene script. No new terminology rule was introduced.

Method: no subagents were used (repo rule; the user did not ask for them this time). The primary agent read eight 500-row chunks, wrote only the changed translations per chunk to scratch files, and a merge script filled the retained rows from `unique_strings.jsonl`, so every idx 261,000–264,999 is present exactly once. Because off-by-one idx labelling is easy in long chunks, each chunk was checked with `qa_check.py --locale` plus a heuristic (length ratio, number set and `?`/`!` ending versus source); this caught and fixed label drift in chunks 1–3, 5 and 6 before appending. Rules re-applied: a `<...>` tag wrapping a whole sentence stays 100% identical (idx 262002, 262820, 264328, 264746 kept as source; idx 262592 keeps the tag content and translates only the outside text); a stray Chinese developer label (idx 264212, 264920) is kept verbatim; `#H`/`#Y` colour tokens are preserved exactly (a `#H`→`#Y` slip at idx 264903 was caught by QA and fixed); `s`→`d` and `d`→`h` duration shorthand applied (e.g. `5#Es`→`5#Ed`, `90d`→`90h`).

Batch and whole-dictionary `qa_check.py --locale` passed with 0 PROMPT_LEAK, MARKUP, EMPTY and IDX findings (436,704 entries checked). `python tools/progress.py --write` regenerated the progress tables.

Overall: 436,704 / 461,704 unique strings (94.59%), in-game coverage 97.71%.

See [[idx-261000-264999]] and [[Phase-12]].

## 2026-10-09 — Session 117: idx 265,000–268,999

This iteration translated and appended **4,000 rows, idx 265,000–268,999**, the requested 4,000-row cadence for Phase 12. Phase 12 now contains **19,000/30,000 rows (63.33%)**. Next idx: **269,000**. No commit was created by this session.

The batch contains 3,037 changed translations and 963 retained names, poetic titles, developer/location labels, internal area labels, stat labels and opaque cases under standing conventions. Dialogue uses gue/lo; UI and system text remain neutral. Long prose was translated in full, including the Mohist-city weaving diary, Zhao Dali's letter, the Gu Wenqiu five-colored-cake story, Qianye and Han Xiangxun's Blissful Retreat scene, Xue Chou's backstory, Jiang Wei's ghost-marriage story, Wei Zhixi's Wolf Mountain story, Yu Xiaoyao's love-study story, the Qinghe Northern Alliance inscription and the Wang Qing / Mohist diary. No new terminology rule was introduced.

Method: no subagents were used (repo rule; the user did not ask for them). The primary agent read the source as a compact `idx<TAB>text` view with newlines escaped, translated only the rows that needed translation into nine TSV scratch files (outside the repository), and a merge script filled the retained rows from `unique_strings.jsonl`, so every idx 265,000–268,999 is present exactly once. After every chunk a check script compared the `TOKEN` multiset and newline count against the source for each translated row, so label drift or dropped tokens were caught immediately (0 findings at the end). Rules re-applied: `<...>` tags, `<LINK ...>` blocks, `<TEXT ...>` and `<IMAGE ...>` tags kept character-for-character (only the text outside them translated); `@T[...]`, `$T(...)`, `{...}`, `%s`/`%d` preserved; literal backslash-n sequences kept (idx 268330); `s`→`d` and `d`→`h` duration shorthand applied (e.g. `1s`→`1d`, `3d`→`3h`, `30d`→`30h`, `{total_day:d} d {diff_hour:d} h`→`{total_day:d} h {diff_hour:d} j`); stray Chinese developer labels and `{区域N}` placeholders kept verbatim; "Old X" without "Man" kept English (Old Jin, Old Zhang, Old Yao); "Senior/Junior Brother" and "Brother X" names left as-is.

Batch and whole-dictionary `qa_check.py --locale` passed with 0 PROMPT_LEAK, MARKUP, EMPTY and IDX findings (440,704 entries checked). Additional checks confirmed all 4,000 sequential idx, matching literal backslash-n counts and no Cyrillic characters. `python tools/progress.py --write` regenerated the progress tables.

Overall: 440,704 / 461,704 unique strings (95.45%), in-game coverage 98.07%.

See [[idx-265000-268999]] and [[Phase-12]].

## 2026-10-09 — Session 121: idx 269,000–272,999

Completed the requested **4,000-row iteration**, bringing Phase 12 to **23,000/30,000 (76.67%)**. Next idx: **273,000**. No commit was created. The batch contains 3,034 changed translations and 966 retained names, poetic titles, stat labels, internal developer labels and opaque cases.

The primary agent worked directly without subagents. Temporary TSV files and validation scripts stayed outside the repository. Long text was translated in full, including the letter to Grandmaster Halcyon, the black-clad heroine's story, Ding Xiao's indebtedness, the Sheng Wen/Sheng Wu campfire story, the village-school letter and drawings, Horsehelm Village well-digging rules, Qiu Yuehai and Gu Le's falling-out, the Peace Bell Tower love-and-duty story, and Lan Ao/Fang Bai's comb story. Dialogue follows gue/lo; UI and narrative use neutral Indonesian. No new terminology rule was introduced.

The 4,000-row batch and full 450,704-row locale dictionary passed `qa_check.py --locale`: 0 PROMPT_LEAK, MARKUP, EMPTY and IDX findings. Additional checks confirmed contiguous idx, dollar/date/link placeholders, newline and literal backslash-n counts, no Cyrillic characters, and no unchanged or suspiciously shortened long prose. Two token issues were fixed before append: the opening #N in idx 271709, and the multiline `<1 ... >` token swallowed by TOKEN in developer text at idx 272183. Plain angle tags and IMAGE/LINK opening tags remain intact. Duration shorthand follows the existing s→d and d→h conventions. Retained-row review caught and translated two missed dialogue lines (269536 and 272575). Locale rows use LF endings.

`python tools/progress.py --write` regenerated the status tables. Overall includes the separate Phase 13 checkpoint saved during this session.

Overall: 450,704 / 461,704 unique strings (97.62%), in-game coverage 98.99%.

See [[idx-269000-272999]] and [[Phase-12]].

## 2026-10-09 — Session 123: Phase 12 partial checkpoint (idx 273,000–273,499)

The requested 4,000-row iteration (idx 273,000–276,999) is not complete. Saved 500 validated rows, idx 273,000–273,499; 3,500 rows remain in this iteration (idx 273,500–276,999). Phase 12 now contains 23,500/30,000 rows (78.33%). Next idx: 273,500. No commit was created.

Primary agent only. 378 rows translated; 122 retained names, poetic titles, stat labels, internal labels and opaque tags under standing conventions. Dialogue uses gue/lo; UI remains neutral. Long prose includes Ji Zha's promised sword, displaced people's Haven, Xiuyan Jade and the Prosperity Soup recipe. No new terminology rule. Scratch files remained outside the repository.

The scratch batch and full 455,204-entry locale dictionary passed QA with 0 PROMPT_LEAK, MARKUP, EMPTY and IDX findings. The second chunk was not saved because its write command exceeded the Windows process command-length limit; none of that chunk was appended. The 4,000-row cadence remains the user's target. `python tools/progress.py --write` regenerated the tables.

Overall: 455,204 / 461,704 unique strings (98.59%), in-game coverage **99.41%**.

See [[idx-273000-273499]] and [[Phase-12]].

## 2026-10-10 — Session 124: Phase 12 — 4,000-row iteration completed (idx 273,000–276,999)

Completed the outstanding **3,500 rows, idx 273,500–276,999**. Together with the prior 500-row checkpoint, the requested **4,000-row iteration, idx 273,000–276,999, is complete**. Phase 12 now contains **27,000/30,000 rows (90.00%)**. Next idx: **277,000**; the phase has 3,000 rows remaining. No commit was created.

The 3,500 new rows contain 2,739 changed translations and 761 retained names, short poetic titles, stat labels, internal labels and opaque tags. Review also corrected one missed dialogue in the prior checkpoint (idx 273453, “Feeling any better?”). Across the completed 4,000-row iteration, 3,116 rows differ from source and 884 remain verbatim under existing conventions. Dialogue follows gue/lo; UI uses neutral Indonesian. No new glossary rule was introduced.

The user explicitly authorized subagents. The primary agent translated 273,500–273,999; three subagents handled separate 1,000-row ranges 274,000–274,999, 275,000–275,999 and 276,000–276,999. Their saved files were recovered after the usage-limit interruption, reviewed and merged by the primary agent. Scratch files stayed outside the repository. Twelve rows with encoding-damaged punctuation were repaired, including the proper-name apostrophe in Cheng’en Town (274196).

Long prose was translated in full, including Tianyou's chess training, Shi Zhen and Murong Yuan, Brother Qiang's letter, the golden-bird painter, Zhang Huaishen's childhood, Old Snow, Jiang Wei's reunion, Swallow's flight experiments, the Mohist identification handbook, Arena rules and the complete Lucky Turtle reward probabilities. Plain angle tags remain unchanged, including idx 275391.

The merged **4,000-row iteration**, the **3,500-row remainder**, and the full **458,704-entry locale dictionary** passed `qa_check.py --locale`: 0 PROMPT_LEAK, MARKUP, EMPTY and IDX findings. Additional checks confirmed exact sequential idx, dollar/date placeholders, actual newlines, literal backslash-n and backslash-v markers, no introduced Cyrillic or replacement characters, and no suspiciously shortened or untranslated long prose. Numeric-review differences were harmless formatting (1,000 to 1.000) and spelling out a repeated count of three. `python tools/progress.py --write` refreshed the generated tables.

Overall: 458,704 / 461,704 unique strings (99.35%), in-game coverage 99.73%.

See [[idx-273500-276999]], [[idx-273000-273499]] and [[Phase-12]].

## 2026-10-10 — Session 125: Phase 12 completed (idx 277,000–279,999)

Completed all remaining **3,000 rows, idx 277,000–279,999**. Phase 12 now contains **30,000/30,000 rows (100.00%)**, covering idx 250,000–279,999 without gaps or duplicates. All original phases and Update-1 are complete. No commit was created.

The user explicitly authorized subagents. Three translators each handled a disjoint 1,000-row range in temporary files outside the repository; the primary agent reviewed and merged their output before appending. This batch contains 2,327 changed translations and 673 retained names, short poetic titles, stat labels, internal labels and opaque tags under standing conventions. Dialogue follows gue/lo; UI remains neutral. No new glossary rule was introduced.

Long prose was translated in full, including Hui at Skybrim Market, the village-school letters, Xi Huai's letter accompanying Yuelu, Qi Yitang and Gu Pan's wedding, Huiyao's backstory and Lie Yan's battle. Plain angle tags and opaque placeholders remain intact.

Merged-batch and whole-dictionary `qa_check.py --locale` passed with 0 PROMPT_LEAK, MARKUP, EMPTY and IDX findings. Additional review checked exact sequential idx, dollar/date placeholders, actual newlines, literal backslash-n and backslash-v markers, introduced replacement/Cyrillic characters, retained rows and long-text completeness. `python tools/progress.py --write` regenerated the progress tables.

Overall: 461,704 / 461,704 unique strings (100.00%), in-game coverage 100.00%.

See [[idx-277000-279999]] and [[Phase-12]].
