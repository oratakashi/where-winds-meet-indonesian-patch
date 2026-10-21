# CN Quick Reference — Read This First (WWM China)

Rules for translating `cn/translation_work/unique_strings.jsonl` (Mandarin → Indonesian).
The CN game will show these lines **next to ~1 million lines bridged from the global
translation**, so the target style is exactly the global one: every rule in the global
[[Quick-Reference]] applies unchanged (what stays English, honorifics, units, dates,
markup). This page only adds what is specific to a Mandarin source.

## 1. Find the global name first — `term_lookup.py`

Proper nouns (people, places, sects, skills, items, titles) must match what the bridged lines
say. Look every one of them up before inventing anything — many terms per call:

```bash
python cn/tools/term_lookup.py 止戈定音技 告辞 开封府
python cn/tools/term_lookup.py --contains 燕云 --limit 5
```

It prints how the same Mandarin text was rendered in global (English) and in the Indonesian
translation. Use the Indonesian form (e.g. `止戈定音技` → "Skill Truce Attunement",
`开封府` → "Kaifeng Prefecture", `宝箱` → "Peti Harta").

## 2. CN-only names (no hit in `term_lookup`) — default, not yet locked

Global keeps names in their official English form. CN-only names have none, so until the
owner decides otherwise:
- **People, places, sects**: Pinyin without tone marks, each word capitalized, syllables of a
  given name joined (`陈嘉铮` → "Chen Jiazheng"). For a compound like a sect or district, take
  the pattern of its siblings from `term_lookup --contains <last character>`.
- **Skills, items, gear, titles**: translate the meaning into Indonesian, following the
  global label formats ([[Quick-Reference]] §2) — but if a sibling term from the same set
  exists in global (`term_lookup --contains`), copy its pattern instead.
- Record every new CN-only name in the phase log so later batches reuse it.

## 3. Markup

Identical token set to global, validated by `tools/qa_check.py --profile cn --locale`:
`#Y…#E`, `#406182`, `%s`/`%d`, `{0}`, `<Name|780|#C|15>`, `$STEADY_MIN_PRO_ATK_B:.1f$`.
Also keep `$S…$E` poem wrappers, `$N`/`$D` and `#x$y#` alternates exactly as in global.
Full-width punctuation becomes ASCII (`，` → `, `, `。` → `.`, `“…”` → `"…"`,
`（）` → `()`, `：` → `: `), but never inside a token.

## 4. Internal editor strings (CN build only)

Developer names, table labels and placeholders are never shown to players (see
[[CN-Phase-Roadmap]]). Don't spend effort on them, but still write a row, because phase files
must be gap-free:
- A bare developer name → Pinyin (`周雨霖` → "Zhou Yulin").
- `奖励表-陈佳伟` / `entity表-刘铭册` style labels → literal and short ("Tabel hadiah-Chen Jiawei",
  "Tabel entity-Liu Mingce").
- Placeholder/test text → literal translation, no polish.

## 5. Per-key overrides — when inherited text doesn't fit the CN layout

Bridged lines were written for the global UI. When one breaks a CN screen (overflow, overlap,
letters stretched by a `\n` the CN layout doesn't expect), fix it in `cn/overrides.jsonl`, one
row per keyHash: `{"h": "<key>", "src": "<current Mandarin text>", "v": "<Indonesian>",
"note": "<where/why>"}`. It wins over the bridge and `cn/locale/`; keep the same number of
lines and the same tags as `src`. `patch_all` skips (with a warning) a row whose `src` no
longer matches the game text, and `qa_check --profile cn` checks its markup. Get key + text
from `cn/work/strings_zh_cn*.jsonl`.

First rows (2026-10-20, login screen): `017804555d77cc34` health notice (3 fixed lines — the
global line 3 wrapped onto the ISBN block) and `d87da9c4e15aec8a` approval/ISBN/ICP line (kept
on one line; Chinese registration numbers left verbatim, they are legal identifiers).

## 6. Workflow

See [[CN-Resume-Procedure]]. One phase = `cn/locale/phaseN.jsonl`, rows
`{"idx": N, "v": "…"}` in idx order, no gaps.
