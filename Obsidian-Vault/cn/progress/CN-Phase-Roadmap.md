# CN Phase Roadmap

Plan for the **WWM China** translation ([[CN-Current-Status]]). Ranges are fixed in
`tools/profiles.py` (`CN_PHASES`); live numbers are in the generated block of
[[CN-Current-Status]].

## Where the corpus comes from

`translate_words_map_zh_cn` (client of 2026-10-20) has 2,081,685 entries / 627,412 distinct
texts. Its keyHashes are per-entry IDs shared with the global game, so:

| Part | Entries | Handled by |
| --- | ---: | --- |
| Key also in global (`strings.jsonl` + `strings_diff.jsonl`) | 1,062,051 | bridge per key → global `locale/` (already 100 %) |
| CN-only key, but the same Mandarin text occurs on a shared key | 196,495 | bridge per text (most frequent global translation of that text) |
| CN-only key and text never seen in global | 823,139 | this project, `cn/locale/` |

That last part holds **159,176 distinct texts**, ordered by how often they occur (`freq`), so
early phases cover the most entries. `translate_words_map_zh_cn__small` (21,932 entries) adds
no texts — every one also occurs in `zh_cn` — but its keys are mostly not in global any more
(global dropped `__small` in v2.3), so it relies on the per-text bridge and on `cn/locale/`.

**Expect noise in the CN-only part.** The CN build still ships internal editor data that
NetEase strips from global builds: developer names used as owner fields (`陈嘉铮`, `何纪希`,
`周雨霖` — thousands of entries each), table labels (`奖励表-陈佳伟`, `entity表-刘铭册`,
`总表`), placeholders (`这是关卡说明…`, `61阶待定`) and NPC spawn labels
(`九流门草鞋驻地-厢房-九流门弟子`). They are mixed with real CN-only content (newer
updates). See [[CN-Quick-Reference]] §4 for how to handle them quickly.

## Phases (30,000 strings each)

| Phase | idx range | File |
| --- | --- | --- |
| 1 | 0–29,999 | `cn/locale/phase1.jsonl` |
| 2 | 30,000–59,999 | `cn/locale/phase2.jsonl` |
| 3 | 60,000–89,999 | `cn/locale/phase3.jsonl` |
| 4 | 90,000–119,999 | `cn/locale/phase4.jsonl` |
| 5 | 120,000–149,999 | `cn/locale/phase5.jsonl` |
| 6 | 150,000–159,175 | `cn/locale/phase6.jsonl` |

After a CN game update: re-dump, run `python tools/rebuild_unique_strings.py --profile cn`
(appends new CN-only strings, existing idx untouched) and add an `Update-N` block to
`CN_PHASES` in `tools/profiles.py`, same as the global [[Phase-Roadmap]].
