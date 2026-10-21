#!/usr/bin/env python3
"""
find_dict_in_dump.py — recover the plaintext zstd dictionaries of WWM China's
container-v2 files from a memory dump of the running game, and derive the
dictionary keystream from them.

    python cn/tools/find_dict_in_dump.py <yysls.DMP> [more.DMP ...]
        [--v2-dir D:\\yysls\\yysls_fast\\LocalData\\Patch\\HD\\v2] [--out cn/work]
    python cn/tools/find_dict_in_dump.py --pid <yysls.exe PID> [--v2-dir ...]
        # scans the running game directly; the CN client runs elevated, so this
        # has to be started from an Administrator terminal

Background (see Obsidian-Vault/knowladge/Format-Spec.md, "Container v2"):
a mode-6 v2 container stores its zstd dictionary encrypted as
`plain XOR keystream`, and every file in the client uses the *same* keystream.
The game keeps the decrypted dictionary in memory (ZSTD_createDDict copies it),
so a process dump contains `37 A4 30 EC <dictID>` followed by the plaintext.
Any recovered dictionary yields `keystream[:len] = encrypted XOR plain`, which
then decrypts every other file's dictionary of up to that length.

How to make the dump (no debugger / OpenProcess needed): start the CN client,
wait for the login screen, then Task Manager -> Details -> right-click the game
process (yysls.exe) -> "Create memory dump file". Task Manager prints where it
wrote the .DMP (usually %LOCALAPPDATA%\\Temp).

Outputs (in --out, gitignored):
  dict_<dictID>.zdict   every validated plaintext dictionary
  keystream_v2.bin      the longest keystream all recovered dictionaries agree on
"""
import argparse, glob, mmap, os, struct, sys

try:
    import zstandard as zstd
except ImportError:
    sys.exit("butuh: pip install zstandard")

HERE = os.path.dirname(os.path.abspath(__file__))
CN = os.path.dirname(HERE)
DICT_MAGIC = b'\x37\xa4\x30\xec'
CONTAINER_MAGIC = b'\xef\xbe\xad\xde'


def load_v2(path):
    """Return dict info for a mode-6 v2 container, or None for anything else."""
    with open(path, 'rb') as f:
        d = f.read()
    if len(d) < 16 or d[:4] != CONTAINER_MAGIC:
        return None
    ver, mode, ds = struct.unpack_from('<III', d, 4)
    if ver != 2 or mode != 6:
        return None
    q = 16 + ds
    cnt, = struct.unpack_from('<Q', d, q)
    offs = struct.unpack_from('<%dI' % cnt, d, q + 8)
    base = q + 8 + cnt * 4
    blocks = []
    for i in range(min(cnt, 4)):                 # a few blocks to validate with
        s = base + (offs[i - 1] if i else 0)
        codec, cs, us = struct.unpack_from('<BII', d, s)
        blocks.append((d[s + 9:s + 9 + cs], us))
    frame = blocks[0][0]
    if frame[4] & 3 != 3:
        raise ValueError("%s: first frame has no 4-byte dictID" % path)
    return {"path": path, "enc": d[16:16 + ds], "id": frame[5:9], "blocks": blocks}


def validates(plain, info):
    try:
        zd = zstd.ZstdCompressionDict(plain, dict_type=zstd.DICT_TYPE_FULLDICT)
        dc = zstd.ZstdDecompressor(dict_data=zd)
        for frame, us in info["blocks"]:
            if len(dc.decompress(frame, max_output_size=us)) != us:
                return False
        return True
    except zstd.ZstdError:
        return False


def scan(buf, base, read, infos, found):
    """Find dictionaries in buf (bytes or mmap) whose first byte sits at address
    `base`; read(addr, n) fetches n bytes at an absolute address."""
    pos, hits = buf.find(DICT_MAGIC), 0
    while pos != -1:
        hits += 1
        did = bytes(read(base + pos + 4, 4))
        info = infos.get(did)
        if info and did not in found:
            plain = bytes(read(base + pos, len(info["enc"])) or b'')
            if len(plain) == len(info["enc"]) and validates(plain, info):
                found[did] = plain
                print("  OK  %s %-36s @ 0x%x" % (did.hex(), os.path.basename(info["path"]),
                                                  base + pos))
        pos = buf.find(DICT_MAGIC, pos + 1)
    return hits


def iter_process(pid, chunk=16 << 20):
    """Yield (address, bytes, read) for every committed readable region of a
    live process. Read-only: PROCESS_QUERY_INFORMATION | PROCESS_VM_READ."""
    import ctypes
    from ctypes import wintypes

    class MBI(ctypes.Structure):
        _fields_ = [("BaseAddress", ctypes.c_void_p), ("AllocationBase", ctypes.c_void_p),
                    ("AllocationProtect", wintypes.DWORD), ("PartitionId", wintypes.WORD),
                    ("RegionSize", ctypes.c_size_t), ("State", wintypes.DWORD),
                    ("Protect", wintypes.DWORD), ("Type", wintypes.DWORD)]

    k = ctypes.WinDLL('kernel32', use_last_error=True)
    k.OpenProcess.restype = wintypes.HANDLE
    k.VirtualQueryEx.argtypes = [wintypes.HANDLE, ctypes.c_void_p, ctypes.POINTER(MBI), ctypes.c_size_t]
    k.VirtualQueryEx.restype = ctypes.c_size_t
    k.ReadProcessMemory.argtypes = [wintypes.HANDLE, ctypes.c_void_p, ctypes.c_void_p,
                                    ctypes.c_size_t, ctypes.POINTER(ctypes.c_size_t)]
    h = k.OpenProcess(0x0400 | 0x0010, False, pid)
    if not h:
        sys.exit("OpenProcess(%d) gagal (error %d) — jalankan dari terminal Administrator"
                 % (pid, ctypes.get_last_error()))

    def read(addr, n):
        buf = ctypes.create_string_buffer(n); got = ctypes.c_size_t()
        k.ReadProcessMemory(h, ctypes.c_void_p(addr), buf, n, ctypes.byref(got))
        return buf.raw[:got.value]

    MEM_COMMIT, PAGE_NOACCESS, PAGE_GUARD = 0x1000, 0x01, 0x100
    mbi, addr, total = MBI(), 0, 0
    try:
        while k.VirtualQueryEx(h, ctypes.c_void_p(addr), ctypes.byref(mbi), ctypes.sizeof(mbi)):
            start, size = mbi.BaseAddress or 0, mbi.RegionSize
            if (mbi.State == MEM_COMMIT and not mbi.Protect & (PAGE_NOACCESS | PAGE_GUARD)
                    and mbi.Protect):
                for off in range(0, size, chunk):
                    n = min(chunk + 8, size - off)      # +8: magic+dictID across chunks
                    data = read(start + off, n)
                    if data:
                        total += len(data)
                        yield start + off, data, read
            addr = start + size
            if addr >= 1 << 47:
                break
    finally:
        k.CloseHandle(h)
        print("dibaca: %.1f GB memori proses" % (total / 2**30))


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('dumps', nargs='*')
    ap.add_argument('--pid', type=int,
                    help='scan the live process instead of / as well as dump files '
                         '(read-only; needs an elevated terminal if the game runs as admin)')
    ap.add_argument('--v2-dir', action='append', default=[],
                    help='extra folder(s) with v2 containers (e.g. the client\'s '
                         'LocalData\\Patch\\HD\\v2) to look for as well')
    ap.add_argument('--out', default=os.path.join(CN, 'work'))
    a = ap.parse_args()
    if not a.dumps and not a.pid:
        ap.error("beri file dump dan/atau --pid")

    cands = glob.glob(os.path.join(CN, 'translate_words_map_*'))
    for d in a.v2_dir:
        cands += glob.glob(os.path.join(d, '*')) + glob.glob(os.path.join(d, 'locale', '*'))
    infos = {}
    for p in cands:
        if os.path.isfile(p):
            i = load_v2(p)
            if i:
                infos.setdefault(i["id"], i)
    if not infos:
        sys.exit("tidak ada container v2 mode 6 yang ditemukan")
    print("dictionary dicari: %d (%s)" % (len(infos), ", ".join(
        "%s=%s" % (os.path.basename(i["path"]), k.hex()) for k, i in sorted(infos.items()))))

    os.makedirs(a.out, exist_ok=True)
    found = {}
    if a.pid:
        hits = 0
        for base, data, read in iter_process(a.pid):
            hits += scan(data, base, read, infos, found)
        print("pid %d: %d kemunculan magic kamus" % (a.pid, hits))
    for dump in a.dumps:
        with open(dump, 'rb') as f, mmap.mmap(f.fileno(), 0, access=mmap.ACCESS_READ) as m:
            hits = scan(m, 0, lambda addr, n: m[addr:addr + n], infos, found)
            print("%s: %d kemunculan magic kamus" % (dump, hits))

    if not found:
        sys.exit("tidak ada kamus yang ditemukan/valid — pastikan game sudah di layar "
                 "login (bukan launcher)")
    ks = b''
    for did, plain in sorted(found.items(), key=lambda kv: -len(kv[1])):
        with open(os.path.join(a.out, 'dict_%s.zdict' % did.hex()), 'wb') as f:
            f.write(plain)
        part = bytes(x ^ y for x, y in zip(infos[did]["enc"], plain))
        n = min(len(ks), len(part))
        if ks[:n] != part[:n]:
            sys.exit("keystream dari %s tidak cocok dengan kamus lain — asumsi keystream "
                     "tunggal salah, laporkan" % did.hex())
        if len(part) > len(ks):
            ks = part
    with open(os.path.join(a.out, 'keystream_v2.bin'), 'wb') as f:
        f.write(ks)
    print("kamus ditemukan: %d/%d, keystream: %d byte -> %s" % (
        len(found), len(infos), len(ks), os.path.join(a.out, 'keystream_v2.bin')))
    short = [os.path.basename(i["path"]) for i in infos.values() if len(i["enc"]) > len(ks)]
    if short:
        print("PERHATIAN: keystream lebih pendek dari kamus milik:", ", ".join(short))


if __name__ == '__main__':
    main()
