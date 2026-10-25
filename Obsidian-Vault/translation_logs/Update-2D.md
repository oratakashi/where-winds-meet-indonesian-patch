# Update-2D Translation Log (idx 534,257–535,918)

Strings new in the **game v2.3 hotfix** (2026-10-17), documented in [[Update-2]]. Output: `locale/update2d.jsonl` (created by the first translation session). Standard [[Quick-Reference]] conventions apply.

All 1,662 strings come from `translate_words_map_en_diff` — the hotfix left the base `translate_words_map_en` byte-identical — so their in-game effect shows up in the `_diff` coverage row of [[Current-Status]]. Remember the Steam caveat in `CLAUDE.md`: the `_diff` overlay only sticks on the official NetEase launcher.

## Batches

- **Session 149 (2026-10-18)** — idx 534,257–535,918 (all 1,662 rows) → `locale/update2d.jsonl`, translated in seven chunks (199/230/250/250/250/250/233 rows) by the primary agent only. Not committed yet. 1,407 changed / 255 retained. Overall after the session: 535,919 / 535,919 unique strings (100.00%), in-game coverage 100.00% (base) / 100.00% (`_diff`).

## Decisions & notes

- No new glossary rule; the batch is UI/event/quest text, Qingzhou "Opening Month" (Inkbound Order dormitory inspection, Gift of Gab card game, Crownleek/leek-farming quest chain), Phantom Bubble (Mystic Skill) descriptions, guild/prize-pool rules and a handful of long classics (Mencius, Odes of Zhou South) and short stories.
- Gift of Gab terms stay English: Gab Card, Quip Card, Maxim Card, Fluff Card, Hidden Channel, Inner Bastion, Path names (Bluster, Provocation, Rebuttal, Filibuster) and their tactic names, Presence, Play the Field. `<LTD> …` emote/card lines stay verbatim.
- Study Token, Literary Heart Point, Stash Point, Wind Sense, Dorm Inspector/Monitor/Inspection Log, Delight Page and similar new feature labels stay English; ordinary words around them are translated. "Prohibited Item(s)" → Barang Terlarang.
- Channel names inside the chat-display hint (`#Team Channel#`, `#all Mode Channels#`, …) stay English; the `#X…#` wrapper letters are counted by the QA token regex, so the first letter of a wrapped phrase must not change.
- `$D`/`$S…$E`/`$link…$`, `@T[...]`/`@t[...]` and `{}`/`{n}` placeholders preserved; `#x$y#` alternate-text wrappers (535279, 535354, 535462, 535727) keep their `$`/`#` markers while the words are translated.
- Dates → Indonesian (Jun/Jul/Agustus…), times kept as in the source, `s` → `d`, `h` → `j`, decimals → comma, thousands separators → dot.
- QA slips fixed before append: 534818 (`#E` count), 535462 (space in `# Chang'an$ …#`), 535804 (`#H…#E` pair).

## Place-name wiki audit & fix (2026-10-20)

- Audited all 133 Fandom wiki location names (see [[Place-Names-Wiki]]) against `unique_strings.jsonl` / `locale/*.jsonl`. 440 existing translations edited in place (idx unchanged, no new rows): Indonesian renderings (Taman Belakang, Istana Kekaisaran, Kota/Prefektur Kaifeng, Lapangan Latihan, Pasir Mengaum, ...) replaced by the wiki-spelled English name; surrounding words untouched.
- Also fixed idx 74855 ("Wayfarer - Jade Gate Pass"), whose translation was an unrelated sentence.
- Left on purpose: "Warga Harvestfall" (= Harvestfall Villagers, a different phrase).
- **Known source/translation misalignments, not fixed** (translation text unrelated to source): idx 74631, 74420, 74679 — need re-translation.
- QA after: 0 PROMPT_LEAK / MARKUP / EMPTY; `progress.py --check` OK; progress counts unchanged (all 100% in-game coverage).

## Standalone place-name rows (2026-10-25)

- Found while fixing CN names (the CN bridge and `term_lookup.py` copy global's Indonesian): 22
  place names not on the wiki list were translated in their **bare-name row** although 95–100% of
  the other rows containing them keep the English name. Restored to English (idx unchanged):
  5549 Millennium Crossing, 5868 Gasping Cliff, 8142 Dragonbend Mountain, 20553 Imperial Garden,
  5598 Meow Meow Temple, 7534 Nimbus Tower, 9797 Ascendant Peak, 462833 Another Tavern,
  10757 Camel Station, 321215 Tianjing Dock, 9511 Baima Station, 23980 Willow Bank,
  19434 Incomparable Tavern, 8404 Harmonic Span, 462227 Flower Boat Market,
  92282 Mechanism Warehouse, 42509 Skyreach Terrace, 33449 Verdant Hill Village, 19503 Lion Tomb,
  33089 Fragrant Riverbank, 17595 Vigilance Garden, 13159 Purity Tower.
- The same Indonesian forms inside 23 other rows (e.g. 88295 "Persimpangan Milenium", 405484
  "Gunung Dragonbend", 486376 "Kedai Lain") replaced the same way. 45 rows over 15 locale files.
- Open: a wider scan finds ~1,600 bare Title-Case rows translated while kept English elsewhere
  (Inkbound Order → "Perintah Terikat Tinta", Jianghu Legacy, Sleeping Daoist, Wandering Paths,
  Beyond the Chill, …). Many are names (Inner Way/quest/item), some legitimate translations —
  needs a reviewed pass, not done here.
