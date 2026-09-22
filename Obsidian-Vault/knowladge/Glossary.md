# Glossary — Where Winds Meet Indonesian Patch

The canonical, current rule-set for translating `unique_strings.jsonl` →
`locale/phase*.jsonl`, split by topic into `knowladge/glossary/`. When a rule changes,
search-and-replace across the phase files to keep everything consistent (deduplication
makes revision cheap).

See also: [[Format-Spec]] (why format tokens must be preserved byte-for-byte),
[[Initial-Brief]] (the original request these rules implement), and the
`translation_logs/Phase-N` notes (the session-by-session reasoning behind each decision
below — linked from the relevant rule in each topic file).

**For a routine translation session, read [[Quick-Reference]] instead of the topic files
below** — it condenses the rules used most often into one file. Only open a specific topic
file when Quick-Reference doesn't cover the case, you need the reasoning/history behind a
rule, or the term is on the "not yet locked" list.

## Topic files

- [[Names-Not-Translated]] — proper nouns copied verbatim: character/NPC names, places,
  factions, named weapons/skills/gear-sets/boats, unique bosses, legendary items, hero
  titles, Homestead materials.
- [[Kept-In-English-Terms]] — game/UI vocabulary kept in English by convention: stat
  labels, common loanwords (Guild, Event, Boss...), resource/meter names, skill-label
  formatting conventions, game modes.
- [[Translated-Common-Terms]] — ordinary words and narrative text that IS translated:
  quest/chapter titles, generic NPC roles, item/lore/dialogue text, and a set of
  single-term locks (Achievements, Treasury, Loot, royal titles, common animals/plants).
- [[Honorifics-And-Titles]] — kinship/status terms before a name (Aunt→Bibi, Young
  Master→Tuan Muda, ...), "Master" as teacher vs. org-leader vs. loanword, Doctor→Tabib,
  Wayfarer→Pengembara.
- [[Wuxia-And-Cultural-Terms]] — Sect→Sekte (and the Mohist Sect exception), TCM terms,
  native festivals, zodiac hours, the "Trial" cutover.
- [[Placeholders-And-Formatting]] — every QA-critical mechanical rule: format-token
  preservation, `<...>`/`#...#` tag gotchas, duration conversions, untagged runtime
  placeholders. Read this before translating anything with `#`, `%`, `{`, `<`, or `$`.
- [[Special-Cases]] — edge-case source content: the player-username Hall of Fame list, a
  leaked Lua code block, leftover Chinese developer notes, very long lore entries.
- [[Tone-And-Style]] — dialogue vs. UI tone (gue/lo), and the dedup-then-expand process
  decision.
- [[Open-Questions]] — terms without a fully locked rule yet (Power, Trial, Union, Senior/
  Junior Sister/Brother, "Old X"). Check here before inventing a new convention.

## Progress tracking

The `unique_strings.jsonl` / `locale/phase*.jsonl` file-relationship map and the final
expansion procedure now live in [[Resume-Procedure]] (it's a workflow/pipeline description,
not a terminology rule).
