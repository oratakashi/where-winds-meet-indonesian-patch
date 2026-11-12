#!/usr/bin/env python3
"""
make_manifest.py — tulis manifest.json untuk hasil patch_all.py.

    python tools/make_manifest.py [--profile cn] [--outdir patched] [--commit SHA]

Untuk tiap varian profil yang ada di --outdir, catat sha256 + ukuran file ORIGINAL
(binary sumber di repo) dan file PATCHED (hasil patch_all.py). Manifest ini ikut
dirilis (release `latest` / `cn-latest`) dan dibaca app Patch Guard (app/) untuk
memutuskan, hanya dari hash file yang terpasang di folder game:

  = patched_sha256   -> sudah terpasang, tidak ada yang perlu dilakukan
  = original_sha256  -> tertimpa patcher NetEase, aman dipasang ulang
  selain itu         -> versi game lain; JANGAN pasang build ini di atasnya

Jadi `original_sha256` adalah satu-satunya bukti bahwa sebuah build cocok dengan
file game tertentu — jangan pernah diisi dari sumber selain binary yang benar-benar
dipakai patch_all.py.
"""
import argparse, datetime, hashlib, json, os, subprocess, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, "tools"))

import profiles

SCHEMA = 1
NAME = "manifest.json"


def digest(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest(), os.path.getsize(path)


def current_commit():
    sha = os.environ.get("GITHUB_SHA")
    if sha:
        return sha
    try:
        return subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT,
                                       stderr=subprocess.DEVNULL).decode().strip()
    except (OSError, subprocess.CalledProcessError):
        return ""


def build(profile, outdir, commit):
    P = profiles.get(profile)
    files = []
    for name in P.files:
        src, out = P.path(name), os.path.join(outdir, name)
        if not (os.path.exists(src) and os.path.exists(out)):
            continue                    # varian yang tidak dikirim (mis. _mobile)
        o_sha, o_size = digest(src)
        p_sha, p_size = digest(out)
        files.append({"name": name,
                      "original_sha256": o_sha, "original_size": o_size,
                      "patched_sha256": p_sha, "patched_size": p_size})
    return {"schema": SCHEMA, "profile": profile, "commit": commit,
            "built_at": datetime.datetime.now(datetime.timezone.utc)
                        .strftime("%Y-%m-%dT%H:%M:%SZ"),
            "files": files}


def main():
    ap = argparse.ArgumentParser()
    profiles.add_argument(ap)
    ap.add_argument("--outdir", help="folder hasil patch_all.py (default: patched_dir profil)")
    ap.add_argument("--commit", help="default: $GITHUB_SHA atau git rev-parse HEAD")
    a = ap.parse_args()
    outdir = a.outdir or profiles.get(a.profile).patched_dir
    m = build(a.profile, outdir, a.commit if a.commit is not None else current_commit())
    if not m["files"]:
        sys.exit("tidak ada file hasil patch di %s, jalankan tools/patch_all.py dulu" % outdir)
    path = os.path.join(outdir, NAME)
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(m, f, indent=2)
        f.write("\n")
    for e in m["files"]:
        print("%-36s original=%s  patched=%s" % (e["name"], e["original_sha256"][:12],
                                                 e["patched_sha256"][:12]))
    print("->", path)


if __name__ == "__main__":
    main()
