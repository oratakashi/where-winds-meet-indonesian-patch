# CN Phase 6 Translation Log (idx 150,000–159,175)

Session history and terminology for `cn/locale/phase6.jsonl`. Rules:
[[CN-Quick-Reference]]; live progress: [[CN-Current-Status]]; precedents: [[CN-Phase-3]].

Phase 6 holds the rarest strings of the CN-only corpus (frequency order: 1–2 occurrences each),
mostly internal editor/spawn labels (`…-战利品刷新点N`, `S14-战令奖励-付费26级`,
`…对应普通装备宝箱`), plus NPC dialogue, Skill/gear descriptions, notices and lore.

## Batches

- **2026-10-24 — Phase 6 batches 2–14 (idx 154,000–159,175, 5,176 rows), phase complete.**
  Continued on the owner's request to finish the phase. Translated directly in the main
  session (no subagents) as thirteen chunks (154,000–154,399 … 158,800–159,175; twelve of
  400 rows, the last of 376), merged into `cn/locale/phase6.jsonl`. Every chunk was checked
  before merging: idx continuity, newline/tab counts against the source, no Han leftovers, and
  a digit-sequence drift check that flags rows whose numbers differ from the source (caught
  a swapped pair, 154291/154292, and a one-row skip at 155536 — both fixed before QA). Exact
  contiguous range 150,000–159,175 (9,176 rows, no gaps or duplicates). QA first run: 7 MARKUP
  findings (155893, 156087, 156898, 157573, 158204, 158421, 158537 — `<…>` tags containing Han
  text restored verbatim); final full CN QA **0 findings**. Phase 6: **9,176 / 9,176
  (100.00%)**. **Overall: 83,176 / 159,176 unique strings (52.25%); CN-only in-game coverage
  723,592 / 823,139 (87.91%).** Generated with `python tools/progress.py --profile cn --write`.
  Decisions: see "Batches 2–14 decisions" below.

- **2026-10-24 — Phase 6 batch 1 (idx 150,000–153,999, 4,000 rows).** Started on the owner's
  explicit request ("Phase 6"), although Phases 3–5 are still open — the phase files are
  independent, so order does not matter for the patch. Translated directly in the main session
  (no subagents) as eight 500-row chunks, each merged with an idx-continuity and
  newline-count check against the source, written to the new `cn/locale/phase6.jsonl`.
  Exact contiguous range 150,000–153,999 (4,000 rows, no gaps or duplicates). Full CN QA:
  1 MARKUP finding on the first run (152790 `<临时文本>` tag kept verbatim), then
  **78,002 entries** including 2 overrides, **0 findings**. Phase 6: **4,000 / 9,176
  (43.59%)**; next idx **154,000**. **Overall: 78,000 / 159,176 unique strings (49.00%); CN-only
  in-game coverage 718,416 / 823,139 (87.28%)** (includes the parallel Phase 3 batch 5).
  Generated with `python tools/progress.py --profile cn --write`.

## Decisions / terminology (reuse in later Phase 6 batches)

Global names via `term_lookup.py` (kept in the Indonesian-global form):
Kilnfire Ridge (烧瓷岭), Battlecrest Slope (七伐坡), Witherwilds (荒蚀林), Palace of Annals
(春秋别馆), Peace Bell Tower (太平钟楼), Heaven's Pier (神仙渡), Mirkvale (晦谷), Halo Peak
(佛光顶), Hidden Mountain (不见山), Thousand-Buddha Village (千佛村), Revelry Hall (樊楼),
Ghostlight Market (鬼市子), Gracetown (承恩镇), Stonewash Strand (浣石浦), Silken Veil Society
(羽衣楼), Dragonbend Academy (封龙书院), Inkbound Order (文津馆), Imperial Service (武德司),
Purity Tower (太清楼), Kunning Hall / Funing Hall, Perception Forest (觉障林), Lone Cloud (孤云),
Wandering Tales (万事知), Battle Pass (战令), Echo Jade (长鸣玉), Oscillating Jade (振玉),
Silkbind Jade (牵丝玉), Echo Bead (长鸣珠), Attunement Stone (定音石), Jiazhong Melody
(夹钟奏雅), Hero Title (侠名), Bamboocut (破竹), Cloudshear (裁云), Flawless Guardian (御无缺),
Tiltrim (倾觞), Ivorybloom (浣花), Whirlsnow (梨雪), Beyond the Chill (不知寒), Nameless Sword /
Spear, Qiankun's Lock (乾坤定), Assist Point (帮帮值), Precept Value (守律值), Bond (默契值),
Sword Flower (剑花), Intoxicated→Mabuk (醉酒), Paralysis→Kelumpuhan, Exhausted→Kelelahan.

Choices made here (no global match, provisional):
- Editor labels stay literal and short: `战利品刷新点` → "titik respawn jarahan",
  `战令奖励-付费N级` → "hadiah Battle Pass-berbayar level N", `氛围` → "Suasana",
  `巡逻` → "Patroli", `宝箱` → "Peti harta", `世界等级` → "level dunia",
  `对应普通装备宝箱` → "sesuai peti perlengkapan biasa", `签到` → "Absen".
- 不见山 place names without a global match stay in the Hidden Mountain family
  ("Gua Karya Langit" for 天工地窟, "Tebing Burung Bertengger" for 栖鸟岩).
- 仙乡宴 → "Perjamuan Negeri Abadi" (global form); 乌鹭币 → "koin Wulu" (as in Phase 3);
  藏锋志 → "Catatan Menyimpan Ketajaman"; 夹缬 → "Jiaxie"; 同心码 → "Kode Sehati";
  万事知 stays "Wandering Tales" inside labels (matches global).
- Pinyin for CN-only persons/places (Feng Jisheng, Zhou Yuanxin, Qiancui Village, Yunjia'ao,
  Luancheng, Kanerzi, Bolangsha, Danya, Cangxian Ridge, Longhu Ridge, Laiseng Shore, Cixin Town, …).
- Protected tokens kept verbatim: `<临时文本>` (152790), `{罪行名}` (152238), `{}` / `{0}` /
  `{1}` / `$T(…)` / `%s` / `%d` / `#Y…#E` / `#R…#E` family; Han characters inside `{}`/`<>`
  are never translated.
- 153639 is a loading placeholder (`数据加载中` × 80, truncated mid-word); rendered as
  "Memuat data" × 80 + "Memu".
- Numbers: 万 → juta/ribu (1000万 → "10 juta", 5万 → "50 ribu"); dates written in Indonesian
  order; lunar month words (闰十一月初八) → "Tanggal delapan bulan sebelas kabisat".

## Batches 2–14 decisions (idx 154,000–159,175)

Content: still mostly internal editor/spawn/reward labels (`…-战利品刷新点N`, `S9–S15-战令奖励-免费/付费N级`,
`…对应普通装备宝箱`, `第N期总星级达到N星奖励`, `任意流派塔全3星通关-第N期`), plus NPC dialogue, Skill/gear
affix descriptions, ancient-text items (157011 Qin-slip *Rishu*, translated plainly) and lore.

Global names found with `term_lookup.py` (use these, not Pinyin):
- Gear sets: 倾觞 Tiltrim, 烟柳 Yanliu (no global hit, kept), 不知寒 Beyond the Chill, 浣花 Ivorybloom,
  梨雪 Whirlsnow, 桂月 Moonflare, 时雨 Rainwhisper, 玄甲 Ebonward, 飞隼 Hawkwing, 鹰扬 Eaglerise,
  易相 Formbend, 江凝 Calmwaters, 玉斗 Jadeware, 连星 Starweave, 抱玉 Jadeclasp, 燕归 Swallowcall,
  盈盏 Brimflow, 鸣珂 Reinchime, 尚义 Honorbound, 御无缺 Flawless Guardian; no hit (kept Pinyin):
  山倾 Shanqing, 急风 Jifeng, 啸玉 Xiaoyu, 破风·竹 Pofeng, 撼天 Hantian, 凌波步 Langkah Gelombang.
- Weapon skills: 积矩九剑 Strategic Sword, 醉梦游春 / 九重春色 Everspring / Vernal Umbrella,
  十方破阵 Phalanxbane Blade, 嗟夫刀法 Thundercry Blade, 无名枪法 / 乾坤定 Nameless Spear / Qiankun's Lock,
  骑龙回马 Soaring Spin, 衍九矢 Ledakan Sembilan, 无相金身 Tubuh Emas, 摄星拿月 Celestial Seize,
  千军万马 Ribuan Pasukan, 惊涛掌 Palm Strike Master, 聚宝盆 Baskom Harta Karun.
- Streams: 鸣金 Bellstrike, 裂石 Stonesplit, 破竹 Bamboocut, 牵丝 Silkbind (unchanged from earlier phases).
- 侠痕 → **Affix** (Phase 3 rule; 侠迹 stays "Jejak Pendekar"), 侠名 → Hero Title, 狂澜 Raging Tides,
  守律值 Nilai Precept, 短陌钱 Koin Commerce, 天地万籁 Universal Harmony, 四海笺 Adventure Slip,
  八音窍 Harmonic Core, 百业 Guild, 武林录 Jurnal, 博物志 Kompendium, 悬壶 Healer, 打铁花 Sparksmith,
  又一楼 Kedai Lain, 战令 Battle Pass.
- Places (global form): Blissful Retreat (不羡仙), Crimson Cliff (丹崖), Bamboo Abode (竹林旧居),
  Buddha Fort, Mercyheart Monastery, Sage's Knoll, Gleaming Abyss, Lingyin Temple, Wansheng Town,
  Confinement Tower, Bodhi Sea, Moteheart Haven, Velvet Shade, West Lake, Sunken City Lake,
  Dragon Gate Inn, Millennium Crossing. Pinyin kept for CN-only places: Desa Fenghe (丰和村),
  Gunung Biquan (壁泉山), Jalan Bambu Yunqi, Fumazhuang, Bolangsha, Luancheng, Huotuo.

Choices (provisional, no global match):
- 战利品刷新点 → "titik respawn jarahan"; 战令奖励-免费/付费N级 → "hadiah Battle Pass-gratis/berbayar level N";
  回溯 → "Penelusuran"; 万事知 stays "Wandering Tales"; 搜打撤 → "Cari-Tembak-Mundur";
  弈子 → "bidak", 小弈侠 → "Pendekar Catur Kecil", 乌鹭币 → "koin Wulu", 棋珍 → "Permata Catur";
  金拱门 (McDonald's collab) → "Golden Arches"; 金蟾 → "katak emas"; 复影砚 → "Tinta Bayangan Ganda".
- Date variants like `#R政和三年腊月初八1#E;…` (name/date typo tests): translated to the same
  Indonesian form three times; `…年…月…日` dates in Indonesian order; 闰N月 → "bulan N kabisat".
- 万 → ribu/juta (15万 → "150 ribu", 5w → "50 ribu", 100万 → "1 juta"); 9折 → "diskon 10%".
- 157971 (loading placeholder, truncated mid-phrase) → "Memuat data" × 9 with one "Memuat".
- Protected tokens kept verbatim: all `<…>` tags with Han text (155893, 156087, 156898, 157573,
  158204, 158421, 158537), `$T(5211)`/`$T(5224)`/`$T(2001)`, `{param_1}`, `{-1}`/`{-2}`/`{1}`, `%s`, `%d`.

Follow-ups noticed (not done here): batch 1 (150,000–153,999) and `phase2.jsonl` still contain a few
Pinyin/guessed forms for terms that have global names (e.g. "Mingjin" for 鸣金 in 16 Phase 2 rows,
"Shifang Zhenzhen" for 十方破阵 in batch 1) — harmonize in a later pass if wanted.

## Name consistency pass (2026-10-25)

1,893 rows of this phase rewritten so names match global ([[CN-Quick-Reference]] §1–2a, rule
locked the same day): places, NPCs, sects, Path names and Inner Ways were often written as
Pinyin ("Gunung Bujian", "Lembah Hui"), half-translated or translated literally ("Gunung Tak
Terlihat", "Persimpangan Milenium", "Bellstrike·Bayangan", "Teknik Batin") instead of the global
English name (Hidden Mountain, Mirkvale, Millennium Crossing, Bellstrike - Umbra, Inner Way).
Done with `python cn/tools/name_check.py --fix` (Pinyin variants, label segments that are exactly a
name, curated `_LITERAL`/`_SUFFIX` tables); idx unchanged, no new rows. Names listed earlier in
this log in an Indonesian or Pinyin form are superseded by the global form. CN-only names now
stay Pinyin (never Indonesian). Remaining mismatches: `python cn/tools/name_check.py --idx <range>`.

