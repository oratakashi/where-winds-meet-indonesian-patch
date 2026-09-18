# Handoff — Terjemahan Indonesia WWM

**Baca `plan.md` sekali di awal sesi** untuk tahu pembagian fase & saran ukuran batch.
File ini (`handoff.md`) adalah status kerja yang **diupdate tiap akhir sesi** — sumber
kebenaran untuk "sudah sampai mana".

## Status saat ini

- **Fase 0, Fase 1, Fase 2, Fase 3, dan Fase 4 SELESAI** (idx 0–19.999). **Fase 5**
  (rentang idx 20.000–49.999, 30.000 unik, batch 2.000/sesi) masih **jalan**: idx
  20.000–42.999 (23.000 baris) ada di `locale/phase5.jsonl`, next idx (kalau lanjut
  fase 5) = 43.000 — **tidak disentuh sesi ini**.
- Total baris di `strings.jsonl`: **963.050** (versi lama; versi 2026-09 update = 826.388
  di file utama, lihat bagian "Update game 2026-09-16")
- Total string unik: **461.704** (429.887 asli + 31.817 dari update game, idx 0–461.703)
- **Progress Fase 0–5: idx 0–42.999 selesai (43.000/429.887 string dari batch asli) —
  tidak disentuh sesi ini.**
- **Progress Fase 9: idx 429.887–439.136 selesai (9.250/31.817 string, ~29.07%
  dari Fase 9) — `locale/phase9.jsonl`, next idx = 439.137.**
- Sesi ini (2026-09-18, batch kelima Fase 9): idx 437.137–439.136 (2.000 baris baru,
  batch penuh sesuai preferensi user) diterjemahkan langsung oleh sesi utama (bukan
  didelegasikan ke subagent — beda dari batch sebelumnya), diproses dalam 4 sub-batch
  500 lalu digabung. **1 mismatch token ditemukan & diperbaiki** sebelum di-append:
  idx 438753 (tag pendek `<Refuses to enter the stone square>`, deskripsi aksi anjing
  menggonggong, sempat ikut diterjemahkan jadi `<Menolak masuk ke alun-alun batu>` —
  dikembalikan ke Inggris apa adanya sesuai aturan tag-pendek-non-format-dibiarkan-
  Inggris, sama pola dengan idx 436442 sesi sebelumnya). Spot-check manual tambahan
  untuk konvensi "Player selalu Inggris" dijalankan atas seluruh batch (grep
  `[Pp]emain` di file output) — **0 pelanggaran ditemukan** kali ini (satu match
  "pemain" di idx 438221 dikonfirmasi bukan pelanggaran: itu kalimat sastra
  metaforis "Dalam permainan cinta, para pemainnya selalu buta" dalam cerita lore
  Ding Jiexiang/Ye Zhuo, bukan istilah mekanik game). 0 duplikat/gap idx dicek lintas
  semua file `locale/phase*.jsonl` sekaligus (total 52.250 baris/idx unik tercatat
  tanpa tabrakan), `phase9.jsonl` sendiri kontigu penuh 429.887–439.136 tanpa lubang.
- **Catatan istilah baru sesi ini**: "Mohist Sect" (nama sekte/ordo di Hidden Mountain,
  BEDA dari "Sect" generik yang diterjemahkan "Sekte") **dibiarkan UTUH bahasa
  Inggris** sebagai nama faksi proper noun, konsisten dengan pola "Mohist Hill",
  "Mohist City" yang sudah dikunci. "Senior Sister"/"Junior Sister"/"Senior Brother"/
  "Junior Brother" (sebutan wuxia untuk sesama murid seperguruan) **dibiarkan Inggris**
  untuk sesi ini (belum ada padanan Indonesia yang dikunci — kandidat untuk direview
  ulang kalau sering muncul lagi). String kode Lua developer yang bocor ke data
  lokalisasi (idx 437267, blok kode lengkap dengan komentar Inggris) **dibiarkan 100%
  tidak diterjemahkan** — bukan teks pemain, berisiko rusak kalau disentuh.

### Riwayat batch Fase 9 sebelumnya (untuk referensi)

- Sesi 2026-09-17 (batch ketiga Fase 9): idx 433.887–435.886 (2.000 baris baru di
  `locale/phase9.jsonl`, ukuran iterasi 2.000/sesi — user eksplisit konfirmasi ulang
  ukuran ini di sesi ini), diproses dalam 8 sub-batch 250 lalu digabung, tervalidasi 0
  mismatch token pada percobaan final (5 mismatch sempat ditemukan saat validasi
  pertama — idx 434451 & 435245 & 435796 tiga tag `<...>` panjang satu-token yang
  sempat diterjemahkan padahal harus dibiarkan 100% Inggris [aturan lama sejak idx
  12448, masih sering kejadian ulang], idx 434808 kasus baru: string `"#Talk to the
  Guard"` — `#` di sini BUKAN tag highlight sungguhan, tapi kebetulan cocok regex TOKEN
  `#[A-Za-z]` sebagai `#T`; solusinya bukan menerjemahkan literal apa adanya, tapi
  menyisakan `#Talk` utuh di awal & menerjemahkan sisanya (`#Talk dengan Penjaga`) supaya
  token `#T` tetap cocok, dan idx 435165 kehilangan 1 dari 3 kemunculan
  `#YMartial Art Triggered Effects#E` — paragraf kedua "Applicable #Y...#E include:"
  sempat diterjemahkan sebagai teks polos tanpa tag — kelimanya diperbaiki sebelum
  di-append). Ditemukan juga (di luar validasi token, lewat audit manual) 2 kasus
  "steward" (idx 434159, 435261, huruf kecil/peran generik) yang sempat dibiarkan
  Inggris — diperbaiki jadi "pelayan" supaya konsisten dengan aturan GLOSSARY.md lama
  "Steward -> Pelayan" (Ritual Steward & #YSteward#E di idx 434848/435076 dibiarkan
  Inggris karena berfungsi sebagai judul/label kapital, bukan sebutan generik). 0
  duplikat/gap idx (dicek lintas semua file `locale/phase*.jsonl` sekaligus + batch
  baru, total 43.000 baris/idx unik tercatat tanpa tabrakan, `phase9.jsonl` sendiri
  kontigu penuh 429.887–435.886 tanpa lubang).
- Sesi sebelumnya (2026-09-17, batch pertama): idx 429.887–431.886 (2.000 baris),
  diproses dalam 4 sub-batch 500 lalu digabung, tervalidasi 0 mismatch token pada
  percobaan final (2 mismatch sempat ditemukan di sub-batch keempat — idx 431459 salah
  tag `#G` alih-alih `#Y` pada satu segmen, dan idx 431600 nambahin `#E` penutup yang
  tidak ada di sumber — keduanya diperbaiki sebelum di-append).
  **Catatan penting**: karena ini fase baru dari update game (bukan lanjutan dari
  `unique_strings.jsonl` versi lama), semua istilah wuxia standar dari GLOSSARY.md tetap
  dipakai, tapi ada beberapa keputusan baru untuk istilah spesifik konten update ini
  (lihat "Catatan istilah baru Fase 9" di bawah) — termasuk keputusan **"Farmer" (unit
  sistem Homestead) dibiarkan UTUH bahasa Inggris** (kapital, seperti Retainer), berbeda
  dari role NPC generik biasa yang diterjemahkan.
- **File progress dipecah per fase** di `locale/phase{N}.jsonl` (mis. `locale/phase0.jsonl`,
  `locale/phase1.jsonl`, `locale/phase2.jsonl`, dst. — mengikuti nomor fase & rentang idx
  di tabel `plan.md`). Tiap file berisi `{"idx": N, "v": "..."}` per baris, `idx` yang
  dipakai adalah idx **absolut** dari `unique_strings.jsonl` (bukan di-reset ke 0 per file),
  jadi antar-file tetap gampang di-cross-reference. Sudah ada: `locale/phase0.jsonl` (800
  baris, lengkap), `locale/phase1.jsonl` (1.200 baris, lengkap), `locale/phase2.jsonl`
  (3.000 baris, lengkap), `locale/phase3.jsonl` (5.000 baris, lengkap), `locale/phase4.jsonl`
  (10.000 baris, lengkap), `locale/phase5.jsonl` (23.000 baris, **jalan** — rentang penuh
  fase ini 30.000 baris/idx 20.000–49.999, next idx = 43.000 — **tidak disentuh sesi
  ini**), `locale/phase9.jsonl` (9.250 baris, **jalan** — rentang penuh fase ini
  31.817 baris/idx 429.887–461.703, next idx = 439.137).
  **Baris terakhir di file fase AKTIF = idx terakhir yang selesai.**
  Cek dengan: `wc -l locale/phase{N}.jsonl` (N = nomor fase yang mau dilanjutkan — **sekarang
  ada dua fase jalan sekaligus, 5 dan 9, jadi tanya user dulu fase mana kalau tidak
  disebutkan eksplisit**); next idx = idx_awal_fase + jumlah_baris.

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

## Update 2026-09-17: `_diff` yang ter-install tidak bisa dipatch permanen

Ditemukan (lewat investigasi langsung ke folder instalasi game + `LocalData\patch_log\`):
game punya **dua salinan terpisah** `translate_words_map_en_diff` — satu di `Package\HD\oversea\locale\`
(yang selama ini di-patch tools repo ini, stabil/permanen), satu lagi di
`LocalData\Patch\HD\oversea\locale\` yang benar-benar dipakai game untuk layer `_diff`. Salinan
kedua ini **diverifikasi checksum & di-restore otomatis dari CDN NetEase tiap game start**
(`StagePatchList`/`StageCheck`/`StageDownload` di log launcher) — dikonfirmasi langsung: file yang
sudah dipatch balik jadi ukuran byte identik dengan versi asli dalam hitungan menit setelah
dipatch. Detail teknis lengkap & alasan kenapa ini tidak bisa diakali dengan aman ada di
`CLAUDE.md` bagian "Deployment gotcha: the installed `_diff` file gets re-verified by the game's
own CDN patcher".

**Implikasi ke progress terjemahan**: fokuskan ke `translate_words_map_en` (base) + `__small` saja
— dua-duanya permanen begitu di-deploy. Isi `_diff` (~213rb entri, ~26% dari total entri versi
2026-09) akan tetap English di in-game sampai NetEase suatu saat merge `_diff` itu balik ke base
package (siklus update normal mereka), titik di mana bagian yang sudah diterjemahkan otomatis ikut
permanen tanpa kerja tambahan.

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

### Catatan istilah baru dari sesi Fase 5 lanjutan (idx 39000-40999)

- **"Master" sebagai honorifik di depan nama tokoh (bukan pemimpin organisasi)
  dikonfirmasi ulang DITERJEMAHKAN "Guru"** (mis. "Master Frost" -> "Guru Frost",
  "Master Pu" -> "Guru Pu", "Master Qi" -> "Guru Qi") — konsisten dengan aturan lama
  Fase 4 ("Master Wuhen" -> "Guru Wuhen"), diterapkan juga untuk nickname performer/
  tokoh individual, bukan cuma guru ilmu silat formal.
- **"Daoist" (sebagai gelar depan nama, mis. "Daoist Zhang") DITERJEMAHKAN "Daois"**
  (transliterasi umum istilah Tao dalam bahasa Indonesia) — keputusan baru sesi ini,
  konsisten diterapkan ke semua kemunculan (Daoist Zhang -> Daois Zhang, Daoist Xi ->
  Daois Xi, Scabby Daoist -> Daois Berkudis).
- **Tag `<LINK id='...' color='...' goto_id='...' is_underline='true'>teks</LINK>`**
  ditemukan (idx 39080) — regex `TOKEN` mencocokkan tag pembuka `<LINK ...>` sebagai SATU
  token (berhenti di `>` pertama) dan `</LINK>` sebagai token terpisah, jadi teks DI ANTARA
  keduanya (mis. "Echo event") adalah teks bebas yang aman diterjemahkan — sama seperti
  pola tag `<...>` pendek lainnya, bukan pola tag-panjang-satu-token.
- **"Companion" (sistem pendamping non-mount, beda dari Retainer/Kitty/Doggy nickname)
  dibiarkan Inggris** sebagai istilah sistem (mis. "Select Companion" -> "Pilih Companion",
  "Companion Rotation" -> "Rotasi Companion").
- **Kata benda umum untuk hewan/burung yang jarang dipakai (Muntjac, Bustard,
  Blackberry Lily)** — muntjac diterjemahkan "Kijang Merah" (nama umum Indonesia untuk
  kijang), Bustard dibiarkan sebagai nama burung (tidak ada padanan baku umum), Blackberry
  Lily (bunga) dibiarkan Inggris karena tidak ada nama umum Indonesia yang mapan.
- **String Han/Chinese leftover TIDAK ditemukan di batch ini** (beda dari beberapa batch
  Fase 5 sebelumnya) — kemungkinan area distribusi frekuensi ini sudah habis stok leftover-
  nya untuk sementara.
- **"Grandmaster"/"Master Artisan" (gelar tertinggi sekte Mohist Hill, setara istilah
  sistem) dikonfirmasi dibiarkan Inggris**, beda dari "Core Disciple"/"Outer Disciple"
  yang diterjemahkan (Disciple = role generik yang sudah dikunci diterjemahkan).
- Batch ini (2.000 string, idx 39000-40999) dikerjakan sebagai satu sesi, diproses dalam
  4 sub-batch 500 lalu digabung & divalidasi sekaligus sebelum di-append — 0 mismatch
  token pada percobaan pertama di semua 4 sub-batch (tidak ada koreksi diperlukan).

## PENTING: sisa pekerjaan sangat besar

388.887 string unik lagi setelah progress ini. Lihat `plan.md` untuk perkiraan jumlah
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

### Catatan istilah baru dari sesi Fase 5 lanjutan (idx 22000-22999)

- **Label animasi/teknik combat internal berpola `"<Senjata/Style> - <Nama Gerakan>"`**
  (mis. "Spear Red Spirit - Normal Attack Combo 2 - Sweep", "Chicken Simulator - Goose
  Heavy Attack 1 - PvE", "Assist Tang Blade - Heavy Attack Chain Strikes", "Qianye
  Showdown - Backward Dodge", "Gauntlets - Punch Slam", "Silkbind - Deluge: Default")
  **DIBIARKAN UTUH bahasa Inggris** — dikunci sebagai kelanjutan pola nama senjata/tipe
  serangan yang sudah Inggris, ini label dev-facing untuk animasi/hitbox, bukan teks
  naratif untuk pemain. Konsisten dengan "Silkbind - Deluge" yang sudah ada di fase
  sebelumnya. Pengecualian: kalau ada kata kerja instruksi eksplisit di depannya (mis.
  "Use Skill Theft - Thundercry Blade") kata instruksinya (`Use`) tetap diterjemahkan
  (`Gunakan`), tapi nama skill/animasinya sendiri tetap Inggris.
- **"Player"/"player" dikonfirmasi ulang SELALU dibiarkan Inggris** (bukan "Pemain"),
  termasuk saat muncul sebagai hitungan generik ("0 Player") — diverifikasi lewat grep
  ke semua `locale/phase*.jsonl` sebelumnya, seluruh precedent dari fase 0-4 konsisten
  memakai "player"/"Player" apa adanya. Ini mengoverride bacaan literal glosarium yang
  memasukkan "Player" ke daftar peran NPC generik yang diterjemahkan — dalam praktiknya
  istilah ini nyaris selalu dipakai sebagai istilah mekanik/teknis (system term), bukan
  label NPC in-world, jadi konvensi de facto adalah dibiarkan Inggris.
- **"Melodies of Peace" (judul chapter) DITERJEMAHKAN jadi "Melodi Kedamaian"**, termasuk
  dalam bentuk gabungan seperti "Kaifeng - Melodi Kedamaian" — mengikuti precedent
  terbaru di akhir Fase 4 (idx 18999), bukan precedent lama di Fase 3 yang membiarkannya
  Inggris dalam bentuk gabungan (idx 5178, 6352, dst. — sudah terlanjur, dibiarkan apa
  adanya, tidak diretrofit).
- **Durasi literal `Ns` di dalam kalimat naratif biasa (bukan singkatan shorthand)
  TIDAK dikonversi ke "Nd"** kalau kata "seconds"/"detik" ditulis penuh (bukan huruf
  tunggal "s") — konversi `Ns`->`Nd` hanya berlaku untuk shorthand literal pendek
  (mis. "18s", "#Y70s#E"), bukan untuk kalimat yang sudah eksplisit menulis kata
  "seconds" secara penuh (itu diterjemahkan biasa jadi "detik").
- **"Union" (struktur sosial pemain) tetap dibiarkan Inggris** (mis. "Scholars' Union",
  "Total Weekly Union Contribution") — masih konsisten dengan ambiguitas yang dicatat
  di Fase 5 sesi sebelumnya.
- **Nama kartu remi/board-game internal** (Number Card, Wild Card, Big Landlord, Little
  Landlord) dibiarkan Inggris sebagai istilah aturan permainan kartu spesifik.

### Catatan istilah baru dari sesi Fase 5 lanjutan (idx 23000-24999)

- **User menaikkan ukuran batch jadi 2.000 string/sesi** mulai sesi ini (sebelumnya 1.000,
  lihat catatan preferensi di `plan.md`) — dikerjakan sebagai satu batch besar dalam satu
  sesi tanpa masalah validasi, jadi 2.000/sesi jadi default baru sampai ada instruksi lain.
- **"Sword Trial" dikonfirmasi ulang sebagai nama mode dibiarkan Inggris** (dipakai ratusan
  kali di batch ini sebagai bagian dari kalimat "Defeat #YBoss#E in Sword Trial..."),
  konsisten dengan "Hero's Realm" yang sudah dikunci sebelumnya.
- **"Cultivation" (istilah latihan internal wuxia)** dibiarkan Inggris sebagai loanword
  ketika muncul sebagai label sistem (mis. "Cultivation Instructions" -> "Instruksi
  Cultivation") — belum ada padanan baku yang dikunci, mengikuti pola Qi/Energy yang
  dibiarkan Inggris.
- **Label animasi/kombat internal berpola `"<Senjata> - <Aksi>"`** (mis. "Gauntlets - Leg
  Slam", "Assist Dual Blades - Right Dodge", "Spear Red Spirit - Ground Attack") terus
  dikonfirmasi dibiarkan Inggris utuh, sangat banyak muncul di batch ini — pola dev-facing
  yang sudah dikunci sejak Fase 5 sesi sebelumnya (idx 22000-22999).
- **"Player" dikonfirmasi lagi dibiarkan Inggris** di semua konteks (bukan "Pemain"),
  termasuk "The player canceled the invitation." -> "Player membatalkan undangan."
- **Kalimat pembicaraan dengan honorifik "My lord"** (dari NPC ke karakter pemain, konteks
  bukan bangsawan spesifik) diterjemahkan "Tuan"/"Tuanku" konsisten dengan pola
  Lord->Tuan yang sudah dikunci.
- Tidak ada keputusan konvensi besar baru lain di batch ini — mayoritas nama NPC/gear/
  skill/label kombat mengikuti pola yang sudah terkunci di sesi-sesi Fase 4 dan Fase 5
  sebelumnya.

### Catatan istilah baru dari sesi Fase 5 lanjutan (idx 25000-26999)

- **Suffix stat-node baru "X Optimization"** (mis. "Shadow Optimization") **DIBIARKAN UTUH
  bahasa Inggris**, mengikuti pola "X Enhancement"/"X Boost" yang sudah dikunci — kelanjutan
  konvensi label skill-tree/talent tetap Inggris.
- **Tag `#YSpecial Skill "..."#E`** (nama skill spesifik dalam tanda kutip yang dibungkus tag
  highlight) — WAJIB dipertahankan seluruh tag `#Y...#E`-nya persis, jangan cuma menerjemahkan
  kalimat luar dan membuang tag pembungkusnya (sempat kejadian di idx 26388 pada sesi ini,
  sudah diperbaiki sebelum divalidasi — dua kemunculan "Special Skill" dalam satu string yang
  sama harus dicek satu per satu, bukan cuma yang pertama).
- **"Union"/"Player"/"Enhancement"/"Sword Trial"/"Cultivation"/pola label kombat
  `"<Senjata> - <Aksi>"`** dikonfirmasi ulang konsisten dengan keputusan Fase 5 sesi-sesi
  sebelumnya — tidak ada perubahan konvensi baru untuk istilah-istilah ini di batch ini.
- Batch ini dikerjakan sebagai satu sesi 2.000 string (idx 25000-26999), diproses dalam
  4 sub-batch 500 saat penerjemahan lalu digabung & divalidasi sekaligus sebelum di-append.

### Catatan istilah baru dari sesi Fase 5 lanjutan (idx 27000-28999)

- **"Feast Moment" (fitur boat/gathering event) dibiarkan Inggris** sebagai nama fitur,
  konsisten dipakai berkali-kali di batch ini (mis. "Feast Moment remaining time",
  "Extend Feast Moment Duration") — pola sama dengan Painted Boat/Solo Mode/Co-op Mode.
- **"Farmland"/"Farming" (sistem pertanian umum, bukan nama fitur bertitel) DITERJEMAHKAN**
  jadi "Lahan Pertanian"/"Bertani" — kata umum, beda dari Homestead yang tetap Inggris
  sebagai nama fitur/lokasi.
- **Tag `<...>` pendek yang membungkus deskripsi suara/sound-effect** (mis. idx 27138
  `<sound of rock collapsing>`) **dibiarkan 100% Inggris**, mengikuti aturan tag-satu-token
  yang sudah dikunci — sempat salah diterjemahkan isinya saat draf awal, diperbaiki sebelum
  validasi final.
- **Tag `#Y...#E` yang membungkus dua frasa terpisah dalam satu string panjang** (mis. idx
  27130, ada `#YSpecial Skill "..."#E` DAN `#Yhigh Bleed damage#E` di paragraf yang sama)
  — WAJIB dicek satu-satu, jangan cuma yang pertama; sempat ada tag kedua yang terlepas
  saat translate, sama seperti kejadian idx 26388 di sesi sebelumnya. Pola berulang ini
  jadi pengingat: paragraf panjang dengan >1 tag highlight butuh pengecekan tag per tag,
  bukan sekali baca sekilas.
- **"Union"/"Player"/"Enhancement"/"Sword Trial"/"Cultivation"/"Retainer"/pola label
  kombat `"<Senjata> - <Aksi>"`/nama gear Swallowcall dst.** dikonfirmasi ulang konsisten
  dengan keputusan Fase 5 sesi-sesi sebelumnya — tidak ada perubahan konvensi baru untuk
  istilah-istilah ini di batch ini.
- Batch ini (2.000 string, idx 27000-28999) dikerjakan sebagai satu sesi, diproses dalam
  4 sub-batch 500 saat penerjemahan lalu digabung & divalidasi sekaligus sebelum di-append.

### Catatan istilah baru dari sesi Fase 5 lanjutan (idx 29000-30999)

- **Placeholder durasi majemuk `%dm%ds` (menit+detik)** -> `%dm%dd` — memperluas konvensi
  konversi literal detik `s`->`d` (dari Fase 4/Fase 5 sebelumnya) ke format gabungan
  menit+detik; huruf `m` (menit) dibiarkan apa adanya karena kebetulan sama di kedua
  bahasa, hanya `s`->`d` yang diubah. Pola sama juga dipakai untuk `%sh`->`%sj` (jam).
- **Tag `<...>` yang membungkus SATU PARAGRAF PANJANG dengan tag warna di dalamnya**
  (mis. idx 30170, surat ucapan Tahun Baru yang seluruh isinya — termasuk
  `#8c5823...#E` — dibungkus satu tag `<...>` dari awal sampai akhir) — regex `TOKEN`
  menelan seluruh blok itu jadi SATU token, jadi isinya WAJIB 100% identik Inggris;
  sempat diterjemahkan penuh saat draf awal sebelum disadari, diperbaiki sebelum
  validasi final. Konsisten dengan aturan tag-panjang-satu-token yang sudah dikunci
  sejak idx 12448 (Fase 4) — pengingat: kalau sebuah tag `<...>` melingkupi lebih dari
  satu kalimat, JANGAN sentuh isinya sama sekali, apa pun tag warna di dalamnya.
- **String panjang berisi banyak placeholder `@T[...]`** (mis. idx 30393, deskripsi
  Guild War League dengan banyak tanggal `@T[month_2, day_6,type_noLocal;empty]`) —
  `@T[...]` pakai kurung siku, TIDAK tertangkap regex `TOKEN` (yang cuma cek `#`, `%s`,
  `%d`, `{}`, `<>`), tapi tetap WAJIB dipertahankan persis karakter demi karakter karena
  ini variabel substitusi tanggal runtime — sama seperti aturan placeholder non-standar
  `$VAR$`/`$P`/`$N` yang sudah dikunci sebelumnya.
- **"Union"/"Player"/"Enhancement"/"Sword Trial"/"Cultivation"/"Retainer"/pola label
  kombat `"<Senjata> - <Aksi>"`/nama gear Swallowcall/Veilbright/Nightstar dst.**
  dikonfirmasi ulang konsisten dengan keputusan Fase 5 sesi-sesi sebelumnya.
- Batch ini (2.000 string, idx 29000-30999) dikerjakan sebagai satu sesi, diproses dalam
  4 sub-batch 500 saat penerjemahan lalu digabung & divalidasi sekaligus sebelum
  di-append — 1 mismatch token ketemu & diperbaiki (lihat catatan `<...>` panjang di atas).

### Catatan istilah baru dari sesi Fase 5 lanjutan (idx 31000-32999)

- **"Wanderer" dikonfirmasi ulang konsisten diterjemahkan "Pengembara"** (mis. "Young
  Wanderer" -> "Pengembara Muda"), sesuai aturan lama di `GLOSSARY.md` — sempat banyak
  muncul di batch ini dan diterapkan konsisten (beda dari "Player" yang selalu dibiarkan
  Inggris).
- **Placeholder durasi shorthand `Ns`/`Nh` di dalam kalimat/tag terus dikonversi**
  `s`->`d` (detik) dan `h`->`j` (jam) sesuai konvensi lama, termasuk saat digabung dengan
  literal menit `m` (mis. "7h57m Remaining" -> "7j57m Tersisa", "%sm%ss" -> "%sm%sd").
- **Placeholder tanggal/waktu runtime `@t[...]` (huruf kecil, kurung siku)** — varian baru
  dari `@T[...]` yang sudah dikunci di Fase 5 sesi sebelumnya (idx 29000-30999) — WAJIB
  dipertahankan persis sama, tidak tertangkap regex `TOKEN`.
- **Nama fitur/mode/sistem baru yang dibiarkan Inggris**: Feast Moment, Cultivation,
  Sword Trial, Union, Path (skill tree path), Plan (build/loadout plan — diterjemahkan
  jadi "Rencana" karena kata umum, bukan nama fitur bertitel), label animasi kombat
  `"<Senjata> - <Aksi>"`, nama gear/kostum panjang (Swallowcall, Starweave, dll.) — semua
  konsisten dengan keputusan sesi-sesi Fase 5 sebelumnya.
- **"Loot" dikonfirmasi "Jarahan"/"Jarah"** (kata benda/kerja) sesuai kunci Fase 5 idx
  20000-20999, dipakai di "Loot & Extract" -> "Jarah & Ekstraksi".
- **"Sect Shop" -> "Toko Sekte"** (Sect diterjemahkan konsisten dengan aturan wuxia lama).
- Batch ini (2.000 string, idx 31000-32999) dikerjakan sebagai satu sesi, diproses dalam
  4 sub-batch 500 saat penerjemahan lalu digabung & divalidasi sekaligus sebelum
  di-append — 0 mismatch token pada percobaan pertama.

### Catatan istilah baru dari sesi Fase 5 lanjutan (idx 33000-34999)

- **Tag polos `<...>` tanpa format `|id|#C|n>` yang membungkus deskripsi aksi/suara non-dialog
  singkat** (mis. idx 34071 `<Nods.>`, idx 34671 `<ears drooping> Mmnh...`) **sempat 2x
  diterjemahkan sebelum divalidasi** — pengingat tegas: aturan tag-polos-dibiarkan-Inggris
  (dikunci sejak Fase 3, idx 6041) berlaku juga untuk aksi/deskripsi pendek non-dialog, bukan
  cuma placeholder nama. Regex `TOKEN` menelan seluruh `<...>` sebagai satu token, jadi isinya
  WAJIB 100% identik sumber kalau tidak berformat `|id|#C|n>`.
- **"Steward" -> "Pelayan"** (kepala pelayan/pengurus rumah tangga), **"Squire" -> "Pengawal"**
  (pengawal muda ksatria) — keduanya gelar deskriptif umum, konsisten dengan pola role NPC
  generik yang diterjemahkan.
- **"Passerby" (role NPC generik) DITERJEMAHKAN jadi "Orang Lewat"** — sebelumnya sempat
  muncul tanpa keputusan eksplisit; dikunci sekarang mengikuti daftar role generik
  (Villager/Bandit/dst.) di `GLOSSARY.md`.
- **"Cold Loading" (istilah teknis dev/loading asset) dibiarkan Inggris** — bukan istilah
  gameplay, kemungkinan label debug/internal.
- **Durasi literal `/h` (per jam) di luar tag warna dikonversi ke `/j`** (mis. `0#85d67c +
  0#E/h` -> `0#85d67c + 0#E/j`) — konsisten dengan konvensi `h`->`j` yang sudah dikunci,
  berlaku juga saat literalnya nempel langsung setelah token warna/tag.
- **"Wanderer"/"Union"/"Retainer"/"Cultivation"/"Sword Trial"/pola label kombat
  `"<Senjata> - <Aksi>"`/nama gear panjang** dikonfirmasi ulang konsisten dengan keputusan
  Fase 5 sesi-sesi sebelumnya — tidak ada perubahan konvensi besar untuk istilah-istilah ini.
- Batch ini (2.000 string, idx 33000-34999) dikerjakan sebagai satu sesi, diproses dalam
  4 sub-batch 500 lalu digabung & divalidasi sekaligus sebelum di-append — 2 mismatch tag
  polos ketemu & diperbaiki (lihat poin pertama di atas).

### Catatan istilah baru dari sesi Fase 5 lanjutan (idx 35000-36999)

- **Nickname companion berpola "Kitty: X" / "Doggy: X" DIBIARKAN UTUH bahasa Inggris**
  (mis. "Kitty: Youngest", "Kitty: Ebony", "Doggy: Deng Deng", "Doggy: Tipsy") — keputusan
  baru sesi ini: diperlakukan sebagai nickname/nama companion (seperti nama karakter),
  BUKAN diterjemahkan sebagai kata sifat/benda umum, meski beberapa suffix-nya berupa kata
  Inggris biasa (Youngest, Ball, Drifter, dst.). Konsisten diterapkan ke puluhan kemunculan
  di batch ini — kalau nanti ada precedent lama yang bertentangan (belum ditemukan saat
  audit sesi ini), review ulang.
- **String Han leftover ketiga**: idx 36975 `"倒计时测试徽章"` (label internal developer,
  "Countdown Test Badge") diterjemahkan strukturnya jadi "Lencana Uji Coba Hitung Mundur" —
  konsisten dengan pola idx 14850/19719 (Han leftover = label dev, diterjemahkan penuh
  karena tidak ada nama yang perlu ditransliterasi).
- **"Union"/"Player"/"Enhancement"/"Sword Trial"/"Cultivation"/"Retainer"/"Wanderer"/pola
  label kombat `"<Senjata> - <Aksi>"`/nama gear panjang** dikonfirmasi ulang konsisten
  dengan keputusan Fase 5 sesi-sesi sebelumnya — tidak ada perubahan konvensi besar untuk
  istilah-istilah ini di batch ini.
- Batch ini (2.000 string, idx 35000-36999) dikerjakan sebagai satu sesi, diproses dalam
  4 sub-batch 500 lalu digabung & divalidasi sekaligus sebelum di-append — 0 mismatch token
  pada percobaan pertama (setelah koreksi 1 idx yang sempat terlewat dibaca dari sumber,
  idx 36999, ditangkap saat validasi count sebelum append).

### Catatan istilah baru dari sesi Fase 5 lanjutan (idx 37000-38999)

- **Building/furniture component internal Homestead** (mis. "Red Roof Surface", "Red Roof
  Outer Corner", "Rainbow Resort Partition/Gable Wall/Beam/Double Door", "Greenwood Mansion
  Partition/Bracket/Railing Pillar Head/Arch Bridge/Indoor Stairs", "Bamboo Leaf Screen
  5/6", "Diamond-Patterned Screen") **dikonfirmasi ulang DIBIARKAN UTUH bahasa Inggris** —
  diperlakukan sebagai label internal identifier komponen bangunan/furnitur, konsisten
  dengan precedent lama ("Roof Outer Corner" idx 32086, "Greenwood Mansion Long Railing I"
  idx 36373), bukan diterjemahkan sebagai deskripsi umum.
- **`"Trial: <Nama>"` (mis. "Trial: Gatekeeper's Stand", "Trial: Unbounded World", "Trial:
  Malefic Stars", "Trial: Anchor the Ark") dikonfirmasi ulang DIBIARKAN UTUH bahasa
  Inggris** (termasuk prefix "Trial:"-nya, TIDAK diterjemahkan jadi "Uji Coba:") — mengikuti
  precedent TERBARU (idx 35748 "Trial: Fleeting Trace", idx 36072 "Trial: Master of
  Spectacles", idx 36739 "Trial: Blades in Question", semua tepat sebelum batch ini),
  bukan precedent lebih lama yang sempat menerjemahkan jadi "Uji Coba: ..." — pola nama
  dungeon/trial berjudul dianggap proper noun sejak idx ~35000-an.
- **String Han leftover keempat**: idx 37377 `"地图用-万古一人殿4层"` (label internal peta,
  identik strukturnya dengan idx 14850 tapi lantai 4, bukan 2) diterjemahkan jadi "Untuk
  Peta - Wangu Yiren Hall Lantai 4" — nama aula ditransliterasi apa adanya, konsisten
  dengan pola idx 14850/19719/36975 (Han leftover = label dev, diterjemahkan penuh kecuali
  nama). Sempat terlewat (dibiarkan Han) di draf awal, ditangkap & diperbaiki lewat audit
  manual sebelum di-append (bukan lewat validasi token, karena string CJK tanpa tag tidak
  memicu mismatch pada regex `TOKEN`).
- **"Union"/"Player"/"Enhancement"/"Sword Trial"/"Cultivation"/"Retainer"/"Wanderer"/"Farmer"
  (belum muncul batch ini)/pola label kombat `"<Senjata> - <Aksi>"`/nama gear panjang**
  dikonfirmasi ulang konsisten dengan keputusan Fase 5 sesi-sesi sebelumnya — tidak ada
  perubahan konvensi besar lain untuk istilah-istilah ini di batch ini.
- Batch ini (2.000 string, idx 37000-38999) dikerjakan sebagai satu sesi, diproses dalam
  4 sub-batch 500 lalu digabung & divalidasi sekaligus sebelum di-append — 0 mismatch token
  pada validasi pertama (skrip token); 1 koreksi non-token (Han leftover di atas) ditemukan
  lewat audit manual terpisah.

### Catatan istilah baru dari sesi Fase 9 (idx 429887-431886, batch pertama)

Fase 9 adalah string baru dari update game 2026-09-16 (bukan lanjutan `unique_strings.jsonl`
lama), banyak berisi teks dari chapter/area baru (Hidden Mountain/Mohist Hill lanjutan, Sky
Citadel, Luan City, Qiongqi Artificer, dll.) dan compound gear-name baru. Semua konvensi
GLOSSARY.md lama tetap berlaku; tambahan/klarifikasi baru sesi ini:

- **"Farmer" (unit sistem Homestead yang bisa direkrut/dikerahkan) DIBIARKAN UTUH bahasa
  Inggris** (kapital), mis. "Farmer Management", "Insufficient Farmers", "Ambil foto bersama
  Farmer yang sedang bekerja" — diperlakukan sebagai istilah sistem Homestead seperti
  Retainer/Homestead, BUKAN diterjemahkan sebagai kata umum "petani". Sangat sering muncul
  di batch ini (rekrutmen, dispatch, cohabitation, dll).
- **"Qiongqi Master"/"Qiongqi Warrior"/"Qiongqi Soldier"/"Qiongqi Artificer"** (rank/role
  anggota faksi antagonis "Qiongqi") **DIBIARKAN UTUH bahasa Inggris** sebagai compound
  title/proper-noun, BUKAN diterjemahkan sebagian ("Qiongqi" faksi + rank kata umum) —
  beda dari pola role NPC generik lama (Villager/Bandit) karena rank ini terasa seperti
  label internal/rank musuh spesifik-game, bukan sebutan sosial umum.
- **Royal/imperial title generik (Prince, Empress) DITERJEMAHKAN** konsisten dengan pola
  Lord->Tuan/Young Master->Tuan Muda yang sudah dikunci: "Prince Teng" -> "Pangeran Teng",
  "Empress Wu" -> "Permaisuri Wu".
- **"Master" di depan nama sebagai gelar guru/pemimpin (bukan rank Qiongqi)
  DITERJEMAHKAN "Guru"** konsisten dengan aturan lama Fase 4: "Master Jin" -> "Guru Jin"
  (Jin Zhongyuan, tokoh utama chapter ini, dipanggil begitu berkali-kali), "Academy Master
  Crane" -> "Ketua Akademi Crane" (Master = pemimpin institusi -> Ketua).
- **Nama festival "Lantern Festival" DITERJEMAHKAN "Festival Lampion"** konsisten dengan
  pola nama festival budaya lain yang sudah dikunci (Spring Festival -> Festival Musim Semi).
- **"Treasury" (kata umum "perbendaharaan") DITERJEMAHKAN konsisten** di semua compound-nya:
  "Pledged Treasury" -> "Perbendaharaan Terikat", "Sealed Treasury" -> "Perbendaharaan
  Tersegel", "Imperial Treasury Halls" -> "Aula Perbendaharaan Kerajaan" — beda dari nama
  lokasi majemuk yang dikunci utuh Inggris, karena "Treasury" di sini konsisten dipakai
  sebagai kata umum deskriptif, bukan bagian nama tempat unik.
- **Hewan/makhluk umum non-fantasi (Pangolin) DITERJEMAHKAN** konsisten dengan aturan lama:
  "Pangolin's Trade Tales" -> "Kisah Dagang Trenggiling".
- **Literal `\n` dua-karakter (backslash+n, BUKAN newline asli)** ditemukan di beberapa
  string (idx 430082, 430083, 430581, 430615, 430622, 430699, 431245, 431278, 431731,
  431860, 431862) — WAJIB dipertahankan sebagai literal 2-karakter persis, JANGAN diubah
  jadi newline asli maupun dihapus. Bedakan dari string yang benar-benar punya newline asli
  (mayoritas narasi panjang) — cek dengan `repr()` di Python kalau ragu, newline asli tampil
  sebagai `\n` tunggal di `repr()`, literal dua-karakter tampil sebagai `\\n`.
- **Tag warna `#Y...#E` vs `#G...#E` HARUS dicek presisi tiap kemunculan** — sempat ada 1
  kasus (idx 431459) salah pilih tag warna (`#G` padahal sumbernya `#Y`) yang lolos draf
  awal, ketangkap saat validasi token karena jumlah token per warna beda. Selalu jalankan
  validasi token count-per-jenis (bukan cuma total token), terutama untuk paragraf dengan
  banyak tag warna berbeda dalam satu string.
- **Tag `#994242(...)` tanpa penutup `#E`** (idx 431600, label UI Auto-Management) — sempat
  salah ditambahkan `#E` penutup yang tidak ada di sumber, ketangkap validasi. Selalu ikuti
  jumlah token PERSIS seperti sumber, jangan "membetulkan" tag yang terlihat tidak simetris
  di mata manusia — itu memang polanya di source.
- Batch ini (2.000 string, idx 429887-431886) dikerjakan sebagai satu sesi, diproses dalam
  4 sub-batch 500 lalu digabung & divalidasi sekaligus sebelum di-append — total 2 mismatch
  token ditemukan & diperbaiki (lihat dua poin di atas), keduanya di sub-batch keempat.

### Catatan istilah baru dari sesi Fase 9 lanjutan (idx 431887-433886, batch kedua)

- **Material/resource crafting Homestead berpola `"<Nama> Tier N"`** (mis. "Magnet Stone
  Tier I", "Cloud Sand Tier II", "Turquoise Stone Tier I", "Iron Ore Tier II", "Cinnabar
  Tier I", dan puluhan nama material majemuk lain seperti "Mushroom-Iron Composite",
  "Pine-Copper Cloudsand Extract") **DIBIARKAN UTUH bahasa Inggris** — keputusan baru sesi
  ini: diperlakukan sebagai nama sistem resource/crafting seperti Inspiration/Affection
  yang sudah dikunci di Fase 2, BUKAN kata benda umum yang diterjemahkan (beda dari aturan
  lama "hewan/tumbuhan bernama umum diterjemahkan" karena di sini nama material sudah jadi
  identifier sistem crafting spesifik, bukan makhluk hidup naratif). Konsisten diterapkan
  ke sangat banyak kemunculan di batch ini (puluhan-ratusan item Tier I/II).
- **PENTING — pola `"#Y<label>#E <Tag|id|#C|slot>"` (teks highlight pendek diikuti tag
  placeholder terpisah, mis. sumber "a #Y3rd-Stage#E <Heavy Attack Charged
  Skill|781|#C|20401|20401103>")**: WAJIB dipertahankan sebagai DUA unit terpisah — `#Y...#E`
  membungkus teks yang diterjemahkan (jadi `#YTahap ke-3#E`), lalu tag `<...>` menyusul utuh
  tak tersentuh. JANGAN gabungkan teks highlight ke DALAM tag placeholder (mis. jangan
  ditulis `#YHeavy Attack Charged Skill|781|#C|20401|20401103 Tahap ke-3#E` — ini memecah
  tag `<...>` jadi terbuka tanpa `>` penutup dan bikin `#C` di dalamnya lolos jadi token
  liar). Kesalahan ini terjadi 2x di batch ini (idx 432654, 433428) sebelum ketangkap
  validasi — pola sumber sama persis, jadi begitu ketemu sekali harus diingat buat sisa
  batch.
- **Paragraf dua-bagian dengan pola `"...deal damage...\nApplicable #YMartial Trigger
  Effects#E: ..."`** — kalimat pembuka DAN baris "Applicable ...:" di paragraf kedua
  SAMA-SAMA punya tag `#Y...#E` yang membungkus istilah yang sama (mis. "Martial Trigger
  Effects"), keduanya harus diterjemahkan dan dipertahankan tag-nya secara konsisten.
  Sempat ada 1 kasus (idx 433342) di mana baris kedua kehilangan tag `#Y...#E`-nya saat
  draf awal (diterjemahkan sebagai teks polos "Martial Trigger Effect yang berlaku:"),
  ketangkap validasi.
- **Span `#Y...#E` panjang yang membungkus DUA frasa berurutan lewat satu pasang tag**
  (mis. sumber idx 432181 `"#Y{} Personal Component(s), and {} Diagram(s)#E"` — satu `#Y`
  di depan, satu `#E` di BELAKANG kedua frasa, bukan tiap frasa dapat tag sendiri) — WAJIB
  dijaga sebagai satu span utuh saat diterjemahkan, jangan menutup `#E` di tengah lalu buka
  lagi tanpa `#Y` baru (itu nambah token `#E` ekstra yang nggak ada pasangannya di sumber).
  Ketangkap di idx 432181 sebelum di-append.
- **Qiongqi Master/Farmer/Cultivation/Union/Sword Trial/Wanderer/pola label kombat
  `"<Senjata> - <Aksi>"`/nama gear panjang (Swallowcall/Swallow's Return/Frostbane/
  Nightfarer/dst.)** dikonfirmasi ulang konsisten dengan keputusan Fase 9 batch pertama &
  Fase 5 sesi-sesi sebelumnya — tidak ada perubahan konvensi besar untuk istilah-istilah
  ini di batch ini.
- Batch ini (2.000 string, idx 431887-433886) dikerjakan sebagai satu sesi, diproses dalam
  8 sub-batch 250 lalu digabung & divalidasi sekaligus sebelum di-append — total 4 mismatch
  token ditemukan & diperbaiki (lihat tiga poin di atas), semuanya ketangkap di validasi
  pertama sebelum append (bukan lewat sub-batch bertahap seperti sesi sebelumnya, karena
  batch ini digabung dan divalidasi sekali di akhir, bukan per sub-batch).

### Catatan istilah baru dari sesi Fase 9 lanjutan (idx 433887-435886, batch ketiga)

- **Material/resource crafting Homestead berpola `"<Nama> Tier N"` dikonfirmasi ulang**
  (mis. "Cinnabar Tier II") tetap dibiarkan Inggris, konsisten dengan keputusan batch
  kedua.
- **Nama fitur/industri baru "Cloudfore" (varian Beyond Mundane) DIBIARKAN UTUH bahasa
  Inggris** sebagai nama sistem/fitur, pola sama dengan Arcadian Homestead/Beyond
  Mundane yang sudah dikunci.
- **"Fengputer" (perangkat/oracle mekanis yang menjawab pertanyaan pemain, muncul
  berkali-kali di batch ini) DIBIARKAN UTUH bahasa Inggris** — nama alat/mekanisme
  spesifik-game, bukan kata umum.
- **Nama makanan Tiongkok umum (Hulatang, Tofu Pudding, Pan-Fried Buns, dst.) dalam
  teks lore kuliner panjang DIBIARKAN UTUH bahasa Inggris/pinyin** — loanword kuliner,
  konsisten dengan pola nama makanan lain yang sudah dikunci sebelumnya (mis. He'le
  noodles).
- **"Steward" (peran generik huruf kecil) DITERJEMAHKAN "Pelayan"** — dikunci ulang
  dari GLOSSARY.md lama; sempat lolos dibiarkan Inggris 2x di draf awal batch ini,
  diperbaiki setelah audit manual pasca-validasi token (validasi token TIDAK
  menangkap kasus ini karena "steward" tanpa tag bukan token yang dicek regex).
  **PENTING**: "Steward" berhuruf besar sebagai bagian judul/label kapital (mis.
  "Ritual Steward", `#YSteward#E`) tetap dibiarkan Inggris — beda konteks, berfungsi
  sebagai judul bukan sebutan generik.
- **PENTING — string yang diawali `#` tapi BUKAN tag highlight sungguhan** (ditemukan
  1 kasus baru: idx 434808 `"#Talk to the Guard"` — nama label quest yang kebetulan
  diawali `#`, bukan format token `#Y...#E`): regex `TOKEN` di `qa_check.py` tetap
  mencocokkan `#T` (huruf pertama setelah `#`) sebagai token satu-huruf, JADI kalau
  diterjemahkan biasa (mis. jadi "#Bicara..."), validasi MISMATCH karena huruf
  pertama berubah. Solusi: biarkan `#` + kata pertama tetap Inggris apa adanya kalau
  itu bagian dari string yang jelas bukan pola highlight (`#Talk` dibiarkan, sisanya
  diterjemahkan jadi "#Talk dengan Penjaga") — TIDAK sama dengan menerjemahkan
  seluruhnya lalu berharap huruf pertama kebetulan sama.
- **Paragraf panjang dengan >2 kemunculan tag `#Y...#E` yang membungkus istilah yang
  SAMA berulang kali** (mis. idx 435165, "Martial Art Triggered Effects" muncul 3x
  dalam satu string, masing-masing dibungkus `#Y...#E` terpisah) — sempat ada 1
  kemunculan ketiga (di awal paragraf kedua, "Applicable #Y...#E include:") yang
  lolos tanpa tag saat draf awal. Pengingat: kalau istilah yang sama berulang lebih
  dari 2x dalam satu string, cek SEMUA kemunculan satu per satu, jangan asumsikan
  cuma ada 2 seperti pola umum sebelumnya.
- **Tag `<...>` panjang satu-token (aturan lama sejak idx 12448) masih sering
  kelewatan diterjemahkan** — 3 kasus lagi di batch ini (idx 434451 ucapan anjing
  non-manusia, idx 435245 & 435796 tag placeholder/dialog dalam tanda kutip)
  sebelum ketangkap validasi token. Pola berulang ini menegaskan: SETIAP kali ada
  tag `<...>` baru, cek dulu apakah formatnya `<Label|id|#C|n>` (nama stat, aman
  diterjemahkan sebagian) atau bukan (harus 100% identik Inggris) SEBELUM
  menerjemahkan, bukan sesudahnya.
- **Farmer/Qiongqi Master/Cultivation/Union/Sword Trial/Wanderer/pola label kombat
  `"<Senjata> - <Aksi>"`/nama gear panjang** dikonfirmasi ulang konsisten dengan
  keputusan Fase 9 batch pertama & kedua serta Fase 5 sesi-sesi sebelumnya.
- Batch ini (2.000 string, idx 433887-435886) dikerjakan sebagai satu sesi, diproses
  dalam 8 sub-batch 250 lalu digabung & divalidasi sekaligus di akhir — 5 mismatch
  token ditemukan & diperbaiki (lihat poin-poin di atas) plus 2 inkonsistensi
  "steward" ditemukan lewat audit manual terpisah dari validasi token otomatis.

### Catatan istilah baru dari sesi Fase 5 lanjutan (idx 41000-42999, batch keenam)

- **"Refine"/"Draft"/"Interface"/"Melee"/"AoE"/"Pet"/"Dodge"/"Attuning"/"Cultivator"**
  dan istilah sistem/UI generik sejenis **dibiarkan Inggris** sebagai loanword gaming
  umum, konsisten dengan Guild/Event/Login/Menu/Skill/Item/Quest yang sudah dikunci.
- **"Bustard"/"Black Brant"/"Brant"** (nama burung langka tanpa padanan baku Indonesia)
  tetap dibiarkan Inggris, konsisten dengan keputusan Fase 5 sebelumnya. **"Jackal"**
  DITERJEMAHKAN jadi "Jakal" (loanword umum, beda dari nama burung yang tak ada
  padanannya sama sekali).
- **Label lokasi/dekor berpola `"<Nama Set/Lokasi> <Deskripsi Komponen>"`** (mis.
  "Rainbow Resort Small Roof Inner Corner", "Greenwood Mansion Board Wall", "Chai
  Mansion - Secret Observation") — nama set/lokasi (Rainbow Resort, Greenwood Mansion,
  dst.) dibiarkan Inggris seperti nama lokasi lain, tapi deskripsi komponennya
  (Small Roof Inner Corner, Board Wall, dst.) DITERJEMAHKAN sebagai kata benda umum
  — beda dari label kombat `"<Senjata> - <Aksi>"` yang seluruhnya dibiarkan Inggris.
- **"Hongbao" (transliterasi Tionghoa untuk angpao) DITERJEMAHKAN jadi "Angpao"**
  konsisten dengan "Red Packets"/"Red Envelope" -> "Angpao" yang sudah dikunci Fase 4.
- **String Han leftover ditemukan lagi**: idx 42741 `"待确认文本"` (label dev internal
  berarti "teks yang perlu dikonfirmasi") — diterjemahkan strukturnya ke Indonesia
  sepenuhnya, sama seperti pola string Han leftover sebelumnya (label internal, bukan
  nama yang perlu ditransliterasi).
- **"Union"/"Player"/"Cultivation"/"Sword Trial"/"Retainer"/"Enhancement"/"Dispatch"/
  pola label kombat `"<Senjata> - <Aksi>"`/nama gear panjang (Swallowcall, Etherwrath,
  Hawkwing, Starweave, dst.)** dikonfirmasi ulang konsisten dengan keputusan Fase 5
  sesi-sesi sebelumnya — tidak ada perubahan konvensi besar baru untuk istilah-istilah
  ini di batch ini.
- Batch ini (2.000 string, idx 41000-42999) dikerjakan sebagai satu sesi, diproses
  dalam 4 sub-batch 500 lalu digabung & divalidasi sekaligus sebelum di-append — 0
  mismatch token pada percobaan pertama di semua 4 sub-batch (tidak ada koreksi
  diperlukan).

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
