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

**A name found here is copied exactly, everywhere in the line** — not only when the whole
line is the name. `不见山主线-机关仓库` → "Alur utama Hidden Mountain-Mechanism Warehouse",
never "Gunung Bujian", "Bujianshan" or "Gunung Tak Terlihat". Same for Path names
(`鸣金·影` → "Bellstrike - Umbra", not "Bellstrike·Bayangan"), Inner Ways (`心法` →
"Inner Way", never "Teknik Batin"; `飞仙醉言` → "Skyspeak") and NPCs.

**If the ID column is an Indonesian rendering of a place/NPC/Inner Way name while the EN
column is the name** (e.g. EN "Millennium Crossing", ID "Persimpangan Milenium"), the ID is
an old global mistake: use the EN form and fix the global row too (2026-10-25 fixed 22 such
rows — [[Place-Names-Wiki]]). Global rule [[Quick-Reference]] §1 wins.

## 2. CN-only names (no hit in `term_lookup`) — locked 2026-10-25

Same rule as global: **a name is never translated into Indonesian.** CN-only names have no
official English, so:
- **People, places, sects, Inner Ways, named skills/moves, named items/gear sets/talismans,
  titles**: Pinyin without tone marks, each word capitalized, syllables of a given name joined
  (`陈嘉铮` → "Chen Jiazheng", `行藏帖` → "Xingcang Tie", `牵丝·韵` → "Silkbind - Yun"). Only the
  generic word around it is Indonesian ("Peti", "Desa", "Gua", "lantai").
- Part of the compound has a global name → keep that part ("五音启天平" → "Five Tones
  Qitianping"); a sibling from the same set exists (`term_lookup --contains`) → copy its pattern.
- Ordinary descriptive labels and item descriptions (not names) are still translated.
- Record every new CN-only name in the phase log so later batches reuse it.

## 2a. Check names before ending a batch — `name_check.py`

```bash
python cn/tools/name_check.py --idx 94000-97999          # rows in the batch that miss a global name
python cn/tools/name_check.py --fix                       # replace Pinyin/known variants automatically
python cn/tools/name_check.py --report names.jsonl        # full list (src, v, expected names)
```

It flags every row whose source contains a Mandarin term that global keeps as an English name
(e.g. `晦谷` → "Mirkvale") but whose translation lacks it. `--fix` replaces Pinyin spellings
("Lembah Hui", "Huigu"), label segments that are exactly a name, and the curated variants in
its `_LITERAL`/`_SUFFIX` tables; literal translations it doesn't know stay in the report — fix
those by hand (and add a frequent one to `_LITERAL`). Many remaining hits are loanword labels
(Quest, Event, Boss, Gear, Solo Mode…) in internal editor labels — low priority, but follow
global there too in new batches.

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

## 5a. What the patch can't reach on CN: story/cutscene subtitles

Cutscene and story subtitles on the CN PC client show the Mandarin `formatted_text` baked
into the integrity-checked data packs, not `translate_words_map` ([[Format-Spec]] §9). Their
keys are still translated (the bridge fills them), so the work isn't wasted: it becomes visible
if NetEase ever routes CN subtitles through the map, or on a client that does. Don't spend
extra effort polishing CN-only story lines for the CN PC client until then. When a player
reports "story still Mandarin", that is §9, not a missing translation.

## 6. Workflow

See [[CN-Resume-Procedure]]. One phase = `cn/locale/phaseN.jsonl`, rows
`{"idx": N, "v": "…"}` in idx order, no gaps.
