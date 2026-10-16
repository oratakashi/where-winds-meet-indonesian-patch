# Update-2B Translation Log (idx 491,704–521,703)

Part of the game v2.3 additions documented in [[Update-2]]. Output: `locale/update2b.jsonl`. Standard [[Quick-Reference]] conventions apply. Requested cadence: 4,000 rows per iteration.

## Batches

### 2026-10-16 — Session 142: Update-2B fifth 4,000-row iteration completed (idx 507,704–511,703)

Translated and appended **4,000 sequential rows, idx 507,704–511,703**, to `locale/update2b.jsonl`. **Update-2B is 20,000/30,000 (66.67%)**; next idx **511,704**; 10,000 rows remain. The existing 16,000 rows were preserved byte-for-byte. No commit created.

Three user-authorized subagents translated separate ranges; the primary handled 334 additional rows and six long entries, reviewed and merged. The session resumed after a usage limit. Temporary files stayed outside the repository; transient sandbox temp loss was recovered from this session's own tool-call history or rewritten into stable system temp. **3,729 changed / 271 retained** under existing name, poetic-title, Skill/stat/set, developer placement and opaque-tag conventions. Dialogue gue/lo; UI neutral. No new glossary rule.

Long prose translated in full, including ritual texts (507856, 509202, 510039, 510108), lore and stories (508028, 508389, 508575, 509933, 510600, 511025), the Analects Yan Yuan (509002), the Tribute of Yu (509231), classical poetry (510489), the long classical passage at 510941, and King Hui of Liang (511657). Independent review checked the longest passages and poem line completeness. Review corrected square territory side lengths in 511657, an awkward poetry phrase in 510489, and the ordinary developer note at 509030; internal placement labels remain under existing conventions. Two introduced replacement characters were corrected before append.

The merged 4,000-row batch and the full 514,257-entry locale dictionary passed `qa_check.py --locale` with 0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings. Extra checks confirmed exact sequential idx, dollar/date/printf/literal-marker multisets, per-row newline counts and leading/trailing newline preservation, and no introduced replacement or Cyrillic characters. `python tools/progress.py --write` regenerated the tables.

Overall: 514,257 / 534,257 unique strings (96.26%), in-game coverage 98.22%.

See [[Update-2B]] and [[Update-2]].

### 2026-10-16 — Session 141: Fourth 4,000-row iteration completed (idx 503,704–507,703)

- Translated and appended **4,000 consecutive rows, idx 503,704–507,703**, to `locale/update2b.jsonl` in eight 500-row chunks (primary agent only, no subagents; scratch files outside the repo). **Update-2B is 16,000/30,000 (53.33%)**; next idx **507,704**; 14,000 rows remain. Earlier 12,000 rows untouched. No commit created. (The session was interrupted by a usage limit after the first 1,500 rows and resumed the next day.)
- Dialogue gue/lo, UI neutral; names, poetic titles, Skill/set/stat/Tier labels, developer placement labels and opaque tags retained. No new glossary rule. Dates inside colour tags rendered Indonesian (`16 Mar`, `10 Okt`); `s`→`d` for seconds/cooldowns; `<Limited` truncated titles keep the stray `<`.
- Whole-sentence angle tags (504314, 504551, 506797) and the tag-only part of 505195/507202 kept verbatim; the `<TEXT>`/`<IMAGE>` markup at 506859 kept with only "Debate Companion" translated.
- Long prose translated in full: Mencius Teng Wen Gong (503707), Liji Scholar's Conduct (503827), Yili Officers' Ceremony (504874), Betrothal and Wedding rites (505902, 506753), Zhou Li Earth Officials (506452), Zuo/Gongyang Duke Ai (506907), Yi Zhun hexagram (507240), Canon of Shun (507416), the Gao Sheng story (507418) and the Hundred Flowers poems (506161). Source defect: stray CJK in 507416 rendered in Indonesian.
- QA caught and fixed before append: whole-sentence angle tags translated at 504314/504551, `<lowers voice>` at 505195, and a stray `#E` at 505228. All chunks, the 4,000-row batch (sequential idx confirmed) and the full 510,257-entry dictionary passed with 0 PROMPT_LEAK / MARKUP / EMPTY / IDX.
- Overall: 510,257 / 534,257 unique strings (95.51%), in-game coverage 97.86%.

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
