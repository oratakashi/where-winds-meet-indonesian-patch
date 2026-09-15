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
  Greenwood (bandit), NetEase, dll.
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
Bamboocut/Stonesplit/Formless Attack (nama tipe damage khas game ini), Tier, Stage, Lv.

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
