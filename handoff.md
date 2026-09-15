# Handoff — Terjemahan Indonesia WWM

**Baca `plan.md` sekali di awal sesi** untuk tahu pembagian fase & saran ukuran batch.
File ini (`handoff.md`) adalah status kerja yang **diupdate tiap akhir sesi** — sumber
kebenaran untuk "sudah sampai mana".

## Status saat ini

- **Fase 0, Fase 1, dan Fase 2 SELESAI** (idx 0–4.999). Fase saat ini: **Fase 3**
  (rentang idx 5.000–9.999, saran batch 800–1.000/sesi) — **4.300/5.000 dari Fase 3
  selesai, sisa idx 9.300–9.999 (700 lagi)**.
- Total baris di `strings.jsonl`: **963.050**
- Total string unik (setelah dedup): **429.887**
- **Sudah diterjemahkan: idx 0–9.299 dari 429.887 (9.300 string unik, ~2.16%)**
- Sesi terakhir mengerjakan: 2026-09-15 (idx 8.500–9.299, 800 string, Fase 3 lanjut —
  banyak mekanisme pertarungan/Inner Way, lore asal-usul Skygrasp rope art & Hu Li,
  kisah Pertempuran Xiande Dinasti Zhou, deskripsi busana/senjata Imperial Guard, dan dialog NPC)
- **File progress dipecah per fase** di `locale/phase{N}.jsonl` (mis. `locale/phase0.jsonl`,
  `locale/phase1.jsonl`, `locale/phase2.jsonl`, dst. — mengikuti nomor fase & rentang idx
  di tabel `plan.md`). Tiap file berisi `{"idx": N, "v": "..."}` per baris, `idx` yang
  dipakai adalah idx **absolut** dari `unique_strings.jsonl` (bukan di-reset ke 0 per file),
  jadi antar-file tetap gampang di-cross-reference. Sudah ada: `locale/phase0.jsonl` (800
  baris, lengkap), `locale/phase1.jsonl` (1.200 baris, lengkap), `locale/phase2.jsonl`
  (3.000 baris, lengkap), `locale/phase3.jsonl` (4.300 baris dari target 5.000 — belum
  lengkap, sisa 700 baris untuk menuntaskan Fase 3).
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

## PENTING: sisa pekerjaan sangat besar

420.587 string unik lagi setelah progress ini. Lihat `plan.md` untuk perkiraan jumlah
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
with open('locale/phase3.jsonl', encoding='utf-8') as f:  # <- ganti sesuai fase aktif
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

## Catatan lain

- `strings.jsonl` dan `translate_words_map_en` masih ke-staged di git (bukan diubah
  sesi ini atas permintaan user — "biarkan saja"). File-file kerja terjemahan
  (`translation_work/`, `strings.jsonl`) sudah ditambahkan ke `.gitignore` supaya
  tidak ikut ter-commit ke depannya, tapi staging area yang sudah ada tidak disentuh.
