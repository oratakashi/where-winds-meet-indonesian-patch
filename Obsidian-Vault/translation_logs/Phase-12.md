# Phase 12 Translation Log (idx 250,000–279,999)

**Reconstructed retroactively on 2026-09-26** — this file was not written during
the session that did the work, so it only records what the commit history and
`locale/phase12.jsonl` show, not session-by-session reasoning.

Phase 12 began with a starter batch, committed 2026-09-25
(`147c8b7` — "Start Phase 12 translation (idx 250000-251999, 2000 rows)").
`locale/phase12.jsonl` now has 15,000 rows; the phase is **not complete** — see
[[Current-Status]] for the full idx range still outstanding. No terminology
decisions were recorded for this batch; check [[Quick-Reference]] and
[[Glossary]] for standing conventions before continuing it.

## Batches

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
