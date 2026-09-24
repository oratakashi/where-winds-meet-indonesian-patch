#!/usr/bin/env python3
"""
Claude Code Stop hook (didaftarkan di .claude/settings.json).

Kalau sesi ini mengubah locale/*.jsonl (perubahan belum di-commit) dan blok
progress di Obsidian-Vault/progress/Current-Status.md sudah basi, Claude tidak
boleh mengakhiri turn: dia diminta menjalankan `tools/progress.py --write` dan
melaporkan overall progress ke user dulu.

Tidak ada perubahan di locale/ -> langsung lolos (tanpa biaya ~4 detik).
"""
import json, os, subprocess, sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(ROOT, "tools"))


def main():
    try:
        payload = json.load(sys.stdin)
    except ValueError:
        payload = {}
    # Sudah diblok sekali di turn ini: jangan mengunci Claude dalam loop.
    if payload.get("stop_hook_active"):
        return 0

    changed = subprocess.run(
        ["git", "status", "--porcelain", "--", "locale"],
        cwd=ROOT, capture_output=True, text=True).stdout.strip()
    if not changed:
        return 0

    import progress
    if not progress.is_stale():
        return 0

    print(json.dumps({
        "decision": "block",
        "reason": (
            "locale/ changed this session but the generated progress block in "
            "Obsidian-Vault/progress/Current-Status.md is stale. Run "
            "`python tools/progress.py --write`, then include its overall line "
            "(unique strings done/total/% and in-game coverage %) in the "
            "Current-Status/Session-History write-up and in your final message "
            "to the user (Resume-Procedure step 8)."),
    }))
    return 0


if __name__ == "__main__":
    sys.exit(main())
