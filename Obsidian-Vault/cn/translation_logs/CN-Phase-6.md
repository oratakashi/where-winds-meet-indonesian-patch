# CN Phase 6 Translation Log (idx 150,000–159,175)

Session history and terminology for `cn/locale/phase6.jsonl`. Rules:
[[CN-Quick-Reference]]; live progress: [[CN-Current-Status]]; precedents: [[CN-Phase-3]].

Phase 6 holds the rarest strings of the CN-only corpus (frequency order: 1–2 occurrences each),
mostly internal editor/spawn labels (`…-战利品刷新点N`, `S14-战令奖励-付费26级`,
`…对应普通装备宝箱`), plus NPC dialogue, Skill/gear descriptions, notices and lore.

## Batches

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
