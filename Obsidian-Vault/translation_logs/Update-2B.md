# Update-2B Translation Log (idx 491,704–521,703)

Part of the game v2.3 additions documented in [[Update-2]]. Output: `locale/update2b.jsonl`. Standard [[Quick-Reference]] conventions apply. Requested cadence: 4,000 rows per iteration.

## Batches

### 2026-10-16 — Session 145: Update-2B seventh 4,000-row iteration completed (idx 515,704–519,703)

Translated and appended **4,000 sequential rows, idx 515,704–519,703**, to `locale/update2b.jsonl`. **Update-2B is 28,000/30,000 (93.33%)**; next idx **519,704**; 2,000 rows remain. Three user-authorized subagents translated disjoint ranges; the primary completed 334 rows and three long passages, reviewed, merged and validated. The earlier 24,000 rows were preserved byte-for-byte (SHA-256 prefix verified); new rows use the existing CRLF line endings. No commit created.

**3,709 changed / 291 retained** under the established names, short poetic titles, Skill/stat/set/Tier labels, developer placement labels and opaque-tag conventions. Dialogue gue/lo; UI neutral. No new glossary rule. Nearby precedents were checked and reused for Laureate Lin, Twin Laureates, Ten Sages and Sage Tang; Shengle Sect remains a faction name. Truncated source fragments remain truncated, including 519537 and the stray `<` at 519650.

Long prose translated fully, including mourning rituals (515805), the birds and beasts anthology (515967), Miscellaneous Records II (516047), Kun and its commentaries (516313), the classical allusion anthology (516370), Minor Odes (516375), Duke Yin first/second-year chronicle (516508), the six-part Li Zhaoxue story (516703), Tai Bo (516772), the ritual inheritance text (516785), the allusion anthology (517137), Hundred Flowers II (517656), the Silkworm Mother treatise (517989), Shan Fu's story (518325), the hexagram sequence (518832), the market instructions (519107), Po's letter (519212), Great Declaration II (519648), and the Bin poems (519689). Independent review found no omissions in 516703/516508; review preserved noble-rank distinctions in 516508. A poem typo at 515967 was corrected before merge.

All scratch files stayed outside the repository. The merged 4,000-row batch and full locale dictionary passed `qa_check.py --locale` with **0 PROMPT_LEAK / MARKUP / EMPTY / IDX findings**. Extra checks confirmed exact sequential idx, dollar/date/printf/literal-marker multisets, per-row newline counts and leading/trailing newlines, and no introduced replacement or Cyrillic characters. `python tools/progress.py --write` regenerated the tables. The overall snapshot also includes a parallel Update-2A session's work (27,300 rows at this run).

Overall: 529,557 / 534,257 unique strings (99.12%), in-game coverage 99.58%.

See [[Update-2B]] and [[Update-2]].

### 2026-10-16 — Session 144: Update-2B sixth 4,000-row iteration completed (idx 511,704–515,703)

Translated and appended **4,000 sequential rows, idx 511,704–515,703**, to `locale/update2b.jsonl` in eleven chunks of 340–400 rows by the primary agent only (no subagents, per the repo rule; the user did not ask for them). Each chunk was written to a scratch file outside the repository and checked with `qa_check.py --locale`; the merged batch was then checked with an extra script (exact sequential idx 511,704–515,703, per-row newline counts and leading/trailing newline match, `$VAR$`/`$T(n)`/`@t[...]`/printf/`{...}` multisets, no replacement or Cyrillic characters) before appending. **Update-2B is 24,000/30,000 (80.00%)**; next idx **515,704**; 6,000 rows remain. The existing 20,000 rows were preserved byte-for-byte (the file uses CRLF line endings; the new rows were appended with CRLF to match). No commit created.

**3,374 changed / 626 retained** under standing conventions (character, place and sect names, short poetic titles, Skill/set/stat/Tier labels, developer placement labels such as "Hangzhou Resident N" / "Placeholder Artisan N", opaque tags, whole-sentence angle tags at 515433 and 515525). Dialogue gue/lo; UI/system text neutral. No new glossary rule. Precedents reused: "Kini Menampilkan [X]"; "Membuka Emote …"; Wanderer → Pengembara; Senior/Junior Sister/Brother stay English; Crafter/Homestead/Gift of Gab stay English; seconds `s` → `d` (including `%02dm%02ds` → `%02dm%02dd`); English dates → Indonesian inside colour tags ("9 Feb", "10 Jun"); decimals → comma (`#Y15,00%#E`, `0,5 detik`); `<Limited` truncated titles keep the stray `<`.

Long prose translated in full, including Gongyang Duke Xuan (511736), Airs of Zheng (511778), Duke Ai Zuo/Gongyang (512076), the Xian and Dun hexagrams (512562, 512791), the Qian hexagram with its commentaries (512759), the Rockspire Street essay (512582), the Qi Xiaozhou and Wei Yidu story (513626), the Moth Immortal journal (513741), the Wellspring Town essay (513803), the Yueji (514080, Chinese source rendered in Indonesian), Interpretation of the Classics No. 26 (514359), Mencius King Hui of Liang Part II (514871), the Erudition Hall essay (514888), the Rite of Sacrifices (513056), the Odes of Wei (515473) and the Shan'er cat, Elder Hu pancake, Good the temple cat and reed-shadow short stories. Source defects kept: leading-comma and trailing-comma fragments (511891, 512251, 514602, 515596), unterminated `<` at 513171, 514109, 514807, 515306; the `<LINK ...>`/`<IMAGE ...>` markup at 514055 and 514433 kept intact with only text outside the tags translated (shop names inside the link left in English).

Process notes: one chunk-8 typo ("berretorika") and awkward wording at 514359 were fixed before append; idx 515576 had its two `#Y…#E` pairs merged and was restored to the source structure after the QA flagged a MARKUP mismatch. A parallel Update-2A session (143) finished its sixth iteration while this one ran, so the overall line below also includes Update-2A's 24,000 rows. `python tools/progress.py --write` regenerated the tables.

Overall: 522,257 / 534,257 unique strings (97.75%), in-game coverage 98.93%.

See [[Update-2B]] and [[Update-2]].

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
