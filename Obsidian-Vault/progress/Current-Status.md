# Current Status

**Last updated: 2026-09-24 (session 12).** This is the single source of truth for "how
far are we" — it gets overwritten each session, not appended to. For the
full timeline, see [[Session-History]]; for the phase plan, see
[[Phase-Roadmap]]; for the resume checklist, see [[Resume-Procedure]].

## Totals

- Total lines in `strings.jsonl`: **963,050** (pre-update version; the
  2026-09 update's main file has 826,388 — see [[Session-History]] §2026-09-16).
- Total unique strings: **461,704** (429,887 original + 31,817 added by the
  2026-09 game update, idx 0–461,703).

## Phase status

| Phase | Range           | Status      | Next idx                                       |
| ----- | --------------- | ----------- | ---------------------------------------------- |
| 0     | 0–799           | done        | —                                              |
| 1     | 800–1,999       | done        | —                                              |
| 2     | 2,000–4,999     | done        | —                                              |
| 3     | 5,000–9,999     | done        | —                                              |
| 4     | 10,000–19,999   | done        | —                                              |
| 5     | 20,000–49,999   | done        | —                                              |
| 6     | 50,000–99,999   | **active**  | **77,000** (27,000/50,000 rows done, 54.00%)   |
| 7     | 100,000–129,999 | not started | 100,000                                        |
| 8     | 130,000–159,999 | not started | 130,000                                        |
| 9     | 160,000–189,999 | not started | 160,000                                        |
| 10    | 190,000–219,999 | not started | 190,000                                        |
| 11    | 220,000–249,999 | not started | 220,000                                        |
| 12    | 250,000–279,999 | not started | 250,000                                        |
| 13    | 280,000–309,999 | not started | 280,000                                        |
| 14    | 310,000–339,999 | not started | 310,000                                        |
| 15    | 340,000–369,999 | not started | 340,000                                        |
| 16    | 370,000–399,999 | not started | 370,000                                        |
| 17    | 400,000–429,886 | not started | 400,000                                        |
| Update-1 | 429,887–461,703 | **active** | **452,737** (22,850/31,817 rows done, ~71.82%) |


Phases 0–17 draw from the original `unique_strings.jsonl` (idx 0–429,886).
Update-1 is the addition from the 2026-09-16 game update (idx
429,887–461,703 — see [[Phase-Roadmap]]). Phases 7–17 replace the old
100,000/229,887-string "Phase 7"/"Phase 8" split with eleven ~30,000-string
phases (rebalanced 2026-09-22 — see [[Phase-Roadmap]]).

**Phase 6 and Update-1 are both in progress in parallel.** If the user
doesn't say which one to continue, ask before starting — see
[[Resume-Procedure]].

## Most recent session (2026-09-24, session 12)

Phase 6, batch 27: idx 76,000–76,999 (1,000 rows — user requested 1,000 for
this session, overriding the 3,000 default set by the prior parallel
session) translated and appended to `locale/phase6.jsonl`. Total now
27,000/50,000 rows done for Phase 6 (54.00%). Mix of content: a long Kaifeng
main-story letter (the Tubo-hostage prince's final "three questions"
testament), a lengthy Master Dong/Hu Li Hundred-Schools philosophical
dialogue (Confucianism/Legalism/Daoism/Mohism synthesis), several
Tian-Ying-assassin/Aureate-Pavilion Qinghe backstory entries, gear/skill
tooltips (Strategic Sword's Zenith Sword follow-up, Vagrant Sword dash
mechanics, HP Shield/Hardened Foe interactions, Wind-riding/Blazing Drum
stacking), casual gue/lo NPC dialogue (wine-shop scenes, fishing-village
chatter, Homestead/guild flavor text), a large run of garbled Hall-of-Fame
player usernames (kept verbatim per convention), and Pinyin NPC name
entries (kept verbatim). No new terminology decisions — every case matched
an existing [[Glossary]] entry.

Token/placeholder validation via the standard `TOKEN` regex script found
**1 mismatch on the first pass** (idx 76248 — a `#Y`/`#E` color-tag pair
around `[Frigid Fall]` was dropped when moving the bracketed skill name next
to "Sun Sisi's"). Corrected in place and the batch was re-validated at **0
mismatches** before appending. Full-file re-validation after append:
`locale/phase6.jsonl` now 27,000 lines, all idx unique, 0 duplicates, 0
token mismatches across the entire file.

Update-1 was not touched this session (see the prior session's entry below
for its latest state — 22,850/31,817 rows, ~71.82%, unaffected by this
session's phase 6 work).

## Prior session (2026-09-24, session 11)

Update-1, batch 20: idx 449,737–452,736 (3,000 rows — new larger batch size,
see below) translated and appended to `locale/update1.jsonl`. Total now
22,850/31,817 rows done for Update-1 (~71.82%). Mix of content: Mohist
Hill/Mirkvale/Hidden Mountain lore (the Kuang Anshi "Swan" biography, the
Grand Artisan election and Waterworks Master promotion system, the Lord of
Clouds/Yuelu star-naming folktale, Xue/Su/Mi "Three Junior Paragons"
Mohist-City childhood story, several Luancheng/Zhao Tie war-refugee
vignettes), many long letters and diary entries (the starving-official's
Tongguang-era travel diary, Ding Xiang's gossipy letter to Gu Pan, the
general's letter to Sect Master Jiujue, Halcyon's anonymous-informant
letter, the frantic "help us" temple-prisoner note), gear/skill tooltips
(Bamboocut - Draught/Inebriate mechanics across many tiers, Etherwrath
stacking, Silkbind/Stonesplit/Bellstrike stat-tag templates), casual gue/lo
NPC dialogue throughout Search/Fight/Extract and Homestead/Mahjong Maestro
flavor text, and a large run of item/gear/chest name strings. No new
terminology decisions — every case matched an existing [[Glossary]] entry;
one recurring pattern (`"<Name>'s|id|#C|slot>"` — a possessive skill-name
tag) reconfirmed the existing §6 rule that `<...>` tag content must be
copied byte-for-byte, English possessive included, never restructured
around the translated sentence.

Token/placeholder validation via the standard `TOKEN` regex script found
**3 mismatches on the first pass** (idx 450595, 451246, 452491) — all
caused by translating text that lived *inside* a `<...>` tag (a
skill-name possessive, an "Inebriate-enhanced skills" label) instead of
leaving the tag byte-identical, plus one dropped `#Y`/`#E` pair on a
multi-clause string. All three were corrected in place and the batch was
re-validated at **0 mismatches** before appending. Full-file re-validation
after append: `locale/update1.jsonl` now 22,850 lines, all idx unique, 0
duplicates, 0 token mismatches across the entire file.

Phase 6 was not touched this session.

## Batch size

Mixed as of 2026-09-24: the session-11 Update-1 run used **3,000
strings/session**, but session 12 (this one) used **1,000 strings/session**
for Phase 6, per an explicit user request for that session that overrode
the 3,000 default. Batch size is decided per-session by whatever the user
asks for at the start — don't assume either number carries over. This has
changed several times over the project — see [[Session-History]] for the
full change log. Given a larger batch (e.g. 3,000), sessions should budget
more tool-call rounds for reading source chunks (the `Read` tool caps out
well under 1,000 lines for this file, so a 3,000-row batch needs ~10
sequential 250–300-line reads) and should re-run the token/placeholder
validation script directly against the merged scratch file *before*
appending — the larger the batch, the more likely a stray tag-content edit
slips in somewhere in the middle.
