# Phase 12 Translation Log (idx 250,000–279,999)

**Reconstructed retroactively on 2026-09-26** — this file was not written during
the session that did the work, so it only records what the commit history and
`locale/phase12.jsonl` show, not session-by-session reasoning.

Phase 12 began with a starter batch, committed 2026-09-25
(`147c8b7` — "Start Phase 12 translation (idx 250000-251999, 2000 rows)").
`locale/phase12.jsonl` now has 7,000 rows; the phase is **not complete** — see
[[Current-Status]] for the full idx range still outstanding. No terminology
decisions were recorded for this batch; check [[Quick-Reference]] and
[[Glossary]] for standing conventions before continuing it.

## Batches

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
