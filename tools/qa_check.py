#!/usr/bin/env python3
"""
qa_check.py — validator untuk hasil terjemahan sebelum di-repack.

    python tools/qa_check.py original.jsonl translated.jsonl [--report qa.jsonl]

Memeriksa tiga kelas bug yang paling sering muncul di pack terjemahan MT/LLM:

  1. PROMPT_LEAK   instruksi system prompt ikut tertulis ke dalam nilai
  2. MARKUP        token format engine (#Y ... #E, #E, #aabbcc, %s, %d, {0})
                   jumlahnya tidak sama antara sumber dan terjemahan
  3. EMPTY         sumber tidak kosong tapi terjemahan kosong

Exit code 1 jika ada temuan, jadi bisa dipasang di CI / pre-commit.
"""
import argparse, json, re, sys
from collections import Counter

# Token format milik Messiah/WWM. Urutan alternasi penting:
#   #E          penutup format
#   #aabbcc     warna hex 6 digit
#   #Y #N #R #J #G #H ...   kode format 1 huruf
#   %s %d       printf placeholder
#   {0} {1}     placeholder index
TOKEN = re.compile(r'#E|#[0-9a-fA-F]{6}|#[A-Za-z]|%[sd]|\{\d+\}')

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


def load(path):
    d = {}
    with open(path, encoding='utf-8') as f:
        for line in f:
            if not line.strip():
                continue
            r = json.loads(line)
            d[(r['b'], r['s'])] = r['v']
    return d


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('original')
    ap.add_argument('translated')
    ap.add_argument('--report')
    a = ap.parse_args()

    src, dst = load(a.original), load(a.translated)
    missing = set(dst) - set(src)
    if missing:
        print('FATAL: %d entri tidak ada di file original '
              '(block/slot salah?)' % len(missing))
        return 2

    findings, stats = [], Counter()
    for k, s in src.items():
        t = dst.get(k)
        if t is None:                       # tidak diterjemahkan, itu wajar
            continue
        if any(p in t for p in LEAK_SIGNATURES):
            findings.append((k, 'PROMPT_LEAK', s, t))
            stats['PROMPT_LEAK'] += 1
            continue
        if s.strip() and not t.strip():
            findings.append((k, 'EMPTY', s, t))
            stats['EMPTY'] += 1
            continue
        if sorted(TOKEN.findall(s)) != sorted(TOKEN.findall(t)):
            findings.append((k, 'MARKUP', s, t))
            stats['MARKUP'] += 1

    total = len(dst)
    print('diperiksa : %d entri' % total)
    for kind in ('PROMPT_LEAK', 'MARKUP', 'EMPTY'):
        n = stats[kind]
        print('%-12s: %6d  (%.3f%%)' % (kind, n, 100 * n / total if total else 0))

    if a.report and findings:
        with open(a.report, 'w', encoding='utf-8') as f:
            for (b, s_), kind, s, t in findings:
                f.write(json.dumps({'b': b, 's': s_, 'kind': kind,
                                    'src': s, 'dst': t},
                                   ensure_ascii=False) + '\n')
        print('laporan ditulis:', a.report)

    return 1 if findings else 0


if __name__ == '__main__':
    sys.exit(main())
