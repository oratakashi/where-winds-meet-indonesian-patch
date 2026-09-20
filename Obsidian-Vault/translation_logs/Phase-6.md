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
