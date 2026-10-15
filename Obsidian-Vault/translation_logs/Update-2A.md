# Update-2A Translation Log (idx 461,704–491,703)

Part of the game v2.3 additions documented in [[Update-2]]. Output: `locale/update2a.jsonl`. Standard [[Quick-Reference]] conventions apply. Requested cadence: 4,000 rows per iteration. (This log was created in session 134; sessions 127 and 129 are recorded in [[Update-2]] and [[Current-Status]].)

## Batches

### 2026-10-15 — Session 134: second 4,000-row iteration completed, idx 465,704–469,703

- Appended **4,000 rows** to `locale/update2a.jsonl` in ten 400-row chunks, each validated with `qa_check.py --locale` in a scratch file outside the repository before append. Update-2A is **8,000/30,000 (26.67%)**; next idx **469,704**. No commit created. Primary agent only.
- **2,758 changed / 1,242 retained** (NPC and place names, short poetic titles, Skill/set/stat labels, internal developer labels, opaque tags, plain full-sentence angle tags). Dialogue gue/lo; UI neutral. No new glossary rule. "Trial: <Name>" titles and Inner Way/Skill names stay English per existing precedent.
- Long prose translated in full: Analects ch. 7 (466497) and ch. 2 (468726), Thousand Character Classic (466727), the name catalogue at 467293, Yili cap ceremony (467853), Great Learning (468236), Gongyang Lord Xi (468967), Odes of Zhou South (469235), bird poems (466831), the twenty driving rules (468599) and the AI image-generation notice (469219).
- Source defects: Chinese developer note at 468087 translated structurally (file path untouched); "automaticallywhen" at 469631 normalised.
- Skill tag fix: "and" inside `<Verdant Gust and Boundless Bloom|…>` (469522) must stay English (tag is one token); caught by MARKUP before append.
- Full-dictionary QA (485,704 entries): 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings. Exactly 4,000 sequential idx. `python tools/progress.py --write` refreshed the tables.
- Overall: 485,704 / 534,257 unique strings (90.91%), in-game coverage 95.14%.
