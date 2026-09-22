# Placeholders, Tags & Formatting Rules

Part of [[Glossary]]. These are mechanical, QA-critical rules — get one of these wrong and
`tools/qa_check.py`'s `MARKUP` check flags it, or worse, the game UI renders incorrectly.
Read this file before translating anything containing `#`, `%`, `{`, `<`, or `$`.

## The baseline rule

Format/placeholder tokens (`{0}`, `{}`, `%s`, `%d`, `#E`, `#aabbcc`, `#X`) must be preserved
exactly, same count and order as the source. This is automatically checked by
`tools/qa_check.py`'s `TOKEN` regex: `#[A-Za-z]|#[0-9a-fA-F]{6}|%s|%d|\{[^}]*\}|<[^>]*>`.

## `<...>` tag gotchas

- **A `<...>` tag WITHOUT the `|id|#C|n>` format** (a plain placeholder, e.g. `<Player Name
  7 characters>`) must be kept EXACTLY in English, not translated. This differs from a
  stat-formatted tag like `<Stat Name|780|#C|15>`, where the stat name is naturally left in
  English anyway (because stat names aren't translated) — a plain tag like this is swallowed
  whole as one token by the `TOKEN` regex (`<[^>]*>`), so translating its contents causes a
  mismatch (this happened at idx 6041, already fixed). Whenever a new `<...>` tag appears
  that isn't the standard stat pattern, do not touch its contents at all. *(Phase 3)*
- **A `<...>` tag wrapping ONE ENTIRE LONG SENTENCE** (not the short `<Label|id|#C|n>`
  pattern) is treated by the `TOKEN` regex as ONE WHOLE TOKEN, because `<[^>]*>` matches from
  the first `<` to the next `>` — swallowing any `#color`/`#E` tags inside it as part of that
  same token, not as separate tokens. As a result, the contents of this kind of `<...>`
  (found at idx 12448, a time-frozen item notification) must be kept 100% identical to the
  English source — only text OUTSIDE the `<...>` is translated. Always run token validation
  after translating a paragraph containing a long `<...>` tag to catch this case. *(Phase 4)*
  - Also applies to a `<...>` tag wrapping dialogue/a long sentence (not just a stat name) —
    e.g. non-human NPC speech at idx 18461 `<Pft. One round...>` — kept 100% identical in
    English, including the dialogue content itself. *(Phase 4 continuation, idx
    18100–19099)*
- **A `<...>` tag without a closing `>`** (a truncated/typo'd source string, e.g. `"<Tangled
  Gauze...`) does appear in the source data — preserve the literal `<` at the start as-is;
  don't delete it or add a closing tag manually. *(Phase 4)*
- **The pattern `"#Y<label>#E <Tag|id|#C|slot>"`**: the highlight label and the placeholder
  tag are TWO separate units — translate the contents of `#Y...#E`, then leave the `<...>`
  tag intact right after it without merging text into it. Never put text inside a `<...>` tag
  that already has the `|id|#C|slot` format. *([[Update-1]], batch 2)*

## `#`-tag gotchas

- **Short highlight tags `#H...#E`/`#Y...#E` wrapping a SINGLE common instruction word**
  (Press, night, etc.) have their contents translated (`#HPress#E` → `#HTekan#E`, `#Ynight#E`
  → `#Ymalam#E`) — different from a tag wrapping a skill/weapon name (kept in English, e.g.
  `#HFire Arrow#E`, see [[Names-Not-Translated]]) or a long sentence (kept in English, see
  above). The `TOKEN` regex matches `#H`/`#Y`/`#E` as separate single-letter tokens, so the
  text between them is safe to translate. *(Phase 4 continuation, idx 18100–19099)*
- **Non-standard color tags of the form `#<6-char-hex-ish>NNNN Word#E`**: the `TOKEN` regex
  only matches the first 6 characters after `#` as the color token; the rest (trailing
  digits + word, e.g. "120 Points") is free text safe to translate ("Points" → "Poin"). See
  [[Translated-Common-Terms]]. *(Phase 5 continuation, idx 21000–21999)*
- **The stray tag pattern `#Ttext` that happens to match the `TOKEN` regex as `#T`** (not a
  real highlight tag) — e.g. idx 434808 and idx 444071 `"#Talk to the Dog"` — leave `#Talk`
  intact at the start, translate the rest (`#Talk dengan Anjing`). *([[Update-1]], batch 8)*

## Duration/placeholder conversions

Literal duration text, not a token caught by `qa_check.py`, but still meaningful to convert
consistently:

- **`Nd` (N days, literal letter "d") → `Nh`** (e.g. "30d" → "30h"). *(Phase 4)*
- **Literal seconds duration (`Ns`) inside a `#Y...#E` tag etc. also converts to `Nd`** (e.g.
  `#Y70s#E` → `#Y70d#E`), extending the above to cases where the literal sits inside a
  color/highlight tag. *(Phase 5 continuation, idx 21000–21999)*
- **Combined double-literal duration format `%sd%sh`** (days+hours, e.g. "Remaining:
  %sd%sh") → `%sh%sj` — `%s` itself is preserved exactly (it's a placeholder token). *(Phase
  5, idx 20000–20999)*
- **Compound minute+seconds duration placeholder `%dm%ds` → `%dm%dd`**, and `%sh` (hours) →
  `%sj` — same literal-duration conversion (`s`→`d` seconds, `h`→`j` hours) applied to
  formats combined with `%d`/`%s` placeholders. The letter `m` (minutes) is left as-is.
  *(Phase 5 continuation, idx 29000–30999)*
- **The single-letter duration shorthand `s`→`d`/`h`→`j` also applies** when attached
  directly to a `{}` placeholder (e.g. `{}s`→`{}d`) and inside color tags. *(Phase 5
  closing / Phase 6 opening, idx 49000–50999)*
- **Combined format+literal duration placeholders** (e.g. `{diff_hour:d}h ago`) — the
  `{...}` part must be preserved exactly, while the literal suffix outside the braces is
  free to translate following the duration convention (`d`→`h` for days, `h`→`j` for hours).
  *(Phase 4, close of range idx 19100–19999)*

## Untagged placeholders (not caught by the `TOKEN` regex — preserve exactly anyway)

- `$STEADY_MIN_PRO_ATK_C:.1f$`, `$P`, `$N`, and literals like `Xd, Xh` — runtime
  substitution variables, not ordinary text. *(Phase 4)*
- **`$link<text>^ID^$`** — a special link placeholder; everything between `$link` and
  `^ID^$` (including any English text inside it) MUST be preserved 100% identical, treated
  as one opaque unit. *(Phase 5 closing / Phase 6 opening, idx 49000–50999)*
- **`$S...$E`** — wraps a quote/narrative block in a skill description (different from
  `$D`/`$H`/`$F`, which are pure numeric values) — the content inside IS still translated
  normally; only the `$S`/`$E` markers themselves are preserved exactly. *(Phase 5 closing /
  Phase 6 opening, idx 49000–50999)*
- **Runtime date placeholder `@T[...]`** (square brackets, e.g. `@T[month_2,
  day_6,type_noLocal;empty]`) must be preserved character-for-character. *(Phase 5
  continuation, idx 29000–30999)*

## Long stat-scaling strings

Strings of the form `"Increases <Stat|id|#C|n> based on Agility. Current bonus: ...
Maximum bonus requires ..."` — the opening sentence is translated ("Meningkatkan ...
berdasarkan Agility. Bonus saat ini: ... Bonus maksimum membutuhkan..."), while the stat name
and numbers inside the `<...>`/`#...#` tags stay untouched. *(Phase 3)*

## See also

- [[Special-Cases]] — content that's excluded from translation entirely (Lua code leaks,
  player-name lists), a different concern from token preservation.
