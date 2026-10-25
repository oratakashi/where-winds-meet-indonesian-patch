# CN Phase 5

Mandarin → Indonesian for CN-only unique strings, idx **120,000–149,999**.
Output: `cn/locale/phase5.jsonl`. Rules: [[CN-Quick-Reference]] and the global
[[Quick-Reference]]. Overall coverage excludes the separate global bridge.

## Earlier work

At the start of the 2026-10-25 session, the phase file already contained **25,300
contiguous rows, idx 120,000–145,299**. No phase log existed. This records the
observed baseline; earlier batch dates and terminology decisions are not reconstructed.
Those existing translations are preserved byte-for-byte in this session.

## Batches

- **2026-10-25 — idx 145,300–149,299, 4,000 rows.** Three parallel ranges
  (145,300–146,633; 146,634–147,966; 147,967–149,299), merged into the phase file.
  Phase 5: **29,300 / 30,000 (97.67%)**; next idx **149,300**, 700 rows remain.
  Exact range 120,000–149,299 verified contiguous; original 25,300 rows preserved
  byte-for-byte. Full CN QA: **130,878 entries including 2 overrides, 0 findings**.
  **Overall: 130,876 / 159,176 unique strings (82.22%); CN-only in-game coverage
  789,692 / 823,139 (95.94%).** Generated with `python tools/progress.py --profile cn --write`.
  This snapshot includes 2,400 Phase 4 rows from concurrent work outside this batch.
  No commit created in this session.

## Batch decisions — 2026-10-25

- Source newlines and tabs preserved. Numeric differences reviewed as Chinese numeral
  conversion, dates/month names, ordinal wording or word order. `100万` → 1 juta;
  `1000万` → 10 juta; 95折 → diskon 5%; 半个时辰 → satu jam.
- Long ferry rules (146913), military essay (147196) and flood-relief diary (149220)
  translated in full. The diary retains the source's 31 August and 999 tael.
  Idx 148793 retains all 120 Event-description repetitions; loading fragments and
  repetitions at 147023 and 148385 retained. Malformed repetition at 147684 translated
  without omitting its extra fragment.
- XML link display text at 145927 translated as Event Mendengarkan Nada; link tags and
  attributes retained. Han only remains in protected tokens and the exact font asset
  identifier `禹卫书法云墨繁体.TTF` (147327), which must not be renamed.
- Global lookup forms include Whirlsnow, Ivorybloom, Bodhi Sea, Kilnfire Ridge,
  Battlecrest Slope, Ghost Revelry Hall, Mirkvale, Well of Heaven, Mohist City,
  Mercyheart Monastery, Palace of Annals, Source of Still Shore, Moonveil Stream,
  Wellspring Town, Imperial Artisan Court, Skyward City Ruins, Mount Tai Blizzard,
  Wildmane Ranch, Thundercry Blade, Phalanxbane Blade, Everspring Umbrella, Etherwrath,
  Bamboocut - Draught, Silkbind - Wings/Jade/Deluge, Stonesplit - Might/Strength,
  Oscillating Jade and Manik Gema. 文津馆 follows the lookup's Indonesian precedent
  Perintah Terikat Tinta. Names retain the form returned by the shared-key lookup.
- Provisional CN-only names: 丰和村 Fenghe Village; 抱朴庐 Baopu Cottage;
  丰能奇 Feng Nengqi; 云霄 Yunxiao; 程准 Cheng Zhun; Choulingke, Tianjizi,
  Nangong Lei, Yu Yueyan, Geng Minghuan, Li Liushuo, Meng Zhenming and Lang Youyi.
- Provisional CN-only labels: 漱云笙 Sheng Pembilas Awan; 云中饮 Minuman di Awan;
  牵丝·韵 Silkbind - Rhythm (harmonized across this batch); 过阕更章 Mengganti Lembaran;
  寒霜快斩 Tebasan Cepat Embun Beku; 破竹·樽 Bamboocut Cask. Revisit when official
  shared-key counterparts become available.

## Name consistency pass (2026-10-25)

7,899 rows of this phase rewritten so names match global ([[CN-Quick-Reference]] §1–2a, rule
locked the same day): places, NPCs, sects, Path names and Inner Ways were often written as
Pinyin ("Gunung Bujian", "Lembah Hui"), half-translated or translated literally ("Gunung Tak
Terlihat", "Persimpangan Milenium", "Bellstrike·Bayangan", "Teknik Batin") instead of the global
English name (Hidden Mountain, Mirkvale, Millennium Crossing, Bellstrike - Umbra, Inner Way).
Done with `python cn/tools/name_check.py --fix` (Pinyin variants, label segments that are exactly a
name, curated `_LITERAL`/`_SUFFIX` tables); idx unchanged, no new rows. Names listed earlier in
this log in an Indonesian or Pinyin form are superseded by the global form. CN-only names now
stay Pinyin (never Indonesian). Remaining mismatches: `python cn/tools/name_check.py --idx <range>`.

