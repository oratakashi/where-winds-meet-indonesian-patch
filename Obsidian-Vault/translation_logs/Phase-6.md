# Phase 6 Translation Log (idx 50,000–99,999)

Session-by-session terminology notes for Phase 6, in idx order. The current,
consolidated rules live in [[Glossary]] — this file is the reasoning/history
behind them. Phase 6 also opened with a small carry-over batch from the end
of Phase 5 — see [[Phase-5]] and [[Session-History]] for that transition.

## idx 49,000–50,999 (Phase 5 closing + Phase 6 opening batch)

- **"Pangolin" → "Trenggiling" reconfirmed**, per the older Phase 4 rule
  (idx 18,100–19,099), which had been missed for the NPC "Pangolin
  Peddler" (idx 49,821, 49,857, 50,885) — fixed to "Pedagang
  Trenggiling"/"Lapak Trenggiling" before validation, consistent across
  all three occurrences.
- **Chinese-zodiac double-hour names (e.g. "Hai Hour", "Xu hour")** — a new
  pattern; decided to TRANSLATE "Hour"/"hour" as "Jam", while the hour name
  itself (Hai, Xu, etc.) stays in English/Pinyin as a traditional time term
  with no standard equivalent (e.g. "Hai Hour" → "Jam Hai", "Xu hour" →
  "Jam Xu"). idx 49,014, 50,880.
- **"Form" as a transformation status** (e.g. "Carp form", "Wind form",
  "Vulpine Form", "Feline Form") — decided to TRANSLATE as "Wujud" (e.g.
  "Vulpine Form" → "Wujud Vulpine", "Carp form" → "wujud Carp"), applied
  consistently across every occurrence this session (idx 49,218, 49,451,
  50,686).
- **Generic lowercase "master"/"Master"** (e.g. "the only master who knew
  how to mount it", "three masters" of a smithing craft) is kept in
  English lowercase as a common loanword for "expert/craftsman" — different
  from "Master" as an honorific before a name (still "Guru") and "Master"
  as an organization leader (still "Ketua"). idx 49,566, 49,978.
- **Single-letter duration shorthand keeps converting** `s`→`d` (seconds)
  and `h`→`j` (hours) per the locked convention, including inside a color
  tag (e.g. `#Y3#Es`→`#Y3#Ed`, "1h"→"1j" idx 50,660) and right after a `{}`
  placeholder (e.g. `{}s`→`{}d`, idx 49,991).
- **"Wanderer"/"Wayfarer" reconfirmed consistently "Pengembara"** (including
  "Wayfarer" standing alone, idx 50,695, not just "Wanderer"). **"Player"
  stays in English regardless of context** (verified via a 0-violation
  spot-check). "Retainer"/"Companion"/"Cultivation"/"Sect"→"Sekte" and long
  gear/skill names follow the pattern already locked in earlier Phase 5
  sessions — no new convention changes for these terms in this Phase 6
  opening batch.
- **New placeholder patterns found and preserved exactly**:
  `$link<text>^ID^$` (idx 49,413, 50,398 — everything between `$link` and
  `^ID^$`, including any English text inside it, MUST be kept identical,
  treated as one opaque unit), and `$S...$E` (a quote/narrative block
  wrapper in skill descriptions, e.g. idx 50,135, 50,583, 50,716 —
  different from `$D`/`$H`/`$F`, which are pure numeric values; `$S...$E`
  WRAPS a narrative sentence that is still translated, only the `$S`/`$E`
  markers themselves are preserved exactly).

## idx 51,000–52,999 (batch 2)

No significant new convention decisions — this batch was dominated by
random NPC/player usernames (many unique `freq: 2` strings like
"ZaliThsOng", "MaleniaprocyoN"), generic NPC dialogue, and item/quest lore
following patterns already locked in prior sessions (Wanderer → Pengembara,
Player stays in English, `#Y...#E`/`<...>`/placeholder tags preserved,
duration shorthand s→d/h→j, etc.). One long philosophical lore paragraph
about Mozi/Mohist (idx 51,284) and another about Skyward City (idx 51,171)
were translated in full as standard wuxia narrative text.

## idx 53,000–54,999 (batch 3, 2026-09-21)

Another heavily UI/gear-label/username-dominated batch, same pattern as
batch 2. No new terminology rules were needed — every decision matched an
existing [[Glossary]] entry (Pangolin → Trenggiling, Wayfarer/Wanderer →
Pengembara, Master split by context, Treasury → Perbendaharaan, Red
Envelope → Angpao, `s`→`d`/`h`→`j` duration shorthand including inside
color tags, `$link<...>^ID^$` and `$S...$E` opaque/wrapper handling, Zi/You
Hour → Jam Zi/Jam You, Companion/Retainer kept English).

- **Internal mechanic/animation debug-style labels left untranslated** —
  strings that read like internal engine/animation state names rather than
  player-facing UI or narrative text (e.g. "3-Man Jump Left", "Five-Point
  Jump Skill 3-2D", "Umbrella Jump 2.5D 3-Person Array Skill 3", "Chess
  Low", "Steel Wall Lv.2"). Treated the same as skill/gear labels (kept
  English) since translating them risks breaking a debug/QA tool that
  greps for the English string. Not yet promoted to [[Glossary]] as a
  locked rule — revisit if a much larger cluster of these appears in a
  later phase and a clearer boundary is needed (vs. e.g. "Special
  Speedrun Mechanics" at idx 53013, which reads as player-facing and WAS
  translated).
- Several long narrative/lore entries translated in full per the standard
  rule (see [[Translated-Common-Terms]]): the "Jing and Da" Uncle
  Zhang/ghost-soldier tale at idx 53306 (~2,500 chars, place names Hexi/
  Jade Gate Pass/Yangguan Pass/Qilian Mountains/Chang'an/Great Tang kept
  English), the Ember of East / Gold-Making Vessel frame-story quest
  summary at idx 53412, the Anxi Protectorate prisoners → Raging Tides
  founding-myth lore at idx 53760, and a homesick daughter's letter to her
  father at idx 53901.
- Reconfirmed via a wide unaffected spot-check: gear/skill/costume names
  (Swallowcall, Ivorybloom, Calmwaters, etc.), stat terms (Physical Attack,
  Critical Rate, DMG Bonus), and game-mode/feature names (Guild War,
  Battle Pass, Co-op) all stayed in English with zero exceptions this
  batch.

Token/placeholder validation (the [[Resume-Procedure]] script) ran per
500-row sub-batch and once more on the full merged `locale/phase6.jsonl`
(5,000 lines): 0 mismatches, idx sequential 50,000–54,999, no duplicates.

## idx 55,000–56,999 (batch 4, 2026-09-21)

A 2,000-row batch (double the usual 500-row sub-batch size, per the user's
request to run phase 6 at 2,000 rows/iteration this session) — same overall
mix as prior Phase 6 batches: heavy on random player-username `freq: 2`
strings (odd mixed-capitalization tokens with no spaces, e.g. "ROyalarrow",
"DuSkserPent", "chaosCore"), Chinese-pinyin NPC names, gear/skill UI labels,
and internal technical/debug-style area labels (e.g. "Open World Boss Lion
Dance Player Too Far End Area", "Qinghe Field Boss - Shadow Puppeteer No. 2
Failure Zone") — all kept untranslated per the precedent set in batch 3. No
new terminology decisions — every case matched an existing [[Glossary]]
entry:

- Pangolin Peddler → Pedagang Trenggiling (idx 55123, 55392) reconfirmed.
- Duration shorthand `s`→`d`/`h`→`j` applied throughout, including inside
  `#Y...#E` tags and combined with `{}`/`%` placeholders (many hits, e.g.
  idx 55024, 55676, 55946, 56472, 56496, 56575, 56982).
- `$link<...>^ID^$` (idx 56772) and `$S...$E` (idx 55068) opaque/wrapper
  handling reconfirmed.
- A `<...>` tag wrapping a full sentence (idx 55436, 56663) kept 100%
  identical in English per the long-sentence-tag gotcha.
- Generic NPC roles translated (Passerby → Orang/Wanita Lewat idx 55028,
  56406; Maid → Pelayan idx 55801; Disciple → Murid where the batch's local
  idx-neighborhood used the translated form, e.g. idx 55236, 55954, 56368 —
  consistent with Phase 1's original rule, this range is well before the
  idx ~432000+ neighborhood where [[Open-Questions]] notes disciple was
  left English for local consistency).
- Honorifics per [[Honorifics-And-Titles]]: Miss/Lady/Madam/Mrs. → Nona/
  Nyonya (idx 55011, 55739, 56079, 56283, 56461, 56678), Uncle → Paman (idx
  55981), Lord → Tuan (idx 55723, 56825), Master (teacher) → Guru (idx
  55337, 56894), Constable → Konstabel (idx 56492), Doctor → Tabib (idx
  55470, 56446, 56717, 56881).
- Several long narrative/lore entries translated in full: the Hexi
  merchant's farewell letter to "Patar" (idx 55435), the Guo Xie/Well of
  Heaven origin myth (idx 55726), the Sun Buqi/"Man with One Thousand
  Chrysalis" tale (idx 56523), the Gracetown poem/village description (idx
  56337), the Path of Slaughter/Sufferers creed (idx 56338), the Ni Laoshan
  swordsman poem (idx 55521), and a #M-tagged letter about the "Sleeping
  Puppet Calamity" preserving literal `\v` line-break markers (idx 56934).
- One ambiguous new case, resolved by nearest precedent rather than a new
  rule: "Master Qi"/"Master Pu" as an honorific before a name for a
  non-martial-arts, non-organization-leader expert (a horse breeder, idx
  55911; a scholar known for fast reading, idx 56194) — kept in English,
  treated as the lowercase-craftsman convention rather than "Guru" since
  neither is a martial-arts teacher. Flagged here in case a clearer rule is
  needed if this pattern recurs.
- "Fishing Contest" (idx 55283) and "Specialized Artisan Pavilion" (idx
  56011) kept in English as feature/facility names, consistent with the
  Game Modes/Features convention even though neither is explicitly listed
  in [[Kept-In-English-Terms]] yet.

Token/placeholder validation ran on the full 2,000-row batch (source-text
token-count diff against `unique_strings.jsonl`): **0 mismatches**. An
EMPTY check (source non-blank, translation blank) also ran: **0 hits**. Full
merged `locale/phase6.jsonl` (7,000 lines) re-validated: idx sequential
50,000–56,999, 7,000 unique idx, no duplicates.

## idx 57,000–58,999 (batch 5, 2026-09-21)

Another 2,000-row batch, same overall mix as batches 3–4: heavy on random
player-username `freq: 2` strings (odd mixed-capitalization tokens with no
spaces, often wrapped in `×...×`, e.g. "AsHdIve", "ShikamaruAsh",
"KakashiShade", "SirRoSe") kept untranslated per precedent, Chinese-pinyin
NPC names, gear/skill/costume UI labels (`Tier N <Set>: <Slot>`, `- EX`/
`- Radiance` variants), and internal technical/debug-style strings (e.g.
"AnimController does not exist", "Entity Game id_2", "%s AI file: %s,
Running node: %s", "Non-Server Object", "Server entity") left untranslated
as engine/dev-facing text, consistent with batch 3's precedent. No new
terminology decisions — every case matched an existing [[Glossary]] entry:

- Duration shorthand `s`→`d`/`h`→`j` applied throughout, including inside
  `#Y...#E`/`#V...#E` tags (e.g. idx 57150, 58069, 58723, 58737).
- `$link<...>^ID^$` (idx 58929), `$S...$E` (idx 58412, 58866), and the
  runtime date placeholder `@T[...]` (idx 58464) opaque/wrapper handling
  reconfirmed — all preserved character-for-character.
- A `<...>` tag wrapping a full sentence/phrase kept 100% identical in
  English per the long-sentence-tag gotcha (e.g. idx 57221 `<Little
  Rascal | 30093 |#Y>`).
- Honorifics per [[Honorifics-And-Titles]]: Master (teacher) → Guru (idx
  57129), Grandmaster kept English as a proper title (idx 57022, 58748),
  Lord → Tuan (idx 58602, 58226), Young Master kept English as a role
  label in context (idx 57460, 58078), Doctor → Tabib (idx 57280).
- Generic NPC/role nouns and ordinary descriptive item/monster names
  translated per [[Translated-Common-Terms]] (e.g. "Rising Hero" →
  "Pahlawan yang Bangkit" idx 57017, "Hemostatic Powder" → "Bubuk
  Penghenti Darah" idx 57054), while fantasy-invented proper nouns,
  Homestead material names, and short 1–4 word poetic item/skin names
  (e.g. "Donkey-Pace Serenity" idx 57011, "Dew Drops Adhere" idx 57042)
  stayed in English per [[Names-Not-Translated]].
- Several long narrative/lore entries translated in full: the Nine-Turn
  Water Mill / Zhuge Liang's Wooden Ox passage (idx 57045, 57373), the
  Fu Qianli "Frost and snow" farewell-blade story (idx 57831), the Wang
  Xin battlefield letter to a fallen friend (idx 57604), the Cheng Fang /
  Wuwei Mountain whistling-arrows tragedy (idx 57600), the Yuzhang
  prefecture eulogy (idx 57635), the flower-drum lantern-fair vignette
  (idx 58221), and the Guo Xin / Anxi Army banner passage (idx 58136).

Token/placeholder validation ran on the full 2,000-row batch (source-text
token-count diff against `unique_strings.jsonl`): found and fixed **1
mismatch** at idx 57636 (accidentally singularized `<Light Attacks|...>` to
`<Light Attack|...>` inside a stat-formatted tag — corrected to match the
source exactly). Re-ran after the fix: **0 mismatches**. An EMPTY check
(source non-blank, translation blank) also ran: **0 hits**. A separate check
for untagged placeholders not caught by the `TOKEN` regex (`$VAR$`,
`@T[...]`, `$link<...>^ID^$`) also ran: **0 mismatches**. Full merged
`locale/phase6.jsonl` (9,000 lines) re-validated: idx sequential
50,000–58,999, 9,000 unique idx, no duplicates.

## idx 59,000–60,999 (batch 6, 2026-09-21)

Another 2,000-row batch, same overall mix as batches 3–5: a large share of
random player-username `freq: 2` strings (mixed-capitalization tokens with
no spaces, some wrapped in `×...×`, e.g. "clutchbEacon", "AshapexbouNd",
"RagnarokarmoR", "ZerefanubiS") kept untranslated per precedent, Chinese-
pinyin NPC/place names, gear/skill/costume UI labels (`Tier N <Set>:
<Slot>`, `- EX`/`- Radiance`/`- Weak` variants), and internal debug/dev-
facing strings left untranslated (e.g. "No.6 1 Release - Scene 95008",
"PVE Tutorial Text Placeholder 19", "Monitor Dialogue Opening General
Reading (Letter on Ground)"). No new terminology decisions — every case
matched an existing [[Glossary]] entry:

- Duration shorthand `s`→`d`/`h`→`j` applied throughout, including inside
  `#Y...#E` tags (e.g. idx 59916 "Cooldown: 80s"→"80d", idx 60436, 60620,
  60747, 60782).
- `#e9a358`-style non-standard color tags: only the 6-char hex is a real
  token, free text after it is safely translatable — reconfirmed at idx
  60068 (restructured the sentence so the tag wraps just "Karma Point"
  instead of the whole clause, since the source's literal number "60" sits
  right after the tag text and doesn't need special preservation).
- A `<...>` tag wrapping a full sentence kept 100% identical in English
  per the long-sentence-tag gotcha — caught and fixed one violation at idx
  60957 (`<Lord Crimson Carp silently fell into the money pit.>`, a debug/
  flavor string) where the content inside the tag had been translated by
  mistake; corrected back to the literal English source.
- Honorifics per [[Honorifics-And-Titles]]: Master (organization leader) →
  Ketua (idx 59422 "Current Troupe Master"), Aunt → Bibi (idx 59281,
  59409), Miss → Nona (idx 59027), Lord → Tuan (idx 59065).
- Generic NPC/role nouns and ordinary descriptive item/lore text
  translated per [[Translated-Common-Terms]], while fantasy-invented
  proper nouns, Homestead material names, legendary weapon/gear-set names,
  and short poetic item/skin names stayed in English per
  [[Names-Not-Translated]] (e.g. "Windfarer" idx 59442, "Ghostcap
  Parasol" idx 59474, "Paw-some Friend" idx 59448 — a pun kept in English
  since "Paw-some" doesn't translate).
- A full legal/ToS-style paragraph (Spotlight Mode content-upload terms,
  idx 59457) translated in full into formal-but-plain Indonesian, matching
  the register of other UI legal text already in the file.
- Several long narrative/lore entries translated in full: the eldest
  Zhang son's Buddhist-temple regret story (idx 59176), the Cui Xiaojin
  border-guard farewell letter (idx 59112), the Yuan Xi well-digging /
  Silk-Road water-map passage (idx 59567), the Imperial Archives
  "Inkbound" bamboo-corridor vignette (idx 59864), and the Jiang Dazhen
  lamp-offering prayer for her late mother (idx 60861).

Structural sanity check caught and fixed an authoring error mid-batch:
one source line (idx 59441, "Huai'an Star") was skipped while drafting,
which silently shifted every subsequent idx label in the batch down by
one for about 17 entries, plus produced one fabricated line with no
source counterpart. Caught by re-reading the exact source range and
comparing line-by-line before merging — fixed by re-inserting the missing
entry, shifting the mislabeled run back into alignment, and deleting the
fabricated line. Lesson for future batches: run the idx-sequence check
(below) on each 500-line chunk immediately after drafting it, not only
once at the end of the full batch.

Token/placeholder validation ran on the full 2,000-row batch (source-text
token-count diff against `unique_strings.jsonl`): found and fixed **2
issues** — the `<...>`-tag mistranslation at idx 60957 above, and a
garbled-number typo at idx 60068 introduced while restructuring around a
color tag (both described above). Re-ran after fixes: **0 mismatches**.
An EMPTY check (source non-blank, translation blank) also ran: **0 hits**.
Full merged `locale/phase6.jsonl` (11,000 lines) re-validated: idx
sequential 50,000–60,999, 11,000 unique idx, no duplicates.

## idx 61,000–62,999 (batch 7, 2026-09-21)

Another 2,000-row batch, same overall mix as batches 3–6: a large share of
random player-username `freq: 2` strings (mixed-capitalization tokens with
no spaces, many wrapped in `×...×`, e.g. "radIantvOiD", "MightyGriffin",
"hyperfieRcebond") kept untranslated per precedent, Chinese-pinyin NPC/
place names, gear/skill/costume UI labels (`Tier N <Set>: <Slot>`, `- EX`/
`- Common`/`- Radiance` variants), and internal debug/dev-facing strings
left untranslated (e.g. "HUD interaction button UI: visible=false",
"Someone attacking player, entering the performance subtree.",
"active_interact_exec_after_unlock_actions"). No new terminology
decisions — every case matched an existing [[Glossary]] entry:

- Duration shorthand `s`→`d`/`h`→`j` applied throughout, including inside
  `#V...#E`/`#ffc89c...#E` tags and combined with `{}` placeholders (e.g.
  idx 61010 "15s"→"15d" via `#V15#Ed`, idx 61023 `#ffc89c{0}#Es`→`#Ed`, idx
  62822 "+%sh0m"→"+%sj0m").
- `$link<...>^ID^$` (idx 61801) and the runtime date placeholder `@T[...]`
  (idx 61580) opaque/wrapper handling reconfirmed — preserved
  character-for-character.
- `<LINK id='link2' color='#8c5823' goto_id='660188' is_underline='true'>`
  (idx 61543) — a custom XML-style link tag distinct from the stat-tag
  `<...>` pattern — the tag itself preserved exactly, only the visible
  link text ("Press here to go" → "Tekan di sini untuk pergi") translated,
  consistent with the `<TEXT id=...>...</TEXT>` handling seen at idx
  61276.
- Chinese zodiac double-hour "X Hour" → "Jam X" reconfirmed (idx 62712
  "Wei Hour" → "Jam Wei"), "Form" as a transformation state → "Wujud"
  reconfirmed (idx 61060, 62074, 61929 "Koi/Breeze form" → "Wujud Koi"/
  "Wujud Breeze").
- Honorifics per [[Honorifics-And-Titles]]: Master (teacher) → Guru (idx
  61190 "Master Wen", 61985, 62147, 62417, 62484, 62608, 62771), Master
  (org leader) → Ketua (idx 62154 "Sect Master", 62732 "Pavilion Master"),
  Lord → Tuan, Old X (no "Man") kept English (idx 61126 "Old Lin",
  reconfirmed per [[Open-Questions]]), Senior Brother/Sister kept English
  (idx 61452, 62944, reconfirmed per [[Open-Questions]]).
- A leftover-developer-note-style Chinese string (idx 61164,
  "不建议作为术语，这其实是个UI，翻译为了Preset") translated
  structurally into Indonesian per [[Special-Cases]], same as the
  precedent from Phase 9.
- Several long narrative/lore entries translated in full: the Buddhist
  candle-ritual monastery notice (idx 61028), the Zhuxie Gule/Bei Xiaoyu
  loyalty tale (idx 61550), the Meng Kuan wine-pouch elegy for fallen
  brothers (idx 61565), the "old madman" Imperial Guards camp story about
  Jing and Da (idx 61684, ~2,900 characters), the Mohist Hill imperial
  edict triptych (idx 62897, ~2,000 characters, three dated decrees), and
  the Liangzhou noblewoman/Aynur wine-vision passages (idx 62443, 62713).
- One source-side malformed tag found and preserved as-is rather than
  "corrected": idx 62806's `#YFledgling Appearance Chests#` is missing its
  closing `E` in the English source (bare `#`, not `#E`) — the first
  translation draft "fixed" it to a well-formed `#E`, which the validator
  caught as a token-count mismatch; corrected to reproduce the source's
  typo exactly, per the standing rule to never invent or repair tags that
  don't match the source.

Token/placeholder validation ran on the full 2,000-row batch (source-text
token-count diff against `unique_strings.jsonl`): found and fixed **1
issue** (idx 62806 above). Re-ran after the fix: **0 mismatches**. An
EMPTY check (source non-blank, translation blank) also ran: **0 hits**.
Full merged `locale/phase6.jsonl` (13,000 lines) re-validated: idx
sequential 50,000–62,999, 13,000 unique idx, no duplicates.

## idx 63,000–64,999 (batch 8)

Same mix as prior batches: a very high proportion of `freq: 2` random
player-username strings (kept verbatim per [[Special-Cases]]/
[[Names-Not-Translated]]), Chinese-style NPC names, gear/skill UI labels
(`Tier N <Set>: <Slot>`, `- EX`/`- Edge`/`- Radiance`/`- Umbra` variants),
stat labels, and casual dialogue (gue/lo per [[Tone-And-Style]]). No new
terminology decisions — every case matched an existing [[Glossary]] entry,
though a few required going back to the table rather than pattern-matching
on the surface word:

- **Honorifics applied correctly this batch, all per [[Honorifics-And-Titles]]**:
  Master (teacher, before a name) → Guru (idx 63062 "Master Moonstream" →
  "Guru Moonstream", initially mistranslated kept-English and caught on
  self-review before merging), Master (org/place leader) → Ketua (idx
  64238 "Master of Weiyang City" → "Ketua Weiyang City"), Young Master →
  Tuan Muda (idx 63611, 64290, 64666 — all three initially drafted
  kept-English by pattern-matching on other "Master" cases, caught and
  fixed before merging), Lord → Tuan (idx 64331 "Lord Zhu" → "Tuan Zhu"),
  Granny → Nenek (idx 63703 "Granny Turtle", 64020 "Granny Ren", 64080
  "Granny Yu" — all three initially left kept-English, caught on the same
  review pass), Sect Master → Ketua Sekte (idx 64579 "Lone Cloud Sect
  Master" → "Ketua Sekte Lone Cloud"), Old Man X → Kakek X (idx 64486
  "Old Man Ma" → "Kakek Ma", correct on first pass).
  - **"Young Master" as a standalone gameplay/role label** (idx 64820
    "Young Master gameplay not yet available") was judged to be a
    Profession/role name (same pattern as Scholar/Healer class names, see
    [[Kept-In-English-Terms]]) rather than an honorific address, and kept
    in English — different from the address form above. Flag for
    re-check if more instances of this specific label turn up.
  - **"Sect Rules - <Quest Name>" labels are translated ("Sect" →
    "Sekte")** even in the `X - Y` compound-label format (idx 63740 "Sect
    Rules - Hundred Herbs Trial" → "Aturan Sekte - Hundred Herbs Trial"),
    consistent with the existing "Sect Rules Violation Notice" precedent
    in [[Wuxia-And-Cultural-Terms]] — this is different from the
    kept-English skill/gear `X - Y` label convention in
    [[Kept-In-English-Terms]], because "Sect Rules" is ordinary
    translated vocabulary, not a proper noun or stat label.
- Several long narrative/lore entries translated in full: the Chai-family
  loyalty stele (idx 63034), the Guo Xin/Chen Hu "arrow that would rather
  break than bend" Anxi-army elegy (idx 63732, ~1,600 characters), the
  Zhu Youjia Nine Mortal Ways escape story (idx 63755), the Yan Ying
  chess-strategist tale (idx 63908), the age-reversal/disguise recipe
  (idx 64650), and the Deepwave Vessel/Conch Vessel legend (idx 64139).
- Duration/placeholder conventions reconfirmed with no exceptions this
  batch: `s`→`d`/`h`→`j` shorthand (idx 63433 `{diff_day:d}d{diff_hour:d}h`
  → `{diff_day:d}h{diff_hour:d}j`), `$link<...>^ID^$` opaque handling (idx
  64876), `$S...$E` narrative-wrapper handling with translated content
  (idx 64554, 64717), and `$D$F.../$STEADY_*$`-style pure numeric/runtime
  placeholders preserved character-for-character (idx 63419, 63569, 64128,
  64930, 63328, 63859, 63920 — all required escaping the `$` in the shell
  heredoc used to write the batch, since an unescaped `$` triggers bash
  variable expansion; verified post-write that every `$`-bearing line
  landed with the literal `$` intact).
- Two stat-tag content mistakes caught by validation and fixed before
  merging: idx 63835 (`<Infernal Twinblades'|...>`/`<Light Attacks|...>`
  had their apostrophe/plural dropped while restructuring the sentence
  around them) and idx 64825 (`<Everspring Umbrella's|...>` lost its
  possessive `'s`) — both are the same class of error as the batch 5/6/7
  stat-tag mistakes logged above: the content inside a `<Name|id|#C|...>`
  tag must be copied byte-for-byte from the source, never re-typed from
  memory while translating around it.

Token/placeholder validation ran on the full 2,000-row batch: found and
fixed **2 issues** (idx 63835, 64825 above), then **0 mismatches**. A
separate untagged-placeholder check (`$VAR$`/`$STEADY_*$`, `$P`/`$N`,
`@T[...]`/`@t[...]`, `$link<...>^ID^$`, `$S...$E`) also ran: **0 real
hits** (2 apparent hits on the `$S...$E` check were false positives from
the check script comparing translated content against source content
verbatim — the `$S`/`$E` markers themselves were confirmed intact). An
EMPTY check also ran: **0 hits**. A manual honorific-consistency grep
(`Master`, `Young Master`, `Lord `, `Granny `, `Sect `) caught the 6
mistranslations listed above before merging. Full merged
`locale/phase6.jsonl` (15,000 lines) re-validated: idx sequential
50,000–64,999, 15,000 unique idx, no duplicates.
