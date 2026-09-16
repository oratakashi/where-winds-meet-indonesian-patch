# Format `translate_words_map_*`

Spesifikasi hasil reverse-engineering terhadap file lokalisasi Where Winds Meet
(NetEase Messiah Engine). Semua field **little-endian**.

Terverifikasi round-trip penuh pada 963.050 entri: parse → dump → repack → parse
menghasilkan pasangan `(keyHash, value)` yang 100% identik.

---

## 1. Container

```
offset  tipe                 nilai
0x00    u32                  magic       = 0xDEADBEEF
0x04    u32                  version     = 1
0x08    u32                  blockCount  (mis. 3763)
0x0C    u32                  reserved    = 0
0x10    u32[blockCount]      endOffset
```

`endOffset[i]` adalah offset **akhir** blok ke-i, relatif terhadap akhir tabel
offset itu sendiri:

```
tableEnd   = 0x10 + blockCount * 4
blockStart = tableEnd + (i > 0 ? endOffset[i-1] : 0)
blockEnd   = tableEnd + endOffset[i]
```

Invarian yang berguna untuk validasi cepat:
`tableEnd + endOffset[blockCount-1] == filesize`.

## 2. Blok

Setiap blok punya header 9 byte, lalu payload terkompresi:

```
u8   codec              4 = zstd (satu-satunya yang terlihat)
u32  compressedSize
u32  uncompressedSize
u8[compressedSize]      payload zstd (magic 28 B5 2F FD)
```

Kompresi dilakukan **per blok**, bukan per file. Ini alasan file utuh tidak
terdeteksi sebagai arsip zstd oleh `file(1)`.

## 3. Blok 0 — index

```
u64  totalEntries        mis. 963050
u64  dataBlockCount      mis. 3762  (= blockCount - 1)
u32[dataBlockCount] ids  nilainya 1..dataBlockCount, berurutan
```

Praktisnya cuma berisi metadata; array `ids` redundan.

## 4. Blok 1..n — shard hash map

Tiap shard adalah hash map bergaya **SwissTable** (`absl::flat_hash_map`) yang
diserialisasi apa adanya supaya bisa di-`mmap`.

```
u64  capacity        mis. 511   (selalu 2^k - 1)
u64  size            jumlah slot terisi, mis. ~256
u64  seed            mis. 0x595896DC — sama untuk seluruh file

u8[capacity]  ctrl   0x80 = slot kosong; < 0x80 = terisi (H2, 7 bit)
u8            0xFF   sentinel
u8[15]        clone  salinan 15 byte pertama ctrl (untuk SIMD probing)
u8[...]       pad    sampai kelipatan 8

slot[capacity] {     // 16 byte per slot
    u64 keyHash
    u32 relOffset
    u32 byteLen
}

<string blob>        UTF-8, tanpa NUL terminator
```

### 4.1 Relative pointer — bagian yang paling mudah salah

`relOffset` **bukan** offset dari awal blok, bukan pula dari awal string blob.
Nilainya dihitung dari **alamat field `relOffset` itu sendiri**:

```
valueStart = addressOf(slot.relOffset) + slot.relOffset
value      = block[valueStart : valueStart + slot.byteLen]
```

Dengan `S` = offset awal array slot dan `i` = indeks slot:

```
field      = S + i * 16 + 8
valueStart = field + relOffset
```

Idiom C++ klasik (mirip `boost::offset_ptr`): blok bisa dipetakan ke memori
tanpa relokasi pointer sama sekali. Kalau diasumsikan offset absolut, hasilnya
adalah potongan kalimat yang saling tumpang tindih — gejala khas salah baca ini.

### 4.2 Sharding

```
shardIndex = (keyHash % dataBlockCount) + 1
```

Dikonfirmasi konsisten 100% pada blok 1, 500, 2000, dan 3762.

Artinya: kalau kamu **menambah** key baru, kamu harus menaruhnya di shard yang
benar. Kalau hanya **mengubah nilai** key yang sudah ada (kasus normal
pembuatan pack bahasa), sharding tidak perlu disentuh sama sekali.

### 4.3 Yang TIDAK disimpan: key-nya

File ini hanya berisi `u64 keyHash → string terjemahan`. Teks sumber bahasa
Inggris tidak pernah ditulis ke disk.

Dua konsekuensi besar:

**a. Game wajib diset ke English.** Runtime mengambil string Inggris dari asset
resmi, menghitung hash 64-bit-nya, lalu menukar hasil lookup dengan nilai di
file ini. Set bahasa ke Jerman → hash berbeda → semua lookup miss → teks kembali
ke Jerman. Jadi syarat "Game Language = English" di semua translation pack itu
konsekuensi arsitektur, bukan pilihan desain tool-nya.

**b. Membuat pack jauh lebih sederhana dari yang terlihat.** Kamu tidak perlu
tahu fungsi hash-nya. Cukup pertahankan `ctrl`, `keyHash`, dan posisi slot
apa adanya, lalu tulis ulang string blob dan perbarui `relOffset` + `byteLen`.
Struktur pencarian tidak tersentuh, lookup tetap jalan sempurna.

### 4.4 Fungsi hash: belum dipecahkan (dan tidak perlu)

`ctrl[i]` (H2) jelas bukan turunan sederhana dari `keyHash` — 58 varian shift
diuji, korelasi tertinggi hanya ~2,7% alias noise. Kemungkinan besar ada mixing
dengan `seed` sebelum H1/H2 diturunkan.

Ini **tidak menghalangi** pembuatan pack bahasa, karena pendekatan di §4.3(b)
tidak pernah menghitung ulang penempatan slot. Yang belum bisa dilakukan tanpa
memecahkan hash: **menambahkan key yang benar-benar baru** ke map.

### 4.5 Varian `_diff`: file incremental antar-versi

`translate_words_map_en_diff` memakai **container dan shard yang identik** dengan file penuh —
tidak ada perbedaan struktur biner. Bedanya cuma dua hal, keduanya terverifikasi pada file nyata
(2,4 MB, 3762 shard, 31.471 entri hadir dari klaim `totalEntries` 973.284 di blok index):

1. **Sparse.** Sebagian besar shard nyaris kosong — file ini hanya membawa key yang *berubah*
   sejak versi dasar, bukan seluruh map. Karena itu `entries parsed` (dihitung dari `size` tiap
   shard) akan jauh lebih kecil dari `totalEntries` pada blok index; itu bukan bug parser, itu
   ciri file diff.
2. **Tombstone.** Key yang **dihapus** sejak versi dasar tetap punya slot (ctrl/keyHash tidak
   disentuh), tapi nilainya diganti satu byte tunggal `0xFF` — bukan UTF-8 valid, sengaja dipakai
   sebagai sentinel karena `0xFF` tidak pernah muncul sebagai lead byte UTF-8. Cocok dicek dengan
   `value == b'\xff'`.

`wwm_locmap.py` menangani ini secara transparan lewat konstanta `TOMBSTONE` di `cmd_dump`/
`cmd_patch` — tidak perlu flag CLI terpisah untuk file `_diff` vs file penuh.

### 4.6 Varian `__small` / `__small_diff`: tabel terpisah, bukan bagian dari `_diff`

Update game (2026-09) menambahkan `translate_words_map_en__small` dan `__small_diff`. Keduanya
memakai **container/shard yang identik** dengan file utama — dikonfirmasi lewat round-trip
dump→patch→parse ulang yang byte-identik pada seluruh isinya, tidak ada perubahan format sama
sekali. Bedanya bukan struktur biner, tapi isi:

- Ini tabel hash **terpisah**, dengan address space (`b`/`s`) dan jumlah shard sendiri (16 shard
  di `__small` vs. ribuan di file utama) — bukan delta/diff dari file utama meski namanya mirip.
  Kemungkinan besar subset "hot" (loading screen, splash, dll.) yang di-load lebih cepat/awal.
- Diukur pada update 2026-09: dari ~4.000 key di `__small`, **98% key hash-nya juga ada** di
  `translate_words_map_en`, tapi **193 di antaranya punya value yang beda** dari kopi di file
  utama (duplikat basi/kontekstual — perlakukan sebagai teks sendiri, jangan asumsikan sama), dan
  **27 key hanya ada di `__small`**, tidak di file utama maupun versi sebelum update.
- `__small_diff` mengikuti pola `_diff` yang sama (lihat §4.5) — pada sampel yang diperiksa,
  isinya kosong sama sekali (0 shard data, index-nya cuma metadata tanpa perubahan).

Konsekuensi praktis: jangan pernah asumsikan `keyHash` yang sama antar-file berarti teks yang
sama saat ini. Selalu dump & cocokkan tiap file secara independen berdasarkan teks sumbernya
(lihat `tools/expand_locale.py`/`tools/patch_all.py` — dictionary terjemahan dikunci ke teks
literal, bukan ke hash/alamat, jadi ini otomatis benar untuk variasi file apa pun).

---

## 5. Ringkasan alur decode

1. Baca & validasi header container, ambil tabel `endOffset`.
2. Untuk tiap blok: baca header 9 byte, dekompresi payload zstd.
3. Blok 0 → metadata. Blok 1..n → shard.
4. Per shard: baca `capacity/size/seed`, ambil `ctrl[capacity]`, verifikasi
   sentinel `0xFF` di `24 + capacity`.
5. Hitung `S` = offset array slot (align 8 setelah sentinel + 15 byte clone).
6. Untuk tiap `i` dengan `ctrl[i] < 0x80`: baca slot, resolve relative pointer,
   ambil `byteLen` byte sebagai nilai UTF-8.
7. Sanity check: jumlah slot terisi harus sama dengan `size`.

## 6. Ringkasan alur encode

1. Salin `header + ctrl + sentinel + clone + pad + slot array` **apa adanya**.
2. Bangun string blob baru; dedup string identik menghemat ruang (~4% pada
   file asli, karena original memang menyimpan duplikat).
3. Untuk tiap slot terisi, tulis ulang `relOffset = blobPos - field` dan
   `byteLen = len(value)`.
4. Kompresi tiap blok dengan zstd, susun ulang tabel `endOffset`, tulis
   container.

`keyHash`, `ctrl`, `capacity`, `size`, dan `seed` tidak pernah berubah.
