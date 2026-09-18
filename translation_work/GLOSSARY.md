# Konvensi Terjemahan — Where Winds Meet ID Patch

Dokumen ini adalah aturan kerja yang dipakai konsisten di semua batch terjemahan
`unique_strings.jsonl` -> `translations.jsonl`. Kalau user minta ubah satu keputusan
di sini, cari & ganti di `translations.jsonl` supaya konsisten ulang (dedup = mudah revisi).

## TIDAK diterjemahkan (dianggap "nama")

- **Nama karakter/NPC personal**: Huajian Ke, Jiang Wulang, Lian Daozi, Feng Jisheng,
  Han Xiangxun, Big Zhao, Little Fu, Ye Wanshan, Murong Yuan, Yi Dao, Qinghe, dll.
- **Nama tempat/lokasi**: Sixteen Lanes, Hutuo River, Mirkvale, Kaifeng, Eastwind Pavilion,
  Sunken City Lake, Unbound Cavern, Confinement Tower, Bandit Encampment,
  Great Song Prefecture Hall, Heavenfall, Skybrim Market, Harvestfall Village,
  Blissful Retreat, Wansheng Town, West Market, Forsaken Quarter, East City, dll.
- **Nama faksi/organisasi**: Aureate Pavilion, Bloodscale Hall, Jade Serpent Hall,
  Velvet Shade, Mercyheart Monastery, Ghost Revelry Hall, Sandstorm Tavern, Mohist Hill,
  Greenwood (bandit), NetEase, Raging Tides, dll.
- **Nama senjata/aliran/skill berjudul** (proper noun ability/weapon): Thundercry Blade,
  Strategic Sword, Inkwell Fan, Mortal Rope Dart, Infernal Twinblades, Noname Sword,
  Soulshade Umbrella, Panacea Fan, "Peak's Springless Silence", "Rodent Rampage",
  "Unwithering Bloom", "Inner Balance Strike III", "Sword Horizon", "Meridian Touch",
  dll — nama skill dalam tanda kutip/tag `#Y...#E` dibiarkan bahasa Inggris.
- **Nama boss/entitas unik**: The Void King, Windchaser, Meow Meow, dll.

## Diterjemahkan (bukan "nama")

- **Judul quest/chapter yang puitis** (bukan nama orang): "Death of the Governor",
  "The Homeward Vow", "Where the Heart Stirs", "Melodies of Peace", "Universal Harmony",
  "Karmic Reflections", "Glimmer Against the Dark", "Whispers Beneath the Moon",
  "Parting Ways", "Changeless Heart", "Lucky Seventeen" -> diterjemahkan puitis/casual.
- **Sebutan peran NPC generik**: Villager, Bandit, Soldier, Guard, Scholar, Servant,
  Constable, Swordsman, Commoner, Official, Passerby, Player, Guest, Disciple, Member,
  Resident, Laborer, Attendant, Maid -> diterjemahkan (mis. Villager -> Warga/Penduduk Desa).
  Nama tempat yang menempel di depan sebutan peran TETAP bahasa Inggris
  (mis. "East City Commoner" -> "Warga East City", bukan "Warga Kota Timur").
- **Deskripsi item/lore/dialog** (kalimat penuh): diterjemahkan penuh ke Indonesia
  casual, kecuali nama & tag format di dalamnya.
- **Kalimat dialog percakapan** (first/second person, terasa seperti obrolan lisan):
  pakai gue/lo bila natural.

## Tetap bahasa Inggris (istilah game/UI yang "lebih bagus" bahasa Inggris)

Label stat/attribute karakter selalu dibiarkan penuh bahasa Inggris (konvensi umum
game RPG mobile/PC berbahasa Indonesia):
Critical Rate, Precision Rate, Affinity Rate, Physical Attack/Defense, Max/Min HP,
DMG Bonus/Reduction/Boost, HP Recovery/Bonus, Attack Bonus, DPS, Silkbind/Bellstrike/
Bamboocut/Stonesplit/Formless Attack (nama tipe damage khas game ini), Tier, Stage, Lv, Healer.

Loanword umum lain yang dibiarkan (dari instruksi user + genre gaming ID):
Guild, Event, Login, Logout, Menu, Info, Reset, Boss, Skill, Item, Quest, Level,
Buff/Debuff, Chat.

## Istilah wuxia genre-spesifik (ditambahkan sesi Fase 1)

- **"Sect" -> "Sekte"** — istilah wuxia standar dalam terjemahan Indonesia, DITERJEMAHKAN
  (bukan dibiarkan Inggris). Contoh: "Sect Rules Violation Notice" -> "Pemberitahuan
  Pelanggaran Aturan Sekte".
- **"Wayfarer"/"Wanderer" -> "Pengembara"** — konsisten dipakai untuk kedua kata
  Inggris ini (termasuk "Jianghu Wanderer" -> "Pengembara Jianghu").
- **"Young Master" -> "Tuan Muda"** — honorifik wuxia standar.
- **"Doctor" -> "Tabib"** (bukan "Dokter") — lebih cocok setting historis.
- **Nama set kostum/gear** (Whirlsnow, Ebonward, Formbend, Flawless Guardian, dst.)
  dan **nama boat/kapal khusus** (Painted Boat, Mirage Boat) DIBIARKAN bahasa Inggris
  seperti nama senjata/skill.
- **Nama festival budaya asli** (Spring Festival, Double Ninth Festival, dll.)
  DITERJEMAHKAN ke istilah deskriptif Indonesia (bukan proper noun): "Spring Festival"
  -> "Festival Musim Semi", "Double Ninth Festival" -> "Festival Sembilan Ganda".
- **Istilah TCM (Traditional Chinese Medicine)** dalam teks lore penyakit/pengobatan:
  konsep umum (qi stagnation, damp-heat, dll.) DITERJEMAHKAN ke Indonesia deskriptif
  ("stagnasi qi", "lembap-panas"), tapi **nama penyakit spesifik bertitle** (Shegong's
  Disease, Huhuo Syndrome, Wind-Damp Bi Syndrome) mempertahankan kata inti aslinya
  (Shegong, Huhuo, Bi) + kata generik diterjemahkan ("Syndrome" -> "Sindrom",
  "Disease" -> "Penyakit").

## Istilah baru yang dikunci sesi Fase 2 (idx 3000-3699)

- **"N-th Realm" (mis. "1st Realm", "5th Realm") -> "Realm N"** (mis. "Realm 1", "Realm 5")
  — pola stat/tier progression, dibiarkan Inggris sama seperti Tier/Stage/Lv.
- **"Rank N" tetap dibiarkan Inggris** (mis. "Rank 9") — sama seperti "Level N"/"Lv.N".
- **String `"<Nama Skill> - <Tipe Skill> DMG Boost"` (mis. "Everspring Umbrella -
  Special Skill DMG Boost", "Unfettered Rope Dart - Charged Skill DMG Boost") DIBIARKAN
  UTUH bahasa Inggris** — ini label stat skill upgrade, bukan kalimat naratif.
- **Nama resource/meter tambahan yang dibiarkan Inggris** (mengikuti pola HP/Qi/Energy
  yang sudah ada): Inspiration, Affection, Exploration, Heaven's Will, Battle Will,
  Tenacity, Super Armor, Stagger, Exhaustion (Immunity), Bleed (mis. "Fivefold Bleed").
- **Nama tipe damage/stat tambahan yang dibiarkan Inggris**: Thrust Damage, Deflection
  (Boost), Critical (DMG), Control Immunity, HP Drain, Formless Penetration, Endurance
  Recovery, Healing Boost, Energy Enhancement — pola sama dengan Silkbind/Bellstrike/
  Bamboocut/Stonesplit/Formless Attack yang sudah dikunci sebelumnya.
- **Mode/fitur game yang dibiarkan Inggris sebagai nama fitur** (bukan diterjemahkan):
  Solo Mode, Co-op Mode, Endless - Solo/Duo/Quad, Arena, Sword Trial, Breakthrough,
  Bounty, Draw (sudah ada), Room (dalam konteks matchmaking, mis. "Add to Room").

## Istilah baru yang dikunci sesi Fase 3 (idx 5000-6999)

- **PENTING — tag `<...>` TANPA format `|id|#C|n>` (placeholder polos, mis.
  `<Player Name 7 characters>`) harus dibiarkan PERSIS bahasa Inggris, JANGAN
  diterjemahkan.** Beda dengan tag stat berformat `<Nama Stat|780|#C|15>` yang memang
  nama statnya dibiarkan Inggris secara alami (karena memang tidak diterjemahkan) —
  tag polos ini dianggap satu token utuh oleh `TOKEN` regex di `qa_check.py`
  (`<[^>]*>`), jadi kalau isinya diterjemahkan, validasi token MISMATCH (pernah kejadian
  di idx 6041, sudah diperbaiki). Cek ulang tiap ketemu tag `<...>` baru: kalau formatnya
  bukan pola stat standar, JANGAN sentuh isinya sama sekali.
- **Nama senjata Legendary/Epic** (Jadeware, Swallowcall, Rainwhisper, Cleftpeak,
  Mistwillow, Starweave, Etherwrath, Hawkwing, Ivorybloom, Mountainfall, Whirlwind, dst.)
  DIBIARKAN Inggris — pola sama dengan nama set kostum yang sudah dikunci.
- **String `"<Nama Skill> - EX"`, `"<Nama Skill> - Common"`, `"<Nama Skill>: Common"`, `"<Nama Skill>: Ultimate"`, `"<Nama Skill> - Edge"`, `"<Nama Skill> - Radiance"`** (varian skill icon/tipe, mis.
  "Infernal Twinblades - EX", "Heavenwill Gauntlets: Ultimate", "Inkwell Fan - Radiance") DIBIARKAN UTUH bahasa Inggris, sama seperti pola DMG Boost.
- **String stat-scaling panjang berpola `"Increases <Stat|id|#C|n> based on Agility.
  Current bonus: ... Maximum bonus requires ..."`** — kalimat pembuka diterjemahkan
  ("Meningkatkan ... berdasarkan Agility. Bonus saat ini: ... Bonus maksimum
  membutuhkan..."), nama stat & angka di dalam tag `<...>`/`#...#` dibiarkan utuh.
- **Gelar pahlawan (hero title) dalam tanda kutip** pada kalimat "Unlocks ... hero title
  \"X\"" DIBIARKAN Inggris (perlakukan sebagai proper noun/achievement name), termasuk
  versi tampilan berprefiks kode warna (mis. `#dee8d3Three Pillars of Power`). CATATAN:
  idx 6211 "Best of the Best" sempat diterjemahkan jadi "Yang Terbaik dari yang Terbaik"
  sebelum konvensi ini disadari — biarkan saja (dampak kecil, cuma satu baris), tapi
  untuk kemunculan future gelar yang sama/serupa ikuti aturan gelar-tetap-Inggris ini.

## Istilah baru yang dikunci sesi Fase 4 (idx 10000-10999)

- **Sebutan kekerabatan/status yang menempel di depan nama NPC DITERJEMAHKAN** (pola sama
  dengan "Young Master" -> "Tuan Muda"): Aunt -> Bibi, Uncle -> Paman, Grandpa -> Kakek,
  Granny -> Nenek, Elder -> Tetua, Master (guru ilmu silat) -> Guru, Lady/Madam/Miss -> Nyonya/Nona,
  Mr. -> Tuan. Contoh: "Aunt Han" -> "Bibi Han", "Elder Peng" -> "Tetua Peng", "Master Wuhen"
  -> "Guru Wuhen", "Grandpa Zhang" -> "Kakek Zhang". Nama tetap tidak diterjemahkan.
- **"Master" sebagai gelar pemimpin organisasi/faksi** (bukan guru ilmu silat perorangan)
  -> "Ketua" (mis. "Bloodscale Hall Master" -> "Ketua Bloodscale Hall"). "Vice Master" ->
  "Wakil Ketua".
- **Musim dalam teks efek/buff (Spring/Summer/Autumn/Winter) DITERJEMAHKAN** jadi Musim
  Semi/Musim Panas/Musim Gugur/Musim Dingin ketika dipakai sebagai label deskriptif efek
  musiman (bukan nama fitur bertitel).
- **"Red Envelope"/uang lebaran-style hadiah Tahun Baru -> "Angpao"** (istilah umum
  digunakan di Indonesia untuk hadiah bertema keberuntungan ala Tahun Baru Imlek).
- Placeholder non-standar tanpa tag (mis. `$STEADY_MIN_PRO_ATK_C:.1f$`, `$P`, `$N`,
  atau literal seperti `Xd, Xh`) **tidak ditangkap regex TOKEN qa_check.py tapi tetap
  WAJIB dipertahankan persis** — ini variabel substitusi runtime, bukan teks biasa.
- Konfirmasi ulang: nama gear/senjata/skin (pattern "- Valor/Radiance/Edge/Guard" dst.),
  nama tempat baru (Kaifeng Bathhouse, Crosswind Bazaar, Tubo Camp, dll.), dan istilah
  mekanik CC (Taunt, Bind, Purify sebagai skill judul) tetap dibiarkan bahasa Inggris
  mengikuti pola yang sudah dikunci sebelumnya.
- **"Lord" (gelar bangsawan/pejabat di depan nama) -> "Tuan"**, konsisten dengan
  "Mr."/"Young Master" (mis. "Lord Wang" -> "Tuan Wang", "Lord Shi" -> "Tuan Shi").
- **"Old Man X" -> "Kakek X"** (sama seperti Grandpa). **"Old X" tanpa "Man"** (mis. "Old
  Jin") dibiarkan bahasa Inggris karena ambigu apakah nickname formal atau deskriptif —
  review lagi kalau nama yang sama muncul berulang dengan pola jelas.
  **"Old Caravan Master" -> "Ketua Karavan Tua"** (Master di sini = pemimpin organisasi).
- Tag `<...>` **tanpa penutup `>`** (string sumber terpotong/typo, mis. `"<Tangled Gauze...`)
  memang muncul di data asli — pertahankan tanda `<` literal apa adanya di awal kalimat,
  jangan dihapus maupun ditutup manual.
- **Durasi berformat `Nd` (N hari, literal huruf "d" bukan token)** -> `Nh` (mis. "30d" ->
  "30h"). Ini bukan token yang dicek `qa_check.py`, jadi aman diterjemahkan — beda dengan
  placeholder `$VAR$`/`$P`/`$N` yang harus dipertahankan persis.
- **"Achievements" -> "Pencapaian"** (bukan dibiarkan Inggris) untuk kategori UI umum,
  beda dari achievement/title spesifik dalam tanda kutip yang tetap Inggris.
- **"Healer" (peran trinity tank/DPS/healer)** dibiarkan Inggris seperti Tank/DPS, BEDA
  dengan "Doctor"/"tabib" yang dipakai untuk konteks pengobatan tradisional/NPC in-universe.
- **PENTING — tag `<...>` yang membungkus SATU KALIMAT PANJANG penuh (bukan pola
  `<Label|id|#C|n>` pendek) dianggap SATU TOKEN UTUH oleh regex `qa_check.py`** karena
  regex `<[^>]*>` mencocokkan dari `<` pertama sampai `>` berikutnya — termasuk `#warna`/
  `#E` di dalamnya ikut "tertelan" jadi bagian token itu, BUKAN token terpisah. Akibatnya
  isi di dalam `<...>` jenis ini (ditemukan di idx 12448, notifikasi item beku waktu)
  **wajib dibiarkan 100% identik dengan sumber (bahasa Inggris)** — hanya teks DI LUAR
  tanda `<...>` yang diterjemahkan. Beda dengan tag stat pendek yang isinya nama stat
  (memang sudah dibiarkan Inggris secara alami). Selalu jalankan validasi token setelah
  translate paragraf yang mengandung `<...>` panjang untuk menangkap kasus ini.

## Istilah baru yang dikunci sesi Fase 4 lanjutan (idx 18100-19099)

- **Frasa puitis pendek berdiri sendiri (1-4 kata, tanpa penomoran/struktur quest)** yang
  berfungsi sebagai nama item/skin/mount/emote (mis. "Fleeting Dream", "Clear Glow",
  "Eternal Watch", "Night Glow", "Wildtrail", "Steady Ascent", "Spring's Bounty",
  "Winter's Bloom", "Oats in the Wind", "Paired Shadows", "Haven Astray", "Bright Sky")
  DIBIARKAN Inggris, sama pola dengan nama set kostum/gear. Judul quest/chapter yang
  punya struktur jelas (angka, "Tales Retold:", "Volume", tanda "-" + peran) tetap
  DITERJEMAHKAN seperti biasa.
- **Tag `<...>` yang membungkus dialog/kalimat panjang** (bukan cuma nama stat) — mis.
  ucapan NPC non-manusia idx 18461 `<Pft. One round...>` — dibiarkan 100% identik
  Inggris termasuk isi dialognya, mengikuti aturan tag-panjang-satu-token yang sudah
  dikunci sebelumnya (idx 12448).
- **Tag highlight pendek `#H...#E`/`#Y...#E` yang membungkus SATU KATA instruksi umum**
  (Press, night, dst.) isinya DITERJEMAHKAN (`#HPress#E` -> `#HTekan#E`, `#Ynight#E` ->
  `#Ymalam#E`) — beda dari tag yang membungkus nama skill/weapon (dibiarkan Inggris,
  mis. `#HFire Arrow#E`) atau kalimat panjang (dibiarkan Inggris). Regex `TOKEN` di
  `qa_check.py` mencocokkan `#H`/`#Y`/`#E` sebagai token satu-huruf terpisah, jadi teks
  di antaranya aman diterjemahkan.
- **Item/hewan/tumbuhan bernama umum** (bukan proper noun fantasi buatan) seperti
  "Pangolin", "Sparrow Egg", "Crane Egg", "Snow Ape", "Long-Tailed Pheasant",
  "Lanternfish", "Bamboo Shoot" DITERJEMAHKAN ke istilah Indonesia wajar — beda dari
  nama gear/weapon fantasi buatan (Swallowcall, Jadesong, Voidchant, Darkecho, dst.)
  yang tetap Inggris. Kata majemuk campuran (nama fantasi + kata umum, mis. "Peltwing
  Squirrel") -> kata fantasi dibiarkan Inggris, kata umum diterjemahkan ("Tupai Peltwing").

## Istilah baru yang dikunci sesi penutup Fase 4 (idx 19100-19999)

- **Tier/rank profesi generik format `"Profesi: Tier"`** (mis. "Healer: Novice",
  "Healer: Adept", "Healer: Redemption", "Scholar: Novice") — kata tier-nya DIBIARKAN
  Inggris (sama pola dengan Rank/Tier/Stage/Lv). Kalau tier-nya berupa gelar naratif
  jelas (mis. "Scholar: Refined Gentleman", "Scholar: Silver Tongue") itu DITERJEMAHKAN
  karena berfungsi sebagai gelar naratif, bukan tier numerik/tingkat generik.
- **Nama bahasa di UI pemilihan bahasa** (mis. "Русский язык", "日本語",
  "Español（Latino）") TIDAK diterjemahkan — dibiarkan dalam skrip/bahasa aslinya.
- **Placeholder durasi gabungan format+literal** (mis. `{diff_hour:d}h ago`) — bagian
  `{...}` wajib dipertahankan persis, literal suffix di luar kurung kurawal bebas
  diterjemahkan mengikuti konvensi durasi (`d`->`h` hari, `h`->`j` jam).

## Istilah baru yang dikunci sesi Fase 5 (idx 20000-20999)

- **Placeholder durasi gabungan literal ganda format `%sd%sh`** (hari+jam, mis. "Remaining:
  %sd%sh") -> `%sh%sj` — pola sama dengan konversi `Nd`->`Nh` (hari) dan tambahan `h`->`j`
  (jam) yang sudah dikunci di Fase 4, diterapkan bersamaan saat literal "d" dan "h" muncul
  berdampingan dalam satu string. `%s` tetap dipertahankan persis (itu placeholder token).
- **"Loot" -> "Jarahan"** (item drop), **"Griefing" dibiarkan Inggris** (istilah komunitas
  gaming umum, tidak ada padanan baku), **"Constable" -> "Konstabel"** (loanword umum untuk
  gelar penjaga keamanan era historis, konsisten dengan "Guard"/"Soldier" yang diterjemahkan).
- **"Union" (struktur sosial pemain, beda dari "Guild")** dibiarkan Inggris untuk sesi ini
  karena belum jelas apakah ini sinonim Guild atau fitur terpisah — review lagi kalau
  istilah ini muncul lebih sering dengan konteks lebih jelas.
- **Nama currency/resource khusus tanpa terjemahan baku** (mis. "Bookworms" sebagai nama
  resource, bukan kata umum "kutu buku") dibiarkan Inggris kapital, mengikuti pola
  Inspiration/Affection/dll. yang sudah dikunci di Fase 2.

## Istilah baru yang dikunci sesi Fase 5 lanjutan (idx 21000-21999)

- **"Enhancement" pada label node skill-tree/talent DIBIARKAN UTUH bahasa Inggris**
  (mis. "Momentum Enhancement", "Physical/Critical Resistance Enhancement", "Water
  Clone Enhancement", "Perfect Catch Enhancement", "Scroll & Script Enhancement",
  "Charge Calculation Enhancement", "Qi Struggle Enhancement") — pola sama dengan
  "DMG Boost"/"DMG Bonus"/"DMG Reduction" yang sudah kompound-Inggris. "X Boost"
  (mis. "Advanced Defense Boost") juga ikut aturan ini.
- **"Appearance" sebagai kategori kosmetik/skin UI DITERJEMAHKAN jadi "Tampilan"**
  (mis. "Spear Appearance" -> "Tampilan Spear") — kata umum, bukan proper noun. Nama
  tipe senjata generik yang menempel (Spear, Blade, Gauntlets, dst.) tetap Inggris.
- **Placeholder durasi literal detik (`Ns`) di DALAM tag `#Y...#E` dst. ikut dikonversi
  jadi `Nd`** (mis. `#Y70s#E` -> `#Y70d#E`) — perluasan konvensi durasi detik dari Fase 4,
  berlaku juga saat literalnya ada di dalam tag warna/highlight.
- **Tag warna non-standar berpola `#<6-char-hex-ish>NNNN Word#E`**: regex `TOKEN` di
  `qa_check.py` hanya mencocokkan 6 karakter pertama setelah `#` sebagai token warna;
  sisanya (digit lanjutan + kata, mis. "120 Points") adalah teks bebas yang AMAN
  diterjemahkan ("Points" -> "Poin").
- **"Scholar"/"Healer" sebagai nama kelas Profession** (bukan sebutan NPC generik)
  dikonfirmasi ulang dibiarkan Inggris (mis. "Scholar Jiang Huaiyuan", "Scholar Class 72").
- Nama/istilah warna pigmen dalam kutip pada lore item cat (mis. "vermilion", "Lychee",
  "white") dibiarkan Inggris — naming pun ala pengrajin, bukan teks naratif biasa.

## Istilah baru yang dikunci sesi Fase 5 lanjutan (idx 29000-30999)

- **Placeholder durasi majemuk `%dm%ds` (menit+detik)** -> `%dm%dd`, dan `%sh` (jam)
  -> `%sj` — memperluas konvensi konversi literal durasi (`s`->`d` detik, `h`->`j` jam)
  ke format yang digabung dengan placeholder `%d`/`%s`. Huruf `m` (menit) dibiarkan
  apa adanya.
- **Placeholder tanggal runtime `@T[...]`** (kurung siku, mis. `@T[month_2,
  day_6,type_noLocal;empty]`) TIDAK tertangkap regex `TOKEN` tapi WAJIB dipertahankan
  persis karakter demi karakter — variabel substitusi tanggal, sama kelasnya dengan
  `$VAR$`/`$P`/`$N` yang sudah dikunci sebelumnya.

## Istilah baru yang dikunci sesi Fase 9 (idx 429887-431886, update game 2026-09-16)

- **"Farmer" (unit sistem Homestead yang bisa direkrut/dikerahkan) DIBIARKAN UTUH bahasa
  Inggris** (kapital) — istilah sistem Homestead seperti Retainer/Homestead, BUKAN
  diterjemahkan sebagai kata umum "petani".
- **"Qiongqi Master"/"Qiongqi Warrior"/"Qiongqi Soldier"/"Qiongqi Artificer"** (rank/role
  anggota faksi antagonis "Qiongqi") DIBIARKAN UTUH bahasa Inggris sebagai compound title.
- **Royal/imperial title generik (Prince, Empress) DITERJEMAHKAN**: "Prince Teng" ->
  "Pangeran Teng", "Empress Wu" -> "Permaisuri Wu" — konsisten dengan Lord->Tuan.
- **"Treasury" (kata umum) DITERJEMAHKAN "Perbendaharaan"** di semua compound-nya
  (Pledged/Sealed/Imperial Treasury) — beda dari nama lokasi majemuk unik yang tetap Inggris.
- **"Lantern Festival" -> "Festival Lampion"**, konsisten dengan pola nama festival lain.

## Istilah baru yang dikunci sesi Fase 9 lanjutan (idx 437137-439136, batch kelima)

- **"Mohist Sect" (nama sekte/ordo di Hidden Mountain, muncul pertama kali di batch
  ini) DIBIARKAN UTUH bahasa Inggris** sebagai nama faksi proper noun — BEDA dari
  kata umum "Sect" yang diterjemahkan "Sekte" (mis. "Sect Rules"). Konsisten dengan
  pola "Mohist Hill"/"Mohist City" yang sudah dikunci sebelumnya sebagai nama tempat/
  faksi tidak diterjemahkan.
- **"Senior Sister"/"Junior Sister"/"Senior Brother"/"Junior Brother"** (sebutan wuxia
  antar sesama murid seperguruan) **dibiarkan Inggris untuk sekarang** — belum ada
  padanan Indonesia yang dikunci (opsi seperti "Kakak/Adik Seperguruan" dipertimbangkan
  tapi belum diputuskan); review ulang kalau istilah ini terus sering muncul.
- **Blok kode sumber developer (Lua) yang bocor ke data lokalisasi** (ditemukan di
  idx 437267, satu string berisi kode lengkap dengan komentar bahasa Inggris)
  **dibiarkan 100% tidak diterjemahkan** — bukan teks yang pernah dilihat pemain,
  menerjemahkan sebagian berisiko merusak variabel/sintaks.

## Istilah baru yang dikunci sesi Fase 9 lanjutan (idx 431887-433886, batch kedua)

- **Material/resource crafting Homestead berpola `"<Nama> Tier N"`** (Magnet Stone, Cloud
  Sand, Turquoise Stone, Iron Ore, Cinnabar, dan nama material majemuk seperti
  "Mushroom-Iron Composite", "Pine-Copper Cloudsand Extract") DIBIARKAN UTUH bahasa
  Inggris — diperlakukan sebagai identifier sistem crafting/resource seperti
  Inspiration/Affection, bukan kata benda umum.
- **PENTING — pola `"#Y<label>#E <Tag|id|#C|slot>"`**: label highlight dan tag placeholder
  adalah DUA unit terpisah — terjemahkan isi `#Y...#E`, biarkan tag `<...>` utuh menyusul
  TANPA digabung ke dalamnya. Jangan pernah menaruh teks di dalam tag `<...>` yang sudah
  berformat `|id|#C|slot`.

## Istilah baru yang dikunci sesi Fase 5 penutup + Fase 6 pembuka (idx 49000-50999)

- **"Pangolin" -> "Trenggiling"** (konfirmasi ulang aturan lama Fase 4, sempat
  terlewat untuk NPC "Pangolin Peddler" -> "Pedagang Trenggiling").
- **Jam ganda ala zodiak Tiongkok ("X Hour"/"X hour") -> "Jam X"** (mis. "Hai Hour"
  -> "Jam Hai", "Xu hour" -> "Jam Xu") — nama jam (Hai, Xu, dst.) dibiarkan Inggris/
  Pinyin sebagai istilah waktu tradisional tanpa padanan baku, hanya kata "Hour"
  yang diterjemahkan.
- **"Form" sebagai status transformasi (Carp form, Wind form, Vulpine Form, Feline
  Form) -> "Wujud"** (mis. "Vulpine Form" -> "Wujud Vulpine").
- **"master"/"Master" huruf kecil generik (bukan honorifik di depan nama, bukan
  pemimpin organisasi)** dibiarkan Inggris sebagai loanword "ahli/pengrajin" —
  beda dari "Master" honorifik nama (-> "Guru") dan "Master" pemimpin org (-> "Ketua").
- **Placeholder shorthand durasi huruf tunggal `s`->`d`/`h`->`j` juga berlaku** saat
  menempel langsung ke placeholder `{}` (mis. `{}s`->`{}d`) dan di dalam tag warna.
- **`$link<teks>^ID^$`** — placeholder link khusus, seluruh isi di antara `$link` dan
  `^ID^$` (termasuk teks Inggris di dalamnya) WAJIB dipertahankan 100% identik,
  diperlakukan sebagai satu unit opaque.
- **`$S...$E`** — pembungkus blok kutipan/narasi pada deskripsi skill (beda dari
  `$D`/`$H`/`$F` yang murni nilai angka) — isi di dalamnya tetap DITERJEMAHKAN
  normal, hanya penanda `$S`/`$E` sendiri yang dipertahankan persis.
- **"Wayfarer" (berdiri sendiri, bukan cuma "Wanderer") dikonfirmasi ulang -> "Pengembara"**.

## Catatan ambiguitas yang belum konsisten sempurna (untuk direview kalau ketemu lagi)

- **"Power"**: kadang diterjemahkan "Kekuatan" (kata umum berdiri sendiri), tapi kalau
  muncul sebagai bagian dari daftar "Five Attributes" (bersama Constitution, Defense,
  Agility, Momentum) sebaiknya dibiarkan Inggris seperti stat lain. Idx 282 sudah
  terlanjur "Kekuatan" — biarkan saja (dampak kecil), tapi untuk kemunculan baru dalam
  konteks stat-list, pertimbangkan biarkan Inggris.
- **"Trial"** -> "Uji Coba" (dipakai konsisten untuk mode tantangan/dungeon trial).
- **"Draw"** dibiarkan Inggris (gacha feature), termasuk "Draw Shop", "Draw Appearance".

## Gaya/nada

- Dialog karakter (percakapan orang pertama/kedua) -> casual: gue/lo.
- Label UI/tombol/sistem (Loading, Confirm, Cancel, dst.) -> netral-casual, TANPA gue/lo.
- Placeholder/format token (`{0}`, `{}`, `%s`, `%d`, `#E`, `#aabbcc`, `#X`) WAJIB dipertahankan
  persis, jumlah & urutan sama seperti sumber (dicek otomatis oleh `tools/qa_check.py`).

## Progress tracking

- `unique_strings.jsonl` — 429.887 string unik, terurut frekuensi terbanyak dulu (idx 0..N-1).
- `translations.jsonl` — hasil terjemahan, append-only, `{"idx": N, "v": "..."}` per baris,
  ditulis berurutan sesuai idx (0, 1, 2, ...). **Baris terakhir di file ini menandai idx
  terakhir yang sudah selesai** — untuk resume, cek jumlah baris file ini.
- `handoff.md` — status naratif + instruksi resume untuk sesi berikutnya.
- Final expansion (nanti setelah semua/sebagian besar selesai): script baca
  `unique_strings.jsonl` + `translations.jsonl` -> bikin dict src->tgt, lalu stream
  `../strings.jsonl` asli, replace `v` sesuai dict, tulis ke `strings.translated.jsonl`
  (siap dipakai `qa_check.py` lalu `wwm_locmap.py patch`).
