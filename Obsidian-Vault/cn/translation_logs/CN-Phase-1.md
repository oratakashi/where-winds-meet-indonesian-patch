# CN Phase 1 Translation Log (idx 0–29,999)

Session history and terminology for `cn/locale/phase1.jsonl`. Consolidated rules:
[[CN-Quick-Reference]]; live progress: [[CN-Current-Status]].

## Batches

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
