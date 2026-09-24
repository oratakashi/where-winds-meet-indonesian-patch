# Current Status

**Last updated: 2026-09-24 (session 18).** This is the single source of truth for "how
far are we" — it gets overwritten each session, not appended to. For the
full timeline, see [[Session-History]]; for the phase plan, see
[[Phase-Roadmap]]; for the resume checklist, see [[Resume-Procedure]].

## Totals

- Total lines in `strings.jsonl`: **963,050** (pre-update version; the
  2026-09 update's main file has 826,388 — see [[Session-History]] §2026-09-16).
- Total unique strings: **461,704** (429,887 original + 31,817 added by the
  2026-09 game update, idx 0–461,703).

## Phase status

| Phase    | Range           | Status      | Next idx                                     |
| -------- | --------------- | ----------- | -------------------------------------------- |
| 0        | 0–799           | done        | —                                            |
| 1        | 800–1,999       | done        | —                                            |
| 2        | 2,000–4,999     | done        | —                                            |
| 3        | 5,000–9,999     | done        | —                                            |
| 4        | 10,000–19,999   | done        | —                                            |
| 5        | 20,000–49,999   | done        | —                                            |
| 6        | 50,000–99,999   | **active**  | **85,900** (35,900/50,000 rows done, 71.80%) |
| 7        | 100,000–129,999 | not started | 100,000                                      |
| 8        | 130,000–159,999 | not started | 130,000                                      |
| 9        | 160,000–189,999 | not started | 160,000                                      |
| 10       | 190,000–219,999 | not started | 190,000                                      |
| 11       | 220,000–249,999 | not started | 220,000                                      |
| 12       | 250,000–279,999 | not started | 250,000                                      |
| 13       | 280,000–309,999 | not started | 280,000                                      |
| 14       | 310,000–339,999 | not started | 310,000                                      |
| 15       | 340,000–369,999 | not started | 340,000                                      |
| 16       | 370,000–399,999 | not started | 370,000                                      |
| 17       | 400,000–429,886 | **active**  | **401,000** (1,000/29,887 rows done, 3.35%)  |
| Update-1 | 429,887–461,703 | **done**    | — (31,817/31,817 rows done, 100%)            |

Phases 0–17 draw from the original `unique_strings.jsonl` (idx 0–429,886).
Update-1 is the addition from the 2026-09-16 game update (idx
429,887–461,703 — see [[Phase-Roadmap]]). Phases 7–17 replace the old
100,000/229,887-string "Phase 7"/"Phase 8" split with eleven ~30,000-string
phases (rebalanced 2026-09-22 — see [[Phase-Roadmap]]).

**Update-1 is fully complete.** Phase 6 and Phase 17 are both in progress
(Phase 17 was started ahead of Phase 6 at the user's explicit request).
Resume whichever phase the user asks for; otherwise Phase 6 (larger
remaining share) — see [[Resume-Procedure]].

## Most recent session (2026-09-24, session 18) — Phase 6, batch 32 (partial toward 3,000)

Phase 6, batch 32: idx 85,000–85,899 (900 rows translated and appended to
`locale/phase6.jsonl` this session; user requested 3,000 rows/iteration,
banked as a partial batch per [[Resume-Procedure]] step 9 rather than
rushing quality). Total now 35,900/50,000 rows done for Phase 6 (71.80%).
Mix of content: many gear/skill tooltip strings (Heavenquaker Spear
Soul-Shaken stacking, Silkbind DMG/Healing Bonus stat-tag template,
Umbrella Heavy Attack/Charged Skill Invisibility mechanic, Phalanxbane
Blade damage-reduction debuff, Perfect Dodge/Addled Mind cooldown reset),
several NPC lore/backstory blurbs (Gao Siji "Stormbreaker Spear" origin
legend, the Dragonbend Academy Guardian's sixteen-year-seclusion vignette,
the Liu Xiaomei stone-carving academy notice, the Jadestream Valley monkey-
raft folk tale, Sufferers' Trials weekly ranked-event description), a
Kaifeng-politics/Aureate Pavilion lore block (Lodestar Swordmaster's
Black Gold ultimatum to Swallow), casual gue/lo NPC dialogue throughout
(tavern/Jianghu banter, Homestead flavor text, children's dialogue), a
classical-style Tang poem (Ode to the Stepping Stars — Northern Dipper
star names), and a large run of garbled Hall-of-Fame usernames and Pinyin
NPC name entries (kept verbatim per convention). No new terminology
decisions — every case matched an existing [[Quick-Reference]] entry.

Token/placeholder validation via the standard `TOKEN` regex script found
**0 mismatches** across the full 900-row batch before appending. Full-file
re-validation after append: `locale/phase6.jsonl` now 35,900 lines, all idx
unique and sequential (50,000–85,899, no gaps), 0 duplicates, 0 token
mismatches across the entire file.

Update-1 and Phase 17 were not touched this session. Session ended the
batch at 900/3,000 rows to keep translation quality high within the
session's available capacity — resume at idx 85,900 for the remainder of
this 3,000-row request.

## Prior session (2026-09-24, session 17) — Phase 6, full 3,000-row batch

Phase 6, batch 31: idx 82,000–84,999 (3,000 rows, delivered in two passes
within the same session — 1,199 rows banked and committed first, then the
remaining 1,801 rows completed at the user's explicit request to reach the
full 3,000) translated and appended to `locale/phase6.jsonl`. Total now
35,000/50,000 rows done for Phase 6 (70.00%). Mix of content: many
gear/skill tooltips (Stonesplit Penetration stacking, Silkbind - Jade
Stage, Bamboocut Attack scaling, Soulshade Umbrella/Panacea Fan Common
Martial Art interactions, Sober Sorrow/Strategic Sword combo mechanics,
Heavenwill Gauntlets Vile Condemned evolution), several NPC lore/backstory
blurbs (Fang Hong's grandmaster death story, the Feng Shui/geomancy
treatise excerpt, the Northern Dipper longevity scripture, the
mechanical-invention showcase with mentor/disciple banter, Qin
Yuan/Luo Chengwu puppet-craft diary entries, Zhu Yu's farewell-to-Silver-
Needle vignette, the Astral Rain/Cloudrest Passage event description), a
few Kaifeng-politics lore blocks (The Ember of East/The Shimmer of South
scheme, the Revelry Hall Heroes Assembly gossip column), casual gue/lo NPC
dialogue throughout (tavern/Jianghu banter, Homestead flavor text), several
classical-style poems, a long letter (Zhan Yuelu's message-delivery
request), and a large run of garbled Hall-of-Fame usernames and Pinyin NPC
name entries (kept verbatim per convention). No new terminology
decisions — every case matched an existing [[Quick-Reference]] entry.

Token/placeholder validation via the standard `TOKEN` regex script found
**3 mismatches** in the second 1,801-row sub-batch: idx 84242 and idx 84963
(a `<...>` tag's possessive `'s` was dropped when copying `<Strategic
Sword's|781|#C|10101>` and `<Inkwell Fan's|1600021|#C|103024>` — §6 rule 2
requires the tag content copied byte-for-byte) and idx 84581 (`#Y...#E`
color tags mistyped as `#D...#E` twice while translating "reset"). All
three corrected in place; re-validated at **0 mismatches** before
appending. Full-file re-validation after append: `locale/phase6.jsonl` now
35,000 lines, all idx unique and sequential (50,000–84,999, no gaps), 0
duplicates, 0 token mismatches across the entire file.

Update-1 and Phase 17 were not touched this session.

## Prior session (2026-09-24, session 16) — Phase 17 started

Phase 17, batches 1–2: idx 400,000–400,999 (1,000 rows total) translated
and written to new file `locale/phase17.jsonl`. The user originally asked
for 3,000 rows; batch 1 (401 rows) was banked first rather than rushing
the full 3,000 at lower quality, then batch 2 (599 rows) continued in the
same session at the user's request to reach a round 1,000-row milestone.
Mix of content: NPC dialogue and lore blurbs around Kaifeng/Apricot
Village/Mirkvale (Elder Harrier's letter to the Master of Haven, Xiaoba's
Vale water-track repairs, Wang Fen's lost-daughter story, Crow Brew lore),
several classical-style poems, gear/skill tooltips (Etherwrath stacking,
Petalwhirl Immobilize mechanic, Thundercry Blade defensive riposte), and
many Pinyin NPC name entries (kept verbatim per convention). No new
terminology decisions — every case matched an existing [[Quick-Reference]]
entry.

Token/placeholder validation via the standard `TOKEN` regex script found 3
mismatches across both batches: idx 400199 (a non-standard `# text$hunt#`
token lost its leading space), idx 400754 and idx 400861 (§6 rule 1/3
violations — a `<...>` tag wrapping a full sentence, and one wrapping a
short stage direction, were translated instead of kept verbatim). All
corrected in place. Re-validated at 0 mismatches before saving. Full-file
check after merging both batches: 1,000 lines, idx 400,000–400,999 with no
gaps or duplicates. Phase 6 was not touched this session.

## Prior session (2026-09-24, session 15)

Phase 6, batch 30: idx 80,000–81,999 (2,000 rows — user requested 2,000/iteration for
this session) translated and appended to `locale/phase6.jsonl`. Total now 32,000/50,000
rows done for Phase 6 (64.00%). Mix of content: many gear/skill tooltips (Vagrant Sword/
Nameless Sword's sword-energy mechanics, Scarlet Spin - Perfect Catch/Falling Blossoms
chain, Heng Blade shield-reflection skill, Bellstrike - Splendor Qi Imbalance proc,
Panacea Fan water-clone healing), several NPC lore/backstory blurbs (buddha sculptor's
son, Apricot Village donkey-mill vignette, spirit-tablet dowry lore, oat-cultivation
farewell letter to Elder Peng, Unbound Cavern/Bei Xiaoyu backstory, Shimmer of the
South legend), casual gue/lo NPC dialogue throughout (tavern/Jianghu banter, Homestead
flavor text, children's dialogue), a long war-letter/expenditure ledger (Kang Zhenzhu),
several classical-style poems, and a large run of garbled Hall-of-Fame usernames and
Pinyin NPC name entries (kept verbatim per convention). No new terminology decisions —
every case matched an existing [[Quick-Reference]] entry.

Token/placeholder validation via the standard `TOKEN` regex script found **0 mismatches
on the first pass** — the full 2,000-row batch validated clean before appending.
Full-file re-validation after append: `locale/phase6.jsonl` now 32,000 lines, all idx
unique, 0 duplicates, 0 token mismatches across the entire file.

Update-1 was not touched this session.

## Prior session (2026-09-24, session 14) — Update-1 complete

Update-1, batches 22–38: idx 455,737–461,703 (5,967 rows across this
session, batches of ~350 rows each) translated and appended to
`locale/update1.jsonl`, taking Update-1 from 25,850/31,817 (~81.25%) to
**31,817/31,817 (100%) — Update-1 is now fully translated.** Mix of
content: Mohist Hill/Mirkvale/Hidden Mountain lore (the Dragonbend Academy
founding register and roster across multiple Tianfu/Kaiyun years, the
Ma Pingshan revenge-turned-redemption story, the "Ying Ning" wildfire-child
origin myth, the Fang Bai/Qiniang Golden-Peach war letter, Grand Artisan
Gull's "raising Xiaoxiao" recipe-book story, the Halcyon/Raven/Zhang Wanshi
Derndale-punishment mystery, Tang Wenyao's father's marriage-pressure
letter), many gear/skill tooltips (Silkbind/Bellstrike Attack stat-tag
templates across many tiers, Vile Condemned/Falcon's Pursuit Heavenwill
Gauntlets mechanics, Inebriate-state Dual Blades/Riven Twinblades skill
lists, Path Trial/Battle Pass/Co-construction system-text blocks), casual
gue/lo NPC dialogue throughout Homestead/Mirkvale/Search-Fight-Extract
flavor text, and a large run of gear-chest/tile/title description strings.
No new terminology decisions — every case matched an existing [[Glossary]]
entry.

Token/placeholder validation via the standard `TOKEN` regex script found a
handful of mismatches across these batches, all caught and fixed before
appending: a dropped `#Y` prefix on a translated tag (idx 460865, `#Y
Digabungkan#E` written without the `Y`), reverted in place. Every batch was
re-validated at **0 mismatches** before being appended. After the final
append, a full-file re-validation of `locale/update1.jsonl` (31,817 lines)
confirmed: all idx unique and sequential (429,887–461,703, no gaps), 0
duplicate idx, 0 token mismatches, 0 empty translations across the entire
file.

Phase 6 was not touched this session.

## Prior session (2026-09-24, session 14)

Phase 6, batch 29: idx 78,000–79,999 (2,000 rows — user requested
2,000/iteration for this session) translated and appended to
`locale/phase6.jsonl`. Total now 30,000/50,000 rows done for Phase 6
(60.00%). Mix of content: a large run of gear/skill tooltips (Skygrasp
Rope Dart/Heavenwill Gauntlets Heaven's Might synergy, Soulshade Umbrella
Flowing Flame mechanic, Scarlet Spin Phantom Umbrella resonance, Vernal
Umbrella projectile scaling), several NPC backstory/lore blurbs (Jade
Carving Workshop apprentice, Rat Den "Five Rats" member, Twelve Immortals
dye artisan, Nimbus Tower/Zhou Qiang courtesan lore, TCM five-tone
meridian treatise, Guo Xin's Anxi Garrison epitaph), casual gue/lo NPC
dialogue throughout (tavern scenes, fishing-village banter, Homestead
flavor text, the pig-racing children's-story vignette), a long
body-swap system explanation block, a long Matchmaking Arena/Grand
Assembly event description, many garbled Hall-of-Fame usernames and
gibberish gamer-tags (kept verbatim per convention), and Pinyin NPC name
entries (kept verbatim). No new terminology decisions — every case
matched an existing [[Glossary]] entry.

Token/placeholder validation via the standard `TOKEN` regex script found
**3 mismatches on the first pass**: idx 78577 (a `#Y...#E` tag was
mistakenly split across two words — "Boss" and "menggunakan Energy" —
instead of wrapping only the phrase corresponding to the source's single
tag), idx 78715 (a stray `# ` with a space instead of a proper `#Y...#E`
tag when translating "combined"), and idx 78740 (the `#Ydeflection#E` tag
was dropped entirely when restructuring the sentence into Indonesian word
order). All three corrected in place and the batch was re-validated at
**0 mismatches** before appending. Full-file re-validation after append:
`locale/phase6.jsonl` now 30,000 lines, all idx unique, 0 duplicates, 0
token mismatches across the entire file.

Update-1 was not touched this session.

## Prior session (2026-09-24, session 13) — Update-1

Update-1, batch 21: idx 452,737–455,736 (3,000 rows — new batch size per
user request, "tiap iterasi 3000 string") translated and appended to
`locale/update1.jsonl`. Total now 25,850/31,817 rows done for Update-1
(~81.25%). Mix of content: Mirkvale/Xiaoba mechanism-repair storyline
(the Diversion Complex fix, Wooden Eagle assembly, several music-box/kite
inventions), Mohist Hill lore (the Derivation Array maze, cremation
tradition origin story of Grandmaster Parrot and Parakeet, Gongshu Hui's
"wisdom" backstory), many Sheathed Passage/Raging Tides war-letter entries
(Shi Jiuge's commander correspondence, the northern-foe hostage situation),
gear/skill tooltips (Bamboocut - Draught Inebriate/Deepdaze mechanics
across many tiers, Riven Twinblades/Skystrike Gauntlets combo rotations,
Cleftpeak/Etherwrath/Jadeclasp set-effect stat blocks), casual gue/lo NPC
dialogue throughout Homestead/Search-Fight-Extract/Cutie-Showdown flavor
text, and a large run of gear-chest/tile description strings.

Token/placeholder validation via the standard `TOKEN` regex script found
**9 mismatches on the first pass**: 5 were `<...>`-wrapped inner-thought
lines (e.g. idx 453656, 453754, 453776, 453884, 455027) that had been
translated instead of kept verbatim per Quick-Reference §6 rule 1 (a plain
`<...>` tag without `|id|#C|n>` format must stay in English untouched) —
reverted to the source text; the other 4 (idx 454095, 454164, 454252, 455165) were `#Y...#E` color-tag wraps dropped or mistyped
(`#Ditumpuk#E` instead of `#Ytumpuk#E`, and a missing wrap around
"Enhancement"/"Max-tuned") while restructuring sentences — corrected in
place. Re-validated at **0 mismatches** before appending. Full-file
re-validation after append: `locale/update1.jsonl` now 25,850 lines, all
idx unique and sequential, 0 duplicates, 0 token mismatches.

No new terminology decisions this session — every case matched an existing
[[Glossary]] entry.

## Prior session (2026-09-24, session 12)

Phase 6, batch 27: idx 76,000–76,999 (1,000 rows — user requested 1,000 for
this session, overriding the 3,000 default set by the prior parallel
session) translated and appended to `locale/phase6.jsonl`. Total 27,000/
50,000 rows done for Phase 6 (54.00%) at the end of that session. Mix of
content: a long Kaifeng main-story letter (the Tubo-hostage prince's final
"three questions" testament), a lengthy Master Dong/Hu Li Hundred-Schools
philosophical dialogue (Confucianism/Legalism/Daoism/Mohism synthesis),
several Tian-Ying-assassin/Aureate-Pavilion Qinghe backstory entries,
gear/skill tooltips (Strategic Sword's Zenith Sword follow-up, Vagrant
Sword dash mechanics, HP Shield/Hardened Foe interactions, Wind-riding/
Blazing Drum stacking), casual gue/lo NPC dialogue (wine-shop scenes,
fishing-village chatter, Homestead/guild flavor text), a large run of
garbled Hall-of-Fame player usernames (kept verbatim per convention), and
Pinyin NPC name entries (kept verbatim). No new terminology decisions —
every case matched an existing [[Glossary]] entry.

Token/placeholder validation via the standard `TOKEN` regex script found
**1 mismatch on the first pass** (idx 76248 — a `#Y`/`#E` color-tag pair
around `[Frigid Fall]` was dropped when moving the bracketed skill name next
to "Sun Sisi's"). Corrected in place and the batch was re-validated at **0
mismatches** before appending. Full-file re-validation after append:
`locale/phase6.jsonl` was 27,000 lines, all idx unique, 0 duplicates, 0
token mismatches across the entire file.

Update-1 was not touched this session.

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
caused by translating text that lived _inside_ a `<...>` tag (a
skill-name possessive, an "Inebriate-enhanced skills" label) instead of
leaving the tag byte-identical, plus one dropped `#Y`/`#E` pair on a
multi-clause string. All three were corrected in place and the batch was
re-validated at **0 mismatches** before appending. Full-file re-validation
after append: `locale/update1.jsonl` now 22,850 lines, all idx unique, 0
duplicates, 0 token mismatches across the entire file.

Phase 6 was not touched this session.

## Batch size

Mixed as of 2026-09-24: the session-11 Update-1 run used **3,000
strings/session**, sessions 12 and 13 used **1,000 strings/session**
for Phase 6, sessions 14–15 used **2,000 strings/session** for Phase 6,
and session 17 was asked for **3,000 strings/session** for Phase 6 and
delivered the full 3,000 — in two passes within the same session (1,199
banked and committed first, then 1,801 more completed after the user
asked to continue to the full number) — each per an explicit user request
that overrides the prior session's number. Batch size is decided
per-session by whatever the user asks for at the start — don't assume
either number carries over. This has changed several times over the
project — see [[Session-History]] for the full change log. Given a larger
batch (e.g. 3,000), sessions should budget more tool-call rounds for
reading source chunks (the `Read` tool caps out well under 1,000 lines for
this file, so a 3,000-row batch needs ~10 sequential 250–300-line reads)
and should re-run the token/placeholder validation script directly against
the merged scratch file _before_ appending — the larger the batch, the
more likely a stray tag-content edit slips in somewhere in the middle
(session 17's second sub-batch hit exactly this: 3 tag/token mismatches
across 1,801 rows, all caught and fixed pre-append). Session 17's default
instinct was to bank a partial batch and stop cleanly per
[[Resume-Procedure]] step 9 rather than risk quality on a single huge pass —
but when the user explicitly says they want the full requested count
("aku ingin 3000 baris"), continue past that stopping point in the same
session rather than treating the partial batch as the final answer.
