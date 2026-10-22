# CN Phase 1 Translation Log (idx 0–29,999)

Session history and terminology for `cn/locale/phase1.jsonl`. Consolidated rules:
[[CN-Quick-Reference]]; live progress: [[CN-Current-Status]].

## Batches

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
