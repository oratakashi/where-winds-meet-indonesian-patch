# CN Phase 4 Translation Log (idx 90,000–119,999)

Session history and terminology for `cn/locale/phase4.jsonl`. Rules: [[CN-Quick-Reference]];
live progress: [[CN-Current-Status]]; precedents: [[CN-Phase-3]], [[CN-Phase-6]].

Phase 4 sits in the low-frequency tail of the CN-only corpus. The first 4,000 rows are almost all
internal editor/spawn labels (`…-氛围NPC-…`, `S10-战令奖励-付费23级`, `…对应普通装备宝箱`,
`博物条目N,M档奖励`, `……-填充N`), plus some Skill/gear descriptions, Battle Pass and event
rewards, NPC lines and lore. Labels stay literal and short; prose is translated in full.

## Batches

- **2026-10-25 — Batch 2 (idx 94,000–97,999, 4,000 rows).** Mandarin → Indonesian in
  three parallel ranges (94,000–95,333; 95,334–96,666; 96,667–97,999), appended to
  `cn/locale/phase4.jsonl`. Exact phase range 90,000–97,999 verified contiguous; previous
  4,000 rows preserved byte-for-byte. Newline/tab counts match; numeric differences reviewed
  as Chinese numeral conversions, dates/month names, rank formatting or word order.
  Global names checked with `term_lookup.py` and `name_check.py` (automatic corrections on
  temporary batches, then manual proper-name review; generic/contextual false positives
  rejected). Full CN QA: **137,178 entries including 2 overrides, 0 findings**.
  Phase 4: **8,000 / 30,000 (26.67%)**; next idx **98,000**.
  **Overall: 137,176 / 159,176 unique strings (86.18%); CN-only in-game coverage
  801,139 / 823,139 (97.33%).** Generated with `python tools/progress.py --profile cn --write`.
  Coverage is frequency-weighted, includes internal editor labels and excludes the global bridge.

- **2026-10-25 — Batch 1 (idx 90,000–93,999, 4,000 rows).** Translated directly in the main
  session (no subagents) as ten 400-row chunks, merged into `cn/locale/phase4.jsonl`. Every chunk
  was checked before merging: idx continuity, newline/tab counts against the source, no Han
  leftovers, and a digit-sequence drift check (the only hits were dates and Chinese numerals
  converted to Indonesian). Exact range 90,000–93,999 contiguous, no gaps or duplicates. QA first
  run: 3 MARKUP findings (92624 `<石匠>`, 93356 `{任意}`, 93410 `{任意染色}` — tokens containing Han
  text must stay verbatim) — fixed; final full CN QA: **132,478 entries, 0 findings**.
  Phase 4: **4,000 / 30,000 (13.33%)**; next idx **94,000**.

## Terminology decisions

Global names found with `term_lookup.py` and used as-is (Indonesian form in parentheses where
global translated it): Hidden Mountain (不见山), Mohist City (墨城), Mirkvale (晦谷), Velvet Shade
(醉花阴, from earlier phases), Heaven's Pier (神仙渡), Ghostlight Market (鬼市), Purity Tower (太清楼),
West Reserve (左藏库), Imperial Service (武德司), The Palace Guard Office (殿前司),
Nine Mortal Ways (九流门), Lone Cloud (孤云), Midnight Blades (三更天), Jade Serpent Hall (青蛟堂),
Inkbound Order (文津馆), Dragonbend Academy (封龙书院), Stillheart Grove (清心圃),
Perception Forest (觉障林), Wandering Tales (万事知), Heart of the Healer (悬壶济世; 悬壶 = Healer),
One Leaf, One Life (一叶平生), Blind to the World (红尘无眼), A Dung Beetle's Jianghu (粪斗江湖),
Yellow Robe Donned (黄袍加身), Pitch Pot (投壶), Echo Jade (长鸣玉), Oscillating Jade (振玉),
Zenith Box (同天匣), Battle Pass (战令), Raging Tides (狂澜), Gleaming Abyss (荧渊),
Roaring Sands (博浪沙), Golden House (销金窟), The Formless (无相皇), Xuanyuan Cult (玄元教),
Flower Burial (葬花), Fading Crimson (残红).
Villages/places: Harvestfall Village (丰禾村), Peaceward Camp (望安屯), Brokegate Village (破门村),
Verdant Hill Village (封丘村), Gracetown (承恩镇), Mercyheart Town (慈心镇), Dragonfall Camp
(降龙寨), Sorrowfield Village (达安村), Apricot Village (聆杏村), Baima Crossing (白马津),
Harborlink Crossing (临津渡), Wheatwind Bazaar (麦香集), Skybrim Market (天水集), Derndale (隐峪),
Moteheart Haven (芥子庐), Confinement Tower (束罪阁), Thousand-Buddha Village (千佛村),
Luancheng, Hexi, Qinchuan, Karez (坎儿井, from earlier phases).
Gear sets (回流奖励 = "Hadiah kembali … tingkat N"): Jadeclasp (抱玉), Jadeware (玉斗),
Ivorybloom (浣花), Whirlsnow (梨雪), Starweave (连星), Honorbound (尚义), Flawless Guardian
(御无缺), Ripple Step (凌波步), Moonflare (桂月), Swallowcall (燕归), Eaglerise (鹰扬),
Beyond the Chill (不知寒), Etherwrath (撼天), Calmwaters (江凝), Ebonward (玄甲).
Weapon sets keep earlier precedent: Silkbind (牵丝), Bellstrike (鸣金), Bamboocut (破竹),
Stonesplit (裂石).

Provisional CN-only labels (revisit when official counterparts appear): Smokywillow (烟柳 set),
Mountainfall (山倾 set), Swallow's Return (归燕), Pofeng·Bambu/Layang-layang (破风), Kota Jiuquan
(酒泉镇, kept as a town), Tebing Keluh Kesah (嗟叹崖), Menara Jubah Bulu (羽衣楼, location),
Aula Satu-satunya Sepanjang Masa (万古一人殿), Lembah Tembok Besi (铁壁谷),
Gua Bawah Tanah Karya Langit (天工地窟), Lima Nada (五音, loot-tier label), Seratus Profesi (百业),
Jodoh Baik (善缘, idle-game resource), Semangat Pendekar (侠气 resource), Jejak Pendekar (侠迹).

## Name consistency pass (2026-10-25)

875 rows of this phase rewritten so names match global ([[CN-Quick-Reference]] §1–2a, rule
locked the same day): places, NPCs, sects, Path names and Inner Ways were often written as
Pinyin ("Gunung Bujian", "Lembah Hui"), half-translated or translated literally ("Gunung Tak
Terlihat", "Persimpangan Milenium", "Bellstrike·Bayangan", "Teknik Batin") instead of the global
English name (Hidden Mountain, Mirkvale, Millennium Crossing, Bellstrike - Umbra, Inner Way).
Done with `python cn/tools/name_check.py --fix` (Pinyin variants, label segments that are exactly a
name, curated `_LITERAL`/`_SUFFIX` tables); idx unchanged, no new rows. Names listed earlier in
this log in an Indonesian or Pinyin form are superseded by the global form. CN-only names now
stay Pinyin (never Indonesian). Remaining mismatches: `python cn/tools/name_check.py --idx <range>`.


## Batch 2 terminology and preservation notes (2026-10-25)

Global names used include Divinecraft Dungeon, Steed's Pass, Spiritroost Crag, Stellabed Rocks,
Frostcliff Heart, Mistveil City, Jadestream Valley, Brightlore Hall, Chancellery, Masked Troupe,
Mercyheart Monastery, Aureate Pavilion, Lion Barrow, Meridian Touch, Sealed Treasury,
Skyward City Ruins, Skyreach Terrace, Jiuquan, Bamboocut - Dust/Draught, Still Shore,
Gasping Cliff, Reserve Archives, Sandstorm Tavern, Peace Bell Tower, Halo Peak,
Flute of the Tides and The Void King. 袅袅之音 follows the global translation
**Melodi yang Membekas** (97,646). Perception Forest rank labels use
Veteran/Mythic/Legend/Master/Adept/Beginner and Roman numerals.

CN-only provisional Pinyin: Qinglin Fuyu, Shanqing (山倾; supersedes the earlier Mountainfall
rendering for new rows), Pisha Songwen, Jiuxiang Cao, Baiyu Gu (白玉骨), Xueji Hongzhuang
(血祭红妆), Yuguizhi (玉桂枝), Zhulie (逐猎), Fusha (伏杀) and the dog Xiuxiu (嗅嗅).
雪覆千山 / 血覆千山 have no global lookup hit and share the reading Xuefu Qianshan;
labels distinguish them as **Xuefu Qianshan (salju)** / **Xuefu Qianshan (darah)**.

Long prose translated in full includes the West Lake passage (94,246), monk's account
(94,385), Qingcang (94,776), wandering priest's letter (94,984), folk coin lore (95,511),
fermentation methods (96,396), Xiuxiu's three-paragraph story (97,681) and Guangling San
(97,904). No shortening.

Preservation exceptions: idx **95,120** is the code-like shop identifier
`shop_chouka_鸾倚九霄_622160`, kept exactly (the sole Han outside protected tokens in this
batch). At **97,842**, the source `<=` comparator is consumed by the existing `TOKEN` regex
through the next closed angle tag; that entire protected span, including Mandarin text,
remains verbatim under the current markup rule. Text outside it is translated. Two stray
`#` markers in the middle range initially created false format tokens and were separated
from following Latin letters before final QA.
