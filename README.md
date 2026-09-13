# Where Winds Meet — Indonesian Patch

Toolkit untuk membongkar, menyunting, dan mengemas ulang file lokalisasi
**Where Winds Meet** (NetEase / Everstone Studio, Messiah Engine).

Repo ini berisi **tool dan dokumentasi format**, bukan pack terjemahan jadi.
Baca [Legal & risiko](#legal--risiko) sebelum memakainya.

```
translate_words_map_en  ──dump──>  strings.jsonl  ──edit──>  strings.jsonl
                                                                   │
translate_words_map_en  <──patch───────────────────────────────────┘
```

---

## Status

| | |
|---|---|
| Format terbongkar | ✅ penuh, lihat [`docs/FORMAT.md`](docs/FORMAT.md) |
| Round-trip terverifikasi | ✅ 963.050 entri, 100% identik |
| Decode | ✅ |
| Encode | ✅ |
| Ubah nilai key yang sudah ada | ✅ |
| Tambah key baru | ❌ butuh fungsi hash, belum dipecahkan |

Diuji pada `translate_words_map_en` versi global (963.050 entri, 3.763 blok).

---

## Instalasi

```bash
git clone https://github.com/<user>/where-winds-meet-indonesian-patch.git
cd where-winds-meet-indonesian-patch
pip install -r requirements.txt      # hanya butuh: zstandard
```

Python 3.8+.

---

## Cara pakai

### 1. Cari file-nya

Steam:

```
<Steam>\steamapps\common\Where Winds Meet\Package\HD\oversea\locale\translate_words_map_en
```

Launcher resmi NetEase: struktur `Package\HD\oversea\locale` yang sama, relatif
terhadap folder instalasi.

> **Backup dulu.** Salin file aslinya ke tempat aman sebelum apa pun.
> Kalau kamu pernah memakai GearUP, backup punya mereka ada di folder yang sama
> dengan ekstensi `.gubackup` — itu berisi teks Inggris asli dan sangat berharga.

### 2. Cek isi file

```bash
python wwm_locmap.py info translate_words_map_en
```

```
version        : 1
blocks         : 3763 (1 index + 3762 shard)
entries        : 963050
entries parsed : 963050
```

Kalau `entries parsed` sama dengan `entries`, parser cocok dengan file kamu.

### 3. Dump ke JSONL

```bash
python wwm_locmap.py dump translate_words_map_en strings.jsonl
```

Menghasilkan satu baris JSON per entri (~107 MB untuk 963k entri):

```json
{"b": 1, "s": 3, "h": "c5cadbb857eee8b4", "v": "The leaf?"}
{"b": 1, "s": 4, "h": "b6b9b1230c20a990", "v": "Velvet Shade Guest"}
```

| field | arti |
|---|---|
| `b` | indeks blok/shard |
| `s` | indeks slot di dalam shard |
| `h` | `keyHash` 64-bit (hex) — hanya untuk referensi, **jangan diubah** |
| `v` | nilai string yang ditampilkan game |

### 4. Terjemahkan

Edit field `v`. Lihat [Panduan menerjemahkan](#panduan-menerjemahkan) di bawah —
ada beberapa jebakan yang akan merusak UI kalau diabaikan.

Pasangan `b` + `s` adalah alamat entri. Baris yang tidak kamu ubah boleh
dihapus dari file: `patch` hanya menimpa entri yang ada di JSONL, sisanya
diambil dari file sumber. Jadi patch parsial itu didukung dan jauh lebih ringan.

### 5. Validasi (jangan dilewati)

```bash
python tools/qa_check.py strings_original.jsonl strings_translated.jsonl --report qa.jsonl
```

```
diperiksa : 963050 entri
PROMPT_LEAK :   1817  (0.189%)
MARKUP      :    693  (0.072%)
EMPTY       :      0  (0.000%)
```

Exit code 1 kalau ada temuan, jadi bisa langsung dipasang di CI atau
pre-commit hook.

### 6. Repack

```bash
python wwm_locmap.py patch translate_words_map_en strings.jsonl translate_words_map_en.new
```

Opsi `--level N` mengatur level kompresi zstd (default 19). Level 10–12 cukup
untuk iterasi cepat; pakai 19 untuk rilis.

Salin hasilnya ke folder locale dengan nama `translate_words_map_en`, lalu
jalankan game dengan **Settings → Language → Game Language = English**.

---

## Cara kerjanya

### Kenapa harus diset ke English

File ini **tidak menyimpan teks sumbernya**. Isinya cuma
`u64 keyHash → string terjemahan`.

Runtime mengambil string Inggris dari asset resmi, menghitung hash-nya, lalu
menukar hasil lookup dengan nilai dari file ini. Set bahasa ke Jerman → hash
berbeda → semua lookup miss → teks kembali ke Jerman.

Jadi syarat "Game Language = English" yang muncul di semua translation pack itu
konsekuensi arsitektur file, bukan pilihan desain tool-nya.

### Kenapa kita tidak perlu tahu fungsi hash-nya

Karena key tidak disimpan, mengganti bahasa = mengganti **nilai** dari key yang
sudah ada. Struktur pencarian (`ctrl`, `keyHash`, posisi slot) tidak perlu
disentuh sama sekali:

```
salin  header + ctrl + sentinel + clone + padding + slot array   ← apa adanya
tulis  string blob baru
update relOffset + byteLen di tiap slot
```

Lookup tetap jalan sempurna. Inilah alasan pembuatan pack bahasa jauh lebih
sederhana dari yang biasanya diasumsikan orang — dan, berdasarkan verifikasi
terhadap pack komersial, ini persis pendekatan yang dipakai vendor.

Yang **tidak** bisa dilakukan tanpa memecahkan hash: menambahkan key yang
benar-benar baru ke dalam map.

### Ringkasan format

```
magic 0xDEADBEEF | version 1 | blockCount | reserved
u32[blockCount] endOffset          ← relatif ke akhir tabel ini

per blok: u8 codec=4 (zstd) | u32 compressedSize | u32 uncompressedSize | payload

blok 0     = index  : u64 totalEntries, u64 dataBlockCount, u32[] ids
blok 1..n  = shard  : SwissTable (absl::flat_hash_map)
    u64 capacity | u64 size | u64 seed
    u8[capacity] ctrl        0x80 = kosong, <0x80 = terisi (H2)
    u8 0xFF sentinel + u8[15] cloned ctrl + padding align-8
    slot[capacity] { u64 keyHash; u32 relOffset; u32 byteLen }
    <string blob UTF-8>

shardIndex = (keyHash % dataBlockCount) + 1
```

**Jebakan utama — `relOffset` adalah relative pointer,** dihitung dari alamat
field `relOffset` itu sendiri, bukan dari awal blok atau awal string blob:

```
field      = slotArrayStart + i * 16 + 8
valueStart = field + relOffset
value      = block[valueStart : valueStart + byteLen]
```

Idiom C++ klasik supaya blok bisa di-`mmap` tanpa relokasi pointer. Kalau
diasumsikan offset absolut, hasil dekode berupa potongan kalimat yang saling
tumpang tindih — itu gejala khasnya.

Spesifikasi lengkap: [`docs/FORMAT.md`](docs/FORMAT.md).

---

## Panduan menerjemahkan

### Lindungi token format engine

String di game penuh markup milik Messiah. Kalau token ini diterjemahkan,
digeser, atau hilang, UI bisa render kacau:

| token | arti |
|---|---|
| `#Y` `#N` `#R` `#J` `#G` `#H` | pembuka kode format/warna |
| `#aee5ae` | pembuka warna hex 6 digit |
| `#E` | penutup format — **wajib** berpasangan |
| `%s` `%d` | printf placeholder, diisi runtime |
| `{0}` `{1}` | placeholder berindeks |
| `\n` | ganti baris |

Contoh benar:

```
EN : Mystic Skill #YMeridian Touch#E
ID : Keterampilan Mistik #YSentuhan Meridian#E
```

Teks di dalam `#Y...#E` **boleh** diterjemahkan. Yang tidak boleh berubah
adalah `#Y` dan `#E` itu sendiri, beserta jumlah dan urutannya.

### Jangan terjemahkan nama diri

Nama karakter dan sekte umumnya Pinyin (`Su Jiangyun`, `Gu Zhouyue`,
`Zhang Tiemeng`). Biarkan apa adanya supaya tetap cocok dengan wiki, panduan,
dan chat pemain lain. Pada file rujukan, ~12,5% entri memang sengaja tidak
diterjemahkan dan mayoritas adalah kategori ini.

### Kalau pakai MT/LLM: validasi output-nya

Ini bukan saran teoretis. Pada pack komersial yang diperiksa, ditemukan
**1.817 entri (0,19%) di mana system prompt penerjemah bocor ke dalam nilai**
dan ikut ter-render di dalam game:

```
Penanda format:
  (penanda baris baru) dan tanda kurung [], harus tetap tidak berubah
- Jangan terjemahkan konten dalam {}
- beberapa karakter khusus tidak perlu diterjemahkan, seperti: %d, %s, #G, #E...
<terjemahan sebenarnya baru muncul di sini>
```

Paling parah di entri panjang — lore, encyclopedia, deskripsi item — yang
justru bagian paling penting dari game yang story-driven. Ada 5+ varian wording
prompt, jadi kemungkinan template-nya berganti antar batch tanpa validasi
output sama sekali.

Dua assertion sederhana menangkap seluruh kasus itu sebelum rilis, dan keduanya
sudah ada di `tools/qa_check.py`:

1. output tidak boleh mengandung fragmen prompt
2. multiset token markup di sumber dan terjemahan harus sama

### Entri raksasa

Panjang rata-rata ~56 byte, tapi ada entri sampai **8 KB** (lore panjang).
Kalau pipeline-mu lewat API, siapkan chunking — jangan sampai output terpotong
diam-diam.

---

## Batasan yang diketahui

- **File verification menimpa patch.** Setiap kali launcher menjalankan
  verifikasi berkas, file kembali ke versi resmi. Apply ulang setelahnya.
  Ini bukan bug tool ini, melainkan konsekuensi mengganti file game.
- **Patch ketinggalan setelah update konten.** String baru dari patch resmi
  belum ada di pack lama dan akan tampil dalam bahasa Inggris sampai kamu
  dump ulang lalu menerjemahkan selisihnya.
- **Tidak bisa menambah key baru** (lihat [Cara kerjanya](#cara-kerjanya)).
- **Hanya `codec = 4` (zstd)** yang ditangani. Kalau NetEase menambah codec
  lain, parser akan menolak dengan pesan jelas, bukan menghasilkan data rusak.
- **Audio/voice-over tidak tersentuh.** File ini murni teks.

---

## Legal & risiko

**Baca bagian ini sebelum memakai tool-nya.**

- Proyek ini **tidak berafiliasi** dengan NetEase, Everstone Studio, atau vendor
  translation pack mana pun. Semua merek dagang milik pemiliknya.
- Repo ini **tidak mendistribusikan** teks game, pack terjemahan, atau aset
  apa pun. Hanya tool dan dokumentasi format. Semua teks di dalam file
  lokalisasi adalah karya berhak cipta NetEase, dan pack terjemahan pihak
  ketiga adalah karya turunan milik pembuatnya. **Jangan commit hasil `dump`
  atau pack jadi ke repo publik** — `.gitignore` sudah menyiapkan ini.
- **Where Winds Meet adalah game online**, bahkan mode solo tetap terhubung ke
  server. Memodifikasi file client berpotensi melanggar ToS dan bisa terdeteksi
  oleh sistem anti-tamper, meskipun yang diubah hanya string dan tidak ada
  logic gameplay yang disentuh. Sejauh ini belum ada laporan ban massal khusus
  karena translation pack, tapi **tidak adanya bukti bukan bukti tidak adanya**.
- Risiko sepenuhnya ada di kamu. Jangan jadikan akun utama yang sudah
  ter-invest banyak sebagai kelinci percobaan pertama.
- Tool ini **tidak** memodifikasi logic game, tidak memberi keuntungan
  kompetitif, dan tidak menyentuh apa pun selain string yang ditampilkan.

---

## Struktur repo

```
.
├── wwm_locmap.py        tool utama: info / dump / patch
├── tools/
│   └── qa_check.py      validator prompt-leak, markup, string kosong
├── docs/
│   └── FORMAT.md        spesifikasi format lengkap
├── requirements.txt
├── .gitignore
└── LICENSE
```

## Kontribusi

Yang paling membantu:

- **Memecahkan derivasi H1/H2 dari `keyHash` + `seed`.** Ini membuka
  kemampuan menambah key baru, bukan sekadar mengubah nilai. Catatan
  eksperimen ada di `docs/FORMAT.md` §4.4.
- Varian kebocoran prompt baru untuk `LEAK_SIGNATURES` di `qa_check.py`.
- Token format engine yang belum terdaftar di tabel markup.
- Konfirmasi format pada file locale bahasa lain (`_de`, `_fr`, `_ja`, ...).

Sertakan versi game dan jumlah entri saat melaporkan masalah parsing.

## Lisensi

MIT — lihat [LICENSE](LICENSE). Lisensi ini berlaku untuk kode di repo ini saja,
bukan untuk konten game apa pun yang diproses olehnya.
