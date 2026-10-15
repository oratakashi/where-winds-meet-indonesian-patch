# Update-2A Translation Log (idx 461,704–491,703)

Part of the game v2.3 additions documented in [[Update-2]]. Output: `locale/update2a.jsonl`. Standard [[Quick-Reference]] conventions apply. Requested cadence: 4,000 rows per iteration. (This log was created in session 134; sessions 127 and 129 are recorded in [[Update-2]] and [[Current-Status]].)

## Batches

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
