# Place Names — Wiki Canonical List

Part of [[Glossary]]. Source of truth: the Fandom wiki category
<https://where-winds-meet.fandom.com/wiki/Category:Locations> (133 pages, read 2026-10-20).

**Rule (supersedes the loose "etc." lists in [[Names-Not-Translated]]):** a place name is copied
**byte-for-byte as spelled on the wiki** — no Indonesian rendering, no partial translation, no
respelling. Generic words *around* a name (e.g. "Constable", "Guard") are still translated per
[[Translated-Common-Terms]], but the place-name part stays English: `Kaifeng Prefecture
Constable` → `Polisi Kaifeng Prefecture`, not `Polisi Prefektur Kaifeng`.

## Wiki list (keep verbatim)

Abandoned Mercy Hall, Ayisu, Baima Crossing, Bamboo Abode, Bamboo Retreat, Baoci Palace,
Battlecrest Slope, Beneath the General's Shrine, Blissful Retreat, Bloomveil Monastery, Bodhi
Sea, Buddha Fort, Buddha Statue Site, Central Bureaus, Changchun Hall, Chongde Hall, Chongyuan
Hall, Cleardew Terrace, Crimson Cliff, Dasan Pass, Desperation Ridge, Dreamfall Cliff,
Encircling Lake, Ethereal Mountain, Ever-Normal Granary, Fairgrounds, Finesteed Hamlet,
Fishwood River, Floral Expanse Beyond, Flower Expanse, Forsaken Quarter, Funing Hall, Furnace
Area, General's Shrine, Ghostlight Market, Gleaming Abyss, Golden Sands River, Gourmand's
Grove, Governor's Residence, Gracetown, Granary of Plenty, Grand Canal, Hall of State, Halo
Peak, Harvestfall Village, Heaven's Pier, Heavenfall, Hexi, Hopewell Academy, Huayin North,
Imperial Artisan Court, Imperial Palace, Imperial Service, Imperial Vaults, Imperial Works
Bureau, Inner Chambers, Inner Quarters, Jade Gate Pass, Jade Mirrored Spring, Jadebrook
Mountain, Jadewood Court, Jinming Pool, Jiuquan, Kaifeng, Kaifeng City, Kaifeng Imperial
Palace, Kaifeng Prefecture, Kaifeng Suburbs - East/North/South/West, Kilnfire Ridge, Kunning
Hall, Liangzhou (Fallen), Liangzhou Town, Lion Barrow, Lost Crossing, Marsh Plain, Martial
Temple, Masterwood Hamlet, Mercyheart Monastery, Mercyheart Town, Mistveil Forest, Moonveil
Mountain, Mount Wuwei, Nine Mortal Ways Base, North Imperial Garden, Northern Bamboo Grove,
Northern Vow Ruins, Outer Court, Palace of Annals, Peace Bell Tower, Petalfall Crossing, Pipa
Chasm, Plainfield, Prosperity Haven, Qinchuan Path, Qinghe, Qingshou Hall, Rear Garden,
Reserve Archives, Rivergaze Plateau, Riverside Station, Riverview Terrace, Roaring Sands,
Rustling Meadow, Sage's Knoll, Sealed Treasury, Shengping Tower, Sorrowfield Village, South
Gate Avenue, South Imperial Garden, Starveil Grassland, Steed's Pass, Still Shore, Stillheart
Grove, Stonewash Strand, Straying Steeds Sands, Sundara Land, Sunken City Lake, The Huayu Road,
Thousand-Buddha Vale, Thousand-Buddha Village, Training Grounds, Twinbeast Ridge, Unbound
Cavern, Velvet Shade, Verdant Wilds, Wansheng Town, Wenming Hall, Wildmane Ranch, Witherwilds,
Xiao Pass Old Road.

Notes: the wiki's `Velvet Shade (Location)` page title is a disambiguation — the in-game name
is just `Velvet Shade`. The category listing captured ended at "X"; any Y/Z pages are not
included here.

## Audit of the current translation (2026-10-20)

Method: for each wiki name, every source string in `unique_strings.jsonl` containing it was
compared with its translation in `locale/*.jsonl` (name kept verbatim or not). Most names are
already compliant (95%+ of strings each). **Deviations** — names currently rendered into
Indonesian (idx = the string that is exactly the bare name, where one exists):

| Wiki name | Current rendering (idx) | Deviating / total strings |
|---|---|---|
| Abandoned Mercy Hall | Mercy Hall yang Ditinggalkan (13342) | 1 / 3 |
| Beneath the General's Shrine | Di Bawah Kuil Sang Jenderal (100101) | 2 / 2 |
| Buddha Statue Site | Situs Patung Buddha (5862) | 8 / 22 |
| Desperation Ridge | Punggung Bukit Keputusasaan (6160) | 1 / 21 |
| Encircling Lake | Danau Melingkar (8775) | 2 / 26 |
| Ethereal Mountain | Gunung Ethereal (8569) | 1 / 26 |
| Fairgrounds | Pasar Malam (947) | 2 / 141 |
| Floral Expanse Beyond | Hamparan Bunga di Kejauhan (6103) | 1 / 66 |
| Flower Expanse | Hamparan Bunga (3820) | 10 / 111 |
| Furnace Area | Area Tungku (6610) | 2 / 5 |
| General's Shrine | "Kuil Jenderal" in sentences | 14 / 193 |
| Gourmand's Grove | Hutan Sang Pelahap (6982) | 1 / 23 |
| Governor's Residence | Kediaman Gubernur (5280) | 11 / 23 |
| Granary of Plenty | Lumbung Kelimpahan (22144) | 1 / 34 |
| Harvestfall Village | sentences only | 14 / 221 |
| Imperial Palace | Istana Kekaisaran (6559) | 58 / 352 |
| Imperial Service | sentences only | 16 / 454 |
| Imperial Vaults | Perbendaharaan Kekaisaran (9186) | 1 / 10 |
| Imperial Works Bureau | sentences only | 7 / 42 |
| Inner Chambers / Inner Quarters | Ruang Dalam (17284, 11424) | 1 / 2 and 3 / 10 |
| Kaifeng City | "Kota Kaifeng" in sentences | 86 / 185 |
| Kaifeng Prefecture | "Prefektur Kaifeng" in sentences | 51 / 211 |
| Kaifeng Suburbs - East/North/South | Pinggiran Kaifeng - Timur/Utara/Selatan (19797, 18163, 18320) | 1 each (West is OK) |
| Liangzhou (Fallen) | Liangzhou (Jatuh) (15608) | 4 / 13 |
| Liangzhou Town | sentences only | 6 / 22 |
| Lost Crossing | Persimpangan yang Hilang (20442) | 1 / 11 |
| Marsh Plain | Dataran Rawa (39457) | 1 / 4 |
| Moonveil Mountain | Gunung Moonveil (8501) | 5 / 149 |
| Nine Mortal Ways Base | Markas Nine Mortal Ways (3499) | 17 / 65 |
| Northern Bamboo Grove | Hutan Bambu Utara (127519) | 1 / 13 |
| Outer Court | Halaman Luar (30659) | 3 / 40 |
| Pipa Chasm | Jurang Pipa (8593) | 1 / 28 |
| Rear Garden | Taman Belakang (6549) | 23 / 100 |
| Reserve Archives | Arsip Cadangan (7581) | 2 / 18 |
| Riverside Station | Stasiun Tepi Sungai (11395) | 2 / 64 |
| Roaring Sands | Pasir Mengaum (9547) | 1 / 161 |
| Sage's Knoll | Bukit Sang Bijak (13858) | 1 / 8 |
| Sealed Treasury | sentences only | 12 / 175 |
| Sorrowfield Village | sentences only | 6 / 62 |
| Steed's Pass | Lintasan Sang Kuda (16946) | 1 / 22 |
| The Huayu Road | Jalan Huayu (9364) | 1 / 4 |
| Thousand-Buddha Village | Desa Seribu Buddha (9508) | 2 / 43 |
| Training Grounds | Lapangan Latihan (4521) | 9 / 41 |
| Xiao Pass Old Road | Jalan Kuno Xiao Pass (8686) | 1 / 10 |

About 25 more names have 1–5 stray strings each (Bamboo Retreat, Blissful Retreat, Bodhi Sea,
Crimson Cliff, Dasan Pass, Halo Peak, Heaven's Pier, Jade Gate Pass, Jinming Pool, Martial
Temple, Qinghe, Unbound Cavern, Verdant Wilds, ...). Some "deviations" are legitimate
(a compound that merely contains the name in a different sense) — review before editing.

Not present in the game text at all (wiki-only pages, nothing to fix): Jade Mirrored Spring,
Mount Wuwei, Straying Steeds Sands.

The wiki uses the same English strings the game ships, so no spelling mismatches were found for
names that do appear in game text.

## Fix procedure (applied 2026-10-20, see [[Update-2D]])

For each deviating string, restore the wiki-spelled name inside the existing Indonesian
translation (idx unchanged; edit `v` in the owning `locale/*.jsonl`), then run
`python tools/qa_check.py --locale "locale/*.jsonl"` and `python tools/progress.py --write`.

The same rule covers places **not** on the wiki list: on 2026-10-25 22 bare-name rows
(Millennium Crossing "Persimpangan Milenium", Nimbus Tower, Dragonbend Mountain, Willow Bank,
Tianjing Dock, …) were restored to English — list in [[Update-2D]]. These matter twice: WWM China
reuses global's Indonesian through the bridge and `term_lookup.py` ([[CN-Quick-Reference]]).
