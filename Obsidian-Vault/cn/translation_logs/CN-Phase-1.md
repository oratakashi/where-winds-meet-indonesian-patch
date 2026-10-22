# CN Phase 1 Translation Log (idx 0–29,999)

Session history and terminology for `cn/locale/phase1.jsonl`. Consolidated rules:
[[CN-Quick-Reference]]; live progress: [[CN-Current-Status]].

## Batches

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
