# Phase 4 Translation Log (idx 10,000–19,999)

Session-by-session terminology notes for Phase 4, in idx order. The current,
consolidated rules these sessions produced live in [[Glossary]] — this file
is the reasoning/history behind them.

## idx 14,100–15,099

- **Generic compound location names** (e.g. "Hutuo Region", "Kaifeng -
  Fairgrounds") are kept **fully in English** (not partially translated like
  the "East City Commoner" pattern) — consistent with "Hutuo River"/"Sunken
  City Lake", already locked as English.
- **"Sect Master" → "Ketua Sekte"** (a sect-leader title, same pattern as
  "Master" = organization leader → "Ketua").
- **Short combat-instruction verbs in control prompts** (Press/Hold) such as
  "Deflect"/"Defense"/"Guard" are TRANSLATED into short action verbs
  (Tangkis/Bertahan) when used as a button-instruction label — different
  from a titled skill/tag name, which stays in English.
- **"Red Packets" (synonym of "Red Envelope") → "Angpao"** as well,
  consistent.
- **Mixed day+hour duration "Nd Mh" format** (e.g. "9 d 7 h") → "Nh Mj"
  (e.g. "9 h 7 j") — "h" for days ("hari"), "j" for hours ("jam"),
  extending the already-locked "Nd"→"Nh" convention.
- **"Inner Way" (a collection feature, e.g. "Complete Inner Way Collection
  II")** is kept in English as a feature name, same pattern as Solo
  Mode/Co-op Mode etc.
- **"Main Story" as a chapter category label** (e.g. "Qinghe Main Story Boss
  Battle Space", "Hidden Mountain Main Story P2 - ...") is kept fully in
  English — treated as an internal quest-chapter label, not a narrative
  title to translate.
- A leftover Chinese string appeared (`"地图用-万古一人殿2层"`, idx 14,850) — an
  internal map label (Chinese leftover), translated structurally as "Untuk
  Peta - Wangu Yiren Hall Lantai 2" (the hall name transliterated as-is,
  following the untranslated-place-name pattern).

## idx 15,100–16,099

- **Duration written as literal seconds** (e.g. "18s", "6s") → abbreviated
  "d" (detik), e.g. "18s" → "18d", "Low Tenacity Mode - 6s" → "Mode
  Tenacity Rendah - 6d". This extends the day/hour duration convention
  (Nd→Nh, Nh→Nj) to seconds — applies **only** to plain literal text, never
  to a placeholder/format string (`%d`, `{}`, etc.), which is always
  preserved exactly.
- **A `<...>` tag without a closing `>`** (e.g. idx 15,273 "<Tangled Gauze
  by Martial Art Skill: Crane Wings extends by...") doesn't match
  `qa_check.py`'s `TOKEN` regex (which requires a closing `>`), so it's
  treated as ordinary text — the literal `<` at the start is preserved, the
  rest is translated normally. Different from a tag that DOES have a
  closing `>` (kept fully in English, already locked in a prior session).
- "Sect Master"/generic sect-leader titles etc. are already consistent with
  "Ketua Sekte". No other significant new convention in this batch —
  mostly NPC/gear/skill names following already-locked patterns.

## idx 18,100–19,099

- **Short standalone poetic phrases (1–4 words, no numbering/quest
  structure)** used as an item/skin/mount/emote name (e.g. "Fleeting
  Dream", "Clear Glow", "Eternal Watch", "Night Glow", "Wildtrail", "Steady
  Ascent", "Spring's Bounty", "Winter's Bloom", "Oats in the Wind", "Paired
  Shadows", "Haven Astray", "Bright Sky") are KEPT in English, consistent
  with the locked costume/gear-set naming pattern — different from a
  quest/chapter title with a clear structure (numbering, "Tales Retold:",
  "Volume", a "-" + role), which is still translated.
- **A `<...>` tag wrapping a long dialogue/sentence from a non-human NPC**
  (a cat, etc.), e.g. idx 18,461 `<Pft. One round. We still have three more
  chances...>` — follows the long-tag-is-one-token rule from a prior
  session: kept 100% identical in English, including the dialogue content
  itself.
- **Short highlight tags `#H...#E`/`#Y...#E` wrapping a SINGLE common
  instruction word** (e.g. "Press", "night") have their contents
  TRANSLATED (e.g. "#HPress#E" → "#HTekan#E", "#Ynight#E" → "#Ymalam#E") —
  different from a tag wrapping a skill/weapon name (e.g. "#HFire
  Arrow#E", kept in English) or a long sentence (kept in English).
  `qa_check.py`'s `TOKEN` regex matches `#H`/`#Y`/`#E` as separate
  single-letter tokens, so the text between them is safe to translate
  without causing a mismatch.
- "Main Story" as a category label is reconfirmed kept in English (e.g.
  "New Main Story" → "Main Story Baru"), consistent with the idx
  14,100–15,099 decision.
- **Common-name items/animals/plants** (not a made-up fantasy proper noun)
  such as "Pangolin", "Sparrow Egg", "Crane Egg", "Snow Ape", "Long-Tailed
  Pheasant", "Lanternfish", "Bamboo Shoot" are TRANSLATED into natural
  Indonesian terms, different from made-up fantasy gear/weapon names
  (Swallowcall, Jadesong, Voidchant, Darkecho, etc.), which stay in
  English. For a mixed compound (fantasy name + common word, e.g.
  "Peltwing Squirrel"), the fantasy part stays English and the common part
  is translated ("Tupai Peltwing").
- **"Old X" without "Man"** (e.g. "Old Shi") is still kept in English per
  the earlier decision (ambiguous nickname vs. description).
- A literal Python-style format placeholder like
  `{diff_hour:02d}h{diff_minute:02d}m{diff_second:02d}s` (idx 18,946) is
  **preserved 100% identical** — `qa_check.py`'s `TOKEN` regex matches each
  `{...}` as one token, so the entire string is automatically validated as
  long as it isn't touched at all.

## idx 19,100–19,999 (closing Phase 4)

- **Generic profession tier/rank in `"Profession: Tier"` format** (e.g.
  "Healer: Novice", "Healer: Adept", "Healer: Redemption", "Scholar:
  Novice") — the tier word (Novice, Adept, Redemption, etc.) is KEPT in
  English, same pattern as the already-locked Rank/Tier/Stage/Lv — only the
  generic profession name (Healer, Scholar) is also kept in English, since
  it's already a locked profession-name convention. If the tier is a
  clearly narrative adjective (e.g. "Scholar: Refined Gentleman", "Scholar:
  Silver Tongue"), it IS translated, since it's a narrative title rather
  than a numeric tier.
- A second leftover Chinese string was found: idx 19,719 `"瀑布下时装半透区域"`
  (a costume transparency area under a waterfall) — translated in full into
  Indonesian since it's an internal developer label with no name to
  preserve (different from idx 14,850, which had a specific hall name to
  transliterate).
- **Language names in the language-selection UI** (e.g. "Русский язык",
  "日本語", "Español（Latino）") are **not translated** — kept in their
  original script, since they represent the language itself, not narrative
  text. Standard practice for multi-language games.
- **Combined format+literal duration placeholders** like
  `{diff_day:d}d ago` or `{diff_hour:d}h ago` — the `{...}` (Python format)
  part must be preserved exactly, but the literal suffix outside the braces
  (`d ago`/`h ago`) is free to translate following the locked duration
  convention (`d`→`h` days, `h`→`j` hours), e.g. `{diff_hour:d}h ago` →
  `{diff_hour:d}j lalu`.
- **A short highlight tag wrapping a single adjective/label** (e.g.
  `#aee5aeAdvanced#E`) follows the earlier rule: its contents ARE
  translated (`#aee5aeAdvanced#E` → `#aee5aeLanjutan#E`), since
  `qa_check.py`'s `TOKEN` regex matches the hex color tag
  (`#[0-9a-fA-F]{6}`) and `#E` separately, not as one unit.
- Minor note: several common-noun/nickname pairs of Chinese-loanword style
  (e.g. "Old Pan", "Dog Three") are kept in English, since their status is
  ambiguous between a formal nickname and a description — consistent with
  the earlier "Old X" decision.
