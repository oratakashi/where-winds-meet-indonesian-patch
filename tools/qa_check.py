#!/usr/bin/env python3
"""
qa_check.py — validator untuk hasil terjemahan sebelum di-repack.

    # mode dump: bandingkan dump asli vs dump terjemahan (kunci = b/s)
    python tools/qa_check.py original.jsonl translated.jsonl [--report qa.jsonl]

    # mode locale: validasi file per-idx terhadap translation_work/unique_strings.jsonl
    python tools/qa_check.py --locale locale/phase6.jsonl [locale/update1.jsonl ...]
    python tools/qa_check.py --locale "locale/*.jsonl"

Memeriksa tiga kelas bug yang paling sering muncul di pack terjemahan MT/LLM:

  1. PROMPT_LEAK   instruksi system prompt ikut tertulis ke dalam nilai
  2. MARKUP        multiset token format engine (#Y ... #E, #aabbcc, %s, %d, {0},
                   <Tag|id|#C|n>) tidak sama antara sumber dan terjemahan
                   (jumlah per token dicek, urutan TIDAK)
  3. EMPTY         sumber tidak kosong tapi terjemahan kosong

Mode --locale juga memeriksa idx: harus ada di unique_strings.jsonl, berurutan
tanpa celah di dalam tiap file, dan tidak duplikat antar file.

Exit code 1 jika ada temuan, jadi bisa dipasang di CI / pre-commit.
"""
import argparse, glob, json, os, re, sys
from collections import Counter

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
UNIQUE_PATH = os.path.join(ROOT, "translation_work", "unique_strings.jsonl")

# Token format milik Messiah/WWM — regex kanonik tunggal untuk seluruh proyek.
# Urutan alternasi penting:
#   #aabbcc     warna hex 6 digit — HARUS dicoba sebelum kode 1 huruf, kalau tidak
#               `#e9a35f...` terbaca sebagai `#e` dan hex yang rusak lolos
#   #E #Y #H ...   kode format 1 huruf (termasuk penutup #E)
#   %s %d       printf placeholder
#   {0} {name}  placeholder kurung kurawal
#   <...>       tag stat `<Name|id|#C|n>` atau tag polos — seluruh isinya satu token
TOKEN = re.compile(r'#[0-9a-fA-F]{6}|#[A-Za-z]|%[sd]|\{[^}]*\}|<[^>]*>')

# Tanda tangan kebocoran prompt. Tambahkan sendiri kalau ketemu varian baru.
LEAK_SIGNATURES = (
    'Penanda format',
    'Penanda pemformatan',
    'Formatting markers',
    'tidak perlu diterjemahkan',
    'Jangan terjemahkan konten',
    'Jangan menerjemahkan konten',
    '<P_*>',
)


def check_pair(s, t):
    """Kembalikan jenis temuan untuk pasangan sumber/terjemahan, atau None."""
    if any(p in t for p in LEAK_SIGNATURES):
        return 'PROMPT_LEAK'
    if s.strip() and not t.strip():
        return 'EMPTY'
    if Counter(TOKEN.findall(s)) != Counter(TOKEN.findall(t)):
        return 'MARKUP'
    return None


def iter_jsonl(path):
    with open(path, encoding='utf-8') as f:
        for line in f:
            if line.strip():
                yield json.loads(line)


def load(path):
    d = {}
    for r in iter_jsonl(path):
        if r.get('deleted'):            # tombstone dari *_diff, bukan teks
            continue
        d[(r['b'], r['s'])] = r['v']
    return d


def load_unique():
    return {r['idx']: r['v'] for r in iter_jsonl(UNIQUE_PATH)}


def summarize(findings, total, report):
    stats = Counter(kind for _, kind, _, _ in findings)
    print('diperiksa : %d entri' % total)
    for kind in ('PROMPT_LEAK', 'MARKUP', 'EMPTY', 'IDX'):
        n = stats[kind]
        if n or kind != 'IDX':
            print('%-12s: %6d  (%.3f%%)' % (kind, n, 100 * n / total if total else 0))
    if report and findings:
        with open(report, 'w', encoding='utf-8') as f:
            for key, kind, s, t in findings:
                f.write(json.dumps(dict(key, kind=kind, src=s, dst=t),
                                   ensure_ascii=False) + '\n')
        print('laporan ditulis:', report)
    elif findings:
        for key, kind, s, t in findings[:20]:
            print('  %s %s\n    src: %s\n    dst: %s' % (kind, key, s, t))
        if len(findings) > 20:
            print('  ... %d lagi (pakai --report untuk daftar lengkap)' % (len(findings) - 20))
    return 1 if findings else 0


def run_dump(a):
    src, dst = load(a.original), load(a.translated)
    missing = set(dst) - set(src)
    if missing:
        print('FATAL: %d entri tidak ada di file original '
              '(block/slot salah?)' % len(missing))
        return 2
    findings = []
    for k, s in src.items():
        t = dst.get(k)
        if t is None:                       # tidak diterjemahkan, itu wajar
            continue
        kind = check_pair(s, t)
        if kind:
            findings.append(({'b': k[0], 's': k[1]}, kind, s, t))
    return summarize(findings, len(dst), a.report)


def run_locale(a):
    paths = sorted({p for pat in a.locale for p in (glob.glob(pat) or [pat])})
    src = load_unique()
    findings, seen, total = [], {}, 0
    for path in paths:
        name = os.path.basename(path)
        prev = None
        for r in iter_jsonl(path):
            total += 1
            i, t = r['idx'], r['v']
            if i in seen:
                findings.append(({'idx': i, 'file': name}, 'IDX', 'duplikat, juga ada di ' + seen[i], ''))
            seen[i] = name
            if prev is not None and i != prev + 1:
                findings.append(({'idx': i, 'file': name}, 'IDX', 'tidak berurutan setelah idx %d' % prev, ''))
            prev = i
            s = src.get(i)
            if s is None:
                findings.append(({'idx': i, 'file': name}, 'IDX', 'tidak ada di unique_strings.jsonl', ''))
                continue
            kind = check_pair(s, t)
            if kind:
                findings.append(({'idx': i, 'file': name}, kind, s, t))
    print('file:', ', '.join(os.path.basename(p) for p in paths))
    return summarize(findings, total, a.report)


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('original', nargs='?')
    ap.add_argument('translated', nargs='?')
    ap.add_argument('--locale', nargs='+', metavar='FILE_OR_GLOB',
                    help='validasi file locale per-idx terhadap unique_strings.jsonl')
    ap.add_argument('--report')
    a = ap.parse_args()
    if a.locale:
        return run_locale(a)
    if not (a.original and a.translated):
        ap.error('butuh original.jsonl dan translated.jsonl, atau --locale')
    return run_dump(a)


if __name__ == '__main__':
    sys.exit(main())
