#!/usr/bin/env python3
"""
Claude Code Stop hook (didaftarkan di .claude/settings.json).

Kalau sesi ini mengubah locale/*.jsonl atau cn/locale/*.jsonl (perubahan belum
di-commit) dan blok progress profil itu (Obsidian-Vault/progress/Current-Status.md
atau Obsidian-Vault/cn/progress/CN-Current-Status.md) sudah basi, Claude tidak
boleh mengakhiri turn: dia diminta menjalankan `tools/progress.py [--profile cn]
--write` dan melaporkan overall progress ke user dulu.

Tidak ada perubahan di folder locale -> langsung lolos (tanpa biaya ~4 detik).
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

    import progress, profiles
    for name, folder in (("global", "locale"), ("cn", "cn/locale")):
        changed = subprocess.run(
            ["git", "status", "--porcelain", "--", folder],
            cwd=ROOT, capture_output=True, text=True).stdout.strip()
        if not changed or not progress.is_stale(name):
            continue
        P = profiles.get(name)
        flag = "" if name == "global" else " --profile " + name
        print(json.dumps({
            "decision": "block",
            "reason": (
                "%s/ changed this session but the generated progress block in "
                "%s is stale. Run `python tools/progress.py%s --write`, then include "
                "its overall line (unique strings done/total/%% and in-game coverage %%) "
                "in the status/session write-up and in your final message to the user "
                "(Resume-Procedure step 8)." % (folder, P.rel(P.status), flag)),
        }))
        return 0
    return 0


if __name__ == "__main__":
    sys.exit(main())
