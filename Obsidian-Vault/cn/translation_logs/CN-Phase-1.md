# CN Phase 1 Translation Log (idx 0–29,999)

Session history and terminology for `cn/locale/phase1.jsonl`. Consolidated rules:
[[CN-Quick-Reference]]; live progress: [[CN-Current-Status]].

## Batches

- **2026-10-23 — batch 7, idx 24,000–29,999 (6,000 rows), phase complete.**
  Six parallel ranges of 1,000 rows; recovered partial outputs reviewed against source
  before completing each range. Corrected the recovered idx 24,081 source mismatch.
  Shared-key lookup and cross-range reconciliation applied before append to
  `cn/locale/phase1.jsonl`. Commit pending. Exact final range **0–29,999** verified
  without gaps or duplicates; preceding 24,000 rows retained byte-for-byte.
  Phase 1: **30,000 / 30,000 (100.00%)**. Full CN QA: **44,002 entries**, including
  2 overrides; **0 findings**. Source newline/tab counts retained; all 16 numeric
  review flags were legitimate Chinese-numeral conversions, not lost quantities.
  **Overall: 44,000 / 159,176 unique strings (27.64%); CN-only in-game coverage
  654,416 / 823,139 (79.50%).** Generated with
  `python tools/progress.py --profile cn --write`; this batch adds 18,619 covered
  entries. Coverage includes internal labels and excludes the separate global bridge.
  Resume **Phase 2 at idx 44,000**.

- **2026-10-22 — batch 6, idx 20,000–23,999 (4,000 rows).** Three parallel ranges:
  20,000–21,333, 21,334–22,666 and 22,667–23,999. Validated each segment before append
  to `cn/locale/phase1.jsonl`; exact final range verified without gaps or duplicates and
  the preceding 20,000 rows retained byte-for-byte. Commit pending.
  Phase 1: **24,000 / 30,000 (80.00%)**; next idx **24,000**.
  Full CN QA: **38,002 entries**, including 2 overrides; **0 findings**.
  **Overall: 38,000 / 159,176 unique strings (23.87%); CN-only in-game coverage
  635,797 / 823,139 (77.24%).** Generated with
  `python tools/progress.py --profile cn --write`. Coverage includes internal editor
  labels and excludes the separate global bridge.

- **2026-10-22 — batch 5, idx 16,000–19,999 (4,000 rows).** Three parallel ranges:
  16,000–17,333, 17,334–18,666 and 18,667–19,999. Validated the merged batch before
  appending to `cn/locale/phase1.jsonl`; existing translations retained byte-for-byte.
  Commit pending. Phase 1: **20,000 / 30,000 (66.67%)**; next idx **20,000**.
  Exact contiguous merged range verified without duplicates or gaps. Full CN QA:
  **34,002 entries**, including 2 overrides; **0 findings**.
  **Overall: 34,000 / 159,176 unique strings (21.36%); CN-only in-game coverage
  619,797 / 823,139 (75.30%).** Generated with
  `python tools/progress.py --profile cn --write`, including concurrent Phase 2 progress.
  Coverage includes internal editor labels and excludes the separate global bridge.

- **2026-10-22 — batch 4, idx 12,000–15,999 (4,000 rows).** Three parallel ranges:
  12,000–13,333, 13,334–14,666 and 14,667–15,999. Validated temporary batches before
  appending to `cn/locale/phase1.jsonl`; previous translations retained. Commit pending.
  Phase 1: **16,000 / 30,000 (53.33%)**; next idx **16,000**. Exact contiguous range
  verified without duplicates or gaps. Full CN QA: **26,002 entries**, including 2 overrides;
  **0 findings**. **Overall: 26,000 / 159,176 unique strings (16.33%); CN-only in-game
  coverage 595,797 / 823,139 (72.38%).** Generated with
  `python tools/progress.py --profile cn --write`; coverage includes internal labels and
  excludes the separate global bridge.

- **2026-10-22 — batch 3, idx 8,000–11,999 (4,000 rows).** Three parallel ranges:
  8,000–9,333, 9,334–10,666 and 10,667–11,999. Validated temporary batches before appending
  to `cn/locale/phase1.jsonl`; existing rows retained. Commit pending.
  Phase 1: **12,000 / 30,000 (40.00%)**; next idx **12,000**. Exact contiguous merged
  range verified, without duplicates or gaps. Full CN QA: **22,002 entries**, including
  2 existing overrides; **0 findings**.
  **Overall: 22,000 / 159,176 unique strings (13.82%); CN-only in-game coverage
  576,403 / 823,139 (70.02%).** Generated with
  `python tools/progress.py --profile cn --write`, including concurrent Phase 2 progress.
  Coverage includes internal labels and excludes the separate global bridge.

- **2026-10-22 — batch 2, idx 4,000–7,999 (4,000 rows).** Continued from batch 1,
  with parallel ranges 4,000–5,333, 5,334–6,666, and 6,667–7,999. Appended to
  `cn/locale/phase1.jsonl`; existing translations retained. Commit pending.
  Phase 1: **8,000 / 30,000 (26.67%)**; next idx **8,000**. Exact merged range verified
  contiguous, without duplicates or gaps. Full CN QA: **12,002 entries**, including
  2 existing overrides; **0 findings**.
  **Overall: 12,000 / 159,176 unique strings (7.54%); CN-only in-game coverage
  533,445 / 823,139 (64.81%).** Generated with
  `python tools/progress.py --profile cn --write`. Coverage includes internal labels and
  excludes the separate global bridge.

- **2026-10-22 — batch 1, idx 0–3,999 (4,000 rows).** Opening CN translation session,
  from the Mandarin source in `cn/translation_work/unique_strings.jsonl`. Three parallel
  translation ranges: 0–1,333, 1,334–2,666, and 2,667–3,999. Output:
  `cn/locale/phase1.jsonl`. Commit pending. Next idx: **4,000**.
  Full CN QA: **4,002 entries** including the 2 existing overrides; **0 findings**.
  Exact row count and contiguous idx 0–3,999 verified after merging.
  **Overall: 4,000 / 159,176 unique strings (2.51%); CN-only in-game coverage
  484,616 / 823,139 (58.87%).** Generated with
  `python tools/progress.py --profile cn --write`. This frequency-weighted coverage includes
  internal editor labels, and excludes the separate global bridge coverage.

## Translation decisions — 2026-10-22

- The opening corpus contains many internal editor labels, developer ownership fields,
  spawn labels, and test configurations. Translate their structure literally and briefly;
  retain identifiers, numeric suffixes, and technical code. Developer names use Pinyin.
  Examples: `生态物种` → "Spesies ekologi", `空气墙模板` → "Templat dinding tak terlihat",
  `奖励表-陈佳伟` → "Tabel hadiah-Chen Jiawei", `entity表-刘铭册` → "Tabel entity-Liu Mingce".
- Shared names follow the existing global Indonesian translation, checked with
  `cn/tools/term_lookup.py`, including its established English names. Recurring examples:
  `墨山道` → "Mohist Hill", `不见山` → "Hidden Mountain", `寿昌坊` → "Prosperity Haven",
  `九流门` → "Nine Mortal Ways", `醉花阴` → "Velvet Shade", `天一楼` → "Tianyi Tower",
  `百工坊` → "Imperial Artisan Court", `金明池` → "Jinming Pool".
- Currency/item terminology: `长鸣玉` → "Echo Jade", `振玉` → "Oscillating Jade",
  `宋元通宝` → "Koin Song", `定音石` → "Attunement Stone", `同天匣` → "Zenith Box",
  `近天匣` → "Sky Box". Inner Way names/stat labels follow the global conventions.
- Existing descriptive global names also remain consistent: `南烛公子` → "Kilau dari
  Selatan", `墨守之心` → "Hati yang Tak Berubah", `文津馆` → "Perintah Terikat Tinta",
  `解无间` → "Rahasia Makam", `归去来兮` → "Tempat Jiwa Kembali".
- `归义军` → "Guiyi Army": the exact shared-key lookup returned "Return Amy" / "Kembalikan
  Amy", an apparent source typo that does not describe the historical army. Retain the
  proper army name rather than propagating that error into CN.
- Dialogue uses gue/lo; UI and system text remains neutral. Preserve engine markup,
  placeholders, runtime variables, and complete opaque `<...>` tags verbatim.
- Distinguish `耐力` → "Endurance", `体力` → "Stamina", `真气` → "Qi", and
  `强韧` → "Tenacity", following the shared-key terminology.

## CN-only names introduced in batch 1

No global match was found for these terms. These forms are session precedents for later
CN batches, following [[CN-Quick-Reference]] §2.

| Source | Indonesian / Pinyin |
| --- | --- |
| 云栖竹径 | Yunqi Zhujing |
| 天京岸 | Tianjing Bank |
| 流水广场 | Flowing Water Plaza |
| 供奉司 | Gongfengsi |
| 惊天墨盗团 | Jingtian Modao Tuan |
| 灯缘尊者 | Yang Mulia Dengyuan |
| 雨落刃隐 | Hujan Turun Bilah Tersembunyi |
| 连珠破影 | Rangkaian Mutiara Memecah Bayangan |
| 断雁逐势 | Angsa Patah Mengejar Momentum |
| 引龙擒首 | Memancing Naga Menangkap Kepala |
| 铁绣球 | Bola sulam besi |
| 夜光杯鸟 | Burung cawan bercahaya |
| 汉白玉月光菇 | jamur cahaya bulan marmer putih |
| 山倾套 | Set Gunung Runtuh |
| 急风套 | Set Angin Kencang |
| 红妆 | Riasan Merah |
| 寒刃 | Bilah Dingin |
| 奔狼 | Serigala Berlari |
| 醉劲 | Tenaga Mabuk |
| 雷音 | Suara Guntur |
| 寸拳 | Pukulan Seinci |
| 血尽 | Darah Habis |
| 阴雾林 | Yinwu Lin |
| 云海小屋 | Yunhai Xiaowu |
| 骆驿集 | Luoyiji |

Developer-name examples for resuming the internal-label batches: 陈嘉铮 → Chen Jiazheng,
何纪希 → He Jixi, 周雨霖 → Zhou Yulin, 刘铭册 → Liu Mingce, 袁闵鸿皓 → Yuan Minhonghao.
Additional Pinyin precedents in the second range include Wu Xijin, Chang Ruizhi, Lin Ziyi,
Bao Siqun, Lan Jiarui, Mu Siyu, Shi Jiayi, He Wenjing, Wang Xuanliang, Qian Runhe, Shu Miao,
and Zhan Baodi. All source/name pairs remain indexed in `cn/locale/phase1.jsonl` and the
source corpus.

Additional person-name precedents from idx 2,667–3,999: Wang Yi, Xia Yixuan, Jin Chao,
Lin Yaoyao, Yu Zhipeng, Nangong Lei, Zhang Qinhan, Li Weijin, Jue An, and Gao Yu.

## Translation decisions — batch 2 (2026-10-22)

- Shared-key lookups: `牵丝·翊` → "Silkbind - Wings", `牵丝霖` → "Silkbind Deluge",
  `破竹尘` → "Debu Bamboocut", `天上来` → "Heavenfall", `一叶平生` →
  "Satu Daun, Satu Kehidupan", `北盟遗址` → "Northern Vow Ruins", `羽衣楼` →
  "Silken Veil Society", `飞天残垣` → "Skyward City Ruins".
- Places: `七伐坡` → "Battlecrest Slope", `龙虎寨` → "Tiger Fort", `绣金楼` →
  "Aureate Pavilion", `神仙渡` → "Heaven's Pier", `晦谷` → "Mirkvale", `千年渡` →
  "Persimpangan Milenium", `琼林苑` → "Jadewood Court", `佛爷寨` → "Buddha Fort",
  `白雀经阁` → "Sparrow Sutra Hall". Preserve the complete lore at idx 4,251 and 4,479.
- Equipment: `承音` → "Relaying", `承音装备` → "Relaid Gear", `传律` → "Modulasi",
  `玦` → "Cincin Giok", `变音` → "Re-attuning", `夹钟奏雅` → "Jiazhong Melody",
  `陌刀` → "Mo Blade", `变音石·绝尘` → "Re-attunement Stone: Dustless".
  The Tier 110 description (idx 5,693) retains 4.5%, the three-reset limit, seven-day
  interval, exclusions, and all paragraphs. The Tier 100/105 upgrade description
  (idx 6,148) retains both seasonal and normal requirements.
- Other global precedents: `护宅龙` → "Anjing Penjaga Rumah" (use lookup rather than
  the literal dragon), `么玉` → "Ikan Giok", `心力` → "Energi", `心法心得` →
  "Inner Way Notes", `侠境` → "Hero's Realm", `妙妙喵` → "Meow Meow", `和鸣玉` →
  "Harmony Jade". Shared materials also use "Ebon Iron" and "Softweave".
- Calendar labels preserve lunar month/day as "Tanggal N bulan M" without converting
  to Gregorian dates. Repeated loading/Event placeholders preserve every repetition,
  including partial fragments: idx 5,833 has 27 loading phrases; idx 7,051 has 20;
  idx 7,229 retains the shorter loading fragment and ellipses. Skill descriptions retain
  every action, duration, number and markup token; idx 7,584 retains its follow-up stages.

## CN-only names introduced in batch 2

No exact shared-key match was found for these terms; descriptive labels use Indonesian,
while people/places use the CN Pinyin convention. These are precedents for later batches.

| Source | Indonesian / Pinyin |
| --- | --- |
| 迟横 | Chi Heng |
| 迟顺 | Chi Shun |
| 伏虎寨 | Fuhu Zhai |
| 浪通天 | Lang Tongtian |
| 杭西赏游社 | Hangxi Shangyoushe |
| 魏芷惜 | Wei Zhixi |
| 于杏风 | Yu Xingfeng |
| 陆朗 | Lu Lang |
| 寿焓宇 | Shou Hanyu |
| 素贞 | Suzhen |
| 善慧 | Shanhui |
| 丹紫 | Danzi |
| 沈季文 | Shen Jiwen |
| 叶栖栖 | Ye Qiqi |
| 薛怀 | Xue Huai |
| 唐文睿 | Tang Wenrui |
| 涵洋 | Han Yang |
| 关大缸 | Guan Dagang |
| 净初 | Jing Chu |
| 吴断刀 | Wu Duandao |
| 松领游 | Song Lingyou |
| 葛远志 | Ge Yuanzhi |
| 青儿 | Qing'er |
| 咸宁坊 | Xianning Fang |
| 大绣巷 | Daxiu Alley |
| 石璃首 | Shilishou |
| 碧泉山 | Biquan Shan |
| 铁壁谷 | Tiebi Gu |
| 燕云别馆 | Yanyun Bieguan |
| 飞来峰 | Feilai Peak |
| 黄枫林 | Huangfeng Lin |
| 枯木崖 | Kumu Ya |
| 烽燧堡 | Fengsui Bao |
| 夜雨林 | Yeyu Lin |
| 琴魔 | Iblis Kecapi |
| 余息化影 | Sisa Napas Menjadi Bayangan |
| 千机并驱 | Seribu Mesin Bergerak Bersama |
| 白首同舟 | Bersama Satu Perahu Hingga Beruban |
| 青波棹歌 | Nyanyian Dayung Ombak Hijau |
| 只影逢春 | Sendiri Menyambut Musim Semi |
| 禅心掠影 | Bayangan Hati Zen |
| 荷来清风 | Angin Sejuk dari Teratai |
| 银靥星 | Bintang Perak |
| 惊雷引 | Pemandu Guntur |
| 雷音傲劫 | Guntur Menantang Bencana |
| 墨工雁 | angsa mekanis Mohist |
| 静神 | Menenangkan Jiwa |
| 月华寄梦 | Cahaya Bulan Mengantar Mimpi |
| 血祭红妆 | Pengorbanan Darah Riasan Merah |
| 劫火 | Api Bencana |
| 金缕裁芳 | Benang Emas Menjahit Keharuman |
| 清竹影 | Bayangan Bambu Jernih |
| 曲梅剑花 | Bunga Pedang Plum Melengkung |
| 流岚抱岳 | Kabut Mengalir Memeluk Gunung |
| 碧水间 | Di Antara Air Giok |
| 鸢羽衔珠 | Bulu Elang Menggenggam Mutiara |
| 守势如城 | Bertahan Sekokoh Benteng |
| 鹤游流泉 | Bangau Menjelajah Mata Air |
| 灼灼靥 | Lesung Pipi Membara |
| 叶来归 | Daun Kembali |
| 寒刃剑花 | Bunga Pedang Bilah Dingin |
| 狮子焚天 | Singa Membakar Langit |

Developer-name precedents in batch 2 include Hong Chao, Zhang Xiaosai, Yi Kaining,
Meng Mingheng, Guo Xinyue, Liang Binhui, Xu Chengyun, Yao Nanjun, Liu Zhiqi, Li Yijiang,
Sun Shiqiang, Qu Yinghao, Luo Yuhong, Wang Jing, Li Zengyue, Zhang Yuxi, and Sun Yu.
Source/name pairs remain indexed in the corpus and `cn/locale/phase1.jsonl`.

## Translation decisions — batch 3 (2026-10-22)

- Palace/spawn/editor labels retain all identifiers, suffixes and numeric values. Global
  lookups confirm Chongyuan/Changchun/Chongde/Wenming Hall, Imperial Medical Academy,
  Mohist City, Evershift Labyrinth, Gleaming Abyss, Jadestream Valley, Spiritroost Crag,
  Dragonbend Academy, Squarewall Hold, Apricot Village, Steed's Pass and Cleardew Terrace.
  Final review standardized 太医署 to "Imperial Medical Academy" and 破竹风 to "Bamboocut Wind".
- Other global precedents: 樊楼 → "Revelry Hall", 梨园 → "Masked Troupe", 天泉 →
  "Well of Heaven", 常平仓 → "Ever-Normal Granary", 玄元教 → "Xuanyuan Cult",
  无忧帮 → "Unbound Cavern", 金青 → "Bola Emas", 白马驿 → "Stasiun Baima",
  踏雪 → "Cakar Es", 牵丝·玉 → "Silkbind - Jade", 破竹·鸢 → "Bamboocut - Kite".
- Chouling Ke (愁铃客) and Qingping Jushi (青瓶居士) retain Pinyin: lookup only returned
  generic Unknown/field-Boss labels, without a reliable proper name. 闻灯 → "Wen Deng".
  拾友 → "teman Shiyou" provisionally (idx 10,374), pending player-visible context.
- Preserve complete historical lore at idx 8,575/9,036, story sequences at
  9,594/9,648/10,052/10,207/11,225, and poems at 8,579/9,784/10,628.
  Dialogue uses gue/lo; all Skill mechanics and quantities retained. Loading placeholders
  keep every repetition, including 17 at 8,235 and 19 at 11,656; 8,578 retains its partial
  final fragment. Remaining Mandarin is confined to opaque tokens <尚宫> at 9,042 and
  <尚寝> at 11,024, preserved verbatim for markup QA.

## CN-only names introduced in batch 3

No meaningful global proper-name match was found; Pinyin is provisional per
[[CN-Quick-Reference]]. Internal developer/person labels follow the same convention.

| Source | Indonesian / Pinyin |
| --- | --- |
| 孙琦 | Sun Qi |
| 舒舟阿木 | Shuzhou Amu |
| 宁宁 | Ningning |
| 王思杰 | Wang Sijie |
| 慧澄 | Huicheng |
| 崇师 | Chongshi |
| 小桃宝 | Xiao Taobao |
| 费弈子 | Feiyizi |
| 沙鸣峰 | Shaming Feng |
| 孟南 | Meng Nan |
| 志宏 | Zhi Hong |
| 失三七 | Shi Sanqi |
| 闻柳 | Wen Liu |
| 孟子涵 | Meng Zihan |
| 唐纤 | Tang Xian |
| 宿明光 | Su Mingguang |
| 九女峰 | Jiunv Feng |
| 灵隐集 | Lingyin Ji |
| 胡昂 | Hu Ang |
| 郑颚 | Zheng E |
| 单白 | Dan Bai |
| 栖鸟岛 | Qiniao Dao |
| 郝雪生 | Hao Xuesheng |
| 金博洋 | Jin Boyang |
| 倪文龙 | Ni Wenlong |
| 张小妹 | Zhang Xiaomei |
| 圆照 | Yuan Zhao |
| 丹青娘 | Danqing Niang |
| 右掖门 | Youye Gate (also internal label Youye Men) |
| 崇元门 | Chongyuan Gate |
| 文明门 | Wenming Gate |
| 新柳 | Dedalu Baru |
| 云起 | Awan Bangkit |
| 幽篁 | Rimbun Bambu |
| 赤血流丹 | Darah Merah Mengalir |
| 血覆千山 | Darah Menyelimuti Seribu Gunung |
| 梅幡闹 | Keramaian Panji Plum |
| 云海流星 | Meteor Lautan Awan |

## Translation decisions — batch 4 (2026-10-22)

- Shared-key lookups retain global forms: 青蛟堂 → "Jade Serpent Hall", 太岳台 →
  "Teras Skyreach", 酒香塔 → "Sweetwine Tower", 官财坊 → "Fortune Beyond Parlor",
  束罪阁 → "Confinement Tower", 六疾馆 → "Mercy Hall", 建龙观 → "Jianlong Monastery",
  霁雨楼 → "Clearsky Tower", 天京渡 → "Dermaga Tianjing", 望安屯 → "Peaceward Camp",
  涌泉镇 → "Wellspring Town", 黑财神 → "Dewa Ketamakan".
- Further global precedents: 嗟叹崖 → "Tebing yang Terengah", 升平桥 → "Rentang Selaras",
  丰禾村 → "Harvestfall Village", 骆驼驿 → "Stasiun Unta", 承恩镇 → "Gracetown",
  望月婵媛 → "Moongazing Maiden", 天地万籁 → "Harmoni Semesta", 太仓粟 →
  "Granary of Plenty", 麦香集 → "Wheatwind Bazaar", 花边小报 → "Kolom Gosip",
  长兴集 → "Crosswind Bazaar", 无比客店 → "Kedai Tanpa Tanding", 胡杨集 →
  "Poplar Bazaar", 隐峪 → "Derndale", 天不收 → "Unfated Tian", 连翘 → "Forsythia",
  朱邪骨勒 → "Zhuxie Gule", 楚十二郎 → "Chu the Twelfth", 千奇珏 → "Wonder Jade",
  宝铢 → "Token Harta", 斗财主 → "Tuan Tanah", 绕梁之音 → "Melodi Beresonansi".
- Skill/stat names follow lookup, including 裂石钧 → "Stonesplit Strength", 鸣金虹 →
  "Bellstrike Splendor", 羽化 → "Metamorfosis", and "Bellstrike Umbra", "Wind Sense",
  "Phantom Umbrella", "Fading Crimson", "Huangzhong Resonance" and "Skill Spesial".
- Retain all Chai Rong history and paragraphs at idx 13,435, Nangong Lei lore at 13,157,
  story text at 15,006/15,059 and the poem at 15,249. Combat descriptions retain every
  mechanic, number and duration, including 12,513/12,730/12,754/12,956/13,035/13,086,
  14,917 and 15,995. Loading placeholders retain repetitions and partial fragments
  (12,664: exactly 40 repetitions; 13,606/14,180: interrupted fragments preserved).
  Opaque <尚仪> at 12,630 and Skill link tags remain verbatim; TEXT/IMAGE tags at
  14,880 retain attributes while text between tags is translated.

## CN-only names introduced in batch 4

No exact shared-key match was found for these names. People and places use provisional
Pinyin; short descriptive labels use Indonesian. Source/name pairs are indexed by idx in
`cn/locale/phase1.jsonl` and the source corpus.

| Source | Indonesian / Pinyin |
| --- | --- |
| 林小乙 | Lin Xiaoyi |
| 韩徳栋 | Han Dedong |
| 刘鹿二 | Liu Lu'er |
| 杨晓岸 | Yang Xiao'an |
| 常领游 | Chang Lingyou |
| 弈子 | Yizi |
| 曾睿 | Zeng Rui |
| 毛雨昕 | Mao Yuxin |
| 墨缘生 | Mo Yuansheng |
| 侯味 | Hou Wei |
| 阿舒 | A Shu |
| 田棠 | Tian Tang |
| 聂秀秀 | Nie Xiuxiu |
| 罗有勇 | Luo Youyong |
| 杜峰林 | Du Fenglin |
| 兰化云 | Lan Huayun |
| 浊浪屿 | Zhuolang Yu |
| 双峰岛 | Shuangfeng Dao |
| 中山遗址 | Zhongshan Yizhi |
| 柴宗训 | Chai Zongxun |
| 共工台 | Gonggong Tai |
| 徐民穹 | Xu Minqiong |
| 王嘉珲 | Wang Jiahun |
| 李渔 | Li Yu |
| 水丹清 | Shui Danqing |
| 周四娘 | Zhou Siniang |
| 明善 | Ming Shan |
| 叶劲风 | Ye Jinfeng |
| 明念 | Ming Nian |
| 段青 | Duan Qing |
| 梁章 | Liang Zhang |
| 祝安顺 | Zhu Anshun |
| 韦无知 | Wei Wuzhi |
| 清泉 | Mata Air Jernih |
| 定慧檀香 | Cendana Meditasi Kebijaksanaan |
| 月边星 | Bintang di Sisi Bulan |
| 惑月 | Bulan Memikat |
| 七宝茶 | Teh Tujuh Pusaka |
| 烟笼十里 | Kabut Menyelimuti Sepuluh Li |
| 同气连枝 | Ranting Satu Akar |
| 裂霄 | Membelah Langit |
| 九章谒者 | Utusan Sembilan Bab |
| 齐政天枢 | Poros Langit Penata Pemerintahan |
| 福聚念珠 | Tasbih Pengumpul Berkah |
| 积微成筑 | Sedikit demi Sedikit Menjadi Bangunan |
| 泠泠七弦 | Tujuh Senar Berdenting Jernih |
| 萍水渡缘 | Jodoh di Persinggahan |

Additional CN-only person-name precedents in this batch: Chen Wenjing, Liang Xueqing,
Wu Heng, Shi Wan, Qiu Shan, Zheng Wanhe and Xun Sanniang.

## Translation decisions — batch 5 (2026-10-22)

- Shared-key lookups retain global names: Grand Imperial Temple, Academy of Scholars,
  Taman Kewaspadaan, Peachgrove Pass, Tepi Sungai Harum, Menara Kesucian, Skybrim Market,
  Stillheart Grove, Bloodscale Hall, Huzi Dike, Halo Peak, Fishwood River, Qiongqi Artificer,
  Murong Yuan, Palace of Supreme Oneness and Perception Forest.
- Additional lookup precedents: 临津渡 → "Harborlink Crossing", 玉壶园 → "Jadeheart Garden",
  封桩库 → "Sealed Treasury", 封丘村 → "Desa Verdant Hill", 天工阁 → "Divinecraft Pavilion",
  千佛村 → "Thousand-Buddha Village", 八音窍 → "Inti Harmonis", 悟真璞 → "Batu Giok Wawasan",
  长鸣珠 → "Manik Gema", 袅袅之音 → "Melodi yang Membekas", 磨喝乐 → "Boneka Tanah Liat".
  Other item/Skill names follow lookup: Babi Hutan Terakota, Piaoyao Battleboard, Flamelash,
  Ikan Isi Ilahi, Satin Langit, Inner Way Notes, Tuning Stone, Large Tuning Stone: Vanguard,
  Bubuk Pewarna Softweave, Pewarna Rambut Inkshade and Ember Ayam Beruntung.
- 锦芳苑 → "Jinfang Yuan" at idx 17,147/18,943: lookup only returned a generic Jiangnan
  Outpost placeholder, not a reliable proper name. 天宫阁 → "Tiangong Ge" at idx 16,144
  remains distinct from 天工阁 (Divinecraft Pavilion).
- Preserve full umbrella mechanics at 16,121/16,991, lore at 17,147, cooldown/shield text
  at 17,245/17,290, dialogue at 17,295, board-game rules at 18,172, story at 18,898 and
  poem/lore at 18,943. Loading strings retain every repetition and trailing fragment
  (18,305/18,910); Event placeholders retain repetitions at 19,953.
- Mandarin remains only inside opaque placeholders/tags: `{npc名}` at 17,001,
  `<尚食>` at 17,579, `<司苑>` at 18,545 and `<司宾>` at 19,872. Keep these verbatim.

## CN-only names introduced in batch 5

Provisional forms where lookup supplied no reliable proper-name match; full source/name
pairs remain indexed in the source corpus and `cn/locale/phase1.jsonl`.

| Source | Indonesian / Pinyin |
| --- | --- |
| 墨池 | Mochi |
| 谷羡雨 | Gu Xianyu |
| 陈涉 | Chen She |
| 董曰 | Dong Yue |
| 吴服齐 | Wu Fuqi |
| 释知山 | Shi Zhishan |
| 禹杨碧晗 | Yu Yangbihan |
| 求千刃 | Qiu Qianren |
| 丧魂林 | Sanghun Forest |
| 月明渡 | Yueming Ferry |
| 齐云社 | Qiyun Society |
| 舒韵 | Shu Yun |
| 河乐市 | Hele Shi |
| 大葱村 | Dacong Village |
| 怒虎无双 | Harimau Murka Tanpa Tanding |
| 定心清磬 | Lonceng Jernih Penenang Hati |
| 逐猎 | Perburuan |
| 飞翼 | Sayap Terbang |
| 藏贤掌 | Telapak Kebajikan Tersembunyi |
| 寒芒三剑 | Tiga Pedang Kilau Dingin |
| 寒星剑雨 | Hujan Pedang Bintang Dingin |
| 残血铁骨·绝 | Darah Tersisa Tulang Besi-Ultimate |
| 雪月凝光 | Cahaya Salju dan Bulan |
| 千江有水千江月 | Seribu Sungai Berair Seribu Bulan |
| 天雷辟易 | Guntur Menghalau Segala |
| 身外之饼 | Kue Duniawi |
| 漱云笙 | Seruling Awan |

Additional CN-only precedents: Wenxing Zhai, Huangfen Village, Shenming Zhai, Guanqian Ai,
Mo Choushao, Ximen Hualang, Zheliu Pavilion, Ye Rongmu, Yao Wen and Xing Tianming.

## Translation decisions — batch 6 (2026-10-22)

- Shared-key lookups and final corrections retain global forms: 怜花禅院 → "Bloomveil
  Monastery", 万古一人殿 → "Eternal Solitude Hall", 御猫大王 → "Cat Emperor",
  飞夷由 → "Tupai Peltwing", 玉鸣蝉 → "Jadesong Cicada", 鹰爪连凿 → "Serangan Cakar",
  药叉破魔 → "Yaksha Rush", 春水阁 → "Springwave Pavilion", 南屏山 → "Nanping Hill",
  怀信驿 → "Huaixin Post", 天水集 → "Skybrim Market". Other confirmed forms include
  Mubeli, Lin Terrace, Cao Manor, Bodhi Sea, Amidst Flowers, Sarang Judi Changle,
  Blissful Retreat and Fillet Ikan Iris.
- Further lookup precedents: 来苏蒙学 → "Hopewell Academy", 岱山 → "Mount Tai",
  万胜镇 → "Wansheng Town", 碧水云涛 → "Jadestream Valley", 鹿井丹泉 → "Deerwell Spring",
  弱水岸 → "Still Shore", 武成王庙 → "Martial Temple", 清心圃 → "Stillheart Grove",
  星隐野 → "Starveil Grassland", 黄沙客栈 → "Desert Twilight Inn", 陷城湖 → "Sunken City
  Lake", 稽古斋 → "Antiquity Hall", 驴道人 → "Daois Keledai", 容鸢 → "Murong Yuan".
  文津馆 remains "Perintah Terikat Tinta"; 明川药典 → "Panacea Fan";
  金玉手 → "Meridian Touch"; 红袖甲 → "Red Sleeve Armor".
  关大雁 → "Guan Dayan" and 柳陌 → "Liu Mo" are also confirmed shared-key names.
  毛不拔 uses the shared-key NPC form "Mao Si Pelit" at 23,893.
  Cosmetics at 23,881 follow the individual official/global names; 踏莎行 at 22,949
  follows "Menyeberangi Padang Rumput". The outfit title 花闻诏 at 22,789 uses
  "Titah yang Didengar Bunga", distinct from provisional person-name Pinyin.
- Preserve full Chishan Bu lore at 20,246, historical strategy text (five paragraphs)
  at 21,632, lore at 22,952, and stories at 23,022/23,732. Mechanical descriptions
  retain all actions, quantities and durations, including 20,080/20,668/20,724/20,980,
  21,198/21,234, 23,452/23,647/23,943. The cinematic developer note at 20,216 is retained.
- IMAGE/TEXT tags at 20,511 remain intact while their text nodes are translated.
  Opaque Mandarin tags <司灯> (21,822), <武德司-亲事官> (22,526) and <司赞> (23,064)
  stay verbatim. Loading placeholders at 21,301/23,169 preserve repetitions and
  incomplete source fragments.

## CN-only names introduced in batch 6

Provisional forms where no reliable global match was found; source/name pairs remain
indexed in the corpus and `cn/locale/phase1.jsonl`.

| Source | Indonesian / Pinyin |
| --- | --- |
| 南山陌 | Nanshan Mo |
| 赤山埠 | Chishan Bu |
| 花闻诏 | Hua Wenzhao |
| 河市乐 | Heshile (distinct source order from 河乐市 / Hele Shi) |
| 金瓯鸟 | burung cawan emas |
| 玄冰重剑 | Pedang Besar Es Hitam |
| 青锋卷雪 | Bilah Hijau Menggulung Salju |
| 云霞染膏 | Pewarna Rambut Awan Senja (no exact match; do not assume Inkshade) |
| 风摇扇 | Kipas Bergoyang Angin |
| 天如水 | Langit Bagai Air |
| 接星汉 | Menjangkau Bima Sakti |
| 雷鸣 | Gemuruh Guntur |
| 斗魂 | Jiwa Tempur |
| 松烟阁 | Songyan Pavilion |
| 丰乐楼 | Fengle Lou |
| 月桂林 | Yuegui Lin |
| 栖居寺 | Qiju Temple |
| 熊蛰父 | Xiong Zhefu |
| 徐真 | Xu Zhen |
| 毛不拔 | Mao Si Pelit (NPC cerdas, shared-key) |
| 投胡河 | Touhu He |
| 葵花点穴手 | Tangan Totok Bunga Matahari |
| 牵丝戏 | Pertunjukan Benang Boneka |
| 气垣 | Dinding Qi |
| 破气斩 | Tebasan Pemecah Qi |

## Translation decisions — batch 7 (2026-10-23)

- Cross-range shared-key lookups confirm: 醉花阴 → "Velvet Shade", 大衍迷阵 →
  "Evershift Labyrinth", 不见山 → "Hidden Mountain", 梓匠居 → "Masterwood Hamlet",
  玉壶园 → "Jadeheart Garden", 封龙书院 → "Dragonbend Academy", 岱山 → "Mount Tai",
  青州 → "Qingzhou", 开封 → "Kaifeng". 嗟叹崖 follows the existing global Indonesian
  form "Tebing yang Terengah". These forms also apply inside internal compound labels.
- Context-dependent shared forms: 万事知 → "Kisah-Kisah Pengembara" for the feature
  ("Kompendium" where appropriate); 一叶平生 → "Satu Daun, Satu Kehidupan";
  碧水云涛 → "Jadestream Valley" for the place, "Jadecloud" for the cosmetic.
- Path labels retain 牵丝 → "Silkbind", 鸣金 → "Bellstrike", 裂石 → "Stonesplit",
  破竹 → "Bamboocut"; the available exact shared form for 斫木 is "Penebang Kayu".
- Combat terms: 强韧 → "Tenacity", 重击蓄力 → "Charged Heavy Attack". 内力 as a
  combat resource uses "Inner Power", following 内力不足 → "Inner Power tidak cukup";
  the disease "Stifled Inner Energy" and title "Qi Master" are separate contexts.
- Literal leading `#` must not acquire an accidental format code when followed by
  Indonesian text (`# Titik`, not `#Titik`). Source title brackets `《…》` must not be
  changed into `<…>` tags. Indonesian prose requires spaces around numbers and markup.
- Final compound lookups confirm 南塔寺 → "South Pagoda Temple", 青风阁 →
  "Green Wind Hall", 沧浪居 → "Canglang Lodge", 天一楼 → "Tianyi Tower", and
  栖鸟岩 → "Spiritroost Crag". These replace provisional literal/Pinyin forms in
  this batch. 问津藏书馆 has no shared match and consistently uses "Wenjin Library".
- 流派塔 uses the confirmed "Path Tower" feature name. 斫木·欢 consistently uses
  "Penebang Kayu - Mirth" (shared suffix 欢 → "Mirth"); 斫木·幽 uses
  "Penebang Kayu - Sunyi". 啸风峙玉 is CN-only and consistently uses
  "Giok Menantang Deru Angin". 无相皇 follows the shared name "The Void King".
- 免伤 follows the shared DMG Reduction/Damage Reduction label, including 28,365;
  do not infer complete immunity from the Mandarin label. That description retains
  the 1-second cast, resurrection, three heals, landing damage, 100% Max HP team
  shield, no resurrection-charge consumption, and Super Armor.


## Batch 7 terminology and CN-only names by range

Shared-key forms take precedence over provisional names. The final compound lookups
above resolve cross-range variants; source/output correspondence is preserved by idx.

### idx 24,000–24,999

- 大衍迷阵 = Evershift Labyrinth; 万事知 = Kisah-Kisah Pengembara; 武林录 = Jurnal.
- 博浪沙 = Roaring Sands; 天涯客 = Pengembara; 寒香寻 = Han Xiangxun.
- 春蚕庄主 = Ketua Spring Silkworm Manor; 裴航 = Pei Hang; 陈子奚 = Chen Zixi; 无相皇 = The Void King; 鬼公子 = Ghost Master.
- 迷驿舟 = Wandering Ark; 放排翁 = Old Raftsman; 虞侯 = Perwira Staf; 锦毛狐 = Rubah Sutra.
- 紫芝真人 = Guru Zizhi; 郝善良 = Hao Si Baik Hati; 冯如之 = Feng Ruzhi; 齐阿霜 = Qi Ashuang; 陈慎 = Chen Shen; 郑大安 = Zheng Da'an; 乔红药 = Qiao Hongyao; 刘强 = Liu Qiang; 慕容晚 = Murong Wan; 莫画柳 = Mo Hualiu.
- 风雪岱山 = Mount Tai Blizzard; 鸿沟古渠 = Grand Canal; 百草野 = Verdant Wilds; 聪艺大观 = Pameran Kecerdikan; 花船集 = Pasar Perahu Bunga.
- 棋墨粥 = Bubur Terakhir; 藏针短匕 = Belati dengan Jarum Tersembunyi; 武魄匣 = Peti Martial Spirit; 定音石·飞将 = Attunement Stone: Vanguard (exact lookup takes precedence over similarly named large stone).
- 天地兼鸣 = Gema Penciptaan; 曲径通幽 = Berkelok Menuju Pengasingan; 飞天舞 = Tarian Langit; 行路难 = Perjalanan Berat; 静夜思 = Dalam Kesunyian Malam; 花间集 = Amidst Flowers.
- 天虎军 = Harimau dalam Rantai; 绣金卫 = Penjaga Aureate; 天一楼 = Tianyi Tower; 酒泉镇 = Jiuquan.
- 强韧 = Tenacity; 重击蓄力 = Charged Heavy Attack; 轻击蓄力 = Light Attack Charged; 免伤 = Damage Reduction; 霸体 = Super Armor. idx24029 uses Max HP, perisai HP, Stamina and these exact mechanics.
- Confirmed place forms also include Halo Peak, Hidden Mountain, Dragonbend Academy, Northern Vow Ruins, Jadeheart Garden, Jadestream Valley, Skyward City Ruins, Roaring Sands, Prosperity Haven, Tepi Sungai Harum, Dermaga Tianjing, Gunung Dragonbend, Kaifeng Prefecture, Imperial Medical Academy, Martial Temple, Jade Gate Pass, Well of Heaven, Qingshou Hall, Funing Hall, Kunning Hall, Jinming Pool and Baojin Tower.

#### CN-only/provisional names and labels

Reuse established log forms where present: Shen Jiwen, Jing Chu, Feilai Peak, Tiebi Gu, Yanyun Bieguan, Wenxing Zhai, Shenming Zhai, Hele Shi (河乐市), Heshile (河市乐), Yuegui Lin, Fengle Lou, Jinfang/Tiandistrict conventions. 荒坟村 follows existing Huangfen Village precedent. 小柳青衣 has no exact hit; retain Xiaoliu Qingyi internal label rather than silently dropping 小.

No exact shared-key hit: 奔雷式 = Jurus Guntur Berderap; 妙手有备 = Tangan Terampil Siap Sedia; 水运天文台 = Observatorium Tenaga Air; 鸟纹玉衡 = Giok Yuheng Motif Burung; 玄金甲 = Zirah Emas Hitam; 卧佛听松 = Wofo Tingsong (location); 斫木·幽 = Penebang Kayu - Sunyi (斫木 shared ID Penebang Kayu, no exact path-name hit; 幽 descriptive suffix translated). 大雄宝殿 is descriptive Mahavira Hall.

Other provisional person/developer labels retain untoned Pinyin, including Han Qingyang, Mi Qian'er, Mu Zhiwei, Yang Xin, Zhang Quan, An Yi, Kang Miyi, He Da, Xu Yiyi, Kong Jie, Song Wu, Zhang Meile, Moqi Anshan, Pan Xiaojin, Shen Xiaoran, Jiang Shijiu, Xie Aolin, Daozhi, Amu, Di Ping, He Miaoyi, Sun Changping, Liu Baiyu, Su Da'an, Shui Dexiang, Mu Tugui, Qizhao, Li Song, Kan'erzi, Zhang Shi, Gu Xiang, Shen Yu, He Ran, Daohui, Zhixia, Sha Yi, Chen Chunfen, Bai Kong'er, Li Hongyun, Liu Laosan, Huo Fang, Xu Xiao, Zhou Tayun, Yu Wan, Shao Qian, Qi Zhihuan, Qi Zhiheng, Zhi Yuyan, Su Muqing, Yu Jiexi, Shen Pingbo, Hu Jianli, A Tan, Mu Xiangsong, Su Jinshi, Xiang Jie, Mo Gui, Chen Jingu, Wang Qishun, Zhai Nian, An Ning, Shi Liu, Qin Tianqing, Zhai Xu, Huan Yun, Kang Wobai, Wu Lv, Xu Ningcui, An Lexi, Hao Dayun, Yu Dexian, Cheng Qi and Wen Shijun. Full source/translation correspondence remains recoverable by idx.

### idx 25,000–25,999

- Combat: 内力 = Inner Power; 重击蓄力 = Charged Heavy Attack; 最大气血值 = Max HP. 免伤 follows shared lookup DMG Reduction. 寒冰伤害 = Frost DMG provisionally (no exact lookup hit). 斫木·欢 = Penebang Kayu - Mirth per parent shared-suffix decision; 斫木·幽 = Penebang Kayu - Sunyi.
- Confirmed global names include Path Tower, Netherworld Crossing, Ghostlight Market, Jade Gate Pass, Spiritroost Crag, Hollow Vale, Palace of Annals, Fairgrounds, Grand Canal, Prosperity Haven, Dreamfall Cliff, Moonveil Mountain, Mount Phoenix, Willowshade Village, Tepi Dahan Willow, Distrik Hall of State, Steady Grain Granary, Salju Emas, Swallowgram, Thunder Fluff, and Kampanye Qingzhou.
- Name corrections from lookup: 贺庭筠 = He Tingjun; 李娲 = Li Wo; 乔阿香 = Qiao Xiang; 塔兹莫里 = Tazmoli; 线娘 = Xianniang; 丝娘 = Siniang; 蚕娘 = Nyonya Ulat Sutra; 阿云妹 = Yun; 知娴 = Zhixian; 阿沛 = A'pei; 塞诸葛 = Zhuge Hebat; 昙香 = Tanxiang; 阿丹 = A'dan; 阿青 = Qing; 周伙计 = Pelayan Zhou; 闻悟 = Wenwu; 听言 = Tingyan; 陈老拐 = Kakek Chen.
- Lion Barrow also has global Makam Singa precedent; retained Makam Singa in these labels.
- Opaque Mandarin tags preserved at 25723 <司记>, 25949 <青溪弟子>, 25980 <樱桃果农>. 25075 uses '# Titik' to avoid inventing #T; 25158 uses quotes, not angle brackets.

#### Provisional CN-only names and labels

No reliable exact global proper-name match was found for these queried names; reuse indexed source/output pairs for consistency:

| Mandarin | Chosen form |
| --- | --- |
| 百草先生 | Tuan Baicao |
| 尤勇 | You Yong |
| 阿宽 | A Kuan |
| 齐志桓 | Qi Zhihuan |
| 慕容秋 | Murong Qiu |
| 韦昂 | Wei Ang |
| 崔芷 | Cui Zhi |
| 陆昱 | Lu Yu |
| 吕声 | Lv Sheng |
| 沈拓拓 | Shen Tuotuo |
| 陈鼎康 | Chen Dingkang |
| 齐沧海 | Qi Canghai |
| 张力 | Zhang Li |
| 赵蒲 | Zhao Pu |
| 高盛 | Gao Sheng |
| 阿欢 | A Huan |
| 释智修 | Shi Zhixiu |
| 南塔寺 | South Pagoda Temple |
| 问津藏书馆 | Wenjin Library |
| 天地堂 | Tiandi Hall |
| 青风阁 | Green Wind Hall |
| 南天门 | Nantian Gate |
| 惊雷圃 | Jinglei Pu (global results were generic outpost placeholders, not reliable names) |
| 寒刃寸拳 | Pukulan Pendek Bilah Dingin |

Other provisional Pinyin components occurring within internal labels: Song Xian, Xiaozhang/Xiao Zhang, Tianye, Fenglong Shan, Linjie Pavilion, Yuma Hang, Shilou, Fuma Zhuang, Huaiquan Yu, Guishui, Xian Er'er, Taiyin. Existing phase-log names such as Fengle Lou, Yeyu Lin, Biquan Shan, Tiebi Gu, Tiangong Ge, Dacong Village and Yuegui Lin were reused.

### idx 26,000–26,999

- Batch term_lookup queries covered people, locations, sects, skills, quest titles and resources. Parent-provided exact forms were applied as well. Feature 万事知 uses Kisah-Kisah Pengembara.
- Proper-name corrections include 乌公 → Gagak; 小灵通 → Loper Koran Cerdik; 小栗 → Xiaoli; 六一 → Six-One; 小花蕾 → Hualei; 巧言 → Qiaoyan; 皮老四 → Pi Keempat; 小禄 → Little Lu; 墨雪儿 → Mo Xue; 吴织娘 → Penenun Wu; 尹杜娘 → Bibi Yin Du; 张实诚 → Zhang the Honest; 翟二叔 → Zhai Er; 闻人雅 → Wen Renya; 关野 → Magi si Jujur; 洛神 → Water Lady; 张朵儿 → Zhang Duo; 吕嘉南 → Lyu Jianan; 子豆花 → Bunga Kedelai; 燕喜儿 → Yan Xier; 鸿 → Brant; 小米 → Mi; 小磊 → Lei; 仇衙内 → Squireling Qiu; 从不少 → Bu Shao; 江心鬼 → River Ghost; 周老铁 → Zhou Tie; 大曹 → Big Cao; 阿念 → A'nian; 萨里合 → Sali He.
- Multiple lookup variants: 墨听澜 uses literal matching Mo Tinglan rather than cross-key Huo Tinglan; 霍听澜 uses Huo Tinglan. 小月鹿 uses Xiao Yuelu. 史大人 uses Tuan Shi and 松老 uses Tetua Song, following global honorific rules even where lookup leaves the English title.
- Shared places: 中渡桥 Zhongdu Bridge; 隐月山 Moonveil Mountain; 千年渡 Persimpangan Milenium; 太平钟楼 Peace Bell Tower; 松树湾 Pine Bend; 慈心镇 Mercyheart Town; 白头城 Whitecrown Fortress; 酒泉镇 Jiuquan; 云华楼 Menara Nimbus; 飞花渡 Petalfall Crossing; 四季宅邸 Four Seasons Residence; 又一楼 Kedai Lain; 万应水肆 Panacea Pavilion; 达安村 Sorrowfield Village; 凤凰山 Mount Phoenix; 封龙山 Gunung Dragonbend. 杨柳岸 and 柳岸 both use Tepi Dahan Willow. 青溪 as sect uses Silver Needle.
- Quest/feature titles follow lookup: 解无间 Rahasia Makam; 追光的人 Ngengat Menuju Kilauan; 红尘无眼 Buta akan Dunia; 酒逢知己 Companion; 归去来兮 Tempat Jiwa Kembali; 天地不仁 Dunia Ini Kejam; 生如归舟 Kembali ke Pantai; 工不应求 Tenaga Kerja Langka; 冰上菁华 Es di Lutut; 海棠醉梦 Renungan Bunga Bermekaran; 静夜思 Dalam Kesunyian Malam; 逐浪光阴 Menunggangi Gelombang; 踏岳寻珍 Berkelana Mencari Harta; 敲醒鹅滴心 Angsa-kan Hatimu; 迷驿舟 Wandering Ark; 辞生无泪 Perpisahan Tanpa Air Mata; 有美一人 Lone Beauty; 缘生缘灭 Perpisahan Takdir; 葱王争霸 Kontes Crownleek; 欲望深渊 Abyss of Desire; 以利天下 For the Good of All.
- Shared items/resources: 木鸢 Elang Kayu; 百目摄心莲 Teratai Bermata Seratus; 梦中情猫 Kucing Mimpi; 黄河鬼棺 Peti Mati Sungai Kuning; 黑煞阴阳 Yin-Yang yang Mengerikan; 冰心花 Bunga Hati Beku; 羽绒草 Feathergrass; 辟邪行缠 Gelang Penangkal; 宋元通宝 Koin Song; 周元通宝 Koin Zhou; 侠名 Gelar Pahlawan.
- Path labels: Silkbind - Jade/Wings/Deluge; Bellstrike - Splendor/Umbra; Stonesplit - Strength/Might; Bamboocut - Wind/Dust. No exact or contains match for 斫木·欢 / 斫木·幽; use Penebang Kayu - Mirth / Penebang Kayu - Sunyi, preserving confirmed base 斫木 → Penebang Kayu.
- Combat/stat text: Max HP, Physical Attack, Charged Heavy Attack, DMG, Super Armor. 免伤 uses confirmed DMG Reduction rather than immunity. idx 26886 retains half-energy cost, 10 seconds, 5%, non-refresh and both release effects. idx 26657 retains 8%, 15 seconds and 10-second trigger interval.
- Full prose retained at 26228, 26275, 26359, 26385, 26447, 26641, 26886–26887. Dialogue uses gue/lo where personal pronouns occur. idx 26275 一刻 is rendered explicitly as a quarter hour. All embedded newlines retained. Date idx 26808 remains numeric 2026-8-19 00:00:00~9-14 4:59:59.

#### CN-only names / provisional forms

No reliable exact shared-key name was found for these. People and places use Pinyin, while descriptive titles/items use Indonesian. Existing phase precedents were reused where available.

| Source | Form |
| --- | --- |
| 无名樵 | Wuming Qiao |
| 田慧 | Tian Hui |
| 高盛 | Gao Sheng |
| 柴壮 | Chai Zhuang |
| 张增辉 | Zhang Zenghui |
| 汪东宁 | Wang Dongning |
| 鲁墨 | Lu Mo |
| 蔡猫猫 | Cai Maomao |
| 戚柚柚 | Qi Youyou |
| 陶自在 | Tao Zizai |
| 白草野 | Baicao Ye |
| 南塔寺 | South Pagoda Temple |
| 槐泉峪 | Huaiquan Yu |
| 沧浪居 | Canglang Lodge |
| 宝塔寺 | Baota Si |
| 问津藏书馆 | Wenjin Library |
| 雾隐林 | Wuyin Lin (distinct source from 隐雾林 / Mistveil Forest) |
| 青风阁 | Green Wind Hall |
| 北山街 | Jalan Beishan |
| 赤甲红 | Merah Zirah Kirmizi |
| 闲夏游心 | Bersantai di Musim Panas |
| 衣来衣往 | Pakaian Datang dan Pergi |
| 戏草弄尘 | Bermain Rumput dan Debu |
| 骨骼清奇·凡 / ·绝 | Bakat Tulang Istimewa-Biasa / -Ultimate |
| 痴儿何从 | Ke Mana Perginya Si Lugu |
| 啸风峙玉 | Giok Menantang Deru Angin |
| 星象推演 | Deduksi Astrologi |
| 东京小富 | Si Kaya Kecil Dongjing |
| 漱云笙 | Seruling Awan (prior Phase 1 precedent) |
| 同甘共苦 | Senasib Sepenanggungan |

### idx 27,000–27,999

Applied global/CN Quick Reference and CN Phase 1 log. Batched term_lookup over 1,634 source fragments, plus targeted exact and contains queries. Ignore misleading generic shared-key mappings when no reliable proper-name match exists.

- Path Tower suffixes: Penebang Kayu - Mirth; Penebang Kayu - Sunyi (parent's cross-range ruling).
- Other paths: Stonesplit Might / Stonesplit Strength; Bellstrike Splendor / Bellstrike Umbra; Debu Bamboocut / Bamboocut Wind; Silkbind Deluge / Silkbind Jade / Silkbind - Wings.
- Combat resource: Inner Power; stats Max HP, Stamina and Qi DMG. Dialogue gue/lo.
- Confirmed shared names: Masterwood Hamlet, Dragonbend Academy, Jadeheart Garden, Tebing yang Terengah, Pine Bend, Squarewall Hold, Persimpangan Milenium, Floral Expanse Beyond, Divinecraft Dungeon, Grand Canal, Spiritroost Crag, Apricot Village, Wellspring Town, Roaring Sands, Mount Tai Blizzard, Sundara Land, Menara Kesucian, Flower Expanse, Moonveil Mountain, Petalfall Crossing, Prosperity Haven, Willowshade Village, Sorrowfield Village.
- Shared features and Quest names: Kartu Madiao, Warisan Jianghu, Momok Sang Pahlawan, Rahasia Makam, Cloud Steps, Hati yang Tak Berubah, Alasan Sempurna, Ratapan Berlapis Emas, Rangkaian Kebaikan, Tanpa Kata di Antara Kita, Much Ado About Mechanisms, Perfectly Matched, History's Dry Tracks, Spirits of the Lake.
- Shared people/entities with non-obvious forms: Fatalist Jiang, Little Leaf, Xiaowu, Little Mohe, Little Taobao, Istri Fu San, Cemberut, Bu San, Zheng the Frostbloom, Si Ketiga, Erwa, Old Glutton Wang, Big Sun, Daoheng, Tian Danui, Meng Ao, Jingyan, Si Emas, Tempat Tinta.
- Shared materials/resources: Echo Jade, Ebon Iron, Bubuk Pewarna Softweave, Huangzhong Resonance. Jadecloud cosmetic meaning is not used for the Jadestream Valley place labels.
- Generic internal labels remain brief and literal; preserve IDs and counts. CN-only people/places use Pinyin. CN-only descriptive Skill/item/Quest labels use Indonesian meanings.

#### CN-only / provisional context choices

- Thundercry Blade-Teguh (27053) uses the official core name with literal suffix.
- Mingxin medicinal manual (27113) has no exact shared match: Kitab Obat Mingxin.
- Shigang, Shenmu Ya, Fuyao Shan, South Pagoda Temple, Wucheng Wangmiao, Xuanquan Post, Huanghun Village and Yisheng Hall retain provisional place forms where exact lookup has no reliable match.
- Golden Colossus Taicu (27000) is confirmed; Event title Gelombang Petir Golden Colossus is descriptive (no exact shared title).
- Rantai Dewa Maut is provisional for the CN-only chain-story title.
- Ghost-realm race title retains Bunga Kepompong Alam Hantu: the sole shared hit was a generic Ghost Hand Xu Fu Boss label, not a reliable full title.
- Lonceng Duka Patah Hati (27950), Batu Truce Attunement-Pasir Bergolak (27434), Batu Melodi Emas-Dustless (27679), Cawan Giok Penuh Wangi (27065), and Kabut Mata Air Kediaman Sunyi (27437) are descriptive CN-only labels.
- 27586 preserves every repeated Event description and both truncated fragments.
- 27286 preserves its embedded newline; every translated row preserves source newline counts.

#### Selected source/name context pairs for future reuse

- 27006: "莲池弄鲤" -> "Memancing ikan koi di kolam teratai"
- 27041: "破门村密室祠堂钥匙-璇玑钥" -> "Kunci kuil leluhur ruang rahasia Brokegate Village-Kunci Armillary"
- 27053: "嗟夫刀法·坚" -> "Thundercry Blade-Teguh"
- 27065: "玉盏盈香" -> "Cawan Giok Penuh Wangi"
- 27070: "死神之锁-送行-龙彦而" -> "Rantai Dewa Maut-Mengantar kepergian-Long Yan'er"
- 27093: "韦青（狂澜男弟子）" -> "Wei Qing (murid pria Raging Tides)"
- 27112: "解无间-山崖血" -> "Rahasia Makam-Crimson Cliff Elixir"
- 27113: "偷师洞窟明心药典-一等心法宝箱" -> "Gua Skill Theft Kitab Obat Mingxin-Peti Harta Inner Way tingkat satu"
- 27139: "岑三味-苦海氛围组" -> "Cen Sanwei-Kelompok suasana Bodhi Sea"
- 27141: "冬季新增-宜圣殿- 明悠 高级宫女9" -> "Tambahan musim dingin-Yisheng Hall- Ming You Dayang Istana Pangkat Tinggi 9"
- 27146: "时楼-偷师-凌云踏-时楼屋檐" -> "Temporal Pavilion-Skill Theft-Cloud Steps-Atap Temporal Pavilion"
- 27167: "棠生-要塞-非战之罪" -> "Tangsheng-Benteng-Bukan Salah Perang"
- 27169: "制作夹缬" -> "Membuat kain celup jepit jiaxie"
- 27191: "万事知-寻二马记-马芒正" -> "Kisah-Kisah Pengembara-Di Mana Sang Makhluk Liar Berada-Ma Mangzheng"
- 27205: "观星台-转藏轮" -> "Observatorium-Roda kitab berputar"
- 27207: "鬼市子-付坟典小二-潘峥峥-氛围" -> "Ghostlight Market-Petugas The Buried Tomes-Pan Zhengzheng-Suasana"
- 27242: "猫猫·浮金" -> "Kucing-Emas Mengapung"
- 27255: "不见山-千年渡-侠客莫非同(全)" -> "Hidden Mountain-Persimpangan Milenium-Pendekar Mo Feitong (lengkap)"
- 27261: "话术npc-江缘起" -> "NPC percakapan-Fatalist Jiang"
- 27265: "P2-设法出逃-悬撞空交互" -> "P2-Mencari jalan kabur-Interaksi kosong tabrakan gantung"
- 27277: "钱庄-有名-管事-孙易算" -> "Bank-Bernama-Pengelola-Sun Yisuan"
- 27279: "市民商贩女-青年【卿小雨】" -> "Pedagang wanita warga kota-Muda [Qing Xiaoyu]"
- 27283: "破戒头陀乙（窃听-佛光玉1）" -> "Biksu Nakal B (menguping-Buddha's Light Jade 1)"
- 27284: "阿风" -> "A Feng"
- 27286: "范常安\n（狂澜男弟子）" -> "Fan Chang'an\n(disciple pria Raging Tides)"
- 27290: "伊寒会面椅子" -> "Kursi pertemuan Yi Han"
- 27296: "不见山-自习小院-杨璇儿" -> "Hidden Mountain-Halaman belajar mandiri-Yang Xuan'er"
- 27298: "峰林区-神木崖-葛木" -> "Suixiang-Shenmu Ya-Ge Mu"
- 27304: "迷驿舟-悬泉驿牌匾" -> "Wandering Ark-Plakat Xuanquan Post"
- 27305: "又一楼-向孤山走NPC2" -> "Kedai Lain-NPC berjalan ke Gushan 2"
- 27306: "梅隐真姐妹2" -> "Saudari Mei Yinzhen 2"
- 27310: "李海清-寿昌坊" -> "Li Haiqing-Prosperity Haven"
- 27317: "孙凤儿（女童）" -> "Sun Feng'er (anak perempuan)"
- 27325: "河西万事知-重蹈涸辙-墨山道门人玦" -> "Kisah-Kisah Pengembara Hexi-History's Dry Tracks-Anggota Moshan Dao Jue"
- 27328: "封龙书院-触发回忆的书信碎片二" -> "Dragonbend Academy-Potongan surat pemicu kenangan dua"
- 27334: "春节第五章-朱小禄" -> "Festival Musim Semi bab kelima-Zhu Xiaolu"
- 27335: "春秋别馆转动机关1" -> "Chunqiu Bieguan-Mekanisme berputar 1"
- 27336: "云游-昙香-昙香" -> "Pengembaraan-Tanxiang-Tanxiang"
- 27339: "冰上菁华-房华-冰上氛围3" -> "Es di Lutut-Fang Hua-Suasana es 3"
- 27340: "悬檐之下-丁慕儿（成年）" -> "Di Bawah Atap Orang Lain-Ding Mu'er (dewasa)"
- 27345: "武城王庙-山门-沿路上山的工匠" -> "Wucheng Wangmiao-Gerbang gunung-Pengrajin mendaki sepanjang jalan"
- 27358: "青州夜爬-面馆两人组-林不晚" -> "Pendakian malam Qingzhou-Duo kedai mi-Lin Buwan"
- 27369: "都城驿-有名-本地猫-伐开心" -> "Capital Relay-Bernama-Kucing setempat-Cemberut"
- 27382: "濮雨泊-博浪沙" -> "Pu Yubo-Roaring Sands"
- 27399: "话术氛围-荆老实" -> "Suasana percakapan-Jing yang Jujur"
- 27409: "翟煦家放置手札" -> "Letakkan catatan di rumah Zhai Xu"
- 27434: "加分符*1、止戈定音石·惊沙*1" -> "Talisman Bonus*1, Batu Truce Attunement-Pasir Bergolak*1"
- 27437: "幽居泉霭" -> "Kabut Mata Air Kediaman Sunyi"
- 27440: "何以为报-小阿田" -> "Utang Budi-Xiao A'tian"
- 27449: "【正式】河西P6-花坨坨" -> "[Resmi] Hexi P6-Hua Tuotuo"
- 27452: "猴魁" -> "Hou Kui"
- 27459: "天上来-沐三娘" -> "Heavenfall-Mu Sanniang"
- 27465: "花边小报-吴鑫" -> "Kolom Gosip-Wu Xin"
- 27479: "墨子号——纸鸢" -> "The Mozi-Layang-layang Kertas"
- 27484: "传闻，这灵隐新寺中有位玉兰仙子，仙子尤爱诗文，手稿亦字字珠玑……" -> "Konon, di New Lingyin Temple ada peri Yulan yang amat menyukai puisi dan sastra. Setiap kata dalam manuskripnya pun berharga bak mutiara..."
- 27485: "组织宿舍-从不散" -> "Asrama organisasi-Bu San"
- 27489: "宫女香蒲" -> "Dayang Xiangpu"
- 27491: "不见山滹沱南岸-万事知-霜厓心藏-醉倒的鹖鸡" -> "Hidden Mountain-Tepi selatan Hutuo-Kisah-Kisah Pengembara-Frostcliff Heart-Burung pegar mabuk tergeletak"
- 27492: "不见山-嗟叹崖-观添文" -> "Hidden Mountain-Tebing yang Terengah-Guan Tianwen"
- 27500: "话术npc-静衍" -> "NPC percakapan-Jingyan"
- 27535: "长生鹿仙人村长手信" -> "Surat kepala desa Changsheng Luxianren"
- 27549: "隐雾林碎片化交互-未寄出的书信" -> "Interaksi terpisah Yinwu Lin-Surat yang belum dikirim"
- 27559: "《离思赋》_任务交互物_野外boss闺怨诗" -> "\"Ratapan Berlapis Emas\"_Benda interaksi Quest_Puisi Kesepian Boss lapangan"
- 27560: "不见山奇遇-铸剑-姜仲" -> "Pertemuan Hidden Mountain-Pedang Terakhir-Jiang Zhong"
- 27571: "小剧场-尚不去" -> "Teater kecil-Shang Buqu"
- 27578: "何宵-中年" -> "He Xiao-Paruh baya"
- 27614: "仙灵所隐" -> "Tempat Roh Abadi Bersembunyi"
- 27619: "神秘据点-多人复刷-孟鄂" -> "Pos misterius-Respawn Co-op-Meng Ao"
- 27627: "【正式】河西P6-八字胡" -> "[Resmi] Hexi P6-Handlebar Hu"
- 27644: "长鸣玉×20，乌金铁·黄钟正响×2" -> "Echo Jade×20, Ebon Iron-Huangzhong Resonance×2"
- 27660: "缩骨功-day07-三娃" -> "Kontorsi-day07-Si Ketiga"
- 27674: "良人胡不归-柳五娘" -> "Kenapa Dia Belum Kembali-Liu Wuniang"
- 27679: "金妙音石·绝尘*1" -> "Batu Melodi Emas-Dustless*1"
- 27689: "击败指定的1名来生岸精英2" -> "Kalahkan 1 Elite Floral Expanse Beyond 2 yang ditentukan"
- 27693: "芸娘子倾慕成痴" -> "Nyonya Yun tergila-gila karena cinta"
- 27694: "npc助战-杜乔仙" -> "NPC bantuan tempur-Du Qiaoxian"
- 27698: "【叶子戏】建隆观-道珩(三人）-陪玩" -> "[Kartu Madiao] Jianlong Temple-Daoheng (tiga orang)-Teman bermain"
- 27699: "不见山-自习小院-薛吒" -> "Hidden Mountain-Halaman belajar mandiri-Xue Zha"
- 27702: "解无间-痴然木" -> "Rahasia Makam-Trancewood Elixir"
- 27715: "温盈献策-天真男孩" -> "Wen Ying Memberi Saran-Bocah polos"
- 27729: "一封书信，落款为“何曾”。 " -> "Sepucuk surat, ditandatangani \"He Zeng\". "
- 27742: "云槎飞车万事知-无嫌猜-修好的木鸢" -> "Kisah-Kisah Pengembara Sky Shuttle-Tanpa Kata di Antara Kita-Wooden Eagle yang Diperbaiki"
- 27752: "聆杏村氛围NPC-伏三娘" -> "NPC suasana Apricot Village-Istri Fu San"
- 27770: "千佛村：木鱼锤" -> "Thousand-Buddha Village: pemukul genderang kayu"
- 27785: "太仓粟-常平仓-镇守-阿郑" -> "Granary of Plenty-Ever-Normal Granary-Kampanye-A'zheng"
- 27816: "醉花阴-繁花居氛围-褚红衣" -> "Velvet Shade-Suasana Kediaman yang Ramai-Chu Hongyi"
- 27838: "摸鱼王-贾功则" -> "Raja Bermalas-malasan-Jia Gongze"
- 27853: "释清明" -> "Shi Qingming"
- 27858: "河市乐-观众2" -> "Riversong Theater-Penonton 2"
- 27871: "组织管家-温无双" -> "Pengurus organisasi-Wen Wushuang"
- 27874: "做梦撸猫-球儿" -> "Bermimpi Mengelus Kucing-Qiu'er"
- 27879: "使用太极将#Y木板击飞#E，避开观潮客的横扫技能" -> "Gunakan Tai Chi untuk #Ymementalkan papan kayu#E dan menghindari Skill sapuan Tidewatcher"
- 27888: "虫二酒" -> "Arak Chong'er"
- 27898: "青州汉墓-诸葛非非-罩袍" -> "Makam Han Qingzhou-Zhuge Feifei-Jubah luar"
- 27900: "祈盖（智能NPC）" -> "Qi Gai (NPC Cerdas)"
- 27907: "牵丝翊武学投放-温好" -> "Penempatan Skill Bela Diri Silkbind - Wings-Wen Hao"
- 27914: "醉花阴-外围前段氛围-李有鑫" -> "Velvet Shade-Suasana luar bagian depan-Li Youxin"
- 27919: "流派塔斫木·幽第7层-无忧帮朱邪骨勒小首领" -> "Path Tower Penebang Kayu - Sunyi lantai 7-Pemimpin kecil Zhuxie Gule Unbound Cavern"
- 27923: "胡杨集-米亮" -> "Poplar Bazaar-Mi Liang"
- 27926: "遗失的语录-慧静" -> "Lost Quotes-Huijing"
- 27942: "易容委托-染色秘方-薛之虔*" -> "Komisi penyamaran-Wajah Asli-Xue Zhiqian*"
- 27948: "望安屯-沙通天" -> "Peaceward Camp-Sha Tongtian"
- 27949: "典墨鹅-小庭" -> "Tempat Tinta-Xiaoting"
- 27950: "可在#H包裹快捷道具#E中配置断肠愁铃" -> "Lonceng Duka Patah Hati dapat dipasang di #HItem cepat Tas#E"
- 27961: "封龙用npc-丁苹" -> "NPC untuk Fenglong-Ding Ping"
- 27979: "十二年前-郑阮" -> "Dua belas tahun lalu-Zheng the Frostbloom"
- 27989: "蒙学考场内氛围NPC2杨阿庆男/8" -> "NPC suasana dalam ruang ujian dasar 2-Yang Aqing pria/8"
- 27991: "对话用伏禄宝" -> "Fu Lubao untuk dialog"
- 27994: "万事知-佳偶成单-段母" -> "Kisah-Kisah Pengembara-Perfectly Matched-Mrs. Duan"

### idx 28,000–28,999

Batch term_lookup.py queried 1,410 exact source segments plus focused compound/contains queries. Shared names use the existing global Indonesian form, including Evershift Labyrinth, Hidden Mountain, Masterwood Hamlet, Roaring Sands, Prosperity Haven, Netherworld Crossing, Velvet Shade, Dragonbend Academy, General's Shrine, Whitecrown Fortress, Five Dragons Temple, Jade Gate Pass, Imperial Artisan Court, South Gate Avenue, Goldflow Slope, Fairgrounds, Ghostlight Market, Old Mohist City, Blackiron Pangolin, Cloud Steps, Strategic Sword, Echo Jade, Oscillating Jade, Springtide Bell, Fieryheart, Yan Ying, Puding, Kepodang, Mentega and Little Goldie.

- 流派塔: Path Tower. 牵丝·霖: Silkbind - Deluge; 牵丝·翊: Silkbind - Wings.
- 斫木·欢: Penebang Kayu - Mirth; 斫木·幽: Penebang Kayu - Sunyi (cross-range decision from parent; 欢 lookup also confirms Mirth).
- 免伤 at 28365: DMG Reduction; 霸体: Super Armor; 耐力 at 28992: Endurance (exact shared lookup).
- HP / Max HP / Physical Attack / Critical Rate retain locked stat wording. Descriptive prose has spaces around numbers and format tags.
- 解无间: Rahasia Makam; 天上来: Heavenfall; 我的朋友: Untuk Sahabat, Sampai Akhir; 来自心底的声音: Hanya untuk Telingamu; 不义之约: Kehormatan di Antara Pencuri; 佳偶成单: Perfectly Matched; 比赛第一: No Gears, No Glory; 缓笔秘籍: Alasan Sempurna; 生而所执: Born For This; 话山论见: Adu Kecerdikan; 赌圣秘藏: Rahasia Sang Penjudi; 英雄与美: Kecantikan dan Pedang; 夜台良辰: Riddled Echoes; 墨道之声: Suara Mohist; 待办事项: Mengenangmu; 此木非梁: Kayu yang Tak Layak; 嗟来之食: Amal Sang Kucing; 长生鹿仙人: Doa yang Sia-Sia; 发丘道人: Rahasia yang Terguncang Petir.
- 九重·灼魂: Tier 9·Membakar Jiwa (shared 九重 = Tier 9; full compound no exact hit).
- CN-only descriptive terms: 风使罗盘 = Kompas Utusan Angin; 啸风峙玉 = Giok Menantang Deru Angin; 浮白载笔 = Meneguk Arak Membawa Kuas; 血次元 = Dimensi Darah; 墨守巳蛇 = Pertahanan Mohist Ular Si; 金制一品宝箱 = Peti Harta Emas Kelas Satu. No shared exact matches; translated meaning per CN rules.

#### CN-only / provisional Pinyin

No reliable standalone shared match for these names in the batch query; retain Pinyin and established phase-log precedents where applicable:

| Mandarin | Output |
| --- | --- |
| 支雨檐 | Zhi Yuyan |
| 魏器平 | Wei Qiping |
| 石小付 | Shi Xiaofu |
| 丁布董 | Ding Bu Dong (shared exact name, retained spelling) |
| 梓匠居 | Masterwood Hamlet (shared; avoid literal translation) |
| 锦芳园 | Jinfang Yuan |
| 丰乐楼 | Fengle Lou |
| 夜雨林 | Yeyu Lin |
| 栖鸟岩 | Spiritroost Crag |
| 问津藏书馆 | Wenjin Library |
| 四海田 | Sihai Tian |
| 松贤贾 | Song Xianjia |
| 舒韵 | Shu Yun (prior phase convention, if encountered elsewhere) |
| 柳哀 | Liu Ai |
| 独立名字鹄 | Hu |
| 粘衮 | Nian Gun |
| 郑晚苓 | Zheng Wanling |
| 唐新词 | Tang Xinci |
| 温修远 | Wen Xiuyuan |
| 封继升 | Feng Jisheng (distinct source from 冯继升 / Feng Jisheng) |
| 阿东 | A Dong |
| 五十 | Wushi (cat name in feeding-object label 28705) |
| 天一楼 | Tianyi Tower |
| 陈府 | Chen Manor |
| 萧府 | Xiao Manor |
| 郑宅 | Zheng Manor |
| 沧浪居 | Canglang Lodge (shared title compound) |
| 河西波陈区 | Area Bochen |
| 南塔寺 | South Pagoda Temple (contains lookup) |

Chouling Ke remains Pinyin at 28347: shared exact lookup only yields generic Unknown, consistent with existing phase log. The line uses gue/lo. Naming rows that are ordinary developer labels use short literal Indonesian and Pinyin.

### idx 29,000–29,999

- Key lookup corrections: 张降龙 Zhang Jianglong; 盗猴儿 Monyet Pencuri; 小寿 Little Shou; 斡朵 Wo Duo; 张卤儿 Zhang Luer; 桑绿芸 Sang Lüyun; 康帅拂 Master Kang; 松风散人 Pine Breeze Recluse; 李阿牛 Li Niu; 宝贝囡 Sayang; 郝开勋 Hao the Jolly; 人中师 Lai En; 通畅 Tongchang; 通便 Tongbian; 清玄 Qingxuan; 尉景 Wei Jing.
- Other confirmed shared forms: 角门里 Forsaken Quarter; 烈阳印 Sun Mark; 长留 Kusir Kereta; 鸿沟古渠 Grand Canal; 裂石·威 Stonesplit - Might; 善妙洲 Sundara Land; 仙人帽 Abadi Purr-fect; 振玉 Oscillating Jade; 无颜 Tanpa Wajah; 雷公絮 Thunder Fluff; 鸡祥桶宝 Ember Ayam Beruntung; 夜行刀 Terjalin dengan Niat Jahat; 荒魂村 Wraith Village; 破门村 Brokegate Village; 观象灵台 Abacus Master; 五龙王庙 Five Dragons Temple; 迷驿舟 Wandering Ark; 朴永石 Park Yeongseok; 共鸣 Resonansi.
- Existing vault names reused for all familiar locations, including Hidden Mountain, Gunung Dragonbend, Dragonbend Academy, Spiritroost Crag, Sunken City Lake, Mount Tai, Jadestream Valley, Jadeheart Garden, Evershift Labyrinth, Velvet Shade, Unbound Cavern, Revelry Hall, Well of Heaven, Perintah Terikat Tinta, and Northern Vow Ruins.
- CN-only/provisional names with no exact shared hit in this lookup: 支雨檐 Zhi Yuyan; 支雨瞻 Zhi Yuzhan; 释慧羽 Shi Huiyu; 高盛 Gao Sheng; 齐志横 Qi Zhiheng; 宁非鬼 Ning Feigui; 侯三泰 Hou Santai; 阿狄 A Di; 龙湖寨 Longhu Zhai; 梨花潭 Lihua Tan; 白草野 Baicao Ye. 清都山水郎 Qingdu Shanshui Lang is an internal label without exact shared match. 斫木·幽 uses Penebang Kayu - Sunyi; 斫木·欢 uses Penebang Kayu - Mirth (欢 shared lookup = Mirth; full compound no exact hit).
- Opaque Mandarin tokens preserved exactly: <富商> at 29187, {目标积分值} at 29237, <青蛟堂师爷> at 29832. Do not translate inside these tokens.
- idx29395 begins a literal # followed by Chinese in source. Output uses '# Titik...' to preserve # without accidentally introducing a #T markup token. Initial MARKUP finding fixed.
- Loading sequences at 29355 and 29713 preserve all full repetitions and the final partial fragment, where present. Source tabs at 29095 retained; all per-row newline/tab counts verified.

## Name consistency pass (2026-10-25)

4,313 rows of this phase rewritten so names match global ([[CN-Quick-Reference]] §1–2a, rule
locked the same day): places, NPCs, sects, Path names and Inner Ways were often written as
Pinyin ("Gunung Bujian", "Lembah Hui"), half-translated or translated literally ("Gunung Tak
Terlihat", "Persimpangan Milenium", "Bellstrike·Bayangan", "Teknik Batin") instead of the global
English name (Hidden Mountain, Mirkvale, Millennium Crossing, Bellstrike - Umbra, Inner Way).
Done with `python cn/tools/name_check.py --fix` (Pinyin variants, label segments that are exactly a
name, curated `_LITERAL`/`_SUFFIX` tables); idx unchanged, no new rows. Names listed earlier in
this log in an Indonesian or Pinyin form are superseded by the global form. CN-only names now
stay Pinyin (never Indonesian). Remaining mismatches: `python cn/tools/name_check.py --idx <range>`.

