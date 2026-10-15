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

### 2026-10-15 — Session 133: idx 529,704–533,703 (4,000 rows)

- Primary agent only, no subagents; ten 400-row chunks, each translated into a scratch file (outside the repo; only translated rows were written, a script filled retained rows from source), QA-checked and appended to `locale/update2c.jsonl` one by one. No commit created.
- Next idx: **533,704**; 553 rows remain in Update-2C (the final short batch, 533,704–534,256).
- 3,739 rows differ from source; 261 retained (character names, short poetic item/emote/skin titles, Skill/set/chest labels, `$T(n)` variables, internal debug labels such as "Qingzhou Oddity - Hedgehog Bell - Gym 4", opaque tags).
- Decisions (no new glossary rule): dialogue gue/lo, UI neutral; "Wanderer" → Pengembara; honorifics per [[Quick-Reference]] §3; Inkbound Order, Tianyi Tower, Archives, Well of Heaven, Spring's Return Hall and other place names untouched; Gift of Gab, Trial, Tier/Lv., Path, Homestead, Crafter, Attunement/Tuning/Retuning, Inspiration Point and stat labels kept English; "Cooldown: 60s" style durations rewritten `s`→`d` per placeholder rules; decimal commas/thousand dots used in numeric prose (11,25% / 13.800) while token content (`{0:.2%}`, `$STEADY_...$`, `@T[...]`) stays untouched.
- Source defects: stray CJK characters inside the Tang poem anthologies (529883: 偏僻, 胜过, 未, 空自, 陆, 冠, 稀疏, 碧, 清) rendered in Indonesian; truncated sentences (532112, 530912) stay unterminated; a plain `<Wipes tears>` tag (531096) kept verbatim (QA caught an initial translation of it); `/n` literal in 532490 preserved.
- Long text translated in full: Book of Rites excerpts (530032, 530171, 532276), Gongyang Duke Wen (530087), Analects Xiang Dang (530316), Odes of Wei and Chen (530385, 530443), Mencius King Hui of Liang (531130), the Yao Said chapter (532178), the Tang mounted-archery manual (530830), Three Character-style primer (532385), Shan Fu's porter story (530397), Zhu Yinghan's story (530672), Jiang Canghai's story (531919), Zhu Chougong's story (533078), and the Inkbound Order history chronicle (533456).
- Process note: the chunk-4 append ran before the QA exit status was checked (a `| tail` pipe hid the MARKUP finding at 531096); the row was corrected in place in `locale/update2c.jsonl` and the full file re-validated. Later chunks gate the append on the QA counts being zero.
- QA: every chunk, the 12,000-row `update2c.jsonl` (exactly sequential 521,704–533,703) and the full 484,504-entry dictionary passed `qa_check.py --locale` with 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings; extra scripted checks covered per-row newline counts and `$`/`@T`/`$T()` token multisets. `python tools/progress.py --write` regenerated the tables.
- Overall: 484,504 / 534,257 unique strings (90.69%), in-game coverage 94.93%. (This total also includes Update-2A rows, 6,800/30,000, saved by a separate session during this one.)

### 2026-10-15 — Session 135: idx 533,704–534,256 (553 rows) — Update-2C complete

- Primary agent only, no subagents; two scratch files (280 + 273 rows, outside the repo) converted to JSONL by a script, QA-checked once merged and appended to `locale/update2c.jsonl`. No commit created.
- **Update-2C is finished: 12,553/12,553 rows (100.00%)**, strictly consecutive 521,704–534,256. Nothing remains in this block.
- 462 rows differ from source; 91 retained (Inner Way/Note/set/stat labels, names, `Face:`/`Eastwind Soarer`-style item labels, Skill names, opaque tags).
- Decisions (no new glossary rule): dialogue gue/lo, UI neutral; Wanderer → Pengembara; Senior/Junior Brother/Sister stay English, Uncle → Paman, Master → Guru; Inkbound Order, Spring's Return Hall, Tianyi Tower, Archives, Ten Sages, Twin Mavens, Laureate, Clarion Chime, Goosebound Order and other proper names untouched; Gear Tuning/Retune/Re-attuning/Attunement/Modulating terms kept English per earlier precedent (`Menyetel ulang gear` for plain "retunes"); durations `{}s`→`{}d`, `minutes`→`menit`; Tang poems prefixed `(Dinasti Tang)`; percent decimals localized (0,8%) while token content is untouched.
- Long text translated in full: Wellspring Town history (533762), Inkbound Order Archives/Printing Workshop/Ten Sages/Study Halls lore, the Ai Kong "Full of Ai" parody (533854), Yingying's Zongzi serial (533903), the dormitory-life essay (533942), the Confucius statue essay (534125), Water/Sleepyhead cat stories (534043, 534232), the Tempest tournament rules (534123), and the Tang poems of Li Bai, Cen Shen, Jia Dao, Zhang Ji and an anonymous verse.
- QA: the 553-row batch and the full 486,257-entry dictionary passed `qa_check.py --locale` with 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings; extra scripted checks confirmed exact sequential idx 533,704–534,256, per-row newline counts, `$`/`@t[]`/`{}`/`%` token multisets and no CJK/replacement characters. `python tools/progress.py --write` regenerated the tables.
- Overall: 486,257 / 534,257 unique strings (91.02%), in-game coverage 95.14%.
