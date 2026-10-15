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

### 2026-10-15 — Session 132: idx 525,704–529,703 (4,000 rows)

- Primary agent only, no subagents; ten 400-row chunks, each translated into a scratch file (outside the repo), QA-checked and appended to `locale/update2c.jsonl` one by one. No commit created.
- Next idx: **529,704**; 4,553 rows remain in Update-2C (the next 4,000-row iteration ends at 533,703, leaving 553).
- 3,180 rows differ from source; 820 retained (character names, short poetic item/emote/skin titles such as "Hard Shell"-style labels, Skill/set/chest labels, `$T(n)` variables, plain angle tags `<whispers>` / `<Stares blankly>`, opaque test/placeholder labels).
- Decisions (no new glossary rule): dialogue gue/lo, UI neutral; "Wanderer" → Pengembara; "Master/Elder" honorifics per [[Quick-Reference]] §3; Inkbound Order, Tianyi Tower, Archives, Another Tavern, Seafarer's Alley and other place names untouched; "Gift of Gab", "Trial", Tier/Lv., Path, Homestead, Crafter and stat labels kept English; Chinese commentary/classic titles (Analects, Mencius, Gongyang, Doctrine of the Mean, Odes of Zhou) kept as English book names while the body text is translated; official titles in the Zhou Li list (526988) translated except the office-name headings, which stay English.
- Source defects: a stress-test string of repeated CJK "分" characters (529690) keeps its CJK tail verbatim (only "Congratulations" → "Selamat"); the stray `PLACEHOLDER`-style rows (e.g. "Upper Canon · ䷀", "710 New 4") stay verbatim; `<LINK …>` (527074), `$Link^50224^…^$` (529231) and `$D$G…$E` (528327) blocks preserved character-for-character; `\n` literal (527167) preserved.
- Long text translated in full: Gongyang (525887, 526149, 527492), Zhou Li Minister of Justice (526988), Mencius (527231, 529259), Analects Zilu (528290), Doctrine of the Mean (529651), Odes of Zhou (527942, 528696, with the `<br>` line structure kept), Seedling Temple record (528415), Tang mounted-archery manual (529598), Li Si and Deng Deng (526641), Yang Fan (527253), Gu Chuanyang (527020), Penglai/Inkbound Order overview (527055), Fellowship Hall plaza (526998).
- QA: every chunk and the full 477,704-entry dictionary passed `qa_check.py --locale` with 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings; extra checks confirmed exact sequential idx, per-row actual-newline counts and `$`/`@t[]`/`<br>`/`{}` token multisets. `python tools/progress.py --write` regenerated the tables.
- Overall: 477,704 / 534,257 unique strings (89.41%), in-game coverage 94.13%.
