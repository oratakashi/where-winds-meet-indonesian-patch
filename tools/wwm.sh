#!/bin/sh
# wwm.sh — one entry point for the repo tooling, Rust (`wwm` binary) or Python.
#
#   sh tools/wwm.sh <subcommand> [args...]      # same args as the Python script
#   sh tools/wwm.sh toolchain                   # print which toolchain would run
#
# Subcommand -> Python fallback:
#   info | dump | patch      wwm_locmap.py <cmd>
#   qa                       tools/qa_check.py
#   expand                   tools/expand_locale.py
#   rebuild-unique           tools/rebuild_unique_strings.py
#   progress                 tools/progress.py
#   patch-all                tools/patch_all.py
#   manifest                 tools/make_manifest.py
#   build-unique             translation_work/build_unique.py
#   compare-update           .claude/skills/wwm-game-update/scripts/compare_update.py
#   hook stop                tools/hooks/claude_stop_check.py
#   cn term-lookup | name-check | find-dict    cn/tools/*.py
#
# Which toolchain runs:
#   WWM_TOOLCHAIN=python     always Python (failsafe)
#   WWM_TOOLCHAIN=rust       always Rust; error if the binary is missing (CI sets this)
#   WWM_TOOLCHAIN=auto/unset Rust when rust/target/release/wwm[.exe] exists, else Python.
#                            A binary older than rust/ sources is rebuilt with cargo first
#                            (WWM_NO_BUILD=1: skip the build and use Python instead).
#   WWM_BIN=/path/to/wwm     use this binary as is (no staleness check)
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
MODE=${WWM_TOOLCHAIN:-auto}

warn() { echo "wwm.sh: $*" >&2; }

find_bin() {
    if [ -n "${WWM_BIN:-}" ]; then
        [ -x "$WWM_BIN" ] || [ -f "$WWM_BIN" ] || { warn "WWM_BIN=$WWM_BIN tidak ada"; return 1; }
        BIN=$WWM_BIN
        return 0
    fi
    for b in "$ROOT/rust/target/release/wwm" "$ROOT/rust/target/release/wwm.exe"; do
        if [ -f "$b" ]; then BIN=$b; break; fi
    done
    if [ -z "${BIN:-}" ]; then
        [ "$MODE" = rust ] && warn "binary Rust belum di-build (cargo build --release di rust/)"
        return 1
    fi
    # rebuild when any Rust source is newer than the binary
    stale=$(find "$ROOT/rust/crates" "$ROOT/rust/Cargo.toml" "$ROOT/rust/Cargo.lock" \
                 -newer "$BIN" -type f 2>/dev/null | head -n 1)
    [ -z "$stale" ] && return 0
    if [ -n "${WWM_NO_BUILD:-}" ]; then
        warn "binary Rust basi ($stale lebih baru), WWM_NO_BUILD diset"
        [ "$MODE" = rust ] && return 0
        return 1
    fi
    if command -v cargo >/dev/null 2>&1; then
        warn "binary Rust basi — cargo build --release ..."
        if cargo build --release --quiet --manifest-path "$ROOT/rust/Cargo.toml" >&2; then
            return 0
        fi
        warn "cargo build gagal"
    else
        warn "binary Rust basi dan cargo tidak ada"
    fi
    [ "$MODE" = rust ] || warn "pakai Python"
    return 1
}

python_cmd() {
    for p in python3 python; do
        if command -v "$p" >/dev/null 2>&1 && "$p" -c "import sys; sys.exit(sys.version_info < (3, 8))" 2>/dev/null; then
            PY=$p
            return 0
        fi
    done
    warn "python tidak ditemukan"
    exit 127
}

run_python() {
    python_cmd
    cmd=$1
    shift
    case "$cmd" in
        info|dump|patch)  exec "$PY" "$ROOT/wwm_locmap.py" "$cmd" "$@" ;;
        qa)               exec "$PY" "$ROOT/tools/qa_check.py" "$@" ;;
        expand)           exec "$PY" "$ROOT/tools/expand_locale.py" "$@" ;;
        rebuild-unique)   exec "$PY" "$ROOT/tools/rebuild_unique_strings.py" "$@" ;;
        progress)         exec "$PY" "$ROOT/tools/progress.py" "$@" ;;
        patch-all)        exec "$PY" "$ROOT/tools/patch_all.py" "$@" ;;
        manifest)         exec "$PY" "$ROOT/tools/make_manifest.py" "$@" ;;
        build-unique)     exec "$PY" "$ROOT/translation_work/build_unique.py" "$@" ;;
        compare-update)   exec "$PY" "$ROOT/.claude/skills/wwm-game-update/scripts/compare_update.py" "$@" ;;
        hook)
            [ "${1:-}" = stop ] || { warn "hook tak dikenal: ${1:-}"; exit 2; }
            shift
            exec "$PY" "$ROOT/tools/hooks/claude_stop_check.py" "$@" ;;
        cn)
            sub=${1:-}
            [ $# -gt 0 ] && shift
            case "$sub" in
                term-lookup) exec "$PY" "$ROOT/cn/tools/term_lookup.py" "$@" ;;
                name-check)  exec "$PY" "$ROOT/cn/tools/name_check.py" "$@" ;;
                find-dict)   exec "$PY" "$ROOT/cn/tools/find_dict_in_dump.py" "$@" ;;
                *) warn "subcommand cn tak dikenal: $sub"; exit 2 ;;
            esac ;;
        *) warn "subcommand tak dikenal: $cmd"; exit 2 ;;
    esac
}

[ $# -gt 0 ] || { sed -n '2,30p' "$0" >&2; exit 2; }

BIN=
use_rust=0
case "$MODE" in
    python) ;;
    rust)   find_bin || exit 1; use_rust=1 ;;
    auto)   if find_bin; then use_rust=1; fi ;;
    *)      warn "WWM_TOOLCHAIN tak dikenal: $MODE (rust|python|auto)"; exit 2 ;;
esac

if [ "$1" = toolchain ]; then
    if [ $use_rust = 1 ]; then echo "rust $BIN"; else echo "python"; fi
    exit 0
fi

if [ $use_rust = 1 ]; then
    exec "$BIN" "$@"
fi
run_python "$@"
