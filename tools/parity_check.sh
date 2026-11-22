#!/bin/sh
# parity_check.sh — run the Rust `wwm` binary and the Python scripts on the real repo
# data and check that they agree. Keeps the Python failsafe honest: CI runs this on
# every change to rust/ or tools/ (rust-ci.yml, job `parity`).
#
#   sh tools/parity_check.sh            # needs rust/target/release/wwm[.exe] (or WWM_BIN)
#
# Compared: dump (base + _diff), qa --locale (global + cn), progress block (global + cn),
# expand, rebuild-unique --dry-run, patch-all (decoded contents), manifest (original
# hashes + file list). Prints the time of each toolchain per step. Exit 1 on any mismatch.
set -u

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT"
BIN=${WWM_BIN:-}
[ -n "$BIN" ] || for b in rust/target/release/wwm rust/target/release/wwm.exe; do [ -f "$b" ] && BIN=$b; done
[ -n "$BIN" ] || { echo "parity: binary Rust tidak ada — cargo build --release di rust/" >&2; exit 2; }
PY=python3
command -v python3 >/dev/null 2>&1 && python3 -c "" 2>/dev/null || PY=python

T=$(mktemp -d)
trap 'rm -rf "$T"' EXIT
FAIL=0

now() { $PY -c "import time; print(time.time())"; }
# run NAME TOOL CMD... : stdout (CR stripped) -> $T/NAME.TOOL, prints the elapsed time
run() {
    name=$1 tool=$2
    shift 2
    s=$(now)
    "$@" 2>&1 | tr -d '\r' > "$T/$name.$tool"
    e=$(now)
    $PY -c "print('  %-6s %-22s %6.2fs' % ('$tool', '$name', $e - $s))"
}
same() {
    if cmp -s "$1" "$2"; then
        echo "OK    $3"
    else
        echo "DIFF  $3"
        diff "$1" "$2" | head -n 10
        FAIL=1
    fi
}
step() { # step NAME "rust args" "python cmd" : compare stdout
    run "$1" rust sh -c "$BIN $2"
    run "$1" py sh -c "$PY $3"
    same "$T/$1.rust" "$T/$1.py" "$1"
}

echo "== dump"
for f in translate_words_map_en translate_words_map_en_diff; do
    run "dump-$f" rust "$BIN" dump "$f" "$T/$f.rust.jsonl"
    run "dump-$f" py $PY wwm_locmap.py dump "$f" "$T/$f.py.jsonl"
    same "$T/$f.rust.jsonl" "$T/$f.py.jsonl" "dump $f"
done
# progress.py / expand_locale.py / rebuild_unique_strings.py read the root dumps
[ -f strings.jsonl ] || cp "$T/translate_words_map_en.py.jsonl" strings.jsonl
[ -f strings_diff.jsonl ] || cp "$T/translate_words_map_en_diff.py.jsonl" strings_diff.jsonl

echo "== qa / progress / rebuild-unique"
step qa-global "qa --locale 'locale/*.jsonl'" "tools/qa_check.py --locale 'locale/*.jsonl'"
step qa-cn "qa --profile cn --locale 'cn/locale/*.jsonl'" "tools/qa_check.py --profile cn --locale 'cn/locale/*.jsonl'"
step progress-global "progress" "tools/progress.py"
step progress-cn "progress --profile cn" "tools/progress.py --profile cn"
step rebuild-unique "rebuild-unique --dry-run" "tools/rebuild_unique_strings.py --dry-run"

echo "== expand"
run expand rust "$BIN" expand --out "$T/expand.rust.jsonl"
run expand py $PY tools/expand_locale.py --out "$T/expand.py.jsonl"
same "$T/expand.rust.jsonl" "$T/expand.py.jsonl" "expand"

echo "== patch-all (--level 3) + manifest"
run patch-all rust "$BIN" patch-all --outdir "$T/p-rust" --level 3
run patch-all py $PY tools/patch_all.py --outdir "$T/p-py" --level 3
for f in translate_words_map_en translate_words_map_en_diff; do
    "$BIN" dump "$T/p-rust/$f" "$T/pd.rust" > /dev/null
    "$BIN" dump "$T/p-py/$f" "$T/pd.py" > /dev/null
    same "$T/pd.rust" "$T/pd.py" "patch-all $f (decoded)"
done
run manifest rust "$BIN" manifest --outdir "$T/p-rust" --archive "$T/rust.zip" --commit parity
run manifest py $PY tools/make_manifest.py --outdir "$T/p-py" --archive "$T/py.zip" --commit parity
for tool in rust py; do
    $PY -c "
import json, sys
m = json.load(open(sys.argv[1], encoding='utf-8'))
print(m['schema'], m['profile'], m['commit'], sorted(m))
for f in m['files']:
    print(f['name'], f['original_sha256'], f['original_size'])
" "$T/p-$tool/manifest.json" > "$T/manifest.$tool.txt"
done
same "$T/manifest.rust.txt" "$T/manifest.py.txt" "manifest (schema, original hashes, files)"
$PY -c "
import zipfile, sys
z = zipfile.ZipFile(sys.argv[1])
assert all(i.compress_type == zipfile.ZIP_STORED and i.date_time == (1980, 1, 1, 0, 0, 0) for i in z.infolist())
print('OK    release zip (Rust) readable, stored, fixed timestamp:', ', '.join(z.namelist()))
" "$T/rust.zip" || FAIL=1

if [ $FAIL = 0 ]; then echo "parity: semua cocok"; else echo "parity: ADA PERBEDAAN"; fi
exit $FAIL
