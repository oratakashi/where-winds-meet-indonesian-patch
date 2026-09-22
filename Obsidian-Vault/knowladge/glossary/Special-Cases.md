# Special Cases & Edge-Case Content

Part of [[Glossary]]. Unusual source content that needs judgment-call handling rather than a
simple term lookup.

## Player username lists (Hall of Fame)

The "Max-Level Characters" Hall of Fame entry is kept 100% untranslated, copied verbatim
including any Han characters embedded in nicknames (e.g. "Summer丶", "Worship灬") — this
isn't narrative text, it's a collection of player names/usernames, consistent with the
"names aren't translated" rule in [[Names-Not-Translated]]. Found as a single, massive JSONL
entry (idx 444030) containing hundreds of names. *([[Update-1]], batch 8)*

## Developer content that leaked into localization data

- **Developer Lua source code that leaked into localization data** (found at idx 437267, a
  single string containing full code with English comments) is left 100% untranslated —
  never seen by players; partially translating it risks breaking variables/syntax.
  *([[Update-1]], batch 5)*
- **Leftover Chinese developer notes in parentheses** (e.g. idx 443681 `"(仅作为玩法名称)"` =
  "for gameplay-mode naming only") ARE translated structurally into Indonesian, since
  they're ordinary developer notes rather than code — see [[Translated-Common-Terms]]. Don't
  confuse this with the Lua code case above, which is left untouched.

## Very long lore entries

Entries over ~3000 characters became more frequent from idx ~442000 onward (the Zou/Yang
"Together in One Boat" tale, the Su/Dragon King family drama, the Dragonbend Academy
register, the Zhang siblings' story). These get no special handling — translate in full
without shortening, keeping only character/place names untranslated per the usual rule. See
[[Translated-Common-Terms]]. *([[Update-1]], batch 8)*
