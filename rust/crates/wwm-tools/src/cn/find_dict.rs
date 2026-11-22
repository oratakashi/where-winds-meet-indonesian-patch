//! `wwm cn find-dict` — port of `cn/tools/find_dict_in_dump.py`: recover the plaintext
//! zstd dictionaries of WWM China's container-v2 files from a memory dump (or, on
//! Windows, the live game process, read-only) and derive the shared keystream.
//!
//! The game keeps decrypted dictionaries in memory, so a dump holds `37 A4 30 EC <dictID>`
//! followed by the plaintext; `keystream[:len] = encrypted XOR plain`.

use crate::Code;
use crate::py::glob;
use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use wwm_locmap::{MAGIC, ZDICT_MAGIC, xor};

pub struct Info {
    pub path: String,
    pub enc: Vec<u8>,
    pub id: [u8; 4],
    /// (zstd frame, uncompressed size) of the first few blocks, to validate with
    pub blocks: Vec<(Vec<u8>, usize)>,
}

fn u32_at(d: &[u8], p: usize) -> Option<u32> {
    d.get(p..p + 4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
}

/// Dictionary info of a mode-6 v2 container, or None for anything else.
pub fn load_v2(path: &str) -> Result<Option<Info>> {
    let d = std::fs::read(path)?;
    if d.len() < 16 || u32_at(&d, 0) != Some(MAGIC) {
        return Ok(None);
    }
    let (ver, mode, ds) = (
        u32_at(&d, 4).unwrap(),
        u32_at(&d, 8).unwrap(),
        u32_at(&d, 12).unwrap() as usize,
    );
    if ver != 2 || mode != 6 {
        return Ok(None);
    }
    let q = 16 + ds;
    let cnt = d
        .get(q..q + 8)
        .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
        .context("terpotong")? as usize;
    let base = q + 8 + cnt * 4;
    let mut blocks = Vec::new();
    for i in 0..cnt.min(4) {
        let s = base
            + if i > 0 {
                u32_at(&d, q + 8 + (i - 1) * 4).unwrap() as usize
            } else {
                0
            };
        let cs = u32_at(&d, s + 1).context("terpotong")? as usize;
        let us = u32_at(&d, s + 5).context("terpotong")? as usize;
        blocks.push((d[s + 9..s + 9 + cs].to_vec(), us));
    }
    let frame = &blocks[0].0;
    if frame[4] & 3 != 3 {
        anyhow::bail!("{path}: first frame has no 4-byte dictID");
    }
    Ok(Some(Info {
        path: path.to_string(),
        enc: d[16..16 + ds].to_vec(),
        id: frame[5..9].try_into().unwrap(),
        blocks,
    }))
}

pub fn validates(plain: &[u8], info: &Info) -> bool {
    let Ok(mut dc) = zstd::bulk::Decompressor::with_dictionary(plain) else {
        return false;
    };
    info.blocks.iter().all(|(frame, us)| {
        dc.decompress(frame, *us)
            .map(|r| r.len() == *us)
            .unwrap_or(false)
    })
}

/// Find dictionaries in `buf` (first byte at address `base`); `read(addr, n)` fetches
/// bytes at an absolute address. Returns the number of magic hits.
pub fn scan<R: FnMut(usize, usize) -> Vec<u8>>(
    buf: &[u8],
    base: usize,
    mut read: R,
    infos: &BTreeMap<[u8; 4], Info>,
    found: &mut BTreeMap<[u8; 4], Vec<u8>>,
) -> usize {
    let mut hits = 0;
    for pos in memchr::memmem::find_iter(buf, &ZDICT_MAGIC) {
        hits += 1;
        let did: Option<[u8; 4]> = read(base + pos + 4, 4).try_into().ok();
        let Some(did) = did else { continue };
        let Some(info) = infos.get(&did) else {
            continue;
        };
        if found.contains_key(&did) {
            continue;
        }
        let plain = read(base + pos, info.enc.len());
        if plain.len() == info.enc.len() && validates(&plain, info) {
            println!(
                "  OK  {} {:<36} @ 0x{:x}",
                hex(&did),
                crate::py::basename(&info.path),
                base + pos
            );
            found.insert(did, plain);
        }
    }
    hits
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

#[cfg(windows)]
fn scan_process(
    pid: u32,
    infos: &BTreeMap<[u8; 4], Info>,
    found: &mut BTreeMap<[u8; 4], Vec<u8>>,
) -> Result<usize> {
    use std::ffi::c_void;
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Diagnostics::Debug::ReadProcessMemory;
    use windows_sys::Win32::System::Memory::{
        MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_GUARD, PAGE_NOACCESS, VirtualQueryEx,
    };
    use windows_sys::Win32::System::Threading::{
        OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
    };

    let h = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid) };
    if h.is_null() {
        crate::exit!(
            "OpenProcess({pid}) gagal (error {}) — jalankan dari terminal Administrator",
            std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
        );
    }
    let read = |addr: usize, n: usize| -> Vec<u8> {
        let mut buf = vec![0u8; n];
        let mut got = 0usize;
        unsafe {
            ReadProcessMemory(
                h,
                addr as *const c_void,
                buf.as_mut_ptr() as *mut c_void,
                n,
                &mut got,
            )
        };
        buf.truncate(got);
        buf
    };
    const CHUNK: usize = 16 << 20;
    let (mut addr, mut total, mut hits) = (0usize, 0usize, 0usize);
    let mut mbi: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
    while unsafe {
        VirtualQueryEx(
            h,
            addr as *const c_void,
            &mut mbi,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        )
    } != 0
    {
        let (start, size) = (mbi.BaseAddress as usize, mbi.RegionSize);
        if mbi.State == MEM_COMMIT
            && mbi.Protect & (PAGE_NOACCESS | PAGE_GUARD) == 0
            && mbi.Protect != 0
        {
            let mut off = 0;
            while off < size {
                let n = (CHUNK + 8).min(size - off); // +8: magic+dictID across chunks
                let data = read(start + off, n);
                if !data.is_empty() {
                    total += data.len();
                    hits += scan(&data, start + off, &read, infos, found);
                }
                off += CHUNK;
            }
        }
        addr = start + size;
        if addr >= 1 << 47 {
            break;
        }
    }
    unsafe { CloseHandle(h) };
    println!(
        "dibaca: {:.1} GB memori proses",
        total as f64 / (1u64 << 30) as f64
    );
    Ok(hits)
}

#[cfg(not(windows))]
fn scan_process(
    _: u32,
    _: &BTreeMap<[u8; 4], Info>,
    _: &mut BTreeMap<[u8; 4], Vec<u8>>,
) -> Result<usize> {
    crate::exit!("--pid hanya didukung di Windows")
}

pub fn run(
    root: &Path,
    dumps: &[String],
    pid: Option<u32>,
    v2_dirs: &[String],
    out: Option<&str>,
) -> Result<Code> {
    if dumps.is_empty() && pid.is_none() {
        crate::exit!("beri file dump dan/atau --pid");
    }
    let cn = root.join("cn");
    let out: PathBuf = out.map(PathBuf::from).unwrap_or_else(|| cn.join("work"));
    let mut cands = glob(&cn.join("translate_words_map_*").to_string_lossy())?;
    for d in v2_dirs {
        cands.extend(glob(&Path::new(d).join("*").to_string_lossy())?);
        cands.extend(glob(
            &Path::new(d).join("locale").join("*").to_string_lossy(),
        )?);
    }
    let mut infos: BTreeMap<[u8; 4], Info> = BTreeMap::new();
    for p in cands {
        if Path::new(&p).is_file()
            && let Some(i) = load_v2(&p)?
        {
            infos.entry(i.id).or_insert(i);
        }
    }
    if infos.is_empty() {
        crate::exit!("tidak ada container v2 mode 6 yang ditemukan");
    }
    let list: Vec<String> = infos
        .iter()
        .map(|(k, i)| format!("{}={}", crate::py::basename(&i.path), hex(k)))
        .collect();
    println!("dictionary dicari: {} ({})", infos.len(), list.join(", "));
    std::fs::create_dir_all(&out)?;
    let mut found = BTreeMap::new();
    if let Some(pid) = pid {
        let hits = scan_process(pid, &infos, &mut found)?;
        println!("pid {pid}: {hits} kemunculan magic kamus");
    }
    for dump in dumps {
        let f = std::fs::File::open(dump).with_context(|| dump.clone())?;
        let m = unsafe { memmap2::Mmap::map(&f)? };
        let hits = scan(
            &m,
            0,
            |a, n| {
                m.get(a..(a + n).min(m.len()))
                    .map(<[u8]>::to_vec)
                    .unwrap_or_default()
            },
            &infos,
            &mut found,
        );
        println!("{dump}: {hits} kemunculan magic kamus");
    }
    if found.is_empty() {
        crate::exit!(
            "tidak ada kamus yang ditemukan/valid — pastikan game sudah di layar login (bukan launcher)"
        );
    }
    let mut ks: Vec<u8> = Vec::new();
    let mut by_len: Vec<(&[u8; 4], &Vec<u8>)> = found.iter().collect();
    by_len.sort_by_key(|(_, p)| std::cmp::Reverse(p.len()));
    for (did, plain) in by_len {
        std::fs::write(out.join(format!("dict_{}.zdict", hex(did))), plain)?;
        let part = xor(&infos[did].enc, plain);
        let n = ks.len().min(part.len());
        if ks[..n] != part[..n] {
            crate::exit!(
                "keystream dari {} tidak cocok dengan kamus lain — asumsi keystream tunggal salah, laporkan",
                hex(did)
            );
        }
        if part.len() > ks.len() {
            ks = part;
        }
    }
    let kp = out.join("keystream_v2.bin");
    std::fs::write(&kp, &ks)?;
    println!(
        "kamus ditemukan: {}/{}, keystream: {} byte -> {}",
        found.len(),
        infos.len(),
        ks.len(),
        kp.display()
    );
    let short: Vec<String> = infos
        .values()
        .filter(|i| i.enc.len() > ks.len())
        .map(|i| crate::py::basename(&i.path))
        .collect();
    if !short.is_empty() {
        println!(
            "PERHATIAN: keystream lebih pendek dari kamus milik: {}",
            short.join(", ")
        );
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wwm_locmap::{Header, MODE_ZSTD_DICT, build_index, build_shard, write_container_bytes};

    #[test]
    fn recovers_dictionary_and_keystream_from_a_dump() {
        let samples: Vec<Vec<u8>> = (0..300)
            .map(|i| {
                format!("entry {i} of the dictionary corpus Kaifeng Hexi {}", i * 3).into_bytes()
            })
            .collect();
        let plain = zstd::dict::from_samples(&samples, 2048).unwrap();
        let ks: Vec<u8> = (0..plain.len()).map(|i| (i * 17 + 3) as u8).collect();
        let hdr = Header {
            version: 2,
            mode: MODE_ZSTD_DICT,
            enc_dict: Some(xor(&plain, &ks)),
            zdict: Some(plain.clone()),
        };
        let blocks = vec![
            build_index(1, 1),
            build_shard(8, 0, &[(1, 9, b"entry 5 of the dictionary corpus")]),
        ];
        let d = tempfile::tempdir().unwrap();
        let root = d.path();
        std::fs::create_dir_all(root.join("cn")).unwrap();
        std::fs::write(
            root.join("cn").join("translate_words_map_zh_cn"),
            write_container_bytes(&hdr, &blocks, 3).unwrap(),
        )
        .unwrap();
        // fake "memory dump": noise + plaintext dictionary + noise
        let mut mem = vec![0x55u8; 1000];
        mem.extend_from_slice(&ZDICT_MAGIC); // a false hit with an unknown dictID
        mem.extend_from_slice(&[1, 2, 3, 4]);
        mem.extend_from_slice(&plain);
        mem.extend(vec![0xaa; 500]);
        let dump = root.join("game.DMP");
        std::fs::write(&dump, &mem).unwrap();
        let out = root.join("out");
        run(
            root,
            &[dump.to_string_lossy().into_owned()],
            None,
            &[],
            Some(out.to_str().unwrap()),
        )
        .unwrap();
        assert_eq!(std::fs::read(out.join("keystream_v2.bin")).unwrap(), ks);
    }
}
