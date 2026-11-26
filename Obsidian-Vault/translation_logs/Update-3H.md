# Update-3H Translation Log (idx 733,662–736,933)

Strings new in the **additional game v2.4 update** (2026-11-26), documented in [[Update-3]]. Output: `locale/update3h.jsonl` (created by the first translation session). 3,272 strings. Standard [[Quick-Reference]] conventions apply; Update-3 conventions (see [[Update-3G]], e.g. Jianghu Tactics `Bidak`/`Ronde`) carry over.

All 3,272 strings come from `translate_words_map_en_diff` — the update left the base `translate_words_map_en` byte-identical — so their in-game effect shows up in the `_diff` coverage row of [[Current-Status]]. Remember the Steam caveat in `CLAUDE.md`: the `_diff` overlay only sticks on the official NetEase launcher (and Android).

## Batches

- 2026-11-26 (session 159): idx 733,662–736,933 (3,272 rows). 63 rows filled automatically from already-translated strings that differ only in numbers; the other 3,209 translated by 6 parallel subagents (user-authorized), merged in idx order into `locale/update3h.jsonl`, QA clean on `locale/*.jsonl` (0 PROMPT_LEAK / MARKUP / EMPTY / IDX). **Update-3H complete: 3,272/3,272.** Overall: 736,934 / 736,934 unique strings (100.00%), in-game coverage 100.00% (base) / 100.00% (`_diff`).

## Decisions & notes

- Cross-chunk harmonisation after the merge, following the majority precedent: `Tier N … Set/Piece/Bolt/Wings/Chest` item labels (with or without `Common:`/`Supreme:`) stay English; `mountain demon` -> `iblis gunung`; `Junior`/`Senior Brother/Sister` -> `Adik`/`Kakak Seperguruan` (still listed in [[Open-Questions]]; older blocks are mixed and were not changed); `envoy` -> `utusan`; `Today's Deals` -> `Penawaran Hari Ini`; `Union Knot` kept.
- Jianghu Tactics `Piece` -> `Bidak`, `tile` -> `petak`; `Rhubarb` and `Reaper of Tears` kept (new source wording for the former "Weeping Reaper").
- `Attribute Penetration` (new source wording without "Attack") kept as written.
- Formal letters, verse and item flavour text use aku/kau/hamba instead of gue/lo.
- Known leftover outside this block: a few older Update-3G labels read `… Bidak Swift Gale Tier N` although they are gear item names; not changed here.
