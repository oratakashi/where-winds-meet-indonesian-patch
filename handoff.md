# Handoff — Terjemahan Indonesia WWM

**Baca `plan.md` sekali di awal sesi** untuk tahu pembagian fase & saran ukuran batch.
File ini (`handoff.md`) adalah status kerja yang **diupdate tiap akhir sesi** — sumber
kebenaran untuk "sudah sampai mana".

## Status saat ini

- **Fase 0, Fase 1, Fase 2, Fase 3, dan Fase 4 SELESAI** (idx 0–19.999). Fase saat ini:
  **Fase 5** (rentang idx 20.000–49.999, 30.000 unik, batch 1.000/sesi) — **2 sesi
  selesai**: idx 20.000–21.999 (2.000 baris) sudah ada di `locale/phase5.jsonl`, next idx
  = 22.000.
- Total baris di `strings.jsonl`: **963.050**
- Total string unik (setelah dedup): **429.887**
- **Sudah diterjemahkan: idx 0–21.999 dari 429.887 (22.000 string unik, ~5.12%)**
- Sesi terakhir mengerjakan: 2026-09-16 — Fase 5 lanjut, batch kedua idx 21.000–21.999
  (1.000 baris baru di `locale/phase5.jsonl`), tervalidasi 0 mismatch token & 0
  duplikat/gap idx (dicek lintas semua file `locale/phase*.jsonl` sekaligus, total
  22.000 idx unik tercatat tanpa tabrakan, kontigu penuh 0–21.999 tanpa lubang).
- **File progress dipecah per fase** di `locale/phase{N}.jsonl` (mis. `locale/phase0.jsonl`,
  `locale/phase1.jsonl`, `locale/phase2.jsonl`, dst. — mengikuti nomor fase & rentang idx
  di tabel `plan.md`). Tiap file berisi `{"idx": N, "v": "..."}` per baris, `idx` yang
  dipakai adalah idx **absolut** dari `unique_strings.jsonl` (bukan di-reset ke 0 per file),
  jadi antar-file tetap gampang di-cross-reference. Sudah ada: `locale/phase0.jsonl` (800
  baris, lengkap), `locale/phase1.jsonl` (1.200 baris, lengkap), `locale/phase2.jsonl`
  (3.000 baris, lengkap), `locale/phase3.jsonl` (5.000 baris, lengkap), `locale/phase4.jsonl`
  (10.000 baris, lengkap), `locale/phase5.jsonl` (2.000 baris, **jalan** —
  rentang penuh fase ini 30.000 baris/idx 20.000–49.999). Sesi berikutnya lanjutkan
  `locale/phase5.jsonl` mulai idx 22.000.
  **Baris terakhir di file fase AKTIF = idx terakhir yang selesai.**
  Cek dengan: `wc -l locale/phase{N}.jsonl` (N = nomor fase saat ini dari `plan.md`);
  next idx = idx_awal_fase + jumlah_baris.

String diurutkan berdasarkan **frekuensi kemunculan** (bukan urutan asli file), jadi
yang paling sering dipakai di 963rb baris asli dikerjakan duluan — coverage baris
riil lebih cepat naik daripada persentase unik di atas. 3.000 unik pertama ini sudah
mencakup **23.99% dari 963.050 baris total** (karena banyak string berulang ribuan
kali, mis. "Talk" x11.299, "Loading..." x10.002). Semua 3.000 sudah lolos cek token
format (`#E`, `#aabbcc`, `%s`, `%d`, `{0}`/`{value}` dst., termasuk tag `<Label|id|#C|n>`
— lihat script di bawah, sudah diperluas untuk ikut cek tag `<...>` juga. 1 mismatch
sempat ketemu & sudah diperbaiki di idx 1056 — tag `#Y...#E` yang membungkus banyak
kata sempat hilang saat translate, jadi hati-hati kalau tag membungkus lebih dari
satu kalimat).

## Update game 2026-09-16: tool & pipeline disesuaikan

User update game-nya dan menemukan file locale baru: `translate_words_map_en__small` +
`__small_diff`, berdampingan dengan `translate_words_map_en`/`_diff` yang sudah ada.

- **Format binernya TIDAK berubah** — dikonfirmasi round-trip dump→patch→parse byte-identik
  pada keempat file, tanpa mengubah `wwm_locmap.py` sama sekali. Detail temuan (98% key `__small`
  overlap dengan file utama tapi 193 value beda, 27 key eksklusif) ada di `docs/FORMAT.md` §4.6
  dan `CLAUDE.md`.
- File utama versi baru **berkurang** dari 963.050 -> 826.388 entri (138.445 key dihapus, 1.783
  ditambah, 32.397 berubah value) dan re-shard (3.762 -> 3.229 shard) — tidak masalah untuk tool
  karena parser tidak pernah menghitung ulang shard index sendiri.
- Ditemukan (dan diperbaiki) bug di `tools/expand_locale.py`: crash `KeyError: 'v'` kalau dijalankan
  atas dump yang mengandung tombstone (`"deleted": true`, dari file `_diff`) — sekarang di-skip
  sama seperti `qa_check.py`.
- Dua script baru: `tools/rebuild_unique_strings.py` (menambah string baru yang ditemukan setelah
  update ke `unique_strings.jsonl` **tanpa mengubah idx lama** — lihat "Fase 9" di `plan.md`) dan
  `tools/patch_all.py` (menerapkan dictionary terjemahan yang sudah ada ke keempat file variant
  sekaligus, tanpa perlu jalan manual satu-satu).
- Sudah dijalankan di sesi ini: `strings.jsonl` di-dump ulang dari file utama versi baru,
  `unique_strings.jsonl` diperluas ke idx 461.703 (+31.817 string baru dari update), dan dictionary
  20.000 string yang sudah diterjemahkan (fase 0-4) diterapkan ke keempat file lewat `patch_all.py`
  (hasil di folder `patched/`, tidak di-commit — lihat cakupan match per file di bawah). Semua
  hasil lolos `qa_check.py` (0 PROMPT_LEAK/MARKUP/EMPTY).

  | file                              | entri   | cocok (sudah diterjemahkan) |
  | ---------------------------------- | ------- | ---------------------------- |
  | `translate_words_map_en`           | 826.388 | 358.162 (43.34%)              |
  | `translate_words_map_en_diff`      | 212.117 | 38.094 (17.96%)               |
  | `translate_words_map_en__small`    | 4.009   | 2.148 (53.58%)                |
  | `translate_words_map_en__small_diff` | 0     | 0                             |

- **Belum dikerjakan** (di luar scope sesi ini, murni tooling/pipeline): menerjemahkan 31.817
  string baru di idx 429.887–461.703 (Fase 9) — lanjutkan dengan prosedur biasa di `plan.md`/
  bagian "Cara resume" di bawah, sama seperti fase lain.
- Kalau update game berikutnya menambah variant file lagi (misal `translate_words_map_en__small2`
  atau semacamnya), tinggal: `wwm_locmap.py dump <file> strings_<nama>.jsonl`, tambahkan path itu
  ke `--extra` di `rebuild_unique_strings.py` (atau default list-nya), dan tambahkan nama filenya
  ke `FILES` di `tools/patch_all.py`.

## PENTING: sisa pekerjaan sangat besar

407.887 string unik lagi setelah progress ini. Lihat `plan.md` untuk perkiraan jumlah
sesi per fase (kasar: 125–190+ sesi total sampai 100%). Sampaikan ini ke user kalau
ditanya estimasi waktu, dan ingatkan opsi berhenti di ~50% baris (lihat "Titik berhenti
yang masuk akal" di `plan.md`) kalau relevan.

## Cara resume di sesi berikutnya

1. Baca `plan.md` → cek fase saat ini (nomor N), rentang idx-nya, & saran ukuran batch.
2. Baca `translation_work/GLOSSARY.md` — berisi semua aturan konsisten yang HARUS diikuti
   (nama yang tidak diterjemahkan, istilah yang tetap bahasa Inggris, gaya bahasa per
   konteks). **Jangan menerjemahkan tanpa baca file ini dulu**, supaya konsisten dengan
   yang sudah dikerjakan.
3. Cek idx terakhir yang selesai: `wc -l locale/phase{N}.jsonl` (kalau file belum ada,
   berarti fase ini belum mulai — next idx = idx_awal fase itu; kalau sudah ada, next
   idx = idx_awal_fase + jumlah_baris_di_file).
4. Baca batch berikutnya dari `translation_work/unique_strings.jsonl` mulai baris
   (next_idx + 1) (1-indexed) sebanyak sesuai saran batch fase saat ini (`Read` dengan
   `offset`/`limit`).
5. Terjemahkan tiap `v` sesuai aturan di GLOSSARY.md, tulis ke file sementara
   `{"idx": N, "v": "..."}` per baris (idx tetap idx **absolut** dari `unique_strings.jsonl`),
   urut naik dari idx yang sedang dikerjakan.
6. **WAJIB validasi sebelum append**: jalankan cek token/placeholder (lihat contoh
   script di bawah) untuk memastikan `#E`, `#aabbcc`, `#X`, `%s`, `%d`, `{0}` dst.
   jumlahnya sama persis dengan sumber. Ini yang nanti dicek juga oleh `tools/qa_check.py`.
7. Append hasil batch ke `locale/phase{N}.jsonl` (file fase yang sedang aktif — **jangan**
   ditulis ke file fase lain, dan kalau rentang idx fase ini sudah habis di tengah batch,
   potong batch itu supaya sisanya masuk ke `locale/phase{N+1}.jsonl` yang baru).
   Pastikan idx tetap berurutan tanpa lompat/duplikat di dalam tiap file — bisa dicek
   dengan script kecil (lihat contoh validasi di bawah, tinggal ganti nama filenya).
8. Update bagian "Status saat ini" di file ini (idx terakhir, fase, tanggal sesi, dan
   catatan keputusan baru kalau ada istilah ambigu yang baru ditemui — tambahkan juga
   ke `GLOSSARY.md` supaya konsisten ke depannya). Kalau rentang idx fase saat ini sudah
   habis, majukan ke fase berikutnya sesuai tabel di `plan.md`.
9. **Berhenti bersih di akhir batch** — jangan lanjut sampai konteks/token mepet.
   Lebih baik sesi pendek yang tercatat rapi daripada sesi panjang yang terputus
   di tengah tanpa sempat validasi & update handoff.

### Script pengecekan token (jalankan dari root repo — ganti `phase{N}.jsonl` sesuai fase aktif)

```python
import json, re
from collections import Counter

TOKEN = re.compile(r'#[A-Za-z]|#[0-9a-fA-F]{6}|%s|%d|\{[^}]*\}|<[^>]*>')

src = {}
with open('translation_work/unique_strings.jsonl', encoding='utf-8') as f:
    for line in f:
        d = json.loads(line)
        src[d['idx']] = d['v']

mismatches = []
with open('locale/phase4.jsonl', encoding='utf-8') as f:  # <- ganti sesuai fase aktif
    for line in f:
        d = json.loads(line)
        s = src[d['idx']]
        t = d['v']
        cs = Counter(TOKEN.findall(s))
        ct = Counter(TOKEN.findall(t))
        if cs != ct:
            mismatches.append((d['idx'], s, t))

print('mismatches:', len(mismatches))
for m in mismatches[:20]:
    print(m)
```

### Catatan istilah baru dari sesi lanjutan Fase 4 (idx 14100-15099)

- **Nama lokasi majemuk generik** (mis. "Hutuo Region", "Kaifeng - Fairgrounds") dibiarkan
  **utuh bahasa Inggris** (bukan diterjemahkan sebagian seperti pola "East City Commoner") —
  konsisten dengan "Hutuo River"/"Sunken City Lake" yang sudah dikunci sebelumnya.
- **"Sect Master" -> "Ketua Sekte"** (gelar pemimpin sekte, sama pola dengan "Master" =
  pemimpin organisasi -> Ketua yang sudah dikunci).
- **Aksi tempur singkat di instruksi kontrol (Press/Hold)** seperti "Deflect"/"Defense"/
  "Guard" DITERJEMAHKAN jadi kata kerja pendek (Tangkis/Bertahan) ketika muncul sebagai
  label instruksi tombol, BEDA dari nama skill/tag berjudul yang tetap Inggris.
- **"Red Packets" (sinonim "Red Envelope") -> "Angpao"** juga, konsisten.
- **Durasi campuran hari+jam "Nd Mh" (mis. "9 d 7 h")** -> "Nh Mj" (mis. "9 h 7 j") — "h"
  untuk hari, "j" untuk jam, memperluas konvensi "Nd"->"Nh" yang sudah dikunci.
- **"Inner Way" (fitur koleksi, mis. "Complete Inner Way Collection II")** dibiarkan Inggris
  sebagai nama fitur, sama pola dengan Solo Mode/Co-op Mode dll.
- **"Main Story" sebagai label kategori chapter** (mis. "Qinghe Main Story Boss Battle
  Space", "Hidden Mountain Main Story P2 - ...") dibiarkan utuh Inggris — diperlakukan
  sebagai internal quest-chapter label, bukan judul naratif yang diterjemahkan.
- String Han (`"地图用-万古一人殿2层"`, idx 14850) muncul di data — label internal peta
  (Chinese leftover), diterjemahkan strukturnya jadi "Untuk Peta - Wangu Yiren Hall Lantai 2"
  (nama aula ditransliterasi apa adanya, mengikuti pola nama lokasi yang tidak diterjemahkan).

### Catatan istilah baru dari sesi lanjutan Fase 4 (idx 15100-16099)

- **Durasi dalam detik ditulis literal (mis. "18s", "6s")** -> disingkat "d" (detik), mis.
  "18s" -> "18d", "Low Tenacity Mode - 6s" -> "Mode Tenacity Rendah - 6d". Ini memperluas
  konvensi durasi (Nd hari->Nh, Nh jam->Nj) ke satuan detik — HANYA berlaku untuk teks
  literal biasa, BUKAN untuk placeholder/format string (`%d`, `{}`, dst.) yang tetap
  dipertahankan persis.
- **Tag `<...>` tanpa penutup `>`** (mis. idx 15273 "<Tangled Gauze by Martial Art Skill:
  Crane Wings extends by...") tidak match regex TOKEN `qa_check.py` (butuh `>` penutup),
  jadi diperlakukan sebagai teks biasa — literal `<` di awal dipertahankan, sisanya
  diterjemahkan normal. Beda dengan tag panjang yang PUNYA penutup `>` (dibiarkan Inggris
  utuh, sudah dikunci sesi sebelumnya).
- **"Sect Master"/pemimpin sekte generik** dst. sudah konsisten dengan "Ketua Sekte".
- Tidak ada keputusan konvensi baru signifikan lain di batch ini — mayoritas nama
  NPC/gear/skill mengikuti pola yang sudah terkunci di sesi-sesi sebelumnya.

### Catatan istilah baru dari sesi lanjutan Fase 4 (idx 18100-19099)

- **Frasa puitis pendek berdiri sendiri (1-4 kata, tanpa penomoran/struktur quest)** yang
  muncul sebagai nama item/skin/mount/emote (mis. "Fleeting Dream", "Clear Glow", "Eternal
  Watch", "Night Glow", "Wildtrail", "Steady Ascent", "Spring's Bounty", "Winter's Bloom",
  "Oats in the Wind", "Paired Shadows", "Haven Astray", "Bright Sky") **DIBIARKAN Inggris**,
  konsisten dengan pola nama set kostum/gear yang sudah dikunci — beda dengan judul quest/
  chapter yang punya struktur jelas (angka, "Tales Retold:", "Volume", tanda "-" + peran)
  yang tetap DITERJEMAHKAN.
- **Tag `<...>` yang membungkus dialog/kalimat panjang dari NPC non-manusia (kucing, dll.)**
  seperti idx 18461 `<Pft. One round. We still have three more chances...>` — mengikuti
  aturan tag-panjang-satu-token dari sesi sebelumnya, dibiarkan 100% identik Inggris,
  termasuk isi dialognya sendiri (bukan cuma nama stat).
- **Tag highlight pendek `#H...#E`/`#Y...#E` yang membungkus SATU KATA instruksi umum**
  (mis. "Press", "night") **isinya DITERJEMAHKAN** (mis. "#HPress#E" -> "#HTekan#E",
  "#Ynight#E" -> "#Ymalam#E") — beda dengan tag yang membungkus nama skill/weapon
  (mis. "#HFire Arrow#E" dibiarkan Inggris) atau kalimat panjang (dibiarkan Inggris).
  Regex `TOKEN` di `qa_check.py` mencocokkan `#H`/`#Y`/`#E` sebagai token terpisah
  (satu huruf), jadi teks di antaranya aman diterjemahkan tanpa mismatch.
- **"Main Story" sebagai label kategori tetap dikonfirmasi ulang dibiarkan Inggris**
  (mis. "New Main Story" -> "Main Story Baru"), konsisten dengan keputusan sesi idx
  14100-15099.
- **Item/hewan/tumbuhan bernama umum (bukan proper noun fantasi)** seperti "Pangolin",
  "Sparrow Egg", "Crane Egg", "Snow Ape", "Long-Tailed Pheasant", "Lanternfish",
  "Bamboo Shoot" **DITERJEMAHKAN** ke istilah Indonesia yang wajar, beda dari nama
  gear/weapon fantasi buatan (Swallowcall, Jadesong, Voidchant, Darkecho, dst.) yang
  tetap Inggris. Untuk kata majemuk campuran (nama fantasi + kata umum, mis. "Peltwing
  Squirrel"), kata fantasinya dibiarkan Inggris & kata umumnya diterjemahkan ("Tupai
  Peltwing").
- **"Old X" tanpa "Man"** (mis. "Old Shi") tetap dibiarkan Inggris mengikuti keputusan
  sebelumnya (ambigu nickname vs deskriptif).
- Placeholder format Python literal seperti `{diff_hour:02d}h{diff_minute:02d}m{diff_second:02d}s`
  (idx 18946) **dipertahankan 100% identik** — regex `TOKEN` mencocokkan tiap `{...}`
  sebagai satu token, jadi seluruh string ini otomatis tervalidasi asalkan tidak diubah
  sama sekali.

### Catatan istilah baru dari sesi penutup Fase 4 (idx 19100-19999)

- **Tier/rank profesi generik dalam format `"Profesi: Tier"`** (mis. "Healer: Novice",
  "Healer: Adept", "Healer: Redemption", "Scholar: Novice") — kata tier-nya (Novice,
  Adept, Redemption, dst.) **DIBIARKAN Inggris**, sama pola dengan Rank/Tier/Stage/Lv
  yang sudah dikunci — hanya nama profesi generik (Healer, Scholar) juga dibiarkan
  Inggris karena sudah jadi konvensi nama profesi (lihat aturan lama). Kalau tier-nya
  berupa kata sifat naratif jelas (mis. "Scholar: Refined Gentleman", "Scholar: Silver
  Tongue"), itu DITERJEMAHKAN karena berfungsi sebagai gelar naratif, bukan tier numerik.
- **String Han leftover kedua ditemukan**: idx 19719 `"瀑布下时装半透区域"` (area transparansi
  kostum di bawah air terjun) — diterjemahkan strukturnya ke Indonesia sepenuhnya karena
  ini label internal developer, tidak ada bagian nama yang perlu dipertahankan (beda
  dari idx 14850 yang punya nama aula spesifik untuk ditransliterasi).
- **Nama bahasa di UI pemilihan bahasa** (mis. "Русский язык", "日本語", "Español（Latino）")
  **TIDAK diterjemahkan** — dibiarkan dalam skrip/bahasa aslinya masing-masing karena ini
  representasi nama bahasa itu sendiri, bukan teks naratif. Konvensi umum di semua game
  multi-bahasa.
- **Placeholder durasi gabungan format+literal** seperti `{diff_day:d}d ago` atau
  `{diff_hour:d}h ago` — bagian `{...}` (format Python) WAJIB dipertahankan persis, tapi
  literal suffix di luar tanda kurung kurawal (`d ago`/`h ago`) bebas diterjemahkan
  mengikuti konvensi durasi yang sudah dikunci (`d`->`h` hari, `h`->`j` jam), mis.
  `{diff_hour:d}h ago` -> `{diff_hour:d}j lalu`.
- **Tag highlight pendek yang membungkus satu kata sifat/label** (mis.
  `#aee5aeAdvanced#E`) tetap mengikuti aturan dari batch sebelumnya: isinya
  DITERJEMAHKAN (`#aee5aeAdvanced#E` -> `#aee5aeLanjutan#E`) karena regex `TOKEN`
  mencocokkan tag warna heksadesimal (`#[0-9a-fA-F]{6}`) dan `#E` secara terpisah,
  bukan seluruh tag sebagai satu unit.
- Catatan minor lain: banyak kata benda umum berpasangan Chinese-loanword/nickname
  (mis. "Old Pan", "Dog Three") dibiarkan Inggris karena statusnya ambigu antara
  nickname formal vs deskriptif, konsisten dengan keputusan "Old X" sebelumnya.

### Catatan istilah baru dari sesi Fase 5 lanjutan (idx 21000-21999)

- **"Enhancement" pada label node skill-tree/talent** (mis. "Momentum Enhancement",
  "Physical Resistance Enhancement", "Critical Resistance Enhancement", "Water Clone
  Enhancement", "Perfect Catch Enhancement", "Scroll & Script Enhancement", "Charge
  Calculation Enhancement", "Qi Struggle Enhancement") **DIBIARKAN UTUH bahasa Inggris**
  — pola sama dengan "DMG Boost"/"DMG Bonus"/"DMG Reduction" yang sudah dikunci di
  GLOSSARY sebagai compound stat-label bahasa Inggris. Begitu juga "Advanced Defense
  Boost" (pola "X Boost").
- **"Appearance" sebagai kategori kosmetik/skin UI DITERJEMAHKAN jadi "Tampilan"**
  (mis. "Spear Appearance" -> "Tampilan Spear", "Mystic Skill Appearance" -> "Tampilan
  Mystic Skill") — bukan proper noun, kata umum dengan padanan langsung. Nama tipe
  senjata generik (Spear, Blade, Gauntlets, dst.) yang menempel tetap Inggris mengikuti
  pola weapon-class name yang sudah dikunci.
- **Placeholder durasi literal detik (`Ns`) di dalam tag `#Y...#E`/`#G...#E` juga ikut
  dikonversi** -> `Nd` (mis. `#Y70s#E` -> `#Y70d#E`, `#Y20s#E` -> `#Y20d#E`, `0.6s` ->
  `0.6d`) — perluasan konvensi durasi detik dari Fase 4 (idx 15100-16099), berlaku juga
  saat literalnya ada di dalam tag warna/highlight (bukan cuma teks polos).
- **Nomor tag warna non-standar berpola `#<hex-ish>NNNN Word#E`** (mis. "Increase to
  #8C5823120 Points#E of #406182Max Bellstrike Attack#E") — regex `TOKEN` di
  `qa_check.py` mencocokkan 6 karakter pertama setelah `#` sebagai token warna
  (`#[0-9a-fA-F]{6}`), sisanya (angka lanjutan + kata) adalah teks biasa yang AMAN
  diterjemahkan. Contoh: `#8C5823120 Points#E` -> token `#8C5823` + teks bebas
  `120 Poin` + token `#E`. "Points" -> "Poin" dikonfirmasi diterjemahkan dengan pola ini.
- **"Union" (dari catatan Fase 5 sebelumnya, masih ambigu) muncul lagi** (mis. "Healers'
  Union", "Artificer Union", "Union elections", "Union Bonus Draw") — tetap dibiarkan
  Inggris konsisten dengan keputusan sebelumnya, pola organisasi pemain mirip Guild.
- **"Scholar"/"Healer" sebagai nama kelas Profession (bukan sebutan NPC generik)
  dikonfirmasi ulang dibiarkan Inggris** (mis. "Scholar Jiang Huaiyuan", "Scholar Class
  72") — beda dari peran NPC deskriptif biasa (Villager, Bandit, dst.) yang diterjemahkan;
  di sini "Scholar" berfungsi sebagai nama kelas/profesi seperti "Healer" yang sudah
  dikunci di Fase 4.
- **Warna/istilah pigmen dalam tanda kutip pada teks lore item cat** (mis. "Crab Shell",
  "vermilion", "Lychee", "white") **dibiarkan Inggris dalam kutip** — merujuk nama kode
  warna gaya pengrajin (naming pun ala seniman), bukan teks naratif biasa, konsisten
  antar dua kemunculan (idx 21301 & 21940).
- Beberapa nama/istilah baru yang statusnya ambigu dibiarkan Inggris menunggu konteks
  lebih jelas (konsisten dengan pola "Old X"/"Union" sebelumnya): "Loan Cloud" (idx
  21551, kemungkinan typo sumber dari "Lone Cloud" — dipertahankan literal sesuai teks
  sumber, TIDAK dikoreksi), "Retainer" (sistem NPC pendamping homestead), "Static
  Atmosphere Group" (label internal).

Untuk validasi cepat semua fase sekaligus, loop `glob('locale/phase*.jsonl')` dan gabungkan
semua baris sebelum dicek — juga bagus untuk sekalian memastikan tidak ada idx yang
tertulis dobel di dua file fase berbeda.

## Setelah semua (atau cukup banyak) selesai: expand ke strings.jsonl penuh

Belum dibuat scriptnya. Rencana: baca `translation_work/unique_strings.jsonl` + semua
`locale/phase*.jsonl` (gabungkan) untuk bikin dict `src_v -> translated_v`, lalu stream
`strings.jsonl` asli, replace field `v` sesuai dict (skip/biarkan asli kalau belum
diterjemahkan — partial translation didukung oleh `patch` sesuai CLAUDE.md), tulis ke
`strings.translated.jsonl`. Baru lanjut
`python tools/qa_check.py strings.jsonl strings.translated.jsonl` lalu
`python wwm_locmap.py patch translate_words_map_en strings.translated.jsonl <out>`.

## Keputusan/konvensi yang sudah diambil (lihat GLOSSARY.md untuk detail lengkap)

- Dedup dulu (translate per string unik), baru expand ke semua baris — disetujui user.
- Tone: dialog karakter = casual (gue/lo), UI/sistem = netral-casual tanpa gue/lo — disetujui user.
- Nama karakter/tempat/faksi/skill-berjudul TIDAK diterjemahkan.
- Judul quest/chapter puitis (bukan nama orang) DITERJEMAHKAN.
- Label stat/attribute (Critical Rate, Physical Attack, Silkbind/Bellstrike/Bamboocut/
  Stonesplit/Formless Attack, dst.) dibiarkan penuh bahasa Inggris.
- Tag format (`#Y...#E`, `#H...#E`, `#R...#E`, `<Label|id|#C|n>`, `{0}`, `%s`, dll.)
  WAJIB dipertahankan persis — termasuk teks di dalam tag `<...>` yang merujuk nama
  stat (biarkan bahasa Inggris juga, karena kebetulan konsisten dengan aturan stat di atas).
- Sebutan kekerabatan/status di depan nama NPC (Aunt/Uncle/Grandpa/Granny/Elder/Master/
  Lady/Madam/Miss/Mr.) DITERJEMAHKAN konsisten (Bibi/Paman/Kakek/Nenek/Tetua/Guru/Nyonya/
  Nona/Tuan) — dikunci sesi Fase 4, lihat `GLOSSARY.md` bagian "Fase 4" untuk detail &
  pengecualian (Master sebagai pemimpin organisasi -> Ketua, bukan Guru).
- Placeholder non-standar tanpa tag (`$VAR:.1f$`, `$P`, `$N`, literal `Xd, Xh` dst.) tidak
  tertangkap regex TOKEN di `qa_check.py` tapi WAJIB tetap dipertahankan persis karakter
  demi karakter — ini variabel substitusi runtime.

## Catatan lain

- `strings.jsonl` dan `translate_words_map_en` masih ke-staged di git (bukan diubah
  sesi ini atas permintaan user — "biarkan saja"). File-file kerja terjemahan
  (`translation_work/`, `strings.jsonl`) sudah ditambahkan ke `.gitignore` supaya
  tidak ikut ter-commit ke depannya, tapi staging area yang sudah ada tidak disentuh.
