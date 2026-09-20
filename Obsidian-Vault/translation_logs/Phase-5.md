# Phase 5 Translation Log (idx 20,000–49,999)

Session-by-session terminology notes for Phase 5, sorted by idx (the
original session notes were interleaved with Phase 9 batches worked in
parallel — see [[Session-History]] for the actual session order). The
current, consolidated rules live in [[Glossary]] — this file is the
reasoning/history behind them.

## idx 21,000–21,999

- **"Enhancement" on skill-tree/talent node labels** (e.g. "Momentum
  Enhancement", "Physical Resistance Enhancement", "Critical Resistance
  Enhancement", "Water Clone Enhancement", "Perfect Catch Enhancement",
  "Scroll & Script Enhancement", "Charge Calculation Enhancement", "Qi
  Struggle Enhancement") is KEPT fully in English — same pattern as the
  already-locked "DMG Boost"/"DMG Bonus"/"DMG Reduction" compound
  stat-labels. Same for "Advanced Defense Boost" ("X Boost" pattern).
- **"Appearance" as a cosmetic/skin UI category is TRANSLATED as
  "Tampilan"** (e.g. "Spear Appearance" → "Tampilan Spear", "Mystic Skill
  Appearance" → "Tampilan Mystic Skill") — a common word, not a proper
  noun. The attached generic weapon-type name (Spear, Blade, Gauntlets,
  etc.) stays in English following the locked weapon-class naming pattern.
- **Literal seconds duration (`Ns`) inside a `#Y...#E`/`#G...#E` tag is
  also converted** to `Nd` (e.g. `#Y70s#E` → `#Y70d#E`, `#Y20s#E` →
  `#Y20d#E`, `0.6s` → `0.6d`) — extending the seconds-duration convention
  from Phase 4 to cases where the literal sits inside a color/highlight
  tag, not just in plain text.
- **Non-standard color tags of the form `#<hex-ish>NNNN Word#E`** (e.g.
  "Increase to #8C5823120 Points#E of #406182Max Bellstrike Attack#E") —
  `qa_check.py`'s `TOKEN` regex matches only the first 6 characters after
  `#` as the color token (`#[0-9a-fA-F]{6}`); the rest (trailing digits +
  word) is free text safe to translate. Example: `#8C5823120 Points#E` →
  token `#8C5823` + free text `120 Poin` + token `#E`. "Points" → "Poin"
  confirmed translated under this pattern.
- **"Union"** (noted as still ambiguous in an earlier session) appeared
  again (e.g. "Healers' Union", "Artificer Union", "Union elections",
  "Union Bonus Draw") — still kept in English, consistent with the earlier
  decision; a player-organization feature similar to Guild.
- **"Scholar"/"Healer" as a Profession class name** (not a generic NPC
  label) reconfirmed kept in English (e.g. "Scholar Jiang Huaiyuan",
  "Scholar Class 72") — different from a descriptive generic NPC role
  (Villager, Bandit, etc.), which is translated; here "Scholar" functions
  as a class/profession name like the already-locked "Healer".
- **Pigment/color names in quotes in paint-item lore text** (e.g. "Crab
  Shell", "vermilion", "Lychee", "white") are kept in English within
  quotes — an artisan color-naming convention, not ordinary narrative
  text, consistent across both occurrences (idx 21,301 and 21,940).
- A few ambiguous terms are left in English pending clearer context
  (consistent with the "Old X"/"Union" pattern): "Loan Cloud" (idx 21,551,
  likely a source typo for "Lone Cloud" — kept literal as in the source,
  NOT corrected), "Retainer" (the Homestead companion-NPC system), "Static
  Atmosphere Group" (an internal label).

## idx 22,000–22,999

- **Internal combat animation/technique labels in the pattern `"<Weapon/
  Style> - <Move Name>"`** (e.g. "Spear Red Spirit - Normal Attack Combo 2
  - Sweep", "Chicken Simulator - Goose Heavy Attack 1 - PvE", "Assist Tang
  Blade - Heavy Attack Chain Strikes", "Qianye Showdown - Backward Dodge",
  "Gauntlets - Punch Slam", "Silkbind - Deluge: Default") are KEPT fully in
  English — locked as a continuation of the already-English weapon/attack
  naming pattern; these are dev-facing animation/hitbox labels, not
  player-facing narrative text. Consistent with "Silkbind - Deluge" from an
  earlier phase. Exception: an explicit instruction verb in front (e.g.
  "Use Skill Theft - Thundercry Blade") has that verb translated ("Use" →
  "Gunakan"), while the skill/animation name itself stays in English.
- **"Player"/"player" reconfirmed ALWAYS kept in English** (not "Pemain"),
  including as a generic count ("0 Player") — verified via a grep across
  every prior `locale/phase*.jsonl` file, with every Phase 0–4 precedent
  consistently using "player"/"Player" as-is. This overrides a literal
  reading of the glossary that would otherwise group "Player" with
  translated generic NPC roles — in practice this term is almost always
  used as a mechanical/technical term, not an in-world NPC label, so the de
  facto convention is to keep it in English.
- **"Melodies of Peace" (a chapter title) is TRANSLATED as "Melodi
  Kedamaian"**, including in a combined form like "Kaifeng - Melodi
  Kedamaian" — following the more recent precedent from the end of Phase 4
  (idx 18,999), not the older Phase 3 precedent that kept it in English in
  combined form (idx 5,178, 6,352, etc. — already done, left as-is, not
  retrofitted).
- **A literal `Ns` duration inside an ordinary narrative sentence is NOT
  converted to "Nd"** when the word "seconds" is spelled out in full
  (rather than the single-letter shorthand "s") — the `Ns`→`Nd` conversion
  only applies to short literal shorthand (e.g. "18s", "#Y70s#E"), not to a
  sentence that already spells out "seconds" (translated normally as
  "detik").
- **"Union" (a player social structure) stays in English** (e.g.
  "Scholars' Union", "Total Weekly Union Contribution") — still consistent
  with the ambiguity noted in the previous Phase 5 session.
- **Internal card-game names** (Number Card, Wild Card, Big Landlord,
  Little Landlord) are kept in English as specific card-game rule
  terminology.

## idx 23,000–24,999

- **Batch size raised to 2,000 strings/session** starting this session
  (previously 1,000 — see [[Session-History]]) — handled as one large batch
  in a single session with no validation issues, becoming the new default
  until further notice.
- **"Sword Trial" reconfirmed as a mode name kept in English** (used
  hundreds of times in this batch as part of "Defeat #YBoss#E in Sword
  Trial..."), consistent with the already-locked "Hero's Realm".
- **"Cultivation"** (an internal wuxia training term) is kept in English as
  a loanword when used as a system label (e.g. "Cultivation Instructions" →
  "Instruksi Cultivation") — no locked equivalent yet, following the
  Qi/Energy pattern of staying in English.
- **Internal combat/animation labels in the `"<Weapon> - <Action>"`
  pattern** (e.g. "Gauntlets - Leg Slam", "Assist Dual Blades - Right
  Dodge", "Spear Red Spirit - Ground Attack") reconfirmed kept fully in
  English, appearing very often in this batch — the dev-facing pattern
  locked in the previous Phase 5 session.
- **"Player" reconfirmed kept in English** in every context (not
  "Pemain"), including "The player canceled the invitation." → "Player
  membatalkan undangan."
- **"My lord" as a form of address** (from an NPC to the player character,
  not referring to a specific noble) is translated "Tuan"/"Tuanku",
  consistent with the locked Lord→Tuan pattern.
- No other major new convention in this batch — most NPC/gear/skill/combat
  labels follow patterns already locked in prior Phase 4/5 sessions.

## idx 25,000–26,999

- **New stat-node suffix "X Optimization"** (e.g. "Shadow Optimization") is
  KEPT fully in English, following the "X Enhancement"/"X Boost" pattern —
  a continuation of the locked skill-tree/talent label convention.
- **The tag `#YSpecial Skill "..."#E`** (a specific skill name in quotes,
  wrapped in a highlight tag) — the entire `#Y...#E` tag MUST be preserved
  exactly; don't translate the surrounding sentence while dropping the
  wrapping tag (this happened at idx 26,388 in this session, fixed before
  validation — two occurrences of "Special Skill" in the same string must
  each be checked individually, not just the first one).
- "Union"/"Player"/"Enhancement"/"Sword Trial"/"Cultivation" and the
  `"<Weapon> - <Action>"` combat-label pattern are reconfirmed consistent
  with earlier Phase 5 sessions — no new convention changes.

## idx 27,000–28,999

- **"Feast Moment" (a boat/gathering event feature) is kept in English** as
  a feature name, used repeatedly in this batch (e.g. "Feast Moment
  remaining time", "Extend Feast Moment Duration") — same pattern as
  Painted Boat/Solo Mode/Co-op Mode.
- **"Farmland"/"Farming" (the general farming system, not a titled feature
  name) is TRANSLATED** as "Lahan Pertanian"/"Bertani" — a common word,
  different from Homestead, which stays in English as a feature/location
  name.
- **A short `<...>` tag wrapping a sound-effect description** (e.g. idx
  27,138 `<sound of rock collapsing>`) is kept 100% in English, following
  the already-locked single-token-tag rule — this was mistakenly
  translated in an early draft, fixed before validation.
- **A `#Y...#E` tag wrapping two separate phrases in one long string** (e.g.
  idx 27,130, containing both `#YSpecial Skill "..."#E` AND `#Yhigh Bleed
  damage#E` in the same paragraph) — each occurrence must be checked
  individually, not just the first; one tag was dropped during translation
  here, same failure mode as idx 26,388 in the previous session. This
  repeated pattern is a standing reminder: a long paragraph with more than
  one highlight tag needs a tag-by-tag check, not a single skim.
- "Union"/"Player"/"Enhancement"/"Sword Trial"/"Cultivation"/"Retainer" and
  the `"<Weapon> - <Action>"` combat-label pattern / gear names like
  Swallowcall reconfirmed consistent with earlier Phase 5 sessions — no new
  convention changes.

## idx 29,000–30,999

- **Compound minute+seconds duration placeholder `%dm%ds`** →
  `%dm%dd` — extending the literal-duration conversion (seconds `s`→`d`
  from Phase 4/5) to a combined minutes+seconds format; the letter `m`
  (minutes) is left as-is since it happens to match in both languages,
  only `s`→`d` changes. The same pattern applies to `%sh`→`%sj` (hours).
- **A `<...>` tag wrapping AN ENTIRE LONG PARAGRAPH with a color tag
  inside it** (e.g. idx 30,170, a New Year greeting letter where the whole
  content — including `#8c5823...#E` — is wrapped in one `<...>` tag from
  start to finish) — `qa_check.py`'s `TOKEN` regex swallows the whole block
  as ONE token, so its contents must be 100% identical to the English
  source; this was fully translated in an early draft before the mistake
  was caught, fixed before final validation. Consistent with the
  long-tag-is-one-token rule locked since idx 12,448 (Phase 4) — reminder:
  if a `<...>` tag spans more than one sentence, DON'T touch its contents
  at all, regardless of any color tags inside it.
- **A long string with many `@T[...]`-style placeholders** (e.g. idx
  30,393, a Guild War League description with many dates like
  `@T[month_2, day_6,type_noLocal;empty]`) — `@T[...]` uses square brackets
  and is NOT caught by the `TOKEN` regex (which only checks `#`, `%s`,
  `%d`, `{}`, `<>`), but must still be preserved character-for-character
  since it's a runtime date-substitution variable — same class as the
  already-locked `$VAR$`/`$P`/`$N` placeholders.
- "Union"/"Player"/"Enhancement"/"Sword Trial"/"Cultivation"/"Retainer" and
  gear names (Swallowcall, Veilbright, Nightstar, etc.) reconfirmed
  consistent with earlier Phase 5 sessions.

## idx 31,000–32,999

- **"Wanderer" reconfirmed consistently translated as "Pengembara"** (e.g.
  "Young Wanderer" → "Pengembara Muda"), per the locked [[Glossary]] rule —
  appeared often in this batch and applied consistently (different from
  "Player", which always stays in English).
- **Shorthand duration placeholders `Ns`/`Nh` inside sentences/tags keep
  converting** `s`→`d` (seconds) and `h`→`j` (hours) per the locked
  convention, including when combined with a literal minutes `m` (e.g.
  "7h57m Remaining" → "7j57m Tersisa", "%sm%ss" → "%sm%sd").
- **Lowercase runtime date/time placeholder `@t[...]` (square brackets)** —
  a new variant of the already-locked `@T[...]` from the previous session
  — MUST be preserved exactly; not caught by the `TOKEN` regex.
- New feature/mode/system names kept in English: Feast Moment, Cultivation,
  Sword Trial, Union, Path (skill-tree path), Plan (a build/loadout plan —
  translated as "Rencana" since it's a common word, not a titled feature
  name), the `"<Weapon> - <Action>"` combat-animation label pattern, and
  long gear/costume names (Swallowcall, Starweave, etc.) — all consistent
  with earlier Phase 5 sessions.
- **"Loot" reconfirmed "Jarahan"/"Jarah"** (noun/verb) per the Phase 5 idx
  20,000–20,999 decision, used in "Loot & Extract" → "Jarah & Ekstraksi".
- **"Sect Shop" → "Toko Sekte"** ("Sect" translated consistently with the
  locked wuxia term).

## idx 33,000–34,999

- **A plain `<...>` tag (without the `|id|#C|n>` format) wrapping a short
  non-dialogue action/sound description** (e.g. idx 34,071 `<Nods.>`, idx
  34,671 `<ears drooping> Mmnh...`) was **mistakenly translated twice
  before validation** — a firm reminder: the plain-tag-stays-in-English
  rule (locked since Phase 3, idx 6,041) also applies to short
  action/sound descriptions, not just name placeholders. `qa_check.py`'s
  `TOKEN` regex swallows the whole `<...>` as one token, so its contents
  MUST be 100% identical to the source unless it's in the `|id|#C|n>`
  format.
- **"Steward" → "Pelayan"** (a household head-servant/manager), **"Squire"
  → "Pengawal"** (a knight's young attendant) — both generic descriptive
  titles, translated consistently with the generic-NPC-role pattern.
- **"Passerby" (a generic NPC role) is TRANSLATED as "Orang Lewat"** — it
  had appeared before without an explicit decision; now locked following
  the generic-role list (Villager/Bandit/etc.) in [[Glossary]].
- **"Cold Loading" (a technical dev/asset-loading term) is kept in
  English** — not a gameplay term, likely a debug/internal label.
- **Literal `/h` (per hour) outside a color tag is converted to `/j`** (e.g.
  `0#85d67c + 0#E/h` → `0#85d67c + 0#E/j`) — consistent with the locked
  `h`→`j` convention, applying even when the literal is attached directly
  after a color/highlight token.
- "Wanderer"/"Union"/"Retainer"/"Cultivation"/"Sword Trial" and the
  `"<Weapon> - <Action>"` combat-label pattern / long gear names
  reconfirmed consistent with earlier Phase 5 sessions.

## idx 35,000–36,999

- **Companion nicknames in the pattern "Kitty: X" / "Doggy: X" are KEPT
  fully in English** (e.g. "Kitty: Youngest", "Kitty: Ebony", "Doggy: Deng
  Deng", "Doggy: Tipsy") — new decision this session: treated as a
  companion nickname/name (like a character name), NOT translated as a
  common adjective/noun, even though some suffixes are ordinary English
  words (Youngest, Ball, Drifter, etc.). Applied consistently across dozens
  of occurrences in this batch — review again if a conflicting older
  precedent turns up later (none found during this session's audit).
- A third leftover Chinese string: idx 36,975 `"倒计时测试徽章"` (an internal
  developer label, "Countdown Test Badge") translated structurally as
  "Lencana Uji Coba Hitung Mundur" — consistent with the idx 14,850/19,719
  pattern (a Chinese leftover = a dev label, fully translated since there's
  no name to transliterate).
- "Union"/"Player"/"Enhancement"/"Sword Trial"/"Cultivation"/"Retainer"/
  "Wanderer" and the `"<Weapon> - <Action>"` combat-label pattern / long
  gear names reconfirmed consistent with earlier Phase 5 sessions.

## idx 37,000–38,999

- **Internal Homestead building/furniture components** (e.g. "Red Roof
  Surface", "Red Roof Outer Corner", "Rainbow Resort
  Partition/Gable Wall/Beam/Double Door", "Greenwood Mansion
  Partition/Bracket/Railing Pillar Head/Arch Bridge/Indoor Stairs", "Bamboo
  Leaf Screen 5/6", "Diamond-Patterned Screen") are RECONFIRMED KEPT fully
  in English — treated as internal component identifiers for
  buildings/furniture, consistent with earlier precedent ("Roof Outer
  Corner" idx 32,086, "Greenwood Mansion Long Railing I" idx 36,373), not
  translated as a general description.
- **`"Trial: <Name>"` titles** (e.g. "Trial: Gatekeeper's Stand", "Trial:
  Unbounded World", "Trial: Malefic Stars", "Trial: Anchor the Ark") are
  RECONFIRMED KEPT fully in English, including the "Trial:" prefix (NOT
  translated as "Uji Coba:") — following the MOST RECENT precedent (idx
  35,748 "Trial: Fleeting Trace", idx 36,072 "Trial: Master of Spectacles",
  idx 36,739 "Trial: Blades in Question", all shortly before this batch),
  overriding an older precedent that had translated it as "Uji Coba: ..." —
  dungeon/trial titles have been treated as proper nouns since roughly idx
  35,000. **This is a cutover point from the older [[Glossary]] "Trial" →
  "Uji Coba" rule** for titled trial names specifically.
- A fourth leftover Chinese string: idx 37,377 `"地图用-万古一人殿4层"` (an
  internal map label, structurally identical to idx 14,850 but floor 4
  instead of floor 2) translated as "Untuk Peta - Wangu Yiren Hall Lantai
  4" — the hall name transliterated as-is, consistent with the idx
  14,850/19,719/36,975 pattern. This one was initially missed (left in
  Chinese) in the first draft, caught and fixed via manual audit rather
  than token validation (a plain CJK string with no tags doesn't trigger a
  `TOKEN` mismatch).
- "Union"/"Player"/"Enhancement"/"Sword Trial"/"Cultivation"/"Retainer"/
  "Wanderer" and the `"<Weapon> - <Action>"` combat-label pattern / long
  gear names reconfirmed consistent with earlier Phase 5 sessions.

## idx 41,000–42,999 (batch 6)

- **"Refine"/"Draft"/"Interface"/"Melee"/"AoE"/"Pet"/"Dodge"/"Attuning"/
  "Cultivator"** and similar generic system/UI terms are kept in English as
  common gaming loanwords, consistent with the already-locked Guild/Event/
  Login/Menu/Skill/Item/Quest.
- **"Bustard"/"Black Brant"/"Brant"** (rare bird names with no standard
  Indonesian equivalent) stay in English, consistent with earlier Phase 5
  decisions. **"Jackal" is TRANSLATED as "Jakal"** (a common loanword,
  different from bird names that have no equivalent at all).
- **Location/decor labels in the pattern `"<Set/Location Name>
  <Component Description>"`** (e.g. "Rainbow Resort Small Roof Inner
  Corner", "Greenwood Mansion Board Wall", "Chai Mansion - Secret
  Observation") — the set/location name (Rainbow Resort, Greenwood
  Mansion, etc.) stays in English like other place names, but the
  component description (Small Roof Inner Corner, Board Wall, etc.) IS
  translated as an ordinary common noun — different from the
  `"<Weapon> - <Action>"` combat-label pattern, which is kept fully in
  English.
- **"Hongbao" (a Chinese transliteration for a lucky-money gift) is
  TRANSLATED as "Angpao"**, consistent with the already-locked "Red
  Packets"/"Red Envelope" → "Angpao" from Phase 4.
- A leftover Chinese string appeared again: idx 42,741 `"待确认文本"` (a dev
  label meaning "text pending confirmation") — translated structurally into
  Indonesian, same pattern as earlier Chinese leftovers (an internal label,
  not a name to transliterate).
- "Union"/"Player"/"Cultivation"/"Sword Trial"/"Retainer"/"Enhancement"/
  "Dispatch" and the `"<Weapon> - <Action>"` combat-label pattern / long
  gear names (Swallowcall, Etherwrath, Hawkwing, Starweave, etc.)
  reconfirmed consistent with earlier Phase 5 sessions.
