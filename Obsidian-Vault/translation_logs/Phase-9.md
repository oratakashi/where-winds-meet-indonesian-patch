# Phase 9 Translation Log (idx 429,887–461,703)

Session-by-session terminology notes for Phase 9, in idx order. Phase 9 is
the batch of strings added by the 2026-09-16 game update (not a continuation
of the original `unique_strings.jsonl`) — see [[Phase-Roadmap]] for how it
was created and [[Session-History]] for the full session timeline (Phase 9
ran in parallel with Phase 5/6 for most of its history). All standard
[[Glossary]] conventions still apply; this file adds Phase-9-specific
decisions.

Phase 9 content leans heavily on new chapters/areas (Hidden Mountain/Mohist
Hill continuation, Sky Citadel, Luan City, Qiongqi Artificer faction, etc.)
and new compound gear names.

## Batch 1 — idx 429,887–431,886

- **"Farmer" (a recruitable/deployable Homestead unit) is KEPT fully in
  English** (capitalized), e.g. "Farmer Management", "Insufficient
  Farmers", "take a photo with a working Farmer" — a Homestead system term
  like Retainer/Homestead, NOT the common word "petani". Appears very often
  in this batch (recruitment, dispatch, cohabitation, etc.).
- **"Qiongqi Master"/"Qiongqi Warrior"/"Qiongqi Soldier"/"Qiongqi
  Artificer"** (ranks/roles in the antagonist "Qiongqi" faction) are KEPT
  fully in English as compound titles/proper nouns, NOT partially
  translated (faction name "Qiongqi" + a translated common-word rank) —
  different from the generic-NPC-role pattern (Villager/Bandit), because
  these ranks read as internal/enemy-specific rank labels, not general
  social titles.
- **Generic royal/imperial titles (Prince, Empress) are TRANSLATED**,
  consistent with the locked Lord→Tuan/Young Master→Tuan Muda pattern:
  "Prince Teng" → "Pangeran Teng", "Empress Wu" → "Permaisuri Wu".
- **"Master" before a name as a teacher/leader honorific** (not a Qiongqi
  rank) is TRANSLATED "Guru", consistent with the older Phase 4 rule:
  "Master Jin" → "Guru Jin" (Jin Zhongyuan, this chapter's main character,
  addressed this way repeatedly), while "Academy Master Crane" → "Ketua
  Akademi Crane" (here "Master" = institution leader → "Ketua").
- **"Lantern Festival" is TRANSLATED "Festival Lampion"**, consistent with
  the locked festival-name pattern (Spring Festival → Festival Musim
  Semi).
- **"Treasury" (the common word) is TRANSLATED consistently** across all
  its compounds: "Pledged Treasury" → "Perbendaharaan Terikat", "Sealed
  Treasury" → "Perbendaharaan Tersegel", "Imperial Treasury Halls" → "Aula
  Perbendaharaan Kerajaan" — different from a compound unique-place name
  (kept fully in English), because "Treasury" here is consistently used as
  a descriptive common word, not part of a unique place name.
- **Common non-fantasy animals/creatures (Pangolin) are TRANSLATED**,
  consistent with the locked rule: "Pangolin's Trade Tales" → "Kisah Dagang
  Trenggiling".
- **A literal two-character `\n` (backslash+n, NOT an actual newline)**
  appears in several strings (idx 430,082, 430,083, 430,581, 430,615,
  430,622, 430,699, 431,245, 431,278, 431,731, 431,860, 431,862) — MUST be
  preserved as a literal 2-character sequence exactly; do NOT convert it to
  a real newline or remove it. Distinguish it from a string with an actual
  newline (the majority of long narrative text) — check with Python's
  `repr()` if unsure: a real newline shows as a single `\n` in `repr()`,
  while the literal two-character version shows as `\\n`.
- **Color tags `#Y...#E` vs. `#G...#E` must be checked precisely on every
  occurrence** — one case (idx 431,459) picked the wrong color tag (`#G`
  instead of the source's `#Y`) and slipped past the first draft, caught
  during token validation because the per-color token count differed.
  Always run a per-color token-count check (not just a total-token count),
  especially for paragraphs with several different color tags.
- **A tag `#994242(...)` with no closing `#E`** (idx 431,600, an
  Auto-Management UI label) — a closing `#E` was mistakenly added where the
  source had none, caught by validation. Always match the token count
  EXACTLY as in the source; don't "fix" a tag that looks asymmetric to a
  human reader — that's genuinely how the source is written.

## Batch 2 — idx 431,887–433,886

- **Homestead crafting materials/resources in the `"<Name> Tier N"`
  pattern** (e.g. "Magnet Stone Tier I", "Cloud Sand Tier II", "Turquoise
  Stone Tier I", "Iron Ore Tier II", "Cinnabar Tier I", and dozens of other
  compound material names like "Mushroom-Iron Composite", "Pine-Copper
  Cloudsand Extract") are KEPT fully in English — new decision this
  session: treated as crafting/resource-system identifiers like
  Inspiration/Affection (locked in Phase 2), NOT ordinary common nouns
  (different from the "animals/plants with common names get translated"
  rule, because these material names now function as specific
  crafting-system identifiers, not living-creature narrative text). Applied
  consistently across a very large number of occurrences in this batch
  (dozens to hundreds of Tier I/II items).
- **IMPORTANT — the pattern `"#Y<label>#E <Tag|id|#C|slot>"`** (a short
  highlight phrase immediately followed by a separate placeholder tag,
  e.g. source `"a #Y3rd-Stage#E <Heavy Attack Charged
  Skill|781|#C|20401|20401103>"`) MUST be kept as TWO separate units:
  `#Y...#E` wraps translated text (→ `#YTahap ke-3#E`), then the `<...>`
  tag follows completely untouched. NEVER merge the highlighted text INTO
  the placeholder tag (e.g. don't write
  `#YHeavy Attack Charged Skill|781|#C|20401|20401103 Tahap ke-3#E` — this
  breaks the `<...>` tag open with no closing `>` and lets the `#C` inside
  it leak out as a stray token). This mistake happened twice in this batch
  (idx 432,654, 433,428) before being caught by validation — once this
  exact source pattern is recognized, watch for it for the rest of the
  batch.
- **A two-part paragraph following the pattern `"...deal damage...\n
  Applicable #YMartial Trigger Effects#E: ..."`** — the opening sentence AND
  the second paragraph's "Applicable ...:" line BOTH wrap the same term
  (e.g. "Martial Trigger Effects") in a `#Y...#E` tag; both occurrences must
  be translated and keep their tag. One case (idx 433,342) lost its
  `#Y...#E` tag on the second line in an early draft (translated as plain
  text, "Martial Trigger Effect yang berlaku:"), caught by validation.
- **A long `#Y...#E` span wrapping TWO consecutive phrases with a single
  tag pair** (e.g. source idx 432,181
  `"#Y{} Personal Component(s), and {} Diagram(s)#E"` — one `#Y` at the
  front, one `#E` at the very END of both phrases, not each phrase getting
  its own tag) — MUST be kept as one unbroken span when translated; don't
  close `#E` in the middle and reopen without a new `#Y` (that adds an
  extra unpaired `#E` token not present in the source). Caught at idx
  432,181 before appending.
- Qiongqi Master/Farmer/Cultivation/Union/Sword Trial/Wanderer, the
  `"<Weapon> - <Action>"` combat-label pattern, and long gear names
  (Swallowcall/Swallow's Return/Frostbane/Nightfarer/etc.) reconfirmed
  consistent with batch 1 and earlier Phase 5 sessions.

## Batch 3 — idx 433,887–435,886

- **Homestead crafting materials in the `"<Name> Tier N"` pattern
  reconfirmed** (e.g. "Cinnabar Tier II") kept in English, consistent with
  batch 2.
- **"Cloudfore" (a new feature/industry, a variant of Beyond Mundane) is
  KEPT fully in English** as a system/feature name, same pattern as the
  already-locked Arcadian Homestead/Beyond Mundane.
- **"Fengputer" (a mechanical device/oracle that answers player questions,
  appearing repeatedly in this batch) is KEPT fully in English** — a
  specific device/mechanism name, not a common word.
- **Common Chinese food names (Hulatang, Tofu Pudding, Pan-Fried Buns,
  etc.) in a long culinary lore passage are KEPT fully in
  English/Pinyin** — culinary loanwords, consistent with other
  already-locked food names (e.g. He'le noodles).
- **"Steward" (the lowercase generic role) is TRANSLATED "Pelayan"** —
  re-locking the existing [[Glossary]] rule; it had slipped through kept
  in English twice in this batch's early draft, fixed after a manual audit
  post-token-validation (the automated token check does NOT catch this,
  since a plain "steward" with no tag isn't a regex-checked token).
  **IMPORTANT**: a capitalized "Steward" used as part of a title/capitalized
  label (e.g. "Ritual Steward", `#YSteward#E`) still stays in English —
  different context, functioning as a title rather than a generic term.
- **IMPORTANT — a string starting with `#` that is NOT actually a
  highlight tag** (one new case found: idx 434,808 `"#Talk to the Guard"`
  — a quest label name that happens to start with `#`, not the `#Y...#E`
  token format): `qa_check.py`'s `TOKEN` regex still matches `#T` (the
  first letter after `#`) as a single-letter token, so translating the
  string normally (e.g. into "#Bicara...") causes a MISMATCH because the
  first letter changed. Fix: leave `#` + the first word untouched when it's
  clearly not a real highlight pattern (`#Talk` stays as-is, the rest is
  translated: "#Talk dengan Penjaga") — NOT the same as translating the
  whole thing and hoping the first letter happens to match.
- **A long paragraph with MORE THAN TWO occurrences of a `#Y...#E` tag
  wrapping the SAME repeated term** (e.g. idx 435,165, "Martial Art
  Triggered Effects" appears 3 times in one string, each wrapped separately
  in `#Y...#E`) — one occurrence (the third, at the start of the second
  paragraph, "Applicable #Y...#E include:") slipped through without its tag
  in an early draft. Reminder: if the same term repeats more than twice in
  one string, check EVERY occurrence individually — don't assume there are
  only 2 like the more common pattern seen before.
- **The long-single-token `<...>` tag rule (locked since idx 12,448) is
  still frequently missed** — 3 more cases in this batch (idx 434,451, a
  non-human dog's speech; idx 435,245 and 435,796, a placeholder/dialogue
  tag in quotes) before being caught by validation. This recurring pattern
  reinforces: every time a new `<...>` tag appears, check first whether its
  format is `<Label|id|#C|n>` (a stat name, safe to partially translate) or
  not (must be kept 100% identical in English) BEFORE translating, not
  after.
- Farmer/Qiongqi Master/Cultivation/Union/Sword Trial/Wanderer and the
  `"<Weapon> - <Action>"` combat-label pattern / long gear names
  reconfirmed consistent with batches 1 and 2 and earlier Phase 5 sessions.

## Batch 4 — idx 435,887–437,136

This batch (1,250 of a planned 2,000 rows — see [[Session-History]] for why
it was cut short) was delegated to a background subagent. No distinct new
terminology decisions were recorded for it beyond the conventions already
locked in batches 1–3.

## Batch 5 — idx 437,137–439,136

- **"Mohist Sect" (the sect/order at Hidden Mountain, appearing for the
  first time in this batch) is KEPT fully in English** as a proper-noun
  faction name — DIFFERENT from the common word "Sect", which is
  translated "Sekte" (e.g. "Sect Rules"). Consistent with the already-locked
  "Mohist Hill"/"Mohist City" as untranslated place/faction names.
- **"Senior Sister"/"Junior Sister"/"Senior Brother"/"Junior Brother"**
  (wuxia terms of address between fellow disciples) are **kept in English
  for now** — no locked Indonesian equivalent yet (an option like
  "Kakak/Adik Seperguruan" was considered but not decided); revisit if this
  keeps appearing frequently.
- **Developer Lua source code leaked into localization data** (found at
  idx 437,267, a single string with full code and English comments) is
  **left 100% untranslated** — never player-facing, and partial translation
  risks breaking variables/syntax.

## Batch 6 — idx 439,137–441,136

- Long lore paragraphs about Zou/Yang's old friendship at Mohist Hill (idx
  439,837), Heron/Gasping Cliff (idx 440,503), and Zhen Gui/Relief Bureau
  (idx 440,551) were translated in full as standard wuxia narrative text,
  with no significant new terminology.
- **"Senior Brother"/"Senior Sister"/"Junior Brother"/"Junior Sister"**
  (lowercase within a sentence, e.g. "senior brother-ku") reconfirmed kept
  in English per the batch-5 decision — applied consistently in this batch.
- **"Elder Roc"/"Elder Crane"/"Elder Dove"/"Elder Guan"/"Elder Stork"/
  "Elder Shrike"** etc. (Elder + name) reconfirmed consistently "Tetua X"
  per the older Phase 4 rule.
- **System recipe/dish names** (a recipe list with `#U`/`#H` placeholders,
  e.g. "Grilled Melon with Garlic", "Braised Meat with Preserved Peaches")
  are **kept in English** as a formal item/recipe name, different from an
  ordinary narrative food-lore description (e.g. "Pine Rock Stewed Dove" as
  a standalone dish name is also kept in English — revised from an earlier
  draft that had translated it, for consistency with the other
  recipe/item-name pattern).
- **A `<...>` tag wrapping one long phrase/sentence (the single-token-tag
  pattern)** was found again a few more times (idx 440,190, 440,683) — kept
  100% in English per the rule locked since idx 12,448.

## Batch 7 — idx 441,137–442,136

No distinct new terminology decisions recorded for this batch; see
[[Session-History]] for the batch-size change (lowered to 1,000/session)
that happened alongside it.

## Batch 8 — idx 442,137–444,136

- **Player character/username lists (the "Max-Level Characters" Hall of
  Fame, from the Ruibin & Huangzhong trials)** are KEPT 100% UNTRANSLATED,
  copied verbatim including any Han characters embedded in nicknames (e.g.
  "Summer丶", "Worship灬") — treated as a collection of player
  names/usernames, not narrative text, consistent with the
  names-aren't-translated rule. Found as a single, massive JSONL entry (idx
  444,030) containing hundreds of names.
- **Lowercase generic "disciple" reconfirmed KEPT in English** throughout
  this batch — older precedent was mixed ("murid" vs. "disciple"), but
  since this batch sits close to the idx 432xxx range where "disciple"
  dominates, local consistency was prioritized over a global rule. Check
  the nearest idx precedent before deciding if this term recurs much later.
- **"Swordsman Sinan" reconfirmed as a single NAME UNIT, not translated**
  (not "Swordsman" translated + "Sinan" as a name) — used consistently as
  a combined title/name since early Phase 9.
- **The stray-`#` pattern `#Ttext` that happens to match the TOKEN regex
  as `#T`** (not a real highlight tag) appeared again (idx 444,071,
  `"#Talk to the Dog"`) — same fix as idx 434,808 in batch 3: leave
  `#Talk` intact at the start, translate the rest (`#Talk dengan Anjing`).
- **"Chief of the Inner Court" → "Kepala Istana Dalam"**, **"Field
  Commander" → "Komandan Lapangan"** — generic titles/positions translated
  consistently with the Lord→Tuan/Commander→Komandan pattern.
- **A leftover Chinese developer note in parentheses** (e.g. idx 443,681
  `"(仅作为玩法名称)"` = "for gameplay-mode naming only") was translated
  structurally into Indonesian as an ordinary developer note — different
  from the long Lua code block kept 100% untouched (the batch-5 rule from
  idx 437,267).
- Very long lore entries (>3,000 characters) became more frequent in this
  idx range — the Zou/Yang "Together in One Boat" tale (idx 443,599), the
  Su/Dragon King family drama (idx 444,044), the Dragonbend Academy
  register across several years (idx 443,803), and the Zhang siblings'
  Tiger Fort naming drama (idx 442,985) — all translated in full without
  shortening, keeping only character/place names untranslated per the
  usual rule. Also present in this batch: Khitan/Du Zhongwei war letters and
  the dying monologue of the Sky Citadel Grandmaster (idx 442,978–442,979).

## Batch 9 — idx 444,137–445,736

- **A `<...>` tag wrapping one full sentence (the single-token-tag rule
  locked since idx 12,448) was hit again** at idx 444,894
  (`<This is where Senior Yu used to work...>`) — translating its contents
  produced a token mismatch on first draft (the whole bracketed sentence is
  swallowed as ONE token by `qa_check.py`'s regex), fixed by reverting the
  content to 100% English. Reconfirms the existing rule; no new decision,
  but worth flagging since it keeps recurring roughly once per batch.
- **Ordinal sibling nicknames "Pi the Fourth"/"Pi the Fifth"** (idx
  444,500, 444,904) → translated as **"Pi Keempat"/"Pi Kelima"** — the name
  root "Pi" stays untranslated (per the names rule) while the English
  ordinal is translated, consistent with how other Name+ordinal or
  Name+kinship-term patterns are handled elsewhere in the glossary.
- **"Master Crow" (a teacher honorific before a name) reconfirmed →
  "Guru Crow"**, consistent with the Phase 4 Master→Guru rule (idx
  444,321).
- This batch was thick with long personal-letter and diary lore rather
  than combat-skill strings: the Qin Fen origin story (idx 444,149, ~2,600
  characters), the Fang Bai secret-message letter (idx 444,193), the
  Old Mohist City wind-chime legend (idx 444,537), a Northern Vow New Year
  gift-giving ledger letter (idx 445,157), a Zhenming-era travel diary
  full of running gags about a bandit-slaying rival named Wang Qing (idx
  445,194), and a long homesick letter about academy life, star-charting,
  and a math-vs-feelings debate with Lone Cloud disciples (idx 445,178,
  one of the longest single entries seen in Phase 9 so far). All
  translated in full; only character/place names and the established
  kept-in-English terms were left untouched.
- Homestead facility names in square brackets (`[Stove]`, `[Dining
  Table]`, `[Cloudrest Passage]`, `[Farmland]`, `[Bed]`) reconfirmed KEPT
  in English, consistent with the existing bracketed-system-name
  convention (e.g. `[Interlocking Joint]` locked earlier in Phase 9).
- Session ended partway through a planned 2,000-row batch at the user's
  request (see [[Session-History]] for why); the last sub-batch (200 rows,
  idx 445,537–445,736) had already been drafted and validated before the
  stop request landed, and the user then asked to keep it rather than
  discard it — see [[Current-Status]].
