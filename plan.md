# Rencana Multi-Sesi — Terjemahan Indonesia WWM

Dokumen ini adalah **roadmap statis** (strategi, fase, target per fase). Untuk status
terkini (idx terakhir yang selesai, langkah resume persis) selalu lihat **`handoff.md`**
— itu yang diupdate tiap sesi. `plan.md` ini jarang berubah, cukup dibaca sekali di awal
tiap sesi untuk tahu ada di fase mana dan berapa target batch-nya.

## Kenapa dipecah per sesi

429.887 string unik terlalu besar untuk satu sesi (bahkan beberapa sesi kecil). Supaya
tiap sesi tetap ringan dan hasilnya tetap teliti (bukan asal translate), pekerjaan
dipecah jadi **fase** berdasarkan rentang `idx` di `translation_work/unique_strings.jsonl`
(sudah terurut dari string yang paling sering dipakai duluan). Ukuran batch per sesi
beda-beda per fase — fase awal lebih kecil (banyak teks lore/dialog panjang yang perlu
teliti), fase belakang boleh lebih besar (kebanyakan string pendek/berulang polanya).

## Fase & target

| Fase        | Rentang idx       | Jumlah unik | Kumulatif baris tercakup*       | Saran ukuran batch/sesi | Perkiraan sesi                | File output           |
| ----------- | ----------------- | ----------- | ------------------------------- | ----------------------- | ----------------------------- | --------------------- |
| 0 (selesai) | 0 – 799           | 800         | ~16.2%                          | —                       | selesai                       | `locale/phase0.jsonl` |
| 1 (selesai) | 800 – 1.999       | 1.200       | ~21.28% (aktual, terverifikasi) | —                       | selesai (3 sesi)              | `locale/phase1.jsonl` |
| 2 (selesai) | 2.000 – 4.999     | 3.000       | ~25–28% (aktual, terverifikasi) | —                       | selesai (2 sesi)              | `locale/phase2.jsonl` |
| 3 (selesai) | 5.000 – 9.999     | 5.000       | ~33.5%                          | —                       | selesai (~6 sesi)             | `locale/phase3.jsonl` |
| 4 (selesai) | 10.000 – 19.999   | 10.000      | ~40.0%                          | —                       | selesai (7 sesi)              | `locale/phase4.jsonl` |
| 5 (selesai) | 20.000 – 49.999   | 30.000      | ~50.1%                          | —                       | selesai (15/15 sesi)          | `locale/phase5.jsonl` |
| 6 (jalan)   | 50.000 – 99.999   | 50.000      | ~60.5%                          | 2.000/sesi (fixed)      | ~17–25 sesi (1.5/~25 selesai) | `locale/phase6.jsonl` |
| 7           | 100.000 – 199.999 | 100.000     | ~76.1%                          | 2.500–3.500/sesi        | ~29–40 sesi                   | `locale/phase7.jsonl` |
| 8           | 200.000 – 429.886 | 229.887     | 100%*                           | 3.000–5.000/sesi        | ~46–77 sesi                   | `locale/phase8.jsonl` |
| 9 (jalan)   | 429.887 – 461.703 | 31.817      | tambahan (lihat catatan)        | 2.000/sesi (fixed)      | ~16 sesi (5.6/16 selesai)     | `locale/phase9.jsonl` |

**Fase 9** ditambahkan 2026-09-16 setelah update game — bukan bagian dari 429.887 unik semula.
`tools/rebuild_unique_strings.py` menambah idx 429.887–461.703 (31.817 string baru) dari string
yang baru muncul/berubah di `translate_words_map_en` (versi update), `_diff`, `__small`, dan
`__small_diff` yang **belum pernah tercatat** di `unique_strings.jsonl`. idx 0–429.886 (fase 0–8)
**tidak diubah sama sekali** — progress lama tetap valid tanpa remap. Lihat `handoff.md` bagian
"Update game 2026-09" untuk detail & cara `patch_all.py` menerapkan dictionary ke keempat file
sekaligus.

**Update 2026-09-17 (awal sesi)**: atas permintaan eksplisit user, sesi itu melompat ke
Fase 9 duluan (batch pertama, idx 429.887–431.886) meski Fase 5 belum selesai (berhenti di
idx 37.000). **Kedua fase sekarang jalan paralel** — Fase 5 dan Fase 9 sama-sama punya
progress belum tuntas. Sesi berikutnya **harus tanya user dulu fase mana yang mau
dilanjutkan** kalau tidak disebutkan eksplisit (lihat "Status saat ini" di `handoff.md`
untuk next idx masing-masing).

**Update 2026-09-17 (lanjutan)**: user eksplisit minta lanjut Fase 5 (bukan Fase 9), batch
2.000 string/iterasi dikonfirmasi ulang. idx 37.000–38.999 selesai, next idx Fase 5 = 39.000.
Fase 9 tetap di next idx 435.887, tidak disentuh sesi ini.

**Update 2026-09-17 (batch kelima)**: lanjut Fase 5 lagi dengan batch 2.000 string/iterasi.
idx 39.000–40.999 selesai, next idx Fase 5 = 41.000. Fase 9 tetap di next idx 435.887,
tidak disentuh sesi ini.

**Update 2026-09-17 (batch keenam)**: lanjut Fase 5 lagi dengan batch 2.000 string/iterasi
(dikonfirmasi ulang). idx 41.000–42.999 selesai, next idx Fase 5 = 43.000. Fase 9 tetap
di next idx 435.887, tidak disentuh sesi ini.

**Update 2026-09-18 (batch ketujuh Fase 5)**: user eksplisit minta lanjut Fase 5 (bukan
Fase 9), batch 2.000 string/iterasi dikonfirmasi ulang. idx 43.000–44.999 selesai,
next idx Fase 5 = 45.000. Fase 9 tetap di next idx 439.137, tidak disentuh sesi ini.

**Update 2026-09-18 (batch keempat Fase 9)**: user eksplisit minta lanjut Fase 9 (bukan
Fase 5), target awal 2.000 string/iterasi. Penerjemahan didelegasikan ke subagent
background; user minta subagent dihentikan lebih awal setelah 1.250/2.000 string
("sepertinya sudah cukup"), jadi batch sesi ini **1.250 string** (bukan 2.000 penuh).
idx 435.887–437.136 selesai (setelah validasi token + perbaikan manual, lihat
`handoff.md`), next idx Fase 9 = 437.137. Fase 5 tetap di next idx 43.000, tidak
disentuh sesi ini.

**Update 2026-09-18 (batch kelima Fase 9)**: user lanjut Fase 9 lagi, batch penuh
2.000 string/iterasi (dikerjakan langsung oleh sesi utama, bukan didelegasikan ke
subagent). idx 437.137–439.136 selesai, next idx Fase 9 = 439.137. Fase 5 tetap di
next idx 43.000, tidak disentuh sesi ini.

**Update 2026-09-18 (batch kedelapan Fase 5)**: user eksplisit minta lanjut Fase 5,
batch 2.000 string/iterasi dikonfirmasi ulang. idx 45.000–46.999 selesai, next idx
Fase 5 = 47.000 (12.5/15 sesi awal batch 2.000/sesi selesai — lihat kolom "Perkiraan
sesi" di tabel di atas). Fase 9 tetap di next idx 439.137, tidak disentuh sesi ini.

**Update 2026-09-18 (batch kesembilan Fase 5)**: user lanjut Fase 5 lagi, batch
2.000 string/iterasi dikonfirmasi ulang. idx 47.000–48.999 selesai, next idx Fase 5
= 49.000 (14.5/15 sesi awal batch 2.000/sesi selesai — hanya 1.000 baris tersisa di
Fase 5, idx 49.000–49.999, sesi berikutnya akan jadi batch penutup fase ini). Fase 9
tetap di next idx 439.137, tidak disentuh sesi ini.

**Update 2026-09-18 (batch penutup Fase 5 + batch pembuka Fase 6)**: user minta
lanjut dengan batch 2.000 string/iterasi. idx 49.000–49.999 (1.000 baris) menuntaskan
Fase 5 sepenuhnya (idx 20.000–49.999, 30.000/30.000 baris) — `locale/phase5.jsonl`
final, tidak akan ditambah lagi. Sisa 1.000 baris dari target 2.000/sesi (idx
50.000–50.999) otomatis maju ke Fase 6 sesuai prosedur "potong batch di batas fase"
di `handoff.md`, membuka file baru `locale/phase6.jsonl` (1.000/50.000 baris fase ini
selesai). Next idx Fase 6 = 51.000. Fase 9 tetap di next idx 439.137, tidak disentuh
sesi ini.

**Update 2026-09-18 (batch kedua Fase 6)**: user lanjut Fase 6, batch 2.000
string/iterasi dikonfirmasi ulang. idx 51.000–52.999 (2.000 baris) selesai
(3.000/50.000 baris Fase 6 selesai). Next idx Fase 6 = 53.000. Fase 9 tetap di
next idx 439.137, tidak disentuh sesi ini.

**Update 2026-09-18 (batch keenam Fase 9)**: user lanjut Fase 9, batch 2.000
string/iterasi dikonfirmasi ulang. idx 439.137–441.136 (2.000 baris) selesai
(11.250/31.817 baris Fase 9 selesai, ~35.36%). Next idx Fase 9 = 441.137. Fase 6
tetap di next idx 53.000, tidak disentuh sesi ini.

**Update 2026-09-19 (batch ketujuh Fase 9)**: user lanjut Fase 9, tapi **menurunkan
ukuran batch jadi 1.000 string/iterasi** (dari 2.000 sebelumnya) — ini sekarang
default baru sampai ada instruksi lain, menggantikan preferensi "2.000/sesi" yang
tercatat di bagian "Catatan preferensi user" di bawah. idx 441.137–442.136
(1.000 baris) selesai (12.250/31.817 baris Fase 9 selesai, ~38.50%). Next idx
Fase 9 = 442.137. Fase 6 tetap di next idx 53.000, tidak disentuh sesi ini.

**Update 2026-09-20 (batch kedelapan Fase 9)**: user eksplisit minta lanjut Fase 9
lagi, kali ini dengan **ukuran batch 2.000 string/iterasi** (naik lagi dari 1.000
di sesi sebelumnya) — dikonfirmasi ulang oleh user di awal sesi ini. idx
442.137–444.136 (2.000 baris) selesai, diproses dalam 8 sub-batch 250 lalu
digabung, tervalidasi **0 mismatch token** di seluruh 2.000 baris sekaligus
setelah digabung (dicek dengan script Python, bukan manual), termasuk validasi
ulang penuh `phase9.jsonl` gabungan (14.250 baris) dan cross-check 0
duplikat/gap idx lintas semua file `locale/phase*.jsonl` (total 67.250 baris/idx
unik, tanpa tabrakan). Next idx Fase 9 = 444.137 (14.250/31.817 baris Fase 9
selesai, ~44.78%). Fase 6 tetap di next idx 53.000, tidak disentuh sesi ini.
Batch ini banyak berisi lore panjang Mohist Hill/Hidden Mountain (kisah Zou/Yang
"Together in One Boat", kisah asal-usul Kingfisher, drama keluarga Zou Bao/Tiger
Fort, register Dragonbend Academy, dan satu entry raksasa berisi daftar ratusan
nama karakter pemain Hall of Fame — idx 444.030, diperlakukan sebagai nama dan
dibiarkan 100% tidak diterjemahkan, lihat `handoff.md`).

\* Persentase dari 963.050 baris total di `strings.jsonl`, dihitung dari distribusi
frekuensi aktual (lihat catatan di bawah). Angka fase 2–8 adalah interpolasi kasar,
bukan hitungan presisi per-idx — jangan dianggap eksak.

**Kenapa file dipecah per fase**: satu file JSONL besar (semua 429.887 baris) tidak
praktis dibaca/di-diff/dibuka. Tiap fase punya file output sendiri di `locale/`, berisi
`{"idx": N, "v": "..."}` per baris dengan `idx` **absolut** sesuai `unique_strings.jsonl`
(bukan direset ke 0 tiap file) — jadi gampang digabung lagi nanti tanpa remapping. Detail
teknis (cara resume, validasi) ada di `handoff.md`.

**Total perkiraan realistis: 125–190+ sesi** untuk mencapai 100% string unik. Ini
proyeksi kasar, bisa jauh berubah tergantung seberapa banyak string di ekor
distribusi (fase 6–8) yang ternyata pendek/berulang pola (lebih cepat) vs unik
dan butuh konteks (lebih lambat).

## Titik berhenti yang masuk akal (opsional)

Coverage tidak linear — makin ke ekor distribusi, effort makin besar untuk gain
makin kecil:

- **~50% baris** (akhir fase 5, idx ~50.000) = titik tengah yang wajar untuk jeda
  panjang dan evaluasi ulang apakah lanjut ke ekor masih sepadan.
- String di ekor jauh (fase 7–8) sering berupa item/skill langka, teks debug, atau
  variasi kecil dari string yang sudah diterjemahkan — nilai gameplay-nya lebih
  rendah per unit effort.
- CLAUDE.md & `patch` sudah mendukung **terjemahan parsial** (entry yang belum
  diterjemahkan otomatis dianggap "sengaja dibiarkan asli", bukan error) — jadi
  berhenti kapan pun tetap menghasilkan patch yang valid dan bisa dipakai.

Keputusan berhenti di titik mana ada di tangan user — dokumen ini cuma kasih
konteks supaya keputusan itu punya data.

**Catatan preferensi user**: mulai sesi Fase 4 lanjutan (2026-09-15), user minta ukuran
batch **tetap 1.000 string per iterasi/sesi** (bukan mengikuti rentang saran per-fase di
tabel di atas) sampai ada instruksi lain. Kolom "Saran ukuran batch/sesi" tetap jadi
referensi kasar untuk fase-fase berikutnya. **Update 2026-09-17**: user menaikkan ukuran
batch jadi **2.000 string/sesi** (dua kali lipat dari sebelumnya) — ini sekarang default
aktual sampai ada instruksi lain, dan tabel fase di atas sudah disesuaikan untuk Fase 5.
**Update 2026-09-19**: user menurunkan lagi ukuran batch jadi **1.000 string/sesi**
(kembali ke ukuran sebelum 2026-09-17) — ini sekarang default aktual sampai ada
instruksi lain berikutnya. **Update 2026-09-20**: user menaikkan lagi ukuran
batch jadi **2.000 string/sesi** (kembali ke ukuran 2026-09-17) — ini sekarang
default aktual sampai ada instruksi lain berikutnya.

## Prosedur satu sesi (ringkas — detail lengkap ada di handoff.md)

1. Baca `handoff.md` → tahu idx terakhir yang selesai & fase saat ini.
2. Baca `translation_work/GLOSSARY.md` → ikuti konvensi yang sudah dikunci.
3. Ambil batch berikutnya dari `translation_work/unique_strings.jsonl` sesuai ukuran
   yang disarankan fase saat ini (lihat tabel di atas).
4. Terjemahkan, append ke `locale/phase{N}.jsonl` (N = nomor fase saat ini — lihat
   kolom "File output" di tabel). Jangan campur idx dari fase lain ke file yang salah.
5. Jalankan script validasi token (di `handoff.md`) — wajib 0 mismatch sebelum lanjut.
6. Update `handoff.md`: idx terakhir baru, fase saat ini, tanggal, catatan istilah
   baru (kalau ada keputusan baru → tambahkan juga ke `GLOSSARY.md`).
7. Berhenti di situ untuk sesi ini — jangan paksa sampai konteks/token habis, supaya
   validasi & pencatatan progress tetap rapi (lebih baik berhenti di batas batch yang
   bersih daripada terputus di tengah).

## Setelah semua fase (atau titik berhenti yang dipilih) selesai

Gabungkan semua `locale/phase*.jsonl` (per-unique) lalu expand ke `strings.jsonl` penuh
(per-baris), lalu jalankan `tools/qa_check.py` dan `wwm_locmap.py patch`. Detail rencana
expand ada di bagian akhir `handoff.md` (belum diimplementasi sebagai script — dikerjakan
setelah cukup banyak string selesai, atau di titik berhenti yang dipilih user).
