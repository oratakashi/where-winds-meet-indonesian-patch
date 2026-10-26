# CN Phase 4 Translation Log (idx 90,000–119,999)

Session history and terminology for `cn/locale/phase4.jsonl`. Rules: [[CN-Quick-Reference]];
live progress: [[CN-Current-Status]]; precedents: [[CN-Phase-3]], [[CN-Phase-6]].

Phase 4 sits in the low-frequency tail of the CN-only corpus. The first 4,000 rows are almost all
internal editor/spawn labels (`…-氛围NPC-…`, `S10-战令奖励-付费23级`, `…对应普通装备宝箱`,
`博物条目N,M档奖励`, `……-填充N`), plus some Skill/gear descriptions, Battle Pass and event
rewards, NPC lines and lore. Labels stay literal and short; prose is translated in full.

## Batches

- **2026-10-26 — Batch 6 (idx 110,000–113,999, 4,000 rows).** Translated directly in the main
  session (no subagents) as ten 400-row chunks; same per-chunk checks as batch 5 (idx continuity,
  newline/tab counts, token multiset, Han leftovers, digit drift). Appended to
  `cn/locale/phase4.jsonl`; exact range 90,000–113,999 contiguous, previous 20,000 rows untouched.
  `name_check.py --fix` kept only for the new rows (386 edits). First QA run was clean after
  restoring the Han-bearing token `{疾病名}` (113,176); final full CN QA: **153,178 entries including
  2 overrides, 0 findings**. Phase 4: **24,000 / 30,000 (80.00%)**; next idx **114,000**.

- **2026-10-26 — Batch 5 (idx 106,000–109,999, 4,000 rows).** Translated directly in the main
  session (no subagents) as ten 400-row chunks; same per-chunk checks as batch 4 (idx continuity,
  newline/tab counts, token multiset, Han leftovers, digit drift). Appended to
  `cn/locale/phase4.jsonl`; exact range 90,000–109,999 contiguous, previous 16,000 rows untouched.
  `name_check.py --fix` kept only for the new rows (498 edits). QA first run was clean after
  restoring `#G5%#E` (107,014) and the `<金蟾腾跃>` tag (106,508); final full CN QA: **149,178
  entries including 2 overrides, 0 findings**. Phase 4: **20,000 / 30,000 (66.67%)**; next idx
  **110,000**.

- **2026-10-26 — Batch 4 (idx 102,000–105,999, 4,000 rows).** Translated directly in the main
  session (no subagents) as ten 400-row chunks; same per-chunk checks as batch 3 (idx continuity,
  newline/tab counts, Han leftovers, digit drift). Appended to `cn/locale/phase4.jsonl`; exact range
  90,000–105,999 contiguous, previous 12,000 rows untouched. `name_check.py --fix` kept only for
  the new rows. Full CN QA: **145,178 entries including 2 overrides, 0 findings** (first run: 6
  MARKUP findings — five Han-bearing `<…>` tags and the dict at 105,396 — restored verbatim).
  Phase 4: **16,000 / 30,000 (53.33%)**; next idx **106,000**.

- **2026-10-25 — Batch 3 (idx 98,000–101,999, 4,000 rows).** Translated directly in the main
  session (no subagents) as ten 400-row chunks. Every chunk was checked before merging: idx
  continuity, newline/tab counts against the source, no Han leftovers outside protected tokens,
  and a digit-sequence drift check (hits were only dates, Chinese numerals, lunar-month names
  and word order). Appended to `cn/locale/phase4.jsonl`; exact range 90,000–101,999 contiguous,
  previous 8,000 rows untouched. `name_check.py --fix` was run and its changes kept only for the
  new rows (it also rewrote 7 older Phase 4 rows and 1 Phase 5 row, e.g. "Scented Wine Cao" for
  94,009 — reverted, left for a dedicated name pass). Full CN QA: **141,178 entries including 2
  overrides, 0 findings**. Phase 4: **12,000 / 30,000 (40.00%)**; next idx **102,000**.
  **Overall: 141,176 / 159,176 unique strings (88.69%); CN-only in-game coverage
  805,139 / 823,139 (97.81%).** Generated with `python tools/progress.py --profile cn --write`.
  Coverage is frequency-weighted, includes internal editor labels and excludes the global bridge.

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

## Batch 3 terminology and preservation notes (2026-10-25)

Global names used as-is (found with `term_lookup.py`): Perception Forest ranks — Beginner (出鞘),
Novice (仗剑), Veteran (斩风), Adept (开山), Expert (断水), Master (流云), Legend (飞花), Mythic
(无我) with Roman numerals (supersedes the Indonesian "Awan Mengalir / Bunga Terbang …" renderings
in batch 1; those older rows are still to be harmonized). Places and things: Granary of Plenty
(太仓粟), Masterwood Hamlet (梓匠居), Jadewood Court (琼林苑), Lotus Courtyard (曲院风荷), Hollow Vale
(无心谷), Eternal Solitude Hall (万古一人殿), Skyreach Terrace (太岳台), Skymirror Tower (飞镜阁),
Heaven Below (洞天福地), Floral Expanse Beyond (来生岸), Lion Barrow (狮子坟), Peachgrove Pass
(桃林塞), Sunward Hill (向阳坡), Rockspire Street (景岩街), Dragon Gate Inn (龙门客栈), Crosswind
Bazaar (长兴集), Greenshade Village (青葱里), Everbloom Bazaar (常盛集), Another Tavern (又一楼),
Jade Gate Pass (玉门关), Wandering Ark (迷驿舟), Sky Shuttle (云槎), Palace of Annals (春秋别馆),
Silken Veil Society (羽衣楼), Imperial Palace's Peak (紫禁之巅), Gallery (万相集).
Things and skills: Meridian Touch (金玉手), Journal (武林录), Pitch Pot (投壶), Jianghu Tactics
(江湖弈), Battleboard Game (打马棋), Madiao Cards (叶子戏), Gold-Making Vessel (生金瓯), Clay Doll
(磨喝乐), Oddity (蹊跷, "Keanehan"), Stamina / Prep Step / Conversion / Heavy Stagger, Inner Passion
(热血难凉), Snowbreak Spring (拨雪寻春), Wolflike Frenzy (狗嘴夺食), Talon Strike (鹰爪连凿), Yaksha
Rush (药叉破魔), Dragon's Breath (神龙吐火), Skyward Raiment (霓裳徊天), Everspring Umbrella
(醉梦游春), Strategic Sword (积矩九剑), Phalanxbane Blade (十方破阵), Thundercry Blade (嗟夫刀法),
Silkbind - Jade/Deluge/Wings, Bamboocut - Draught/Kite/Wind, Stonesplit - Might/Strength,
Bellstrike - Umbra/Splendor, Tiltrim (倾觞 set), Swift Gale (急风), Rainwhisper (时雨), Desert Wail
(西凉鬼啸), Divinecraft (天工开物), Nine-colored Deer (九色鹿), Lucky Seventeen (小十七), Inkpincer
Scorpion (笃墨蝎), Wicker Ray (竹文鳐), Jadesong Cicada (玉鸣蝉), Terracotta Boar (当康陶).

CN-only provisional Pinyin: Chouling Ke (愁铃客, the assassin — 98,319 letter and later), Poguan Jin,
Ayinu → name_check now writes **Aynur**, Hongzhuang (红妆 Path), Yuxi Huaying (余息化影), Mofa
Randeng / Jienan Duzhong (moves at 98,714), Baopu Lu with Ziyun and Jingu caves (98,574), Xiaofeng
Zhiyu, Zhouliu Xinghan, Fanyue Xingchuan, Tafei Yan, Tongjian Wenpu, Xuedeng Cangxing, Heipi,
Baishou Tongzhou, Cuijian Podi, Shengcai Youdao.

Preservation exceptions: tags containing Han text kept verbatim (see [[CN-Current-Status]]);
99,521 renders the quoted character 日 as "Ri (matahari)" so no Han remains; the conlang table at
99,862 is transliterated to Pinyin with the same 21 lines; the repeated placeholder at 100,033 is
kept as repeated "Penjelasan acara". Long prose translated in full: Chouling Ke's letter (98,319),
Baopu Lu diary (98,574), the card-game rules (98,952), the political essay "Bagian Kelima"
(101,483), the two-friend swordsman tale (100,952), the Lingyin travelogue (101,757) and the
shrine ledger (101,165).

## Batch 4 terminology and preservation notes (2026-10-26)

Global names used as-is (found with `term_lookup.py`): Crimson Cliff (丹崖), Witherwilds (荒蚀林),
Peace Bell Tower (太平钟楼), Sage's Knoll (藏贤岗), Kilnfire Ridge (烧瓷岭), Heaven's Pier (神仙渡),
Battlecrest Slope (七伐坡), Bodhi Sea (菩提苦海), Gasping Cliff (嗟叹崖), Ghostlight Market (鬼市子),
Ever-Normal Granary (常平仓), Inkbound Order (文津馆), Karez Well (坎儿井), Finesteed Hamlet (伏马庄),
Zhongdu Bridge (中渡桥), Thousand-Buddha Village (千佛村), Zenith Box (同天匣), Evershift Labyrinth
(大衍迷阵), Unbound Cavern (无忧洞), Mohist Hill (墨山道), Mercyheart Monastery (慈心山院), Halo Peak
(佛光顶), Aureate Pavilion (绣金楼), Wildmane Ranch (不伏马场), Seedling Temple (早悟寺), Buddha Fort
(佛爷寨), Floral Expanse Beyond (来生岸), Lion Barrow (狮子坟), Moonveil Mountain (隐月山), Jinming Pool
(金明池), Dragonbend Academy (封龙书院), Revelry Hall (樊楼), Nimbus Tower (云华楼), Funing Hall (福宁殿),
Purity Tower (太清楼), The Palace Guard Office (殿前司), Dreamfall Cliff (望淮南崖), Hangrock Hamlet
(悬空寨), Yunjia Cove (云家岙), Skyward City Ruins (飞天残垣), Divinecraft Dungeon (天工地窟), Eight
Immortals (八仙过海), Jianghu Legacy (侠迹), Windtail (燕叽), Echo Jade (长鸣玉), Skytouch Rail (凌云轨),
Oscillating Jade (振玉), Softweave Dye Powder (素裳染粉), Inscription (题跋), Inner Way Notes (心法心得),
Precept Value (守律值), Mystic Skills (奇术), Chivalrous Mark (侠痕), Thundercry Blade (嗟夫刀法), Nameless
Sword (无名剑), Phalanxbane Blade (十方破阵), Everspring Umbrella (醉梦游春), Strategic Sword (积矩九剑),
Stonesplit - Might (裂石威), Bellstrike - Hong/Umbra, Silkbind - Jade/Yi/Lin/Yun, Pozhu - Kite/Dust/Cawan.
Status words: Vulnerable → "Rentan", Exhausted → "Kelelahan", Deflection, Paralysis → "Kelumpuhan".

CN-only provisional Pinyin: Shihou Zhengsheng (狮吼正声), Cunquan (寸拳), Juying (绝影 Chivalrous
Mark), Jiuchong Chunse (九重春色, Musim Semi Sembilan Lapis in two early rows), Chili Gong (赤鲤公),
Wangyue Chanyuan (望月婵媛), Chouling Ke, Lengzhu (冷烛), Li Si, Tang Xinci, Zhuge Feifei, Lu Yinge,
Luo Qinghan, Lan Qilu, Yuan Xingpei, He Youqu, Shen Yilun, Xin Wangshu, Qingping Jushi, Shanzao
(山臊, kept as a name).

Preservation exceptions: Han-bearing tags kept verbatim (see [[CN-Current-Status]]); the Lua-style dict at
105,396 is kept verbatim including its Chinese comments. Long prose translated in full: the Mohist
Sky Shuttle travelogue (105,624), Nenek Bao's Seven-Treasure tea origin (105,078), the three
students' confessions (104,250), the Changle-lao story (104,404) and the Mohist monkey legend (104,217).
Remaining name_check hits (about 1,000 rows) are loanword labels (Gear, Quest, Normal, Outpost …) in internal
editor rows — low priority.


## Batch 5 terminology and preservation notes (2026-10-26)

Global names used as-is (found with `term_lookup.py`): Sealed Treasury (封桩库), Bamboo Abode (竹林旧居),
Jadebrook Mountain (璧泉山), Palace of Annals (春秋别馆), Blissful Retreat (不羡仙), Imperial Artisan Court
(百工坊), Lingyin Temple (灵隐寺), Chongyuan Hall (崇元殿), Kunning Hall (坤宁殿), Xuanyuan Cult (玄元教),
Hexi (河西), Steed's Pass (饮马隘), Velvet Shade (醉花阴), Penglai's Peaks (蓬山万重), Veil of Lotus
(莲渡幽罗), Lingering Melody (袅袅之音), Resonating Melody (绕梁之音), Attunement Stone (定音石), Floral
Weave (花蕊织), Perception Forest (觉障林), Parting Tears (离人泪), Skyreach Terrace (太岳台), Martial
Temple (武庙), Battle Pass (战令), Wandering Tales (万事知), Skill Theft (偷师), Hutuo (滹沱), Meow Meow
(妙妙喵), Execution/Conversion (处决/易武). `name_check --fix` additionally normalised Jiangnan State,
Well of Heaven (天泉), Hall of State (官署), Contraption House (机关屋), South Gate Avenue (南门大街),
Mercyheart Town (慈心镇), Dao Lord (道主), Path Tower (流派塔), Guild (百业), Outpost, Formbend (易相).

CN-only provisional Pinyin: Zeng Xian, Zheng Ran, Zhao Yan, Chen Zixi, Feng Cheng, Qin Bo, Wu Er,
Yan Ping, Cai Yin, He Hupo, Yu Dinghui, Leng San, Wang Qing (Mohist diary 107,259), Mingshan
(明善, the Lingyin novice, 107,057 / 108,200), Yanshou (延寿), Zhang Wanshi (墨守之心), Shen Haohao /
Shen Tuotuo, Xun Cuiying, Tao Yuxia. Era names kept as Pinyin ("Tahun pertama Longde / Tianfu /
Longxing / Tianxi") in the OCR-variant puzzle rows (106,420 / 106,522 / 107,860).

Preservation exceptions: Han-bearing tags kept verbatim (`<金蟾腾跃>` 106,508, `<暂离>` 109,905);
`$T(30)` / `$T(2005)` name variables kept; the repeated placeholder at 109,234 is exactly 96 ×
"Penjelasan acara" (space-separated, as in 100,033). Long prose translated in full: the garden
chess story (106,282), Mingshan the Shuanglu novice (107,057, 108,200), the Mohist diary
(107,259), Leng San's rules (107,284), A'jin's story (109,118), the Mohist letter to a friend
(109,247) and the Suanshi tea passage (109,756). Remaining name_check hits (about 1,000 rows)
are loanword labels (Gear, Quest, Season, Outpost …) in internal editor rows — low priority.


## Batch 6 terminology and preservation notes (2026-10-26)

Global names used as-is (found with `term_lookup.py`): Heavenquaker Spear (九曲惊神枪), Wraith Village (荒魂村),
Yuchen Hall (玉宸殿), Tianyi Tower (天一楼), Foreigner Street (洋人街), Coolshade Hall (西凉殿), Whitecrown
Fortress (白头城), Goldflow Slope (流金坡), Sweetwine Tower (酒香塔), Squarewall Hold (方盘城), Hero's Tomb
(侠之冢), Brightlore Hall (昭文馆), Sentinel Howlion (镇关吼), Scholars' Hearth (红炉雅集), Jianlong Monastery
(建龙观), Springwave Pavilion (春水阁), Evergreen Grove (拥翠林), Stonewash Strand (浣石浦), Nine-Spire Peaks
(三峰九仞), Drought Demon (旱魃), Yu Hao (喻皓), Plowing Hill (耘丘, not Pinyin), Lady White (白娘子), Echo (聆音),
Hundred Delicacies (珍馐百味), Temporal Pavilion (时楼), Verdant Spire (耸翠楼), Qingshou Hall (庆寿殿), Wolf
Maiden (白狼主), Baima Station (白马驿), Leifeng Pagoda (雷峰塔), Cleardew Terrace (玉露台), Cloud Terrace (凌云台),
Ascendant Peak (扶摇峰), Peltwing Squirrel (飞夷由), Revival Hall (回春堂), Bloodscale Hall (赤龙堂), The Furr-vorite
Child (狸猫太子), A Hero's Bane (英雄剑), Venison Broth (见鹿羹), Lodestar Swordmaster (司南剑客), Prime Essence
(乾元), Jiazhong Melody (夹钟奏雅), Huangzhong Resonance (黄钟正响), Brimflow (盈盏), Qiantang Dream Journal
(钱塘梦行记), Wildtrail (寻野值), Dasan Pass (大散关), Hayagriva Temple (马明王庙), Stellabed Rocks (卧星岩), Beast
Reverie (鱼龙蔓延), Gourd of No Return (一葫不归), Sky Lantern (孔明灯), Lightning Speed (风驰电掣), Wind Sense
(听风辨位), Fire Breathing / Taibai (吐火 / 太白), Wind-kov (燕科夫), Engagement (不肝), Advanced Dye (云霞染),
Tongming / Clever Gambit (Battleboard ranks), Grandmaster (藏锋, Jianghu Tactics rank), Hetu Fragment, Shadow Puppet.
`name_check --fix` additionally normalised Jiangnan State, General's Shrine, Petalfall Crossing, Forsaken Quarter,
Dragonbend Mountain, The Rust-Scented Moon and Oddity (蹊跷) in label rows.

CN-only provisional Pinyin: Chouling Ke's bamboo-forest story (112,237) names the traitor Banzhu (斑竹);
Mo Kongjun (莫箜君, wine), Hu Mengji (虎孟极), Qingping Jushi (青瓶居士), Leiyin (雷音 Chivalrous Mark), Wujin Tie
(乌金铁, crafting material), Jiuming (九命, the cat), Yunxia Ran (云霞染 as a historical poem name),
Jiuchong Chunse (九重春色), Pofeng·Bambu (破风·竹, carried over), mount names in 113,465 (Diaojing Baie Da'e,
Hashiqi, Jizhui Bao, Shuangfeng Tuo, Rendao Lü, Lishan Lie, Xueli'er, Anxiang, Chanyi Weilai, Nanjiang Miyu, Mochen
Danzhu, Ranjin Ming, Penglai Ling, Qingyin Xiao, Ziwei Que), Kanerzi Village (坎尔孜村), Louwailou (楼外楼),
Wubi Inn (无比客店). 丰和村 treated as a variant of Harvestfall Village (丰禾村).

Preservation exceptions: Han-bearing token `{疾病名}` kept verbatim at 113,176; `$T(5149)` / `$T(5227)` name
variables kept; Rank labels of Jianghu Tactics written with Roman numerals (Adept II, Legend III, Grandmaster I).
Long prose translated in full: the "Catatan Cangfeng" opponent blurbs (110,020 / 111,553 / 112,084 / 112,130 /
112,497 / 113,765), Chouling Ke's story (112,237), Jiuming the cat (112,594), Qiancui Village (112,318), the
black-snake sisters (113,122), the Hexi-Liangzhou Qiantang passage (111,175) and the ledger of 113,276.
Remaining name_check hits are loanword labels (Gear, Quest, Season, Outpost …) in internal editor rows — low priority.
