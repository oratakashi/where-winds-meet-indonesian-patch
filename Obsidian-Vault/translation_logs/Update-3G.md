# Update-3G Translation Log (idx 715,919–733,661)

Strings new in **game v2.4 [Across Autumn Waters]** (2026-11-13), documented in [[Update-3]]. Output: `locale/update3g.jsonl` (created by the first translation session). 17,743 strings. Standard [[Quick-Reference]] conventions apply.

## Batches

- 2026-11-16 (session 155): idx 715,919–733,661 (17,743 rows) translated by 18 subagents (17 x 1,000 + 743), merged in order into `locale/update3g.jsonl`, QA clean (0 PROMPT_LEAK / MARKUP / EMPTY / IDX). **Update-3G complete: 17,743/17,743.** Overall after session: 673,662 / 733,662 unique strings (91.82%), in-game coverage 94.63% (base) / 100.00% (`_diff`).

## Decisions & notes
- Conventions as Update-3A/3D: whole-sentence `<...>` tags verbatim (incl. the `@[CON_EXCEEDS_STR<=0]...<Power|780|#C|8>` stat rows, where the TOKEN regex swallows that span, so it stays English); `Countermeasure` -> `Cara menghadapi:`; seconds `s` -> `d`, `Nd` -> `Nh`; `Wanderer` -> `Pengembara`; `Chen Hour` -> `Jam Chen`.
- **Jianghu Tactics (auto-chess) unified (2026-11-16, user request):** generic `Piece(s)` -> `Bidak` (lowercase -> `bidak`), `Chess Piece(s)` -> `Bidak Catur` (also fixed in 3C idx 624143 and 3D idx 633830), round -> `Ronde`, dice roll -> `lemparan`. Left in English: gear/set item names (`Swift Gale Piece`, `Swaying Heights Piece`, `Single Piece`, `N-piece` synergy counts, `gear piece`, `set piece`). Other minigame terms stay English: Synergy, Emblem, Ambush, Chess Coin, Heron Coin, Roll Points, Little Tactician, Painted Boat, Bench, Board.
