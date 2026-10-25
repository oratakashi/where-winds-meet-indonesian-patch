# CN Phase 2 Translation Log (idx 30,000–59,999)

Session history and terminology for `cn/locale/phase2.jsonl`. Consolidated rules:
[[CN-Quick-Reference]]; live progress: [[CN-Current-Status]]; precedents: [[CN-Phase-1]].

## Batches

- **2026-10-22 — batch 1, idx 30,000–33,999 (4,000 rows).** Translated by subagents in
  parallel from the Mandarin source in `cn/translation_work/unique_strings.jsonl` (chunks
  30,000–30,999, 31,000–31,999, 32,000–32,999 as four 250-row pieces, 33,000–33,999). The first
  attempt at 32,000–32,999 as a single 1,000-row chunk failed twice with an API content-filter
  error, so it was re-run in 250-row pieces. Output: `cn/locale/phase2.jsonl`. QA
  (`qa_check --profile cn --locale`) 0 findings. Next idx: **34,000**.

- **2026-10-22 — batch 2, idx 34,000–35,999 (2,000 rows).** Eight subagents, 250 rows each
  (34,000–34,249 … 35,750–35,999). QA 0 findings. 34,725 `{confirm_text="…"}` kept verbatim
  (the whole `{…}` is one markup token). Next idx: **36,000**.

- **2026-10-22 — batch 3, idx 36,000–39,999 (4,000 rows).** Sixteen subagents, 250 rows each
  (36,000–36,249 … 39,750–39,999). The 37,750 chunk came back misaligned (translations shifted
  one row from ~37,816 to 37,963, caught by QA MARKUP findings and a digit-sequence comparison);
  37,800–37,999 was redone in two 100-row pieces built programmatically by idx. QA 18,002
  entries, 0 findings. Next idx: **40,000**.

- **2026-10-22 — batch 4, idx 40,000–43,999 (4,000 rows).** Sixteen subagents, 250 rows each
  (40,000–40,249 … 43,750–43,999), each told to build its output by iterating source idx.
  Merge verified idx-exact (no gaps/shifts). QA first run: 4 MARKUP findings (40,135 and 42,437
  `<…>` content, 41,003 `《…》` turned into `<…>`, 42,395 `#…` hashtag) fixed by hand; final QA
  30,002 entries, 0 findings. Next idx: **44,000**.
- **2026-10-23 — batch 5, idx 44,000–47,999 (4,000 rows).** Sixteen subagents, 250 rows each
  (44,000–44,249 … 47,750–47,999). Merge verified idx-exact; digit-sequence comparison showed no
  shifts (44,558 and 44,674 lost a trailing digit, restored by hand). 梓匠居 in 47,009, 47,127,
  47,217 set to "Masterwood Hamlet" (global name). QA 48,002 entries, 0 findings. Next idx: **48,000**.

- **2026-10-23 — batch 7, idx 52,000–55,999 (4,000 rows).** Translated directly in the main
  session (no subagents), in eight 500-row chunks → `cn/locale/phase2.jsonl`. Each chunk was
  checked against the source by comparing digit sequences per row; the first chunk drifted
  (rows 52,025, 52,215 and 52,262 had been skipped, shifting everything after), caught that way
  and rebuilt before merging. Final merge exact: 30,000–55,999, no gaps or duplicates. QA
  first run 6 MARKUP findings (52,719 `#屋`, 52,857 `<石匠>`, 54,141 `{任意}`, 54,235 `#R`/plain
  残红, 54,715 `#R`, 55,961 `#一`) fixed by hand; final QA 64,002 entries (includes the parallel
  Phase 3 work), 0 findings. Next idx: **56,000**.

- **2026-10-23 — batch 8, idx 56,000–59,999 (4,000 rows), phase complete.** Translated directly
  in the main session (no subagents), in eight 500-row chunks → `cn/locale/phase2.jsonl`. Each
  chunk was verified before merging (idx continuity, per-row digit sequences and newline counts);
  no drift this time. Final merge exact: 30,000–59,999, no gaps or duplicates. QA first run 2
  MARKUP findings (56,010 `<画师>`, 56,397 `{任意染色}`: Mandarin inside `<…>`/`{…}` stays
  verbatim), fixed by hand; final QA 68,002 entries (includes the parallel Phase 3 work), 0
  findings. Phase 2 is **30,000 / 30,000**. Next idx: Phase 3 at **68,000**.

## Translation decisions — 2026-10-22

- All freq-3 strings here are almost entirely internal editor labels (NPC / spawn / entity /
  ambience labels joined with `-`). Translated literally and briefly; person names → Pinyin
  without tones (`翟`=Zhai, `任`=Ren, `覃`=Qin; informal "Xiao X", "Lao X").
- Placeholder filler (`活动说明…`, `数据加载中…`) translated literally ("Penjelasan acara").
- Opaque `<…>` tags keep their Mandarin content verbatim (e.g. `<王府家丁>`, `<司籍>`,
  `<赤龙堂·堂众>`, `<对方沉默收摊中，不愿与你交谈>`) per the markup rule.
- Global forms reused via `term_lookup`: Hexi, Qinghe, Kaifeng, Hangzhou, Qingzhou, Velvet
  Shade, Hidden Mountain, Mirkvale, Aureate Pavilion, Silken Veil Society, Evershift Labyrinth,
  Mohist City, Path Tower (Stonesplit / Bamboocut / Silkbind / Bellstrike / Woodcutter), Echo
  Jade, Oscillating Jade, Zenith Box, Battle Pass, Madiao Cards.
- CN-only names chosen (reuse in later batches): 丰乐楼 "Fengle Tower"; 虫二侠迹 "Jejak
  Pendekar Chonger"; 玉真案 "Kasus Yuzhen"; 驱傩 "Ritual Nuo"; 南塔寺 "South Tower Temple";
  北盟遗址 "Reruntuhan Aliansi Utara"; 赵普宅 "Kediaman Zhao Pu"; 玄金甲 "Zirah Emas Hitam";
  鼓司 "Biro Genderang"; 无忧帮 "Geng Unbound Cavern"; 梦傀 "Boneka Mimpi"; 回鹘 "Uighur";
  吐蕃 "Tibet"; 草贼 "Perampok Rumput".
- Known inconsistencies to harmonize later (all editor labels, no player impact): 流派塔
  sub-labels (Pinyin vs. Indonesian meaning in 2c), 寒菌梦傀 / 胖梦傀 forms, 万事知
  ("Kisah-Kisah Pengembara" vs. "Wanshizhi" vs. "Wandering Tales"), 峰林区.
- Guesses: 镜菱花 "Bunga Cermin Water Caltrop" (32703), 会伤率 / 纳气 / 行会任务 (chunk 2a rows
  155, 189), items at 33506 and 33814.

- **2026-10-23 — batch 6, idx 48,000–51,999 (4,000 rows).** Sixteen subagents, 250 rows each
  (48,000–48,249 … 51,750–51,999). The first run hit the account usage limit; 15 of 16 agents
  were resumed after the reset and finished. Merge check (digit sequences src vs. translation)
  caught a swapped pair at 50,322/50,323 (fixed) and dropped digits at 51,102 and 51,918 (fixed);
  other digit differences are dates/Chinese numerals. QA 0 findings. Next idx: **52,000**.

## Batch 2 decisions (idx 34,000–35,999)

- Mostly editor labels again. Subagents did not all run `term_lookup`, so names are unharmonized:
  万事知 appears as "Wanshizhi" / "Kisah-Kisah Pengembara"; 流派塔 floors as Pinyin
  (Pozhu/Lieshi/Mingjin/Qiansi/Zhuomu + 风 Feng, 钧 Jun, 尘 Chen, 影 Ying…) or Indonesian
  ("Menara Aliran …", "Pecah Batu·Wei"); 嗟叹崖 as Jiete / Jietan / "Jurang Keluh Kesah";
  樊楼 as "Fan Tower" / "Revelry Hall" (global hit — prefer this); 玉壶园 "Taman Yuhu" /
  "Taman Guci Giok" / "Yuhu Garden"; 封桩库 Fengzhuang variants. All editor labels — harmonize later.
- New CN-only names: 松烟阁 "Paviliun Songyan"; 六尚局 "Biro Enam Pejabat"; 化怨榜 "Papan
  Penghapus Dendam"; 常平仓 "Changping Granary"; 无相皇 "Kaisar Tanpa Rupa"; 突厥 "Turk";
  杭西赏游社 "Perkumpulan Wisata Hangxi"; 灵台 "Observatorium Guanxiang"; 九流门 "Sekte Jiuliu";
  龙女泪 "Air Mata Putri Naga"; person names in Pinyin (Murong Wan, Shen Yilun, Li Zhaoxue,
  Zhai Xu, Cai Zhi…).

## Batch 3 decisions (idx 36,000–39,999)

- Still editor labels. Most agents skipped `term_lookup`, so names stay unharmonized (万事知,
  嗟叹崖 "Gasping Cliff"/"Jurang Jietan"/"Jurang Keluh Kesah", 樊楼 "Fan Tower"/"Revelry Hall",
  流派塔 floors Pinyin vs. Indonesian, 九流门 global "Nine Mortal Ways" vs. "Sekte Jiuliu").
- New CN-only names: 铁壁谷 "Lembah Tiebi"; 燕云别馆 "Wisma Yanyun"; 万人殿 "Aula Sepuluh Ribu
  Orang"; 太岳台 "Taiyue Terrace"; 怜花禅院 "Kuil Lianhua"; 束罪阁 "Paviliun Pengikat Dosa";
  武成王庙 "Kuil Raja Wucheng"; 石羊戍垒 "Benteng Shiyang"; 辩经堂 "Aula Debat Sutra";
  熔炉地牢 "Penjara Tungku"; 绣金楼 "Paviliun Sulam Emas"; 太平钟楼 "Menara Lonceng Taiping".
- Lesson: for chunks, tell agents to build output by iterating source idx, and compare digit
  sequences src vs. translation after merging to catch silent one-row shifts.

## Batch 4 decisions (idx 40,000–43,999)

- Still mostly editor labels (spawn/NPC/reward tables, quest/activity names). Agents were given
  the fixed global names (Bamboocut, Stonesplit, Bellstrike, Silkbind, Revelry Hall, Nine Mortal
  Ways, Jadeheart Garden, Kisah-Kisah Pengembara, Tebing yang Terengah) so these are consistent
  in this batch.
- Markup lesson: Mandarin `<…>` and `#…` text inside a line counts as a markup token for QA, so
  it must stay verbatim (e.g. 42,679 `<心通百兽>`, 42,437, 40,135 `<大宋官兵>`, hashtag in 42,395);
  `《…》` book-title marks must not become `<…>` (use quotes).
- New CN-only names seen: 常平堡 "Changping Fort"; 无忧洞 "Gua Wuyou" / 无忧帮 "Geng Wuyou"
  (vs. earlier "Geng Unbound Cavern" — editor label, harmonize later); 文津馆 "Wenjin Guan";
  中渡桥 "Jembatan Zhongdu"; 丰和村 "Desa Fenghe"; 花船集 "Pasar Perahu Bunga".

## Batch 5 decisions (idx 44,000–47,999)

- Still editor labels (NPC ambience/patrol, reward, activity tables). Dev/NPC names in Pinyin.
- New CN-only names: 铁壁谷 "Lembah Tiebi"; 漕帮 "Caobang"; 墨门 "Perguruan Mo"; 贺刀寨 "Benteng Hedao";
  佛爷寨 "Benteng Foye" (46,000 chunk has "Fo Ye Zhai" in 大秘境 labels — harmonize later);
  大秘境 "Rahasia Besar"; 乌鹭币 "koin Wulu"; 律石 "Batu Irama"; 鼓司 "Biro Genderang";
  九流门 "Sekte Sembilan Aliran"; 漱云笙 "Sheng Pembasuh Awan"; 左藏库 "Zuozang" (Gudang Zuozang).
- Tower sets with no global match translated literally (尘 Debu, 鸢 Elang, 虹 Pelangi, 玉塔 Menara Giok).

## Batch 6 decisions (idx 48,000–51,999)

- Editor labels again (check-in/gift tables, NPC gift packs, 大秘境 and 流派塔 spawn tables, dev
  names). Dates rendered Indonesian ("8 November 2025"); 签到 → "Absen"/"Check-in" (agents mixed
  both — harmonize later).
- New CN-only names: 武德司 "Wude Si"; 归义军 "Guiyi"; 玄元教 "Xuanyuan"; 绣金楼 "Paviliun Xiujin"
  (batch 5 used "Paviliun Sulam Emas" — harmonize); 封龙书院 "Akademi Fenglong"; 觉障林 "Hutan
  Juezhang"; 荧渊 "Yingyuan"; 寒渊玦 "Hanyuan Jue"; 左藏库 "Zuozang Ku"; 春蚕庄主 "Pemilik Ulat
  Sutra"; 求蜕护盾 "perisai Pelepasan Kulit"; 侠境 "Xiajing"; 冰心花 "Bunga Hati Es"; 金身 "Tubuh
  Emas"; 血祭 "Persembahan Darah"; 周元通宝 "Koin Zhouyuan"; 自选通宝 "Koin pilihan".

## Batch 7 decisions (idx 52,000–55,999)

- Editor labels again (Battle Pass reward rows, spawn/NPC ambience tables, 大秘境 and 流派塔 rows,
  museum/achievement numbering, dev names). Pattern kept from Phase 3: `S7-战令奖励-付费95级` →
  "S7-Hadiah Battle Pass-Berbayar Level 95", `…掉落世界等级N` → "Jatuhan … Level dunia N",
  `博物条目N,M档奖励` → "Entri Museum N, hadiah Tier M", `成就编号N` → "Nomor pencapaian N".
- Global names reused via `term_lookup.py`: 不见山 Hidden Mountain; 墨城 Mohist City; 万胜镇
  Wansheng Town; 醉花阴 Velvet Shade; 樊楼 Revelry Hall; 鬼市子 Ghostlight Market; 缩骨功
  Contortion; 太清楼 Purity Tower (menara kesucian in ID — kept English); 饮马隘 Steed's Pass;
  天水集 Skybrim Market; 神仙渡 Heaven's Pier; 坎儿井 Karez Well; 晦谷 Mirkvale; 嗟叹崖 Gasping
  Cliff; 凋朱颜 Fleeting Radiance; 绕梁之音 Resonating Melody; 长鸣玉 Echo Jade; 振玉 Oscillating
  Jade; 月华通宝 Koin Cahaya Bulan; 律准石 Tuning Stone; 素裳染粉 Bubuk Pewarna Softweave; 周元通宝
  Koin Zhou; 心法心得 Inner Way Notes; 武学心得 Martial Note; 奇术 Mystic Skills; 羽衣楼 Silken Veil
  Society; 一叶平生 One Leaf, One Life; 蒹葭苍苍 Reeds and Rushes.
- Resolved the earlier "Koin Zhouyuan"/"Koin pilihan" wording: 周元通宝 → "Koin Zhou"; 自选通宝
  stays "Tongbao pilihan" (no global hit).
- Markup lessons: Mandarin text glued to `#` (`#屋`, `#一`) or `<…>`/`{…}` placeholders (`<石匠>`,
  `{任意}`) counts as a token and must stay verbatim; `#R…#E` colour code must be kept as `#R`, not
  turned into `#Y`.
- New CN-only names: 封龙书院 "Dragonbend Academy" (global; batch 6 used "Akademi Fenglong" — harmonize later); 武德司 "Wude Si" (Pengawal rendah Wude Si);
  崇文院 "Akademi Chongwen"; 左藏库 "Gudang Zuozang"; 皇家军备区 "Area Perlengkapan Kerajaan";
  慈心山院 "Cixin Mountain Academy"; 梓匠居 "Masterwood Hamlet" (global name); 觉障林 "Hutan Juezhang"; 乌鹭币 "Koin Wulu"; 墨守之心 "Hati
  Penjaga Tinta"; 无忧帮 "Geng Wuyou"; 漕帮 "Caobang"; 玄元教 "Sekte
  Xuanyuan"; 绣金楼 "Paviliun Xiujin"; 万事知 "Kisah-Kisah Pengembara"; 妙妙喵 "Meow Meow"; 偷师
  "Pencurian Skill"; 九流门 "Nine Mortal Ways"; 佛爷寨 "Buddha Fort" (all global names, rewritten in a
  final pass over idx 52,000–55,999).

## Batch 8 decisions (idx 56,000–59,999)

- Editor labels again (Battle Pass / check-in / museum / achievement rows, spawn and NPC ambience
  tables, 大秘境 and 流派塔 rows, dev names). Patterns kept from batch 7: `S12-战令奖励-付费34级` →
  "S12-Hadiah Battle Pass-Berbayar Level 34"; `…掉落世界等级N` → "Jatuhan … level dunia N";
  `博物条目N,M档奖励` → "Entri Museum N, hadiah Tier M"; `成就编号N` → "Nomor pencapaian N";
  `XXXX年M月D日签到` → "Absen D Bulan XXXX"; `大秘境…第N层` → "Rahasia Besar … Lantai N".
- Global names reused via `term_lookup.py`: Luancheng; Charnel Rise 乱葬岗; Palace of Supreme Oneness
  太一宫; Yuchen Hall 玉宸殿; Wandering Ark 迷驿舟; Jade Gate Pass 玉门关; Mistveil City 隐雾之城 /
  Mistveil Prison 隐雾之狱; Sweetwine Tower 酒香塔; Heroic Chronicles 侠影录; Mohist Hill 墨山道;
  Jiangnan State 江南; Jadewood Court 琼林苑; Divinecraft Corridor 天工回廊; Granary of Plenty 太仓粟;
  Lady White 白娘子; Candlestick → "Kaki lilin"; Madiao Cards 叶子戏; Meow Meow 妙妙喵; Imperial Garden
  → "Taman Istana"; Purity Tower 太清楼; Revelry Hall 樊楼; Velvet Shade 醉花阴; Ghostlight Market 鬼市子.
- Decimal separator: dot (`1.68%`), matching the existing corpus (95 dot vs. a handful of comma uses,
  normalized).
- Idx 59,403 is a pasted Lua script (`hexm.client…`): code kept byte-for-byte (it contains
  `%s`/`%d` format tokens), only the five Chinese `--` comments were translated.
- `#946433…#E` and `#ffc89c2222#E` colour tokens kept exactly; `#R临时#E` → `#RSementara#E`.
- New CN-only names (reuse later): 肉鸽 "Roguelike"; 团圆堡 "Benteng Reuni"; 灵隐寺 "Kuil Lingyin";
  孤山寺 "Kuil Gushan"; 大相国寺 "Kuil Xiangguo Besar"; 建隆观 "Kuil Jianlong"; 白雀经阁 "Paviliun
  Sutra Burung Putih"; 福宁殿 "Aula Funing"; 庆寿殿 "Aula Qingshou"; 坤宁殿 "Aula Kunning"; 宜圣殿
  "Aula Yisheng"; 崇元殿 "Aula Chongyuan"; 升平楼 "Menara Shengping"; 稷下学宫 "Akademi Jixia";
  愁铃客 "Penjaga Lonceng Duka"; 怒虎无双 "Harimau Murka Tiada Tara"; 嗜血 "Haus Darah".

## Name consistency pass (2026-10-25)

8,708 rows of this phase rewritten so names match global ([[CN-Quick-Reference]] §1–2a, rule
locked the same day): places, NPCs, sects, Path names and Inner Ways were often written as
Pinyin ("Gunung Bujian", "Lembah Hui"), half-translated or translated literally ("Gunung Tak
Terlihat", "Persimpangan Milenium", "Bellstrike·Bayangan", "Teknik Batin") instead of the global
English name (Hidden Mountain, Mirkvale, Millennium Crossing, Bellstrike - Umbra, Inner Way).
Done with `python cn/tools/name_check.py --fix` (Pinyin variants, label segments that are exactly a
name, curated `_LITERAL`/`_SUFFIX` tables); idx unchanged, no new rows. Names listed earlier in
this log in an Indonesian or Pinyin form are superseded by the global form. CN-only names now
stay Pinyin (never Indonesian). Remaining mismatches: `python cn/tools/name_check.py --idx <range>`.

