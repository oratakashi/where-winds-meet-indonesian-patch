# CN Phase 3 Translation Log (idx 60,000–89,999)

Session history and terminology for `cn/locale/phase3.jsonl`. Rules:
[[CN-Quick-Reference]]; live progress: [[CN-Current-Status]]; precedents: [[CN-Phase-2]].

## Batches

- **2026-10-24 — Phase 3 batch 5 (idx 72,000–73,999, 2,000 rows).** Mandarin → Indonesian
  in three parallel ranges (72,000–72,666; 72,667–73,333; 73,334–73,999), appended to
  `cn/locale/phase3.jsonl`. Exact range 60,000–73,999 verified without gaps or duplicates;
  previous 12,000 rows preserved byte-for-byte. Newline/tab counts match the source; numeric
  differences reviewed as Chinese numeral conversion, dates/months or ordinal wording.
  Final full CN QA: **74,002 entries** including 2 overrides, **0 findings**. Phase 3:
  **14,000 / 30,000 (46.67%)**; next idx **74,000**. **Overall: 74,000 / 159,176 unique
  strings (46.49%); CN-only in-game coverage 714,416 / 823,139 (86.79%).** Generated with
  `python tools/progress.py --profile cn --write`. Includes internal editor labels, excludes
  the global bridge. Terminology and provisional CN-only names: [[CN-Phase-3]].

- **2026-10-24 — Phase 3 batch 4, idx 68,000–71,999 (4,000 rows).** Mandarin → Indonesian,
  translated directly in the main session (no subagents) as eight 500-row chunks, each merged
  with an idx-continuity and newline-count check against the source, appended to
  `cn/locale/phase3.jsonl`. Exact contiguous range 60,000–71,999 verified (12,000 rows, no
  gaps or duplicates); previous 8,000 rows untouched (diff = +4,000 lines). QA first runs
  flagged 2 MARKUP findings (68396 `<石匠>` tag, 70485 missing `#E` typo) fixed by hand; final
  full CN QA: **72,002 entries** including 2 overrides, **0 findings**. Phase 3:
  **12,000 / 30,000 (40.00%)**; next idx **72,000**. **Overall: 72,000 / 159,176 unique strings
  (45.23%); CN-only in-game coverage 710,416 / 823,139 (86.31%).** Generated with
  `python tools/progress.py --profile cn --write`; includes internal editor labels, excludes
  the global bridge.

- **2026-10-23 — Phase 3 batch 3, idx 66,000–67,999 (2,000 rows).** Mandarin → Indonesian
  in three parallel ranges (66,000–66,666; 66,667–67,333; 67,334–67,999), appended to
  `cn/locale/phase3.jsonl`. Exact contiguous range verified; previous 6,000 rows preserved
  byte-for-byte. Full CN QA: **64,002 entries** including 2 overrides, **0 findings**.
  Newline/tab counts and runtime markers preserved; numeric changes reviewed as Chinese
  numeral conversion, date formatting or word order. Phase 3: **8,000 / 30,000 (26.67%)**;
  next idx **68,000**. **Overall: 64,000 / 159,176 unique strings (40.21%); CN-only in-game
  coverage 694,416 / 823,139 (84.36%).** Overall includes concurrently completed Phase 2
  work. Generated with `python tools/progress.py --profile cn --write`; includes internal
  editor labels, excludes the global bridge. Log: [[CN-Phase-3]].

- **2026-10-23 — Phase 3 batch 2, idx 64,000–65,999 (2,000 rows).** Mandarin → Indonesian
  in three parallel ranges (64,000–64,666; 64,667–65,333; 65,334–65,999), appended to
  `cn/locale/phase3.jsonl`. Exact contiguous range verified; previous 4,000 rows preserved
  byte-for-byte. Full CN QA: **58,002 entries** including 2 overrides, **0 findings**.
  Source newline/tab counts and runtime markers preserved; digit differences reviewed as
  Chinese-numeral conversions or word/date order changes. Phase 3: **6,000 / 30,000 (20.00%)**;
  next idx **66,000**. **Overall: 58,000 / 159,176 unique strings (36.44%); CN-only in-game
  coverage 682,416 / 823,139 (82.90%).** Generated with
  `python tools/progress.py --profile cn --write`; includes internal editor labels,
  excludes the global bridge. Log: [[CN-Phase-3]].

- **2026-10-23 — batch 1, idx 60,000–63,999 (4,000 rows).** Mandarin → Indonesian,
  translated in three parallel ranges (60,000–61,333, 61,334–62,666,
  62,667–63,999). Output: `cn/locale/phase3.jsonl`. Next idx: **64,000**.
  Exact contiguous range verified (4,000 rows, no gaps or duplicates); digit differences
  reviewed as Chinese-numeral conversions or date/word-order changes. Newline/tab counts
  and runtime markers checked. Full CN QA: **52,002 entries** including 2 overrides,
  **0 findings**. Phase 3: **4,000 / 30,000 (13.33%)**.
  **Overall: 52,000 / 159,176 unique strings (32.67%); CN-only in-game coverage
  670,416 / 823,139 (81.45%).** Generated with
  `python tools/progress.py --profile cn --write`; includes internal editor labels,
  excludes the global bridge.

## Batch 1 decisions

- Mostly internal editor labels (NPC/spawn, reward tables, activity and quest labels):
  translate briefly and literally; developer and CN-only person names use Pinyin.
- Global proper nouns follow the shared-key lookup; tokens, numeric identifiers,
  poem wrappers, newlines and tabs retain their source structure.

- Confirmed global names include Mohist Hill, Jadebrook Mountain, South Imperial Garden,
  Granary of Plenty, Wheatwind Bazaar, Skyward City Ruins, Aureate Pavilion, Eternal
  Solitude Hall, Buddha Fort, The Void King, Bodhi Sea, Jadestream Valley, Coffin Master,
  Perception Forest, Well of Heaven, Unbound Cavern, Sky Shuttle, Hollow Vale, An Liuli,
  Velvet Shade, Derndale, Mirkvale, Cloud Steps, Mirage, Hexi and Jade Gate Pass.
- Lookup labels: 文津馆 → Perintah Terikat Tinta; 太医署 → Imperial Medical Academy;
  太清楼 → Menara Kesucian; 天水集 → Skybrim Market; 丝竹雅韵 → Melodi yang Anggun;
  聆音 → Gema; 万相集 → Gallery; 金蟾 → Kodok Emas.
- CN-only forms: 封龙书院 → Fenglong Academy; 月桂林 → Yuegui Forest;
  千年渡 → Qiannian Ferry; 寒刃寸拳 → Bilah Dingin Pukulan Satu Inci;
  守势寒刃 → Bilah Dingin Bertahan; 怒虎无双 → Harimau Murka Tiada Tara;
  玄金甲 → Zirah Emas Hitam. These remain provisional until an official global match exists.
- Bare hashtags with Chinese text (61799, 63116) use a space after `#` before the
  translated text, preventing an accidental `#S`/`#K` markup token. Existing actual
  tokens remain verbatim. An extra trailing newline at 60138 was removed after review.

## Batch 2 decisions

- Mostly internal reward/entity/spawn/editor labels, plus gameplay and lore text. Literal
  concise labels; dialogue follows gue/lo. Full lore and repeated loading text retained.
- Global lookup confirmed Ghostlight Market, Hidden Mountain, Divinecraft Dungeon,
  Mercyheart Town, Jade Serpent Hall, Grand Imperial Temple, Cleardew Terrace, Steed's Pass,
  Netherworld Crossing, Lion Barrow, Jadestream Valley, Blissful Retreat, Moonveil Mountain,
  Ever-Normal Granary, Thousand-Buddha Village, Mount Tai Blizzard, Heaven's Pier,
  Gleaming Abyss, Mirkvale, Mistveil City, Masterwood Hamlet, Moteheart Haven,
  Qingshou Hall, Kaifeng Imperial Palace and Pipa Chasm.
- Shared labels follow lookup: 万事知 → Kisah-Kisah Pengembara; 妙妙喵 → Meow Meow;
  柳岸 → Tepi Dahan Willow; 凉州曲 → Melodi Liangzhou. Equipment families use
  Stonesplit - Strength/Might, Bellstrike - Umbra/Splendor, Silkbind - Jade/Wings/Deluge,
  Bamboocut - Draught/Dust/Wind. Resources include Echo Jade, Oscillating Jade, Ebon Iron.
- Harmonized 高级礼包 → paket istimewa, chest 品 → Grade, 南塔寺 → Nanta Temple,
  北塔寺 → Beita Temple across ranges.
- Provisional CN-only forms: 归燕 → Kembalinya Walet; 月明渡 → Yueming Ferry;
  云栖竹境 → Yunqi Zhujing; 风禾村 → Fenghe Cun; 紫玉苑 → Ziyu Yuan;
  山倾 → Gunung Runtuh; 霜结 → Beku Embun; 左藏库 → Zuocang Treasury;
  丰收祠 → Fengshou Shrine; 古寺 → Gusi; 玄月教 → Xuanyue; 武元 → Wuyuan.
  Revisit these when an official global counterpart appears.
- Bare hashtag at idx 65226 uses a space before translated text to avoid introducing a
  formatting token. At idx 65581, Dark Iron's source level 5 and quantity 5 are both retained
  with the official name Ebon Iron. No gaps, duplicate idx, or untranslated Han text in batch.

## Batch 3 decisions

- Full dialogue/lore and internal labels translated; repeated loading text preserved with
  the same repeat count. Removed extra trailing newlines at 66312, 66397, 66458, 66494,
  66573 before merging. Placeholder Han at 66837 is preserved verbatim inside braces.
- Lookup confirmed Jiangnan State, Nine Mortal Ways, Daoist yang Tidur, Inti Harmonis,
  Tuning Stone, Moonflare, Diving Bell, Guild War, Blade Momentum, Inner Power,
  Cloud Sea Starfall, Inky Plum, Re-attunement Stone, Cry Coin, Crownleek Village,
  Imperial Artisan Court, Strategic Sword, Tally Mohist, Koin Zhou, Wandering Ark,
  Whitecrown Fortress, Coolshade Hall, Dreamfall Cliff, Hawkwing and Ikan Giok.
- Perception Forest ranks follow shared-key Indonesian lookup: 断水 Ahli, 仗剑 Pemula,
  开山 Mahir, 斩风 Veteran, 流云 Master, 飞花 Legenda, 无我 Mythic.
- Composite equipment labels at 67654 use Bronze Skyfall: Master-Tempered and
  Serpent's Hidden Core: Master-Tempered; 百淬 uses Master-Tempered.
- Provisional CN-only names: 啸风峙玉 Angin Menderu Giok Menjulang; 破风·竹 Pemecah
  Angin - Bambu; 小兰澳 Xiao Lan'ao; 汴梁南村 Bianliang South Village; 幽篁里 Youhuangli;
  武城王庙 Wucheng King Temple; 鸟雀呼晴 Burung Memanggil Cerah; 淬锋玉 Tempering Jade;
  天工造物 Kreasi Tiangong; 佛光顶 Foguang Peak; 铁壁谷 Tiebi Valley; 丧魂林 Sanghun Forest;
  青瓶居士 Qingping Jushi. Revisit when official global counterparts appear.
- 小柳青衣 at 66557 has no direct lookup; Liu Qingyi kecil follows the verified base name
  柳青衣. This internal label remains provisional due to limited context.

## Batch 4 decisions

- Batch is ~95% internal editor labels (NPC/entity/spawn names, Battle Pass tiers, achievement
  numbers, sign-in dates, reward tables); translated literally and short per
  [[CN-Quick-Reference]] §4. Repeated formulas fixed across all chunks: `S7-战令奖励-免费89级`
  → "S7-hadiah Battle Pass-gratis Level 89" (付费 → berbayar); `NN阶X·部位·回流奖励` →
  "TierNN X·Zirah Dada/Liontin/Cincin/Pelindung Lengan/Pelindung Tulang Kering·hadiah kembali";
  `…回流装备自选匣` → "Kotak pilihan perlengkapan kembali TierNN·…"; `…掉落世界等级N` →
  "Drop … World Level N"; `…单人模式等级N装备掉落` → "… mode solo drop perlengkapan levelN";
  `YYYY年M月D日签到` → "Login harian D Bulan YYYY"; `成就编号N` → "Nomor pencapaianN".
- Equipment families from lookup, kept English: 抱玉 Jadeclasp, 燕归 Swallowcall, 梨雪
  Whirlsnow, 飞隼 Hawkwing, 破竹 Bamboocut, 牵丝 Silkbind, 裂石 Stonesplit, 鸣金 Bellstrike;
  Echo Jade (长鸣玉), Echo Bead → "Manik Gema" (长鸣珠), Oscillating Jade (振玉),
  Ebon Iron (乌金铁), Wandering Tales → "Kisah-Kisah Pengembara" (万事知), Journal (武林录
  in later rows; earlier rows' "Catatan Bela Diri" kept as is).
- Perception Forest ranks (as in batch 3): 断水 Ahli, 仗剑 Pemula, 开山 Mahir, 斩风 Veteran,
  流云 Master, 飞花 Legenda, 无我 Mythic.
- Tokens kept verbatim because the QA brace/angle-tag check compares them: `<石匠>` (68396),
  `{任意}` (69427), `{任意染色}` (69981), `{当前：#Y{1}次#E}` (70984).
- Long repeated placeholder strings ("数据加载中" ×24 at 68899, ×90 at 70446) kept with the exact
  repeat count as "Data sedang dimuat" ×N. Idx 69135 (Song-dynasty essay on strategy) and
  71049 (nine linked rings) translated in full.
- Provisional CN-only names (revisit when a global counterpart appears): 啸风峙玉 Angin Menderu
  Giok Menjulang (kept from batch 3); 不肝 → "Tanpa Grind" (71860); 惊才/绝艳 → "Bakat
  Mengejutkan"/"Anggun Mutlak" (twin-dancer boss skills); 云家岙 Yunjia'ao; 水碓 → "penumbuk air".

## Batch 5 decisions

- Mostly internal reward/entity/spawn labels, with dialogue, UI, a face-synthesis consent
  notice (72721), full poetry (73614), lore (73320, 73888), and talent instructions (73763).
  Long text translated in full; 73320 retains its unfinished source ending. Earlier batch
  formulas retained. Three accidental trailing newlines removed before merging.
- Shared-key lookup supersedes provisional forms where found: 封龙书院 → Dragonbend Academy
  (earlier Fenglong Academy); 白马驿 → Stasiun Baima; 侠痕 → Affix; 侠境 → Hero's Realm;
  破竹·鸢 → Bamboocut - Kite; 裂石·钧 → Stonesplit - Strength; 裂石·威 → Stonesplit - Might.
  愁铃客 → Weeping Reaper follows contextual `--contains` examples (bare lookup returned
  an unhelpful Unknown). These findings apply to this batch; older rows were left untouched.
- Other confirmed forms: Harmoni Semesta, Satu Daun, Satu Kehidupan, Unfated Tian,
  Ghostlight Market, Imperial Medical Academy, Ever-Normal Granary, Spiritroost Crag,
  Skyward City Ruins, Karez Well, Sunken City Lake, Imperial Service, The Palace Guard Office,
  Roaring Sands, Wellspring Town, Willowshade Village, Wansheng Town, Plainfield,
  Teras Skyreach, Clearsky Tower, Tebing yang Terengah, Biksu Tranquillus,
  Tuan Ikan Mas Merah, Ivorybloom, Calmwaters, Formbend, Rainwhisper, Finesteed Hamlet,
  Raging Tides, Moteheart Haven, Fishwood River, Doa yang Sia-Sia, Mata Merah Wabah,
  Heavenfall, Ghost Revelry Hall, Ikan Giok, Inti Harmonis, Danau Moxin, Dermaga Tianjing,
  Vernal Umbrella, Nektar Bulan, Legendary Melodic Stone: Vanguard, Feathergrass,
  Busur Sieger: Rapidfire, Evercloud Bow: Longshot, Heavenquaker Spear.
- Existing CN-only precedents retained: Fenghe Cun (风禾村), Yueming Ferry,
  Wucheng King Temple, Zuocang Treasury, Yuegui Forest, Qiniaoyan.
- Provisional CN-only places/organizations: 夜雨林 Yeyu Forest; 灵隐集 Lingyin Bazaar;
  临江渡口 Linjiang Crossing; 宜圣殿 Yisheng Hall; 法华禅院 Fahua Monastery;
  丰和村 Fenghe Village (distinct spelling from 风禾村); 决胜都 Jueshengdu;
  中山遗址 Zhongshan Ruins; 蛟吞院 Jiaotun Court; 燕云别馆 Yanyun Annex; 五熟行 Wushu Guild.
- Provisional people: 王守 Wang Shou; 屠余 Tu Yu; 蒲景庐 Pu Jinglu; 罗参 Luo Shen;
  张存秀 Zhang Cunxiu; 董乃信 Dong Naixin; 罗晴妍 Luo Qingyan; 秦细 Qin Xi;
  孟秀兰 Meng Xiulan; 简行之 Jian Xingzhi; 霏斐 Feifei; 单白 Shan Bai; 乔杰冬 Qiao Jiedong;
  金络 Jinluo; 松苓 Songling; 珍凝 Zhenning. Other Pinyin forms in this range:
  Bu Xin, Lin Jingxing, You Sheji, Cheng Qiaoqiao, Zheng Zonglin, Zheng Wenyu.
- Provisional labels/items: 红妆·丹煞 Riasan Merah - Petaka Merah; 百目莲 Teratai Seratus Mata;
  金拱门 Lengkung Emas; 飞去来器 Senjata Bumerang; 戍铠 Zirah Penjaga;
  藏锋初试 Ujian Awal Pedang Tersarung; 乌鹭币 Koin Wulu; 讨北盟檄 Maklumat Melawan Aliansi Utara;
  坠火 Api Jatuh; 两心同 Dua Hati Bersatu; 装藏轮 Zangzang Wheel;
  婵娟泪 Air Mata Rembulan; 月照欢颜 Rembulan Menyinari Senyum;
  寸拳·不屈 Pukulan Satu Inci·Pantang Menyerah; 蓑隐 Tersembunyi Berjas Jerami;
  神龙·爆燃 Naga Ilahi·Ledakan Api. Revisit provisional forms when a global match appears.
- Protected Han tokens kept verbatim: `<出殡百姓>` (72142), `{午清风·时装}{染色}` (72808),
  `<醉劲>` (73153), `<心通百兽>` (73198). Remaining text has no Han leftovers.
- Numeric audit: Chinese Grade/Level words become digits; month numbers become Indonesian
  month names; 第3次 becomes "ketiga"; date order follows Indonesian convention.
  These explain all numeric-multiset differences. Newline/tab counts match for every new row.
