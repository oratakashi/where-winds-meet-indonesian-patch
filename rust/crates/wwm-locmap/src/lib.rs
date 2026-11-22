//! Reader/writer for `translate_words_map_*` (Where Winds Meet / NetEase Messiah Engine).
//!
//! Rust port of `wwm_locmap.py` — same format, same checks, same (Indonesian) error
//! messages. See `Obsidian-Vault/knowladge/Format-Spec.md` for the full spec.
//!
//! ```text
//! Container v1
//!   u32 magic = 0xDEADBEEF | u32 version = 1 | u32 blockCount | u32 reserved = 0
//!   u32[blockCount] endOffset            # relative to the end of this table
//! Container v2 (WWM China, Format-Spec §8)
//!   u32 magic | u32 version = 2 | u32 mode
//!   mode 6 only: u32 dictSize | u8[dictSize] zstd dictionary XOR keystream
//!   u64 blockCount | u32[blockCount] endOffset
//! Block
//!   u8 codec (4 = zstd, 5 = zstd + file dictionary) | u32 compressedSize | u32 uncompressedSize
//! Block 0 = index: u64 totalEntries, u64 dataBlockCount, u32[dataBlockCount] id
//! Block 1..n = shard (absl::flat_hash_map):
//!   u64 capacity, u64 size, u64 seed | u8[capacity] ctrl | 0xFF sentinel + 15 cloned ctrl
//!   + padding to 8 | slot[capacity] { u64 keyHash; u32 relOffset; u32 byteLen } | blob
//!   value = block[address_of_relOffset_field + relOffset ..][..byteLen]   (RELATIVE pointer)
//! ```

use rayon::prelude::*;
use std::fmt;
use std::path::{Path, PathBuf};

pub const MAGIC: u32 = 0xDEAD_BEEF;
pub const CODEC_ZSTD: u8 = 4;
/// Container v2: zstd frame compressed with the file's dictionary.
pub const CODEC_ZSTD_DICT: u8 = 5;
pub const MODE_ZSTD: u32 = 4;
pub const MODE_ZSTD_DICT: u32 = 6;
/// A key removed since the base version (`*_diff` files) is stored as this single byte.
pub const TOMBSTONE: &[u8] = b"\xff";
pub const ZDICT_MAGIC: [u8; 4] = [0x37, 0xa4, 0x30, 0xec];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! bail {
    ($($t:tt)*) => { return Err(Error(format!($($t)*))) };
}

fn rd_u32(d: &[u8], p: usize) -> Result<u32> {
    d.get(p..p + 4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .ok_or_else(|| Error(format!("file terpotong di offset {p}")))
}

fn rd_u64(d: &[u8], p: usize) -> Result<u64> {
    d.get(p..p + 8)
        .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
        .ok_or_else(|| Error(format!("file terpotong di offset {p}")))
}

// ---------------------------------------------------------------- container

/// Container header as read; pass it unchanged to [`write_container_ex`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub version: u32,
    pub mode: u32,
    /// Mode 6: the dictionary exactly as stored (encrypted).
    pub enc_dict: Option<Vec<u8>>,
    /// Mode 6: the decrypted dictionary.
    pub zdict: Option<Vec<u8>>,
}

impl Header {
    pub fn v1() -> Self {
        Header {
            version: 1,
            mode: MODE_ZSTD,
            enc_dict: None,
            zdict: None,
        }
    }
}

/// Default keystream path: `$WWM_KEYSTREAM`, else `<root>/cn/work/keystream_v2.bin`.
pub fn keystream_path(root: &Path) -> PathBuf {
    match std::env::var_os("WWM_KEYSTREAM") {
        Some(p) if !p.is_empty() => PathBuf::from(p),
        _ => root.join("cn").join("work").join("keystream_v2.bin"),
    }
}

pub fn load_keystream(path: &Path) -> Result<Vec<u8>> {
    if !path.exists() {
        bail!(
            "container v2 butuh keystream kamus ({} tidak ada) — buat dengan \
             cn/tools/find_dict_in_dump.py atau set WWM_KEYSTREAM",
            path.display()
        );
    }
    std::fs::read(path).map_err(|e| Error(format!("{}: {e}", path.display())))
}

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(x, y)| x ^ y).collect()
}

/// Where the keystream for a v2 mode-6 file comes from.
pub enum Keystream<'a> {
    /// Load lazily from this path, only when a mode-6 file is met.
    Path(PathBuf),
    Bytes(&'a [u8]),
}

/// Read a v1 or v2 container from bytes. Returns the header and the decompressed blocks.
pub fn read_container_bytes(d: &[u8], ks: &Keystream) -> Result<(Header, Vec<Vec<u8>>)> {
    let magic = rd_u32(d, 0)?;
    let ver = rd_u32(d, 4)?;
    if magic != MAGIC {
        bail!("magic tidak cocok: {magic:08x}");
    }
    let mut hdr = Header {
        version: ver,
        mode: MODE_ZSTD,
        enc_dict: None,
        zdict: None,
    };
    let (cnt, mut p) = match ver {
        1 => (rd_u32(d, 8)? as usize, 16usize),
        2 => {
            hdr.mode = rd_u32(d, 8)?;
            let mut p = 12;
            if hdr.mode == MODE_ZSTD_DICT {
                let ds = rd_u32(d, 12)? as usize;
                let enc = d
                    .get(16..16 + ds)
                    .ok_or_else(|| Error("file terpotong di kamus".into()))?;
                let owned;
                let ksb: &[u8] = match ks {
                    Keystream::Bytes(b) => b,
                    Keystream::Path(path) => {
                        owned = load_keystream(path)?;
                        &owned
                    }
                };
                if ksb.len() < ds {
                    bail!("keystream {} byte < kamus {} byte", ksb.len(), ds);
                }
                let plain = xor(enc, &ksb[..ds]);
                if plain.len() < 4 || plain[..4] != ZDICT_MAGIC {
                    bail!("kamus hasil dekripsi tidak valid (keystream salah?)");
                }
                hdr.enc_dict = Some(enc.to_vec());
                hdr.zdict = Some(plain);
                p = 16 + ds;
            } else if hdr.mode != MODE_ZSTD {
                bail!("mode container v2 tak dikenal: {}", hdr.mode);
            }
            let cnt = rd_u64(d, p)? as usize;
            (cnt, p + 8)
        }
        _ => bail!("versi container tak dikenal: {ver}"),
    };
    if cnt == 0 {
        bail!("container tanpa blok (blockCount = 0)");
    }
    let mut offs = Vec::with_capacity(cnt);
    for i in 0..cnt {
        offs.push(rd_u32(d, p + i * 4)? as usize);
    }
    p += cnt * 4;
    let base = p;
    if base + offs[cnt - 1] != d.len() {
        bail!("ukuran file tidak konsisten dengan tabel offset");
    }
    let ranges: Vec<(usize, usize)> = (0..cnt)
        .map(|i| (base + if i > 0 { offs[i - 1] } else { 0 }, base + offs[i]))
        .collect();
    let zdict = hdr.zdict.clone();
    let blocks = ranges
        .par_iter()
        .enumerate()
        .map_init(
            || {
                None::<(
                    Option<zstd::bulk::Decompressor<'static>>,
                    Option<zstd::bulk::Decompressor<'static>>,
                )>
            },
            |st, (i, &(s, e))| {
                let b = d
                    .get(s..e)
                    .ok_or_else(|| Error(format!("blok {i}: offset rusak")))?;
                if b.len() < 9 || s > e {
                    bail!("blok {i}: header blok terpotong");
                }
                let codec = b[0];
                let cs = u32::from_le_bytes(b[1..5].try_into().unwrap()) as usize;
                let us = u32::from_le_bytes(b[5..9].try_into().unwrap()) as usize;
                let payload = b
                    .get(9..9 + cs)
                    .ok_or_else(|| Error(format!("blok {i}: payload terpotong")))?;
                if st.is_none() {
                    let plain =
                        zstd::bulk::Decompressor::new().map_err(|e| Error(e.to_string()))?;
                    let dict = match &zdict {
                        Some(z) => Some(
                            zstd::bulk::Decompressor::with_dictionary(z)
                                .map_err(|e| Error(e.to_string()))?,
                        ),
                        None => None,
                    };
                    *st = Some((Some(plain), dict));
                }
                let (plain, dict) = st.as_mut().unwrap();
                let dc = match codec {
                    CODEC_ZSTD => plain.as_mut().unwrap(),
                    CODEC_ZSTD_DICT if dict.is_some() => dict.as_mut().unwrap(),
                    _ => bail!("codec tak dikenal: {codec}"),
                };
                let raw = dc
                    .decompress(payload, us)
                    .map_err(|e| Error(format!("blok {i}: zstd: {e}")))?;
                if raw.len() != us {
                    bail!("blok {i}: ukuran hasil dekompresi {} != {us}", raw.len());
                }
                Ok(raw)
            },
        )
        .collect::<Result<Vec<_>>>()?;
    Ok((hdr, blocks))
}

/// Read a container file; the keystream (v2 mode 6 only) comes from `keystream_path(root)`.
pub fn read_container_ex(path: &Path, root: &Path) -> Result<(Header, Vec<Vec<u8>>)> {
    let d = std::fs::read(path).map_err(|e| Error(format!("{}: {e}", path.display())))?;
    read_container_bytes(&d, &Keystream::Path(keystream_path(root)))
}

/// Encode blocks into a container (same header as read). Blocks are compressed in
/// parallel; `level` is the zstd level (Python default 19).
pub fn write_container_bytes(hdr: &Header, blocks: &[Vec<u8>], level: i32) -> Result<Vec<u8>> {
    let codec = if hdr.zdict.is_some() {
        CODEC_ZSTD_DICT
    } else {
        CODEC_ZSTD
    };
    let zdict = hdr.zdict.as_deref();
    let payloads = blocks
        .par_iter()
        .map_init(
            || None::<zstd::bulk::Compressor<'static>>,
            |st, raw| {
                if st.is_none() {
                    let c = match zdict {
                        Some(z) => zstd::bulk::Compressor::with_dictionary(level, z),
                        None => zstd::bulk::Compressor::new(level),
                    }
                    .map_err(|e| Error(e.to_string()))?;
                    *st = Some(c);
                }
                let c = st.as_mut().unwrap();
                // Same frame flags as python-zstandard: content size on, checksum off,
                // dictID on (dictionary mode).
                c.include_contentsize(true)
                    .map_err(|e| Error(e.to_string()))?;
                c.include_checksum(false)
                    .map_err(|e| Error(e.to_string()))?;
                c.include_dictid(true).map_err(|e| Error(e.to_string()))?;
                let comp = c.compress(raw).map_err(|e| Error(e.to_string()))?;
                let mut out = Vec::with_capacity(9 + comp.len());
                out.push(codec);
                out.extend_from_slice(&(comp.len() as u32).to_le_bytes());
                out.extend_from_slice(&(raw.len() as u32).to_le_bytes());
                out.extend_from_slice(&comp);
                Ok(out)
            },
        )
        .collect::<Result<Vec<_>>>()?;
    let mut out = Vec::new();
    if hdr.version == 1 {
        for v in [MAGIC, 1, blocks.len() as u32, 0] {
            out.extend_from_slice(&v.to_le_bytes());
        }
    } else {
        for v in [MAGIC, 2, hdr.mode] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        if hdr.mode == MODE_ZSTD_DICT {
            let enc = hdr
                .enc_dict
                .as_ref()
                .ok_or_else(|| Error("header v2 mode 6 tanpa kamus".into()))?;
            out.extend_from_slice(&(enc.len() as u32).to_le_bytes());
            out.extend_from_slice(enc);
        }
        out.extend_from_slice(&(blocks.len() as u64).to_le_bytes());
    }
    let mut acc = 0usize;
    for p in &payloads {
        acc += p.len();
        out.extend_from_slice(&(acc as u32).to_le_bytes());
    }
    for p in &payloads {
        out.extend_from_slice(p);
    }
    Ok(out)
}

pub fn write_container_ex(path: &Path, hdr: &Header, blocks: &[Vec<u8>], level: i32) -> Result<()> {
    let out = write_container_bytes(hdr, blocks, level)?;
    std::fs::write(path, out).map_err(|e| Error(format!("{}: {e}", path.display())))
}

// -------------------------------------------------------------------- shard

pub fn slots_off(cap: usize) -> usize {
    let o = 24 + cap + 1 + 15;
    o + (8 - o % 8) % 8
}

/// (totalEntries, dataBlockCount) of the index block.
pub fn parse_index(blk: &[u8]) -> Result<(u64, u64)> {
    Ok((rd_u64(blk, 0)?, rd_u64(blk, 8)?))
}

/// One live slot of a shard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry<'a> {
    pub slot: usize,
    pub hash: u64,
    pub value: &'a [u8],
}

/// Parse a shard: (capacity, seed, live entries in slot order).
pub fn parse_shard(blk: &[u8]) -> Result<(usize, u64, Vec<Entry<'_>>)> {
    let cap = rd_u64(blk, 0)? as usize;
    let size = rd_u64(blk, 8)? as usize;
    let seed = rd_u64(blk, 16)?;
    let ctrl = blk
        .get(24..24 + cap)
        .ok_or_else(|| Error("shard terpotong (ctrl)".into()))?;
    if blk.get(24 + cap) != Some(&0xFF) {
        bail!("sentinel hilang — layout tidak cocok");
    }
    let s = slots_off(cap);
    let mut out = Vec::with_capacity(size);
    for (i, &c) in ctrl.iter().enumerate() {
        if c >= 0x80 {
            continue;
        }
        let p = s + i * 16;
        let h = rd_u64(blk, p)?;
        let rel = rd_u32(blk, p + 8)? as usize;
        let ln = rd_u32(blk, p + 12)? as usize;
        let a = p + 8 + rel;
        let value = blk
            .get(a..a + ln)
            .ok_or_else(|| Error(format!("slot {i}: nilai di luar blok")))?;
        out.push(Entry {
            slot: i,
            hash: h,
            value,
        });
    }
    if out.len() != size {
        bail!("jumlah slot terisi != size");
    }
    Ok((cap, seed, out))
}

/// Rebuild a shard with new values. `value_of(slot)` must return the value of every
/// live slot. ctrl / keyHash / slot positions are kept byte-for-byte; identical
/// strings are stored once.
pub fn rebuild_shard<'v, F>(orig: &[u8], mut value_of: F) -> Result<Vec<u8>>
where
    F: FnMut(usize) -> Option<&'v [u8]>,
{
    let cap = rd_u64(orig, 0)? as usize;
    let s = slots_off(cap);
    let mut head = orig
        .get(..s + cap * 16)
        .ok_or_else(|| Error("shard terpotong".into()))?
        .to_vec();
    let mut blob: Vec<u8> = Vec::new();
    let mut pool: std::collections::HashMap<&[u8], usize> = std::collections::HashMap::new();
    let hl = head.len();
    for i in 0..cap {
        if orig[24 + i] >= 0x80 {
            continue;
        }
        let v = value_of(i).ok_or_else(|| Error(format!("slot {i} tidak punya nilai")))?;
        let off = *pool.entry(v).or_insert_with(|| {
            let o = blob.len();
            blob.extend_from_slice(v);
            o
        });
        let field = s + i * 16 + 8; // address of the relOffset field
        let rel = (hl + off - field) as u32;
        head[field..field + 4].copy_from_slice(&rel.to_le_bytes());
        head[field + 4..field + 8].copy_from_slice(&(v.len() as u32).to_le_bytes());
    }
    head.extend_from_slice(&blob);
    Ok(head)
}

/// Build a shard from scratch (test fixtures / synthetic files). `entries` are
/// (slot, keyHash, value); ctrl of a live slot is `keyHash & 0x7f` (any H2 < 0x80 works).
pub fn build_shard(cap: usize, seed: u64, entries: &[(usize, u64, &[u8])]) -> Vec<u8> {
    let s = slots_off(cap);
    let mut b = vec![0u8; s + cap * 16];
    b[0..8].copy_from_slice(&(cap as u64).to_le_bytes());
    b[8..16].copy_from_slice(&(entries.len() as u64).to_le_bytes());
    b[16..24].copy_from_slice(&seed.to_le_bytes());
    for c in &mut b[24..24 + cap] {
        *c = 0x80;
    }
    b[24 + cap] = 0xFF;
    for &(slot, h, _) in entries {
        b[24 + slot] = (h & 0x7f) as u8;
        b[s + slot * 16..s + slot * 16 + 8].copy_from_slice(&h.to_le_bytes());
    }
    for i in 0..15.min(cap) {
        b[24 + cap + 1 + i] = b[24 + i];
    }
    let vals: std::collections::HashMap<usize, &[u8]> =
        entries.iter().map(|&(s, _, v)| (s, v)).collect();
    rebuild_shard(&b, |i| vals.get(&i).copied()).expect("build_shard")
}

/// Index block: totalEntries, dataBlockCount, ids 0..n.
pub fn build_index(total: u64, nblocks: u64) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&total.to_le_bytes());
    b.extend_from_slice(&nblocks.to_le_bytes());
    for i in 0..nblocks as u32 {
        b.extend_from_slice(&i.to_le_bytes());
    }
    b
}

#[cfg(test)]
mod tests;
