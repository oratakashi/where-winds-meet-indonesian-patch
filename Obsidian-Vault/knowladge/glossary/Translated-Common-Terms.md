# Common Words & Text — Translated

Part of [[Glossary]]. Ordinary vocabulary and narrative text that **is** translated into
Indonesian, as opposed to the proper nouns in [[Names-Not-Translated]] and the
game/UI-convention loanwords in [[Kept-In-English-Terms]].

## Poetic quest/chapter titles

Titles that are not a person's name — "Death of the Governor", "The Homeward Vow", "Where
the Heart Stirs", "Melodies of Peace", "Universal Harmony", "Karmic Reflections", "Glimmer
Against the Dark", "Whispers Beneath the Moon", "Parting Ways", "Changeless Heart", "Lucky
Seventeen" — are translated poetically/casually. *(Phase 1)*

A quest/chapter title with a clear structure (numbering, "Tales Retold:", "Volume", a "-" +
role) is translated this way even when short; contrast with the short standalone poetic
item/skin *names* kept in English, see [[Names-Not-Translated]]. *(Phase 4 continuation,
idx 18100–19099)*

## Generic NPC role labels

Villager, Bandit, Soldier, Guard, Scholar, Servant, Constable, Swordsman, Commoner,
Official, Passerby, Player, Guest, Disciple, Member, Resident, Laborer, Attendant, Maid are
translated (e.g. Villager → Warga/Penduduk Desa). A place name attached in front of the role
stays in English (e.g. "East City Commoner" → "Warga East City", not a translated place
name). *(Phase 1)*

- **"Constable" → "Konstabel"** — a common loanword for a historical-era security-guard
  title, consistent with "Guard"/"Soldier" being translated. *(Phase 5, idx 20000–20999)*

## Item / lore / dialogue text

- **Item/lore/dialogue descriptions** (full sentences): translated fully into casual
  Indonesian, except for names and format tags within them.
- **Conversational dialogue** (first/second person, spoken register): uses gue/lo where
  natural — see [[Tone-And-Style]] for the full tone rules.

*(Phase 1)*

- Very long lore entries (>3000 characters) became more frequent from idx ~442000 onward
  (e.g. the Zou/Yang "Together in One Boat" tale, the Su/Dragon King family drama, the
  Dragonbend Academy register, the Zhang siblings' story) — all translated in full without
  shortening, keeping only character/place names untranslated per the usual rule. No special
  handling needed beyond the standard rules on this page. *([[Phase-9]], batch 8)*

## Single-term locks (ordinary translated vocabulary)

- **"Achievements" → "Pencapaian"** (translated, not kept in English) for the general UI
  category, as distinct from a specific quoted achievement/title, which stays in English
  (see [[Names-Not-Translated]]). *(Phase 4)*
- **"Appearance" as a cosmetic/skin UI category is translated as "Tampilan"** (e.g. "Spear
  Appearance" → "Tampilan Spear") — a common word, not a proper noun. The attached generic
  weapon-type name (Spear, Blade, Gauntlets, etc.) stays in English. *(Phase 5 continuation,
  idx 21000–21999)*
- **"Treasury" (a common word) is translated as "Perbendaharaan"** in all its compounds
  (Pledged/Sealed/Imperial Treasury) — different from a unique compound place name, which
  stays in English. *([[Phase-9]])*
- **"Loot" → "Jarahan"** (item drops). *(Phase 5, idx 20000–20999)*
- **Generic royal/imperial titles (Prince, Empress) are translated**: "Prince Teng" →
  "Pangeran Teng", "Empress Wu" → "Permaisuri Wu" — consistent with Lord → Tuan (see
  [[Honorifics-And-Titles]]). *([[Phase-9]])*
- **Common-noun animals/plants** (not a made-up fantasy proper noun) such as "Pangolin",
  "Sparrow Egg", "Crane Egg", "Snow Ape", "Long-Tailed Pheasant", "Lanternfish", "Bamboo
  Shoot" are translated into natural Indonesian terms — different from made-up fantasy
  gear/weapon names (Swallowcall, Jadesong, Voidchant, Darkecho, etc.), which stay in
  English (see [[Names-Not-Translated]]). For a mixed compound (a fantasy name + a common
  word, e.g. "Peltwing Squirrel"), the fantasy part stays English and the common part is
  translated ("Tupai Peltwing"). *(Phase 4 continuation, idx 18100–19099)*
  - **"Pangolin" → "Trenggiling"** reconfirmed (had been missed once for the NPC "Pangolin
    Peddler" → "Pedagang Trenggiling"). *(Phase 5 closing / Phase 6 opening, idx
    49000–50999)*
- **Leftover Chinese developer notes in parentheses** (e.g. idx 443681 `"(仅作为玩法名称)"` =
  "for gameplay-mode naming only") are translated structurally into Indonesian since they're
  ordinary developer notes — different from the long Lua code block left 100% untouched, see
  [[Special-Cases]]. *([[Phase-9]], batch 8)*

## Non-standard color tag free text

`qa_check.py`'s `TOKEN` regex only matches the first 6 characters after `#` as the color
token in tags of the form `#<6-char-hex-ish>NNNN Word#E`; the rest (trailing digits + word,
e.g. "120 Points") is free text safe to translate ("Points" → "Poin"). See
[[Placeholders-And-Formatting]] for the full tag-handling rules. *(Phase 5 continuation, idx
21000–21999)*

## See also

- [[Names-Not-Translated]] — proper nouns that look like ordinary words but are not
  translated.
- [[Honorifics-And-Titles]] — titles/kinship terms, a related but separate translated
  category.
- [[Open-Questions]] — "Power" (ambiguous between translated "Kekuatan" and kept-English
  stat-list usage).
