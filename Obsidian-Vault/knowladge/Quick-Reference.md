# Quick Reference — Read This First

Part of [[Glossary]]. A condensed cheat-sheet of the locked rules used most often, so a
session can read **one file** instead of opening all nine files in `knowladge/glossary/`.
This covers the large majority of cases. Open a specific topic file only when: the case
isn't covered here, you need the reasoning/history behind a rule, or the term is on the
"not yet locked" list at the bottom.

## 1. Never translate — copy verbatim

Character/NPC names, place names, faction/organization names, named weapons/skills/
gear-sets/boats, unique boss/entity names, hero titles in quotes (e.g. `hero title "X"`),
short poetic 1–4-word item/skin/mount/emote names (no numbering/quest structure), Homestead
crafting materials (`"<Name> Tier N"`), Qiongqi faction ranks, "Farmer" (Homestead unit,
capitalized), artisan pigment names in quotes, language names in the language-selector UI,
the player-username Hall of Fame list, leaked developer Lua code.

- **Exception inside an exception**: "Mohist Sect" (proper-noun faction) stays in English —
  different from the generic word "Sect", which IS translated (§5). "Swordsman Sinan" is one
  combined name-unit, not "Swordsman" + a name.

→ Full detail/history: [[Names-Not-Translated]], [[Special-Cases]]

## 2. Kept in English (not proper nouns, but convention)

- **Stat/attribute labels**: Critical/Precision/Affinity Rate, Physical Attack/Defense,
  Max/Min HP, DMG Bonus/Reduction/Boost, Tier, Stage, Lv, DPS, all signature damage-type
  names, Control Immunity, Healing Boost, etc.
- **"N-th Realm" → "Realm N"**, **"Rank N" stays English**, **`"Profession: Tier"` — the
  tier word stays English** (e.g. "Healer: Novice") unless it's a narrative adjective phrase
  (e.g. "Scholar: Refined Gentleman" — then translate it).
- **"Healer"/"Scholar" as a class/profession name** stays English (contrast: "Doctor" →
  "Tabib", §3). Lowercase generic **"disciple"** stays English (local-consistency call).
- **Loanwords**: Guild, Event, Login, Logout, Menu, Info, Reset, Boss, Skill, Item, Quest,
  Level, Buff/Debuff, Chat, Draw (gacha), Griefing, Union (unresolved, see §7).
- Lowercase generic **"master"/"Master"** (craftsman, not an honorific) → loanword, kept
  English (contrast: "Master" as teacher → Guru, as org leader → Ketua; see §3).
- **Resource/meter/currency names**: Inspiration, Affection, Exploration, Heaven's Will,
  Battle Will, Tenacity, Super Armor, Stagger, Exhaustion (Immunity), Bleed, and other
  named-with-no-translation resources (e.g. "Bookworms").
- **Skill/gear label formats** (the name itself is untranslated per §1, this is the
  attached label): `"<Skill> - <Type> DMG Boost"`, `"<Skill> - EX"`, `"<Skill>: Ultimate"`,
  `"<Skill> - Radiance/Edge/Valor/Guard"`, "Enhancement" in talent-node labels, "X Boost".
- **Game modes/features**: Solo/Co-op Mode, Endless - Solo/Duo/Quad, Arena, Sword Trial,
  Breakthrough, Bounty, Room (matchmaking).

→ Full detail/history: [[Kept-In-English-Terms]]

## 3. Honorifics & titles (translated, name untouched)

| English | Indonesian |
|---|---|
| Aunt | Bibi |
| Uncle | Paman |
| Grandpa / "Old Man X" | Kakek |
| Granny | Nenek |
| Elder | Tetua |
| Master (martial-arts teacher) | Guru |
| Master (org/faction leader, e.g. "X Hall Master") | Ketua |
| Lady / Madam / Miss | Nyonya / Nona |
| Mr. / Lord (noble/official) | Tuan |
| Young Master | Tuan Muda |
| Doctor (in-universe/TCM context) | Tabib |
| Wayfarer / Wanderer | Pengembara |
| Chief of the Inner Court | Kepala Istana Dalam |
| Field Commander | Komandan Lapangan |

"Old X" **without** "Man" (e.g. "Old Jin") stays English — ambiguous, unresolved (§7).
"Senior/Junior Sister/Brother" stays English — unresolved (§7).

→ Full detail/history: [[Honorifics-And-Titles]]

## 4. Translated common words

Generic NPC role labels (Villager, Bandit, Soldier, Guard, Scholar, Servant, Constable,
Swordsman, Commoner, Official, Passerby, Disciple(as a role label, not the standalone term
in §2), Member, Resident, Laborer, Attendant, Maid) — a place name attached in front stays
English (e.g. "East City Commoner" → "Warga East City"). All item/lore/dialogue text is
translated in full (including entries >3000 characters — no shortening), except names and
format tags within it.

Single-term locks: **Achievements→Pencapaian**, **Appearance→Tampilan** (cosmetic UI
category), **Treasury→Perbendaharaan**, **Loot→Jarahan**, generic royal titles (Prince→
Pangeran, Empress→Permaisuri). Common-noun animals/plants (Pangolin→Trenggiling, etc.) are
translated — contrast with made-up fantasy gear/weapon names, which stay English (§1); for a
mixed compound, translate only the common-word part. Leftover Chinese developer notes in
parentheses (ordinary notes, not code) are translated structurally.

→ Full detail/history: [[Translated-Common-Terms]]

## 5. Wuxia & cultural terms

- **"Sect" → "Sekte"** (translated) — except "Mohist Sect" (proper noun, stays English, §1).
- TCM **named diseases** keep their core name (e.g. Shegong, Huhuo, Bi); the generic word
  translates ("Syndrome"→"Sindrom", "Disease"→"Penyakit"). General TCM concepts translate
  descriptively.
- Native festivals translate descriptively: "Spring Festival"→"Festival Musim Semi",
  "Lantern Festival"→"Festival Lampion".
- **"Red Envelope" → "Angpao"**. Seasons in effect/buff text translate (Musim Semi/Panas/
  Gugur/Dingin). **"X Hour" → "Jam X"** (hour name itself, e.g. Hai/Xu, stays Pinyin).
  **"Form" (transformation state) → "Wujud"** (e.g. "Vulpine Form"→"Wujud Vulpine").
- **"Trial"**: early precedent → "Uji Coba"; later precedent (idx ~35000+) keeps
  `"Trial: <Name>"` titles fully in English. Check nearest idx precedent before deciding.

→ Full detail/history: [[Wuxia-And-Cultural-Terms]]

## 6. Placeholders & tokens — QA-critical, read before translating any `#`/`%`/`{`/`<`/`$`

Baseline: every token must appear the same number of times as in the source (`tools/qa_check.py`'s
`TOKEN` regex: `#[0-9a-fA-F]{6}|#[A-Za-z]|%[sd]|\{[^}]*\}|<[^>]*>` — hex is tried before the
1-letter code). The check counts tokens, not their order, so moving a token to fit Indonesian word
order is fine. Validate a batch with `python tools/qa_check.py --locale locale/phaseN.jsonl`.

- `<...>` tag rules (four distinct cases):
  1. Plain tag without `|id|#C|n>` format (e.g. `<Player Name 7 characters>`) — keep the
     whole thing in English untouched.
  2. Standard stat tag (`<Stat Name|780|#C|15>`) — stat name naturally stays English anyway.
  3. A `<...>` wrapping ONE ENTIRE long sentence/dialogue — the regex swallows it as one
     token (including any `#`/color tags inside); keep 100% identical, only translate text
     **outside** the tag.
  4. A `<...>` with no closing `>` (a truncated source string) — keep the literal `<` as-is.
  - `"#Y<label>#E <Tag|id|#C|slot>"` — two separate units: translate inside `#Y...#E`, leave
    the `<...>` tag intact right after, don't merge text into it.
- `#H...#E` / `#Y...#E` wrapping a single common instruction word (Press, night) —
  translate the contents. Wrapping a skill/weapon name — don't (§1). A stray `#T` that isn't
  a real tag (e.g. `#Talk to the Dog`) — leave `#Talk` intact, translate the rest.
- Non-standard color tag `#<6-char-hex-ish>NNNN Word#E` — only the first 6 chars after `#`
  are the token; trailing digits+word (e.g. "120 Points") is free text, translate it.
  **Watch for a hex digit glued to the word**: `#e9a35for better rewards#E` = color `#e9a35f` +
  "or better…" — the `f` belongs to the color and must stay (`#e9a35funtuk…`, not
  `#e9a35untuk…`; idx 54382 shipped with this bug until 2026-09-24).
- Duration conversions (literal text, not token-checked but must stay consistent):
  `Nd`(days)→`Nh`, `Ns`→`Nd`, `%sh`→`%sj`, `%dm%ds`→`%dm%dd` (minutes `m` untouched),
  `{}s`→`{}d` — same `s`→`d`(hari)/`h`→`j`(jam) shorthand wherever it appears, including
  inside color tags or attached to a `{}`/`%s` placeholder.
- Untagged placeholders, preserve exactly regardless: `$STEADY_MIN_...$`, `$P`, `$N`,
  `$link<text>^ID^$` (opaque, everything inside preserved as one unit), `$S...$E` (content
  inside IS translated, only the `$S`/`$E` markers stay), `@T[...]` (runtime date, preserve
  character-for-character).

→ Full detail/history: [[Placeholders-And-Formatting]]

## 7. Tone

Character dialogue (first/second-person) → casual **gue/lo**. UI/button/system labels →
neutral-casual, no gue/lo. Process decision (locked, don't revisit): dedupe unique strings
first, translate once, expand to every row later.

→ [[Tone-And-Style]]

## Not yet locked — check nearest idx precedent, don't invent a new rule

"Power" (Kekuatan vs. English in a stat-list context), "Trial" cutover point, "Draw"/"Union"
edge cases, "Senior/Junior Sister/Brother", "Old X" without "Man", lowercase "disciple" in a
new idx range.

→ [[Open-Questions]] — read this section before deciding a term that isn't in §1–7 above.
