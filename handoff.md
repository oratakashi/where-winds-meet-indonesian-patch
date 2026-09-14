# Handoff — Terjemahan Indonesia WWM

**Baca `plan.md` sekali di awal sesi** untuk tahu pembagian fase & saran ukuran batch.
File ini (`handoff.md`) adalah status kerja yang **diupdate tiap akhir sesi** — sumber
kebenaran untuk "sudah sampai mana".

## Status saat ini

- **Fase 0 dan Fase 1 SELESAI** (idx 0–1.999). Fase saat ini: **Fase 2**
  (rentang idx 2.000–4.999, saran batch 600–800/sesi) — belum dimulai.
- Total baris di `strings.jsonl`: **963.050**
- Total string unik (setelah dedup): **429.887**
- **Sudah diterjemahkan: idx 0–1.999 dari 429.887 (2.000 string unik, ~0.47%)**
- Sesi terakhir mengerjakan: 2026-09-15 (idx 1.800–1.999, 200 string, menuntaskan Fase 1)
- File progress: `translation_work/translations.jsonl` (append-only, `{"idx": N, "v": "..."}`,
  berurutan mulai idx 0). **Baris terakhir di file ini = idx terakhir yang selesai.**
  Cek dengan: `wc -l translation_work/translations.jsonl` (nilai ini dikurang 1 = idx terakhir).

String diurutkan berdasarkan **frekuensi kemunculan** (bukan urutan asli file), jadi
yang paling sering dipakai di 963rb baris asli dikerjakan duluan — coverage baris
riil lebih cepat naik daripada persentase unik di atas. 2.000 unik pertama ini sudah
mencakup **21.28% dari 963.050 baris total** (karena banyak string berulang ribuan
kali, mis. "Talk" x11.299, "Loading..." x10.002). Semua 2.000 sudah lolos cek token
format (`#E`, `#aabbcc`, `%s`, `%d`, `{0}`/`{value}` dst., termasuk tag `<Label|id|#C|n>`
— lihat script di bawah, sudah diperluas untuk ikut cek tag `<...>` juga. 1 mismatch
sempat ketemu & sudah diperbaiki di idx 1056 — tag `#Y...#E` yang membungkus banyak
kata sempat hilang saat translate, jadi hati-hati kalau tag membungkus lebih dari
satu kalimat).

## PENTING: sisa pekerjaan sangat besar

427.887 string unik lagi setelah progress ini. Lihat `plan.md` untuk perkiraan jumlah
sesi per fase (kasar: 125–190+ sesi total sampai 100%). Sampaikan ini ke user kalau
ditanya estimasi waktu, dan ingatkan opsi berhenti di ~50% baris (lihat "Titik berhenti
yang masuk akal" di `plan.md`) kalau relevan.

## Cara resume di sesi berikutnya

1. Baca `plan.md` → cek fase saat ini & saran ukuran batch untuk fase itu.
2. Baca `translation_work/GLOSSARY.md` — berisi semua aturan konsisten yang HARUS diikuti
   (nama yang tidak diterjemahkan, istilah yang tetap bahasa Inggris, gaya bahasa per
   konteks). **Jangan menerjemahkan tanpa baca file ini dulu**, supaya konsisten dengan
   yang sudah dikerjakan.
3. Cek idx terakhir yang selesai: `wc -l translation_work/translations.jsonl` (baris N
   berarti idx 0..N-1 sudah selesai, next idx = N).
4. Baca batch berikutnya dari `translation_work/unique_strings.jsonl` mulai baris N+1
   (1-indexed) sebanyak sesuai saran batch fase saat ini (`Read` dengan `offset`/`limit`).
5. Terjemahkan tiap `v` sesuai aturan di GLOSSARY.md, tulis ke file sementara
   `{"idx": N, "v": "..."}` per baris, urut naik dari idx yang sedang dikerjakan.
6. **WAJIB validasi sebelum append**: jalankan cek token/placeholder (lihat contoh
   script di bawah) untuk memastikan `#E`, `#aabbcc`, `#X`, `%s`, `%d`, `{0}` dst.
   jumlahnya sama persis dengan sumber. Ini yang nanti dicek juga oleh `tools/qa_check.py`.
7. Append hasil batch ke `translation_work/translations.jsonl` (pastikan idx tetap
   berurutan tanpa lompat/duplikat — bisa dicek dengan script kecil).
8. Update bagian "Status saat ini" di file ini (idx terakhir, fase, tanggal sesi, dan
   catatan keputusan baru kalau ada istilah ambigu yang baru ditemui — tambahkan juga
   ke `GLOSSARY.md` supaya konsisten ke depannya). Kalau rentang idx fase saat ini sudah
   habis, majukan ke fase berikutnya sesuai tabel di `plan.md`.
9. **Berhenti bersih di akhir batch** — jangan lanjut sampai konteks/token mepet.
   Lebih baik sesi pendek yang tercatat rapi daripada sesi panjang yang terputus
   di tengah tanpa sempat validasi & update handoff.

### Script pengecekan token (jalankan dari folder `translation_work/`)

```python
import json, re
from collections import Counter

TOKEN = re.compile(r'#[A-Za-z]|#[0-9a-fA-F]{6}|%s|%d|\{[^}]*\}|<[^>]*>')

src = {}
with open('unique_strings.jsonl', encoding='utf-8') as f:
    for line in f:
        d = json.loads(line)
        src[d['idx']] = d['v']

mismatches = []
with open('translations.jsonl', encoding='utf-8') as f:
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

## Setelah semua (atau cukup banyak) selesai: expand ke strings.jsonl penuh

Belum dibuat scriptnya. Rencana: baca `unique_strings.jsonl` + `translations.jsonl`
untuk bikin dict `src_v -> translated_v`, lalu stream `../strings.jsonl` asli, replace
field `v` sesuai dict (skip/biarkan asli kalau belum diterjemahkan — partial translation
didukung oleh `patch` sesuai CLAUDE.md), tulis ke `strings.translated.jsonl`. Baru lanjut
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
