# Update-2B Translation Log (idx 491,704–521,703)

Part of the game v2.3 additions documented in [[Update-2]]. Output: `locale/update2b.jsonl`. Standard [[Quick-Reference]] conventions apply. Requested cadence: 4,000 rows per iteration.

## Batches

### 2026-10-15 — Session 139: Third 4,000-row iteration completed (idx 499,704–503,703)

- Translated and appended **4,000 consecutive rows, idx 499,704–503,703**, to `locale/update2b.jsonl` in twelve scratch-file chunks (250–350 rows) by the primary agent only (no subagents). **Update-2B is 12,000/30,000 (40.00%)**; next idx **503,704**; 18,000 rows remain. Earlier 8,000 rows untouched. No commit created.
- **3,768 changed / 232 retained** (names, poetic titles, Skill/set/stat/Tier labels, developer placement labels, opaque tags, whole-sentence angle tag at 502662). Dialogue gue/lo, UI neutral. No new glossary rule; reused "Kini Menampilkan [X]", "Emote Gleam/Splendor/Silhouette: X", "Membuka …", "Skill Mistik", `s`→`d` and `d`→`h` shorthand, Indonesian dates inside colour tags.
- Long prose translated in full: Analects XIV (499783) and Ji Shi (503575), Wuyue investiture chronicle (499853), Gongyang/Zuo Duke Min (501257) and Duke Ding (502122, 502465), Zeng Master ritual questions (501630), Mencius Gaozi (500364) and Jinxin (500647), Zhou Li Xia Guan Sima (502445, Chinese source rendered in Indonesian), Tang knight-errant/craft poems (502570), scholar-meeting ceremony (503674), Inkbound Order history register (503547), Seaward Gate essay (503335), plus the Meng Kuan, Ma Er, duck and Gold Fur short stories.
- Source defects: stray CJK in 502465 and 501257 rendered/dropped; truncated fragments stay unterminated.
- QA caught two slips before append: chunk 10 had every idx from 502973 labelled +1 (shifted back), and chunk 9 had the whole-sentence angle tag at 502662 translated (restored to source). All chunks and the full 502,257-entry dictionary then passed with 0 PROMPT_LEAK / MARKUP / EMPTY / IDX; 4,000 sequential idx confirmed. `python tools/progress.py --write` refreshed status tables.
- Overall: 502,257 / 534,257 unique strings (94.01%), in-game coverage 97.15%.

### 2026-10-15 — Session 137: Update-2B second 4,000-row iteration completed (idx 495,704–499,703)

Translated and appended **4,000 consecutive rows, idx 495,704–499,703**, to `locale/update2b.jsonl`. **Update-2B is 8,000/30,000 (26.67%)**; next idx **499,704**. No commit created.

Three user-authorized subagents translated separate ranges; the primary translated the long Duke Yin excerpt at 499547, reviewed, merged and validated. Scratch files remained outside the repository. **3,741 changed / 259 retained** under existing names, poetic titles, Skill/stat and opaque-tag rules. Dialogue gue/lo, UI neutral; no new glossary rule.

Long ritual, classical, poetry and lore prose translated in full, including Sang Da Ji (496288), Zhejiang history (496298), palace poetry (496970), village archery ceremony (497078), Azhi confession (497707), Yuzhen letter (498081), sea goddess notes (498320), classical prose (498374, 498409, 498600, 498682), and Duke Yin year one (499547). Cross-review corrected minor word choices; a replacement character at 495713 was restored to its source em dash.

Individual and merged batch QA, then the complete locale dictionary, passed with 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings. Extra checks confirmed exactly 4,000 sequential idx, dollar/date/printf placeholders, per-row newline counts and no introduced replacement characters. `python tools/progress.py --write` refreshed generated tables.

Overall: 494,257 / 534,257 unique strings (92.51%), in-game coverage 96.21%.

See [[Update-2B]] and [[Update-2]].

### 2026-10-15 — Session 131: 4,000-row iteration completed, idx 491,704–495,703

- Completed the remaining **3,500 rows, idx 492,204–495,703**, and appended one validated batch to `locale/update2b.jsonl`. With session 128's checkpoint, all **4,000 rows** are complete. Update-2B is **4,000/30,000 (13.33%)**; next idx **495,704**. No commit created.
- User authorized subagents: three handled 492,204–493,203, 493,204–494,203 and 494,204–495,203; primary handled 495,204–495,703, reviewed and merged. Temporary artifacts stayed outside the repository.
- New batch: **3,269 changed / 231 retained**. Full iteration: **3,722 changed / 278 retained**, following existing names, poetic titles, Skill/stat and opaque-tag conventions. Dialogue gue/lo, UI neutral. No new glossary rule. Terminal Pivot Tower remains English per idx 36264.
- All long prose translated in full, including the Duke Yin years 3–5 excerpt (493967), Qian Duoduo's story (495301), Qi Yitang and Gu Pan (495441), Mourning Rites (495488), and Teng Wen Gong, Chapter II (495637). Independent review of ten long entries found no omissions or actionable semantic defects. Source ellipses remain where excerpts end.
- Plain full-sentence angle tags at 492372 and 493715 remain identical. Extra dollar/date/link placeholders and formatted dollar variables preserved, along with actual/literal newline markers. Numeric differences at 494859 and 495142 only pad clock hours to 05:00/04:59.
- Individual and merged-batch QA passed before append; the full 473,704-entry locale dictionary passed with 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings. Full iteration has exactly 4,000 sequential idx; no introduced replacement/Cyrillic characters or suspiciously shortened long prose. `python tools/progress.py --write` refreshed status tables.
- Overall: 473,704 / 534,257 unique strings (88.67%), in-game coverage 93.77%.

### 2026-10-15 — Session 128: partial checkpoint, idx 491,704–492,203

- Saved 500 consecutive rows; no commit created. Primary agent only.
- Requested iteration: idx 491,704–495,703. **Incomplete: 3,500 rows remain, idx 492,204–495,703.** Resume this range before starting a new iteration.
- 453 translations differ from source; 47 names, poetic titles, stat/gear labels and opaque tags retained. Dialogue uses gue/lo; UI remains neutral. No new glossary rule.
- Long texts translated in full: the supplied Gongsun Chou, Part I excerpt (491708), the Silver Needle summer-illness treatise (491796), Mount Tai dialogue (491800), privy-wall account (491896), Boat of Paper history (492121), and Cheng Huaijin biography (492133). The source excerpt ends with an ellipsis; that ending is preserved without inventing a continuation.
- Plain angle tags stay verbatim, including 491814; `$T(...)`, other dollar/date placeholders and actual newlines preserved. Scratch outputs remained in system temporary directories outside the repository.
- Scratch and full-dictionary QA passed: 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings. Additional checks verified exact sequential idx, extra placeholders, newline counts and no replacement characters. `python tools/progress.py --write` refreshed the generated tables.
- Overall: 463,204 / 534,257 unique strings (86.70%), in-game coverage 92.02%.
