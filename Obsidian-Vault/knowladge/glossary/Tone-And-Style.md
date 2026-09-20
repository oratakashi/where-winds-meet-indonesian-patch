# Tone & Process Decisions

Part of [[Glossary]]. How translated text should *sound*, and the two process decisions the
user made early in the project that everything else builds on.

## Tone

- **Character dialogue** (first/second-person conversation) → casual: gue/lo.
- **UI/button/system labels** (Loading, Confirm, Cancel, etc.) → neutral-casual, without
  gue/lo.
- **Format/placeholder tokens** (`{0}`, `{}`, `%s`, `%d`, `#E`, `#aabbcc`, `#X`) must be
  preserved exactly, same count and order as the source (automatically checked by
  `tools/qa_check.py`) — see [[Placeholders-And-Formatting]] for the full rule set.

## Process decisions

- **Deduplicate first** (translate each unique string once), then expand to every row —
  approved by the user.
- Dedup-then-expand and the tone rules above were both explicit user decisions early in the
  project; see [[Initial-Brief]].
