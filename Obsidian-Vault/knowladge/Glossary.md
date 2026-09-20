# Glossary — Where Winds Meet Indonesian Patch

The canonical, current rule-set for translating `unique_strings.jsonl` →
`locale/phase*.jsonl`. When a rule here changes, search-and-replace across
the phase files to keep everything consistent (deduplication makes revision
cheap).

See also: [[Format-Spec]] (why format tokens must be preserved byte-for-byte),
[[Initial-Brief]] (the original request these rules implement), and the
`translation_logs/Phase-N` notes (the session-by-session reasoning behind
each decision below).

## Not translated (treated as "names")

- **Personal character/NPC names**: Huajian Ke, Jiang Wulang, Lian Daozi,
  Feng Jisheng, Han Xiangxun, Big Zhao, Little Fu, Ye Wanshan, Murong Yuan,
  Yi Dao, Qinghe, etc.
- **Place/location names**: Sixteen Lanes, Hutuo River, Mirkvale, Kaifeng,
  Eastwind Pavilion, Sunken City Lake, Unbound Cavern, Confinement Tower,
  Bandit Encampment, Great Song Prefecture Hall, Heavenfall, Skybrim Market,
  Harvestfall Village, Blissful Retreat, Wansheng Town, West Market,
  Forsaken Quarter, East City, etc.
- **Faction/organization names**: Aureate Pavilion, Bloodscale Hall, Jade
  Serpent Hall, Velvet Shade, Mercyheart Monastery, Ghost Revelry Hall,
  Sandstorm Tavern, Mohist Hill, Greenwood (bandits), NetEase, Raging Tides,
  etc.
- **Named weapons/styles/skills** (proper-noun abilities/weapons): Thundercry
  Blade, Strategic Sword, Inkwell Fan, Mortal Rope Dart, Infernal Twinblades,
  Noname Sword, Soulshade Umbrella, Panacea Fan, "Peak's Springless Silence",
  "Rodent Rampage", "Unwithering Bloom", "Inner Balance Strike III", "Sword
  Horizon", "Meridian Touch", etc. — skill names in quotes or inside
  `#Y...#E` tags stay in English.
- **Unique boss/entity names**: The Void King, Windchaser, Meow Meow, etc.

## Translated (not treated as "names")

- **Poetic quest/chapter titles** (not a person's name): "Death of the
  Governor", "The Homeward Vow", "Where the Heart Stirs", "Melodies of
  Peace", "Universal Harmony", "Karmic Reflections", "Glimmer Against the
  Dark", "Whispers Beneath the Moon", "Parting Ways", "Changeless Heart",
  "Lucky Seventeen" → translated poetically/casually.
- **Generic NPC role labels**: Villager, Bandit, Soldier, Guard, Scholar,
  Servant, Constable, Swordsman, Commoner, Official, Passerby, Player,
  Guest, Disciple, Member, Resident, Laborer, Attendant, Maid → translated
  (e.g. Villager → Warga/Penduduk Desa). A place name attached in front of
  the role stays in English (e.g. "East City Commoner" → "Warga East City",
  not a translated place name).
- **Item/lore/dialogue descriptions** (full sentences): translated fully
  into casual Indonesian, except for names and format tags within them.
- **Conversational dialogue** (first/second person, spoken register): uses
  gue/lo where natural.

## Kept in English (game/UI terms that read better that way)

Character stat/attribute labels are always kept fully in English (standard
convention for Indonesian-localized mobile/PC RPGs):
Critical Rate, Precision Rate, Affinity Rate, Physical Attack/Defense,
Max/Min HP, DMG Bonus/Reduction/Boost, HP Recovery/Bonus, Attack Bonus, DPS,
Silkbind/Bellstrike/Bamboocut/Stonesplit/Formless Attack (this game's
signature damage-type names), Tier, Stage, Lv, Healer.

Other common loanwords kept in English (per the original brief + Indonesian
gaming genre convention): Guild, Event, Login, Logout, Menu, Info, Reset,
Boss, Skill, Item, Quest, Level, Buff/Debuff, Chat.

## Wuxia genre-specific terms (locked during Phase 1)

- **"Sect" → "Sekte"** — standard wuxia term in Indonesian translation,
  TRANSLATED (not kept in English). Example: "Sect Rules Violation Notice"
  → "Pemberitahuan Pelanggaran Aturan Sekte".
- **"Wayfarer"/"Wanderer" → "Pengembara"** — used consistently for both
  English words (including "Jianghu Wanderer" → "Pengembara Jianghu").
- **"Young Master" → "Tuan Muda"** — standard wuxia honorific.
- **"Doctor" → "Tabib"** (not "Dokter") — fits the historical setting better.
- **Costume/gear set names** (Whirlsnow, Ebonward, Formbend, Flawless
  Guardian, etc.) and **named boats** (Painted Boat, Mirage Boat) are KEPT
  in English, same as weapon/skill names.
- **Native cultural festival names** (Spring Festival, Double Ninth
  Festival, etc.) are TRANSLATED into descriptive Indonesian (not treated as
  proper nouns): "Spring Festival" → "Festival Musim Semi", "Double Ninth
  Festival" → "Festival Sembilan Ganda".
- **TCM (Traditional Chinese Medicine) terms** in illness/medicine lore:
  general concepts (qi stagnation, damp-heat, etc.) are TRANSLATED
  descriptively ("stagnasi qi", "lembap-panas"), but **named diseases**
  (Shegong's Disease, Huhuo Syndrome, Wind-Damp Bi Syndrome) keep their core
  name (Shegong, Huhuo, Bi) while the generic word is translated ("Syndrome"
  → "Sindrom", "Disease" → "Penyakit").

## Terms locked during Phase 2 (idx 3000–3699)

- **"N-th Realm" (e.g. "1st Realm", "5th Realm") → "Realm N"** (e.g. "Realm
  1", "Realm 5") — a stat/tier progression pattern, kept in English like
  Tier/Stage/Lv.
- **"Rank N" stays in English** (e.g. "Rank 9") — same as "Level N"/"Lv.N".
- **`"<Skill Name> - <Skill Type> DMG Boost"` strings** (e.g. "Everspring
  Umbrella - Special Skill DMG Boost", "Unfettered Rope Dart - Charged
  Skill DMG Boost") are KEPT fully in English — this is a stat/upgrade
  label, not a narrative sentence.
- **Additional resource/meter names kept in English** (following the
  HP/Qi/Energy pattern): Inspiration, Affection, Exploration, Heaven's
  Will, Battle Will, Tenacity, Super Armor, Stagger, Exhaustion (Immunity),
  Bleed (e.g. "Fivefold Bleed").
- **Additional damage-type/stat names kept in English**: Thrust Damage,
  Deflection (Boost), Critical (DMG), Control Immunity, HP Drain, Formless
  Penetration, Endurance Recovery, Healing Boost, Energy Enhancement — same
  pattern as Silkbind/Bellstrike/Bamboocut/Stonesplit/Formless Attack.
- **Game modes/features kept in English as feature names** (not
  translated): Solo Mode, Co-op Mode, Endless - Solo/Duo/Quad, Arena, Sword
  Trial, Breakthrough, Bounty, Draw, Room (matchmaking context, e.g. "Add
  to Room").

## Terms locked during Phase 3 (idx 5000–6999)

- **IMPORTANT — a `<...>` tag WITHOUT the `|id|#C|n>` format** (a plain
  placeholder, e.g. `<Player Name 7 characters>`) must be kept EXACTLY in
  English, NOT translated. This differs from a stat-formatted tag like
  `<Stat Name|780|#C|15>`, where the stat name is naturally left in English
  anyway (because stat names aren't translated) — a plain tag like this is
  swallowed whole as one token by `qa_check.py`'s `TOKEN` regex
  (`<[^>]*>`), so translating its contents causes a MISMATCH (this happened
  at idx 6041, already fixed). Whenever a new `<...>` tag appears that
  isn't the standard stat pattern, DO NOT touch its contents at all.
- **Legendary/Epic weapon names** (Jadeware, Swallowcall, Rainwhisper,
  Cleftpeak, Mistwillow, Starweave, Etherwrath, Hawkwing, Ivorybloom,
  Mountainfall, Whirlwind, etc.) are KEPT in English — same pattern as
  locked costume-set names.
- **`"<Skill Name> - EX"`, `"<Skill Name> - Common"`, `"<Skill Name>: Common"`,
  `"<Skill Name>: Ultimate"`, `"<Skill Name> - Edge"`, `"<Skill Name> - Radiance"`**
  (skill icon/variant labels, e.g. "Infernal Twinblades - EX", "Heavenwill
  Gauntlets: Ultimate", "Inkwell Fan - Radiance") are KEPT fully in
  English, same pattern as DMG Boost.
- **Long stat-scaling strings** of the form `"Increases <Stat|id|#C|n>
  based on Agility. Current bonus: ... Maximum bonus requires ..."` — the
  opening sentence is translated ("Meningkatkan ... berdasarkan Agility.
  Bonus saat ini: ... Bonus maksimum membutuhkan..."), while the stat name
  and numbers inside the `<...>`/`#...#` tags stay untouched.
- **Hero titles in quotes**, in sentences like `Unlocks ... hero title
  "X"`, are KEPT in English (treated as a proper-noun achievement name),
  including versions prefixed with a color code (e.g.
  `#dee8d3Three Pillars of Power`). Note: idx 6211 "Best of the Best" was
  translated as "Yang Terbaik dari yang Terbaik" before this convention was
  established — left as-is (low impact, a single line), but any future
  occurrence of the same/similar title should follow the
  keep-titles-in-English rule.

## Terms locked during Phase 4 (idx 10000–10999)

- **Kinship/status terms preceding an NPC's name are TRANSLATED** (same
  pattern as "Young Master" → "Tuan Muda"): Aunt → Bibi, Uncle → Paman,
  Grandpa → Kakek, Granny → Nenek, Elder → Tetua, Master (martial-arts
  teacher) → Guru, Lady/Madam/Miss → Nyonya/Nona, Mr. → Tuan. Example:
  "Aunt Han" → "Bibi Han", "Elder Peng" → "Tetua Peng", "Master Wuhen" →
  "Guru Wuhen", "Grandpa Zhang" → "Kakek Zhang". The name itself is never
  translated.
- **"Master" as a leader-of-organization/faction title** (not a personal
  martial-arts teacher) → "Ketua" (e.g. "Bloodscale Hall Master" → "Ketua
  Bloodscale Hall"). "Vice Master" → "Wakil Ketua".
- **Seasons in effect/buff text (Spring/Summer/Autumn/Winter) are
  TRANSLATED** as Musim Semi/Musim Panas/Musim Gugur/Musim Dingin when used
  as a descriptive seasonal-effect label (not a titled feature name).
- **"Red Envelope" / lunar-new-year-style lucky gift → "Angpao"** (the
  common Indonesian term for this Chinese New Year tradition).
- Untagged placeholders (e.g. `$STEADY_MIN_PRO_ATK_C:.1f$`, `$P`, `$N`, or
  literals like `Xd, Xh`) are **not caught by `qa_check.py`'s TOKEN regex
  but must still be preserved exactly** — these are runtime substitution
  variables, not ordinary text.
- Reconfirmed: gear/weapon/skin names (the "- Valor/Radiance/Edge/Guard"
  pattern), new place names (Kaifeng Bathhouse, Crosswind Bazaar, Tubo
  Camp, etc.), and CC mechanic terms used as titled skill names (Taunt,
  Bind, Purify) stay in English, following the already-locked pattern.
- **"Lord" (a noble/official title before a name) → "Tuan"**, consistent
  with "Mr."/"Young Master" (e.g. "Lord Wang" → "Tuan Wang", "Lord Shi" →
  "Tuan Shi").
- **"Old Man X" → "Kakek X"** (same as Grandpa). **"Old X" without "Man"**
  (e.g. "Old Jin") is kept in English because it's ambiguous whether it's a
  formal nickname or a description — revisit if the same name recurs with a
  clear pattern. **"Old Caravan Master" → "Ketua Karavan Tua"** (here
  "Master" = organization leader).
- A `<...>` tag **without a closing `>`** (a truncated/typo'd source
  string, e.g. `"<Tangled Gauze...`) does appear in the source data —
  preserve the literal `<` at the start as-is; don't delete it or add a
  closing tag manually.
- **Duration formatted as `Nd` (N days, literal letter "d", not a token)**
  → `Nh` (e.g. "30d" → "30h"). This isn't a token checked by
  `qa_check.py`, so it's safe to translate — unlike `$VAR$`/`$P`/`$N`
  placeholders, which must be preserved exactly.
- **"Achievements" → "Pencapaian"** (translated, not kept in English) for
  the general UI category, as distinct from a specific quoted
  achievement/title, which stays in English.
- **"Healer" (a tank/DPS/healer trinity role)** is kept in English like
  Tank/DPS, DIFFERENT from "Doctor"/"tabib" used for traditional-medicine
  contexts/in-universe NPCs.
- **IMPORTANT — a `<...>` tag wrapping ONE ENTIRE LONG SENTENCE** (not the
  short `<Label|id|#C|n>` pattern) is treated by `qa_check.py`'s regex as
  ONE WHOLE TOKEN, because `<[^>]*>` matches from the first `<` to the next
  `>` — swallowing any `#color`/`#E` tags inside it as part of that same
  token, not as separate tokens. As a result, the contents of this kind of
  `<...>` (found at idx 12448, a time-frozen item notification) **must be
  kept 100% identical to the English source** — only text OUTSIDE the
  `<...>` is translated. This differs from a short stat tag whose contents
  happen to already be in English. Always run token validation after
  translating a paragraph containing a long `<...>` tag to catch this case.

## Terms locked during Phase 4 continuation (idx 18100–19099)

- **Short standalone poetic phrases (1–4 words, no numbering/quest
  structure)** functioning as an item/skin/mount/emote name (e.g. "Fleeting
  Dream", "Clear Glow", "Eternal Watch", "Night Glow", "Wildtrail", "Steady
  Ascent", "Spring's Bounty", "Winter's Bloom", "Oats in the Wind", "Paired
  Shadows", "Haven Astray", "Bright Sky") are KEPT in English, same pattern
  as costume/gear set names. A quest/chapter title with a clear structure
  (numbering, "Tales Retold:", "Volume", a "-" + role) is still TRANSLATED
  as usual.
- **A `<...>` tag wrapping dialogue/a long sentence** (not just a stat
  name) — e.g. non-human NPC speech at idx 18461 `<Pft. One round...>` — is
  kept 100% identical in English, including the dialogue content itself,
  following the long-tag-is-one-token rule locked at idx 12448.
- **Short highlight tags `#H...#E`/`#Y...#E` wrapping a SINGLE common
  instruction word** (Press, night, etc.) have their contents TRANSLATED
  (`#HPress#E` → `#HTekan#E`, `#Ynight#E` → `#Ymalam#E`) — different from a
  tag wrapping a skill/weapon name (kept in English, e.g. `#HFire
  Arrow#E`) or a long sentence (kept in English). `qa_check.py`'s `TOKEN`
  regex matches `#H`/`#Y`/`#E` as separate single-letter tokens, so the
  text between them is safe to translate.
- **Common-noun animals/plants** (not a made-up fantasy proper noun) such
  as "Pangolin", "Sparrow Egg", "Crane Egg", "Snow Ape", "Long-Tailed
  Pheasant", "Lanternfish", "Bamboo Shoot" are TRANSLATED into natural
  Indonesian terms — different from made-up fantasy gear/weapon names
  (Swallowcall, Jadesong, Voidchant, Darkecho, etc.) which stay in English.
  For a mixed compound (a fantasy name + a common word, e.g. "Peltwing
  Squirrel"), the fantasy part stays English and the common part is
  translated ("Tupai Peltwing").

## Terms locked at the close of Phase 4 (idx 19100–19999)

- **Generic profession tier/rank in `"Profession: Tier"` format** (e.g.
  "Healer: Novice", "Healer: Adept", "Healer: Redemption", "Scholar:
  Novice") — the tier word is KEPT in English (same pattern as
  Rank/Tier/Stage/Lv). If the tier is a clearly narrative adjective phrase
  (e.g. "Scholar: Refined Gentleman", "Scholar: Silver Tongue"), it IS
  translated, since it functions as a narrative title rather than a
  numeric tier.
- **Language names in the language-selection UI** (e.g. "Русский язык",
  "日本語", "Español（Latino）") are NOT translated — kept in their
  original script/language.
- **Combined format+literal duration placeholders** (e.g.
  `{diff_hour:d}h ago`) — the `{...}` part must be preserved exactly, while
  the literal suffix outside the braces is free to translate following the
  duration convention (`d`→`h` for days, `h`→`j` for hours).

## Terms locked during Phase 5 (idx 20000–20999)

- **Combined double-literal duration format `%sd%sh`** (days+hours, e.g.
  "Remaining: %sd%sh") → `%sh%sj` — same pattern as the `Nd`→`Nh` (days)
  and `h`→`j` (hours) conversions locked in Phase 4, applied together when
  the literal "d" and "h" appear adjacent in one string. `%s` itself is
  preserved exactly (it's a placeholder token).
- **"Loot" → "Jarahan"** (item drops), **"Griefing" kept in English**
  (a general gaming-community term with no standard equivalent),
  **"Constable" → "Konstabel"** (a common loanword for a historical-era
  security-guard title, consistent with "Guard"/"Soldier" being
  translated).
- **"Union" (a player social structure, distinct from "Guild")** is kept
  in English for now, since it isn't yet clear whether it's a synonym for
  Guild or a separate feature — revisit once this term appears more often
  with clearer context.
- **Named currencies/resources with no standard translation** (e.g.
  "Bookworms" as a resource name, not the common phrase "bookworm") are
  kept in English and capitalized, following the Inspiration/Affection/etc.
  pattern locked in Phase 2.

## Terms locked during Phase 5 continuation (idx 21000–21999)

- **"Enhancement" in skill-tree/talent node labels is KEPT fully in
  English** (e.g. "Momentum Enhancement", "Physical/Critical Resistance
  Enhancement", "Water Clone Enhancement", "Perfect Catch Enhancement",
  "Scroll & Script Enhancement", "Charge Calculation Enhancement", "Qi
  Struggle Enhancement") — same pattern as the already-English compound
  "DMG Boost"/"DMG Bonus"/"DMG Reduction". "X Boost" (e.g. "Advanced
  Defense Boost") follows the same rule.
- **"Appearance" as a cosmetic/skin UI category is TRANSLATED as
  "Tampilan"** (e.g. "Spear Appearance" → "Tampilan Spear") — a common
  word, not a proper noun. The attached generic weapon-type name (Spear,
  Blade, Gauntlets, etc.) stays in English.
- **Literal seconds duration (`Ns`) INSIDE a `#Y...#E` tag etc. is also
  converted to `Nd`** (e.g. `#Y70s#E` → `#Y70d#E`) — extending the seconds
  duration convention from Phase 4 to cases where the literal sits inside
  a color/highlight tag.
- **Non-standard color tags of the form `#<6-char-hex-ish>NNNN Word#E`**:
  `qa_check.py`'s `TOKEN` regex only matches the first 6 characters after
  `#` as the color token; the rest (trailing digits + word, e.g. "120
  Points") is free text safe to translate ("Points" → "Poin").
- **"Scholar"/"Healer" as a Profession class name** (not a generic NPC
  label) is reconfirmed kept in English (e.g. "Scholar Jiang Huaiyuan",
  "Scholar Class 72").
- Pigment/color names in quotes in paint-item lore (e.g. "vermilion",
  "Lychee", "white") are kept in English — an artisan naming convention,
  not ordinary narrative text.

## Terms locked during Phase 5 continuation (idx 29000–30999)

- **Compound minute+seconds duration placeholder `%dm%ds`** → `%dm%dd`, and
  `%sh` (hours) → `%sj` — extending the literal-duration conversion
  (`s`→`d` seconds, `h`→`j` hours) to formats combined with `%d`/`%s`
  placeholders. The letter `m` (minutes) is left as-is.
- **Runtime date placeholder `@T[...]`** (square brackets, e.g.
  `@T[month_2, day_6,type_noLocal;empty]`) is NOT caught by the `TOKEN`
  regex but MUST be preserved character-for-character — a date
  substitution variable, in the same class as the already-locked
  `$VAR$`/`$P`/`$N` placeholders.

## Terms locked during Phase 9 (idx 429887–431886, 2026-09-16 game update)

See [[Phase-9]] for the full session-by-session reasoning.

- **"Farmer" (a recruitable/deployable Homestead unit) is KEPT fully in
  English** (capitalized) — a Homestead system term like
  Retainer/Homestead, NOT translated as the common word "petani".
- **"Qiongqi Master"/"Qiongqi Warrior"/"Qiongqi Soldier"/"Qiongqi
  Artificer"** (ranks/roles of the antagonist "Qiongqi" faction) are KEPT
  fully in English as compound titles.
- **Generic royal/imperial titles (Prince, Empress) are TRANSLATED**:
  "Prince Teng" → "Pangeran Teng", "Empress Wu" → "Permaisuri Wu" —
  consistent with Lord → Tuan.
- **"Treasury" (a common word) is TRANSLATED as "Perbendaharaan"** in all
  its compounds (Pledged/Sealed/Imperial Treasury) — different from a
  unique compound place name, which stays in English.
- **"Lantern Festival" → "Festival Lampion"**, consistent with other
  festival names.

## Terms locked during Phase 9 continuation (idx 431887–433886, batch 2)

- **Homestead crafting materials/resources in `"<Name> Tier N"` form**
  (Magnet Stone, Cloud Sand, Turquoise Stone, Iron Ore, Cinnabar, and
  compound material names like "Mushroom-Iron Composite", "Pine-Copper
  Cloudsand Extract") are KEPT fully in English — treated as
  crafting/resource system identifiers like Inspiration/Affection, not
  ordinary common nouns.
- **IMPORTANT — the pattern `"#Y<label>#E <Tag|id|#C|slot>"`**: the
  highlight label and the placeholder tag are TWO separate units —
  translate the contents of `#Y...#E`, then leave the `<...>` tag intact
  right after it WITHOUT merging text into it. Never put text inside a
  `<...>` tag that already has the `|id|#C|slot` format.

## Terms locked during Phase 9 continuation (idx 437137–439136, batch 5)

- **"Mohist Sect" (the sect/order at Hidden Mountain, first appearing in
  this batch) is KEPT fully in English** as a proper-noun faction name —
  DIFFERENT from the common word "Sect", which is translated as "Sekte"
  (e.g. "Sect Rules"). Consistent with "Mohist Hill"/"Mohist City" already
  locked as untranslated place/faction names.
- **"Senior Sister"/"Junior Sister"/"Senior Brother"/"Junior Brother"**
  (wuxia terms of address between fellow disciples) are **kept in English
  for now** — no locked Indonesian equivalent yet (options like
  "Kakak/Adik Seperguruan" were considered but not decided); revisit if
  this term keeps appearing frequently.
- **Developer Lua source code that leaked into localization data** (found
  at idx 437267, a single string containing full code with English
  comments) is **left 100% untranslated** — never seen by players;
  partially translating it risks breaking variables/syntax.

## Terms locked during the Phase 5 closing + Phase 6 opening batch (idx 49000–50999)

- **"Pangolin" → "Trenggiling"** (reconfirming the old Phase 4 rule, which
  had been missed for the NPC "Pangolin Peddler" → "Pedagang
  Trenggiling").
- **Chinese zodiac double-hour ("X Hour"/"X hour") → "Jam X"** (e.g. "Hai
  Hour" → "Jam Hai", "Xu hour" → "Jam Xu") — the hour name (Hai, Xu, etc.)
  is kept in English/Pinyin as a traditional time term with no standard
  equivalent; only the word "Hour" is translated.
- **"Form" as a transformation state (Carp form, Wind form, Vulpine Form,
  Feline Form) → "Wujud"** (e.g. "Vulpine Form" → "Wujud Vulpine").
- **Lowercase generic "master"/"Master"** (not an honorific before a name,
  not an organization leader) is kept in English as the loanword
  "ahli/pengrajin" — different from "Master" as a name honorific (→
  "Guru") and "Master" as an organization leader (→ "Ketua").
- **The single-letter duration shorthand `s`→`d`/`h`→`j` also applies**
  when it's attached directly to a `{}` placeholder (e.g. `{}s`→`{}d`) and
  inside color tags.
- **`$link<text>^ID^$`** — a special link placeholder; everything between
  `$link` and `^ID^$` (including any English text inside it) MUST be
  preserved 100% identical, treated as one opaque unit.
- **`$S...$E`** — wraps a quote/narrative block in a skill description
  (different from `$D`/`$H`/`$F`, which are pure numeric values) — the
  content inside is still TRANSLATED normally; only the `$S`/`$E` markers
  themselves are preserved exactly.
- **"Wayfarer" (standing alone, not just "Wanderer") reconfirmed →
  "Pengembara"**.

## Terms locked during Phase 9 continuation (idx 442137–444136, batch 8)

- **Player character/username lists (the "Max-Level Characters" Hall of
  Fame)** are KEPT 100% UNTRANSLATED, copied verbatim including any Han
  characters embedded in nicknames (e.g. "Summer丶", "Worship灬") — this
  isn't narrative text, it's a collection of player names/usernames,
  consistent with the "names aren't translated" rule. Found as a single,
  massive JSONL entry (idx 444030) containing hundreds of names.
- **Lowercase generic "disciple" reconfirmed KEPT in English** throughout
  this batch — older precedent was mixed ("murid" vs. "disciple"), but
  since this batch sits close to the idx 432xxx range where "disciple"
  dominates, local consistency was prioritized. If it recurs in a much
  later batch, check the nearest idx precedent before deciding.
- **"Swordsman Sinan" reconfirmed as a NAME UNIT that is not translated**
  (not "Swordsman" translated + "Sinan" as a name) — used consistently as
  a combined title/name since early Phase 9.
- **The stray tag pattern `#Ttext` that happens to match the TOKEN regex
  as `#T`** (not a real highlight tag) appeared again (idx 444071,
  `"#Talk to the Dog"`) — same fix as idx 434808 previously: leave `#Talk`
  intact at the start, translate the rest (`#Talk dengan Anjing`).
- **"Chief of the Inner Court" → "Kepala Istana Dalam"**, **"Field
  Commander" → "Komandan Lapangan"** — generic titles/positions translated
  consistently with the Lord→Tuan/Commander→Komandan pattern.
- **Leftover Chinese developer notes in parentheses** (e.g. idx 443681
  `"(仅作为玩法名称)"` = "for gameplay-mode naming only") are translated
  structurally into Indonesian since they're ordinary developer notes —
  different from the long Lua code block that's left 100% untouched (the
  idx 437267 rule).
- Very long lore entries (>3000 characters) became more frequent in this
  idx range (the Zou/Yang "Together in One Boat" tale at idx 443599, the
  Su/Dragon King family drama at idx 444044, the Dragonbend Academy
  register at idx 443803, the Zhang siblings' story at idx 442985) — all
  translated in full without shortening, keeping only character/place
  names untranslated per the usual rule.

## Ambiguities not yet fully resolved (revisit if encountered again)

- **"Power"**: sometimes translated as "Kekuatan" (as a standalone common
  word), but when it appears as part of the "Five Attributes" list
  (alongside Constitution, Defense, Agility, Momentum) it should probably
  stay in English like the other stats. idx 282 already used "Kekuatan" —
  left as-is (low impact), but consider keeping English for new
  occurrences in a stat-list context.
- **"Trial"** → "Uji Coba" (used consistently for challenge-mode/dungeon
  trials) — note: a later Phase 5 precedent (idx ~35000+) started keeping
  `"Trial: <Name>"` titles fully in English instead; see
  [[Phase-5]] for the exact cutover point.
- **"Draw"** kept in English (a gacha feature), including "Draw Shop",
  "Draw Appearance".

## Tone

- Character dialogue (first/second-person conversation) → casual: gue/lo.
- UI/button/system labels (Loading, Confirm, Cancel, etc.) → neutral-casual,
  WITHOUT gue/lo.
- Format/placeholder tokens (`{0}`, `{}`, `%s`, `%d`, `#E`, `#aabbcc`, `#X`)
  MUST be preserved exactly, same count and order as the source
  (automatically checked by `tools/qa_check.py`).

## Process decisions

- Deduplicate first (translate unique strings), then expand to every row —
  approved by the user.
- Dedup-then-expand and the tone rules above were both explicit user
  decisions early in the project; see [[Initial-Brief]].

## Progress tracking (how these files relate)

- `unique_strings.jsonl` — 429,887 unique strings, ordered by descending
  frequency (idx 0..N-1), plus 31,817 more appended after the 2026-09
  update (idx up to 461,703 — see [[Phase-9]]).
- `locale/phase*.jsonl` — translation output, append-only,
  `{"idx": N, "v": "..."}` per line, one file per phase (see
  [[Phase-Roadmap]]). The last line in the active phase's file marks the
  most recently completed idx.
- Progress tracking itself lives in [[Current-Status]] (snapshot),
  [[Session-History]] (chronological log), and [[Resume-Procedure]] (how to
  pick a session back up).
- Final expansion (once enough phases are done): read `unique_strings.jsonl`
  + all `locale/phase*.jsonl` to build a `src → translated` dictionary, then
  stream the original `strings.jsonl`, replacing `v` per the dictionary,
  writing `strings.translated.jsonl` (ready for `qa_check.py`, then
  `wwm_locmap.py patch`) — see [[Resume-Procedure]].
