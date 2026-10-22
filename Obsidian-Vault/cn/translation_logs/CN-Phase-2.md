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
