# Translation Guidelines

Rules every translated string must follow so the game renders it correctly. For terminology and
tone (Wuxia terms, titles, register), see the condensed
[`Quick-Reference.md`](../Obsidian-Vault/knowladge/Quick-Reference.md) and the full
[`Glossary.md`](../Obsidian-Vault/knowladge/Glossary.md).

- [Engine Format Tokens](#engine-format-tokens)
- [Proper Nouns & Pinyin](#proper-nouns--pinyin)
- [Machine Translation & LLM Quality Control](#machine-translation--llm-quality-control)
- [Oversized Lore Entries](#oversized-lore-entries)

---

## Engine Format Tokens

Strings contain Messiah Engine formatting and color tags. Changing, adding or dropping them breaks
the UI's text rendering:

| Token | Function | Rule |
|---|---|---|
| `#Y` `#N` `#R` `#J` `#G` `#H` | Format / color style opening | Keep unchanged |
| `#aee5ae` | 6-digit hex color tag | Keep unchanged |
| `#E` | Format closing tag | **Must** pair with its opening tag |
| `%s`, `%d` | printf-style specifiers | Runtime values; keep the count |
| `{0}`, `{1}` | Positional placeholders | Keep the indices |
| `<...>` | Markup tags | Keep unchanged |
| `\n` | Line break | Keep the line flow |

**Example**

- **Source**: `Mystic Skill #YMeridian Touch#E`
- **Target**: `Keterampilan Mistik #YSentuhan Meridian#E`

The text *inside* `#Y...#E` is translated; the tags themselves stay intact.

> [!NOTE]
> The `TOKEN` regex in [`tools/qa_check.py`](../tools/qa_check.py) is the single source of truth
> for what counts as a token. The QA check compares the token **multiset** between source and
> translation (counts, not order).

---

## Proper Nouns & Pinyin

Character names, sects and historical places are usually written in Pinyin (e.g. `Su Jiangyun`,
`Gu Zhouyue`, `Zhang Tiemeng`). Keep them as-is so players can cross-reference guides, wikis and
chat. Leaving these untranslated is intentional, not a gap.

---

## Machine Translation & LLM Quality Control

Validate all MT/LLM output strictly. In an audit of a commercial translation pack, **1,817 entries
(0.189%) contained leaked LLM system prompts** in the game text, for example:

```text
Formatting instructions:
  (newline tags) and brackets [] must remain unchanged
- Do not translate content inside {}
- Several special characters must not be translated, such as: %d, %s, #G, #E...
[Actual translated text followed here]
```

`tools/qa_check.py` blocks this by checking every entry for:

1. **`PROMPT_LEAK`** — known prompt fragments (`LEAK_SIGNATURES`; extend it when a new variant
   shows up).
2. **`MARKUP`** — the same multiset of format tokens in source and translation.
3. **`EMPTY`** — no blank or whitespace-only values for non-empty sources.

---

## Oversized Lore Entries

The average string is ~56 bytes, but some encyclopedia and story entries exceed **8 KB**. When
sending text to a translation API, chunk the payload so long entries aren't silently truncated.
