# Update-2C Translation Log (idx 521,704–534,256)

Part of the game v2.3 additions documented in [[Update-2]]. Output: `locale/update2c.jsonl`. Standard [[Quick-Reference]] conventions apply. Requested cadence: 4,000 rows per iteration.

## Batches

### 2026-10-15 — Session 130: idx 521,704–525,703 (4,000 rows)

- Primary agent only, no subagents; ten 400-row chunks, each translated into a scratch file and QA-checked before merge. Retained rows were marked and expanded by a script, so idx/source stay identical. Scratch files stayed outside the repository. No commit created.
- Next idx: **525,704**; 8,553 rows remain in Update-2C.
- 3,535 rows differ from source; 465 retained (character names, short poetic item/emote/skin titles such as "Hard Shell", "Emerald Whip", Skill/set/chest labels, `$T(n)` variables and opaque tags).
- Decisions (no new glossary rule): dialogue gue/lo, UI neutral; "Wanderer" → Pengembara; "Master/Elder" honorifics per [[Quick-Reference]] §3; Inkbound Order, Tianyi Tower, Archives and other place names untouched; "Gift of Gab"/"Gab Card", "Trial", Tier/Lv. and stat labels kept English; "Guandan"/Fun Competition rules translated around untranslated mode names.
- Source defects: stray CJK characters (聘 rite, 昌本, 丘甲, 郎) rendered as "Pin", "Changben", "Qiu Jia", "Wakil Departemen Air"; truncated `<...` fragments (525462) kept the literal `<` and translated the free text; an unterminated sentence (524916, 523223) stays unterminated; the `$linkJoin Now^10243^$` block (525078) is preserved verbatim.
- Long classical text translated in full: Gongyang (522066, 523460, 524370, 525197, 525317), Liji (522898, 523159, 524166, 524452, 523811), Mencius Lilou (524301), Analects (522185, 524829), tribute lists (523826), Yuzhen's story (523663).
- QA: every chunk, the merged 4,000 rows and the full 470,204-entry dictionary passed `qa_check.py --locale` with 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings; exact sequential idx and per-row newline counts verified. `python tools/progress.py --write` regenerated the tables.
- Overall: 470,204 / 534,257 unique strings (88.01%), in-game coverage 93.46%.
