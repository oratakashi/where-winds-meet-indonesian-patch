# Update-2A Translation Log (idx 461,704–491,703)

Part of the game v2.3 additions documented in [[Update-2]]. Output: `locale/update2a.jsonl`. Standard [[Quick-Reference]] conventions apply. Requested cadence: 4,000 rows per iteration. (This log was created in session 134; sessions 127 and 129 are recorded in [[Update-2]] and [[Current-Status]].)

## Batches

### 2026-10-16 — Session 143: Update-2A sixth 4,000-row iteration completed (idx 481,704–485,703)

Translated and appended **4,000 sequential rows, idx 481,704–485,703**, to `locale/update2a.jsonl`. **Update-2A is 24,000/30,000 (80.00%)**; next idx **485,704**; 6,000 rows remain. The existing 20,000 rows were preserved byte-for-byte (SHA-256 prefix verified). No commit created.

Three user-authorized subagents translated separate ranges; the primary translated 300 rows, reviewed and merged. The session resumed after a usage limit. All scratch files stayed outside the repository; transient sandbox temp loss was recovered from the session's own tool-call history and outputs were saved to stable shared system temp. **3,672 changed / 328 retained** under existing name, poetic-title, Skill/stat/set, developer placement and opaque-tag conventions. Dialogue gue/lo; UI neutral. No new glossary rule.

Long prose translated in full, including mourning garments (481747), Lin Xiaoyu (481857), Lu Zundu's record (482181), Taoist poems (482286), Gongsun Chou Part B (482521), Zhao Xiaogang (482616), Bright Hall positions (483109), Wang Zhi (483137), music and geography anthologies (483200, 483454), the medicinal treatise (483287), sacrifice ceremony (483969), Eight Immortals (484158), Bei Airs (484170), Huang Zhishui (484546), Zhou Qiang (484654), Chen Zixi (484683), Tan Gong I (485146), Monthly Ordinance VI (485592), and Li Yuzhi (485608). Independent review of nine long passages found no omitted paragraphs or stanzas. Review corrected sleeve versus arm (482286), the youth-age hierarchy in funeral rites (485146), and ordinary labels such as Waterlogged Notes and Stone Lock Toss. The literal backslash-n at 484901 was restored exactly before append.

The merged 4,000-row batch and the full 518,257-entry locale dictionary passed `qa_check.py --locale` with 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings. Extra checks confirmed exact sequential idx, dollar/date/printf/literal-marker multisets, per-row newline counts and leading/trailing newline preservation, and no introduced replacement or Cyrillic characters. `python tools/progress.py --write` regenerated the tables.

Overall: 518,257 / 534,257 unique strings (97.01%), in-game coverage 98.57%.

See [[Update-2A]] and [[Update-2]].

### 2026-10-16 — Session 140: Update-2A fifth 4,000-row iteration completed (idx 477,704–481,703)

Appended **4,000 rows** to `locale/update2a.jsonl` in twelve 250–350-row chunks (primary agent only, scratch files outside the repo, per-chunk `qa_check.py --locale` plus sequential-idx/newline/placeholder checks before append). Update-2A is **20,000/30,000 (66.67%)**; next idx **481,704**. No commit created.

- ~3,400 changed / ~600 retained (names, poetic titles, Skill/set/stat labels, dev/placeholder labels, whole-sentence angle tags). Dialogue gue/lo; UI neutral. No new glossary rule.
- Long prose translated in full: Kaogongji (477756, raw Chinese source rendered structurally), Shuo Gua (478067), Analects Book 3 (478466) and 17 (478860), Mencius Wan Zhang (479071), Miscellaneous Records (479763), Pitch Pot (478026), Hundred Flowers (479592) and Geography (480536) poem anthologies, Odes of Lu (479929), plus the cat/dog stories (480131, 480507, 479300, 481305, 481456, 481486).
- Source defects kept: unterminated `<` (481532), truncated trailing comma (478317), literal `/n` (479002).
- Conventions reused: Laureate/Champion kept English; `s`→`d`; decimal point → comma; `Unlocks … Emote` → `Membuka Emote …`.
- Overall: 508,757 / 534,257 unique strings (95.23%), in-game coverage 97.73% (includes parallel Update-2B progress).

### 2026-10-15 — Session 138: Update-2A fourth 4,000-row iteration completed (idx 473,704–477,703)

Translated and appended **4,000 sequential rows, idx 473,704–477,703**, to `locale/update2a.jsonl` in ten 400-row chunks by the primary agent only (no subagents, per the repo rule; the user did not ask for them). Each chunk was written to a scratch file outside the repository, checked with `qa_check.py --locale` and an extra script (exact sequential idx, newline count and leading/trailing-newline match per row, `$VAR$`/`@t[...]`/printf token multisets, no replacement or Cyrillic characters) and only then appended. **Update-2A is 16,000/30,000 (53.33%)**; next idx **477,704**; 14,000 rows remain. The existing 12,000 rows were preserved byte-for-byte. No commit created.

**3,280 changed / 720 retained** under standing conventions (character, place and sect names, short poetic titles, Skill/set/stat/Inner Way labels, Tier-gear labels, opaque tags, plain angle tags such as `<The cat will do as you say!>` and the whole-sentence tag at 475360). Dialogue gue/lo; UI/system text neutral. No new glossary rule. Established precedents were reused: Crafter / Homestead / Gift of Gab / Sound Jade / Harmonic Core stay English; "Wanderer" → Pengembara; "Master" (teacher) → Guru; seconds `s` → `d`, days `d` → `h`; decimals in prize-pool tables → comma (5,05%).

Long prose was translated in full, including the Rites of Zhou office list (474013), the Tang geography and music anthologies (474124, 476147), the "Inscription on a Humble Abode" (474023), Analects ch. 4 (474840) and Book 15 (476817), the Mencius Lilou II excerpt (477360), the Zuo Zhuan, Book of Changes and Book of Songs blurbs (475894, 474399, 475778), the Odes of Qin (476086) and Tang (477203), the woodblock-printing steps, the Xingwang secret report (474879), the six-part White Plume short story (476567) and the weapon/ink/brush/paper poem anthology (477524). Source defects: a stray Chinese title at 477628 was rendered as "Shenyi, Bagian ke-39"; the dangling trailing `
` at 477045 and the leading `
` at 477654 were kept; the literal `	` at 475078 and the `./w` marker at 475876 were preserved.

Process slips caught before release: idx 475992 first gained a trailing newline that the source lacks (removed in place after append; the check script now compares trailing newlines per row), and a typo "levelkartu-kartumu" at 477307 was fixed. The batch passed per-chunk QA and the full 501,407-entry locale dictionary with 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings. `python tools/progress.py --write` regenerated the tables.

Overall: 501,407 / 534,257 unique strings (93.85%), in-game coverage 97.07% (this total also includes a parallel Update-2B session's progress, 11,150/30,000 at the time of the run).

See [[Update-2A]] and [[Update-2]].

### 2026-10-15 — Session 136: Update-2A third 4,000-row iteration completed (idx 469,704–473,703)

Completed **4,000 sequential rows, idx 469,704–473,703**, appended to `locale/update2a.jsonl` after scratch QA. Update-2A is **12,000/30,000 (40.00%)**, next idx **473,704**; 18,000 rows remain. The original 8,000 rows were preserved byte-for-byte. No commit created.

Three user-authorized subagents translated disjoint 1,300/1,300/1,400-row ranges; the primary reviewed retained entries, corrected generic UI labels and free text outside TEXT tags, and merged the result. **3,281 changed / 719 retained** under existing name, poetic-title, material, Skill/stat and opaque-tag conventions. Dialogue gue/lo; UI neutral. No new glossary rule. All scratch files stayed outside the repository.

All 18 entries longer than 1,100 source characters were translated fully. Independent review covered the long classics and poetry: Great Oath (469716), Thousand Character Classic III (469901), Canon of Yao (470014), Duke Xi (470617), Analects Yong (470745) and Learning (471545), Gongyang history (470950), Rites of Zhou (471617), Astronomy poems (472039), Great Appendix (472277), Mohist primer (472292), ritual history (472372), seasonal poems (472690), and Lord Xi (473628). Review restored the source unit miles, Confucius and named places/stars in the Canon of Yao.

The merged batch and full locale dictionary passed `qa_check.py --locale`: 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings. Extra checks confirmed exact sequential idx, dollar/date/formatted-percent placeholders, actual and literal newline markers, no introduced replacement/Cyrillic characters, and no shortened long prose. `python tools/progress.py --write` regenerated the tables.

Overall: 490,257 / 534,257 unique strings (91.76%), in-game coverage 95.85%.

See [[Update-2A]] and [[Update-2]].

### 2026-10-15 — Session 134: second 4,000-row iteration completed, idx 465,704–469,703

- Appended **4,000 rows** to `locale/update2a.jsonl` in ten 400-row chunks, each validated with `qa_check.py --locale` in a scratch file outside the repository before append. Update-2A is **8,000/30,000 (26.67%)**; next idx **469,704**. No commit created. Primary agent only.
- **2,758 changed / 1,242 retained** (NPC and place names, short poetic titles, Skill/set/stat labels, internal developer labels, opaque tags, plain full-sentence angle tags). Dialogue gue/lo; UI neutral. No new glossary rule. "Trial: <Name>" titles and Inner Way/Skill names stay English per existing precedent.
- Long prose translated in full: Analects ch. 7 (466497) and ch. 2 (468726), Thousand Character Classic (466727), the name catalogue at 467293, Yili cap ceremony (467853), Great Learning (468236), Gongyang Lord Xi (468967), Odes of Zhou South (469235), bird poems (466831), the twenty driving rules (468599) and the AI image-generation notice (469219).
- Source defects: Chinese developer note at 468087 translated structurally (file path untouched); "automaticallywhen" at 469631 normalised.
- Skill tag fix: "and" inside `<Verdant Gust and Boundless Bloom|…>` (469522) must stay English (tag is one token); caught by MARKUP before append.
- Full-dictionary QA (485,704 entries): 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings. Exactly 4,000 sequential idx. `python tools/progress.py --write` refreshed the tables.
- Overall: 485,704 / 534,257 unique strings (90.91%), in-game coverage 95.14%.
