//! `wwm info|dump|patch` — port of the `wwm_locmap.py` CLI.

use crate::Code;
use crate::dump::DumpRec;
use crate::py::{NL, Obj, for_each_jsonl};
use anyhow::{Context, Result, bail};
use indexmap::IndexMap;
use rayon::prelude::*;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use wwm_locmap::{
    TOMBSTONE, parse_index, parse_shard, read_container_ex, rebuild_shard, write_container_ex,
};

pub fn info(src: &Path, root: &Path) -> Result<Code> {
    let (hdr, blocks) = read_container_ex(src, root)?;
    let (total, nblocks) = parse_index(&blocks[0])?;
    println!("version        : {}", hdr.version);
    println!(
        "blocks         : {} (1 index + {} shard)",
        blocks.len(),
        nblocks
    );
    println!("entries (index): {total}");
    let counts = blocks[1..]
        .par_iter()
        .map(|b| {
            let (_, _, ents) = parse_shard(b)?;
            Ok((
                ents.len(),
                ents.iter().filter(|e| e.value == TOMBSTONE).count(),
            ))
        })
        .collect::<wwm_locmap::Result<Vec<_>>>()?;
    let n: usize = counts.iter().map(|c| c.0).sum();
    let deleted: usize = counts.iter().map(|c| c.1).sum();
    println!("entries parsed : {n}");
    if deleted > 0 {
        println!("  deleted (tombstone 0xFF): {deleted}");
    }
    if n as u64 != total {
        println!(
            "note: parsed count != index total -- file terlihat seperti \
             *_diff (hanya berisi key yang berubah/terhapus sejak versi dasar)"
        );
    }
    Ok(0)
}

/// Render one shard's entries as dump lines (Python `cmd_dump` format).
fn dump_block(bi: usize, blk: &[u8]) -> Result<(String, usize)> {
    let (_, _, ents) = parse_shard(blk)?;
    let mut s = String::new();
    let mut deleted = 0;
    for e in ents {
        let o = Obj::new()
            .int("b", bi as i64)
            .int("s", e.slot as i64)
            .str("h", &format!("{:016x}", e.hash));
        let line = if e.value == TOMBSTONE {
            deleted += 1;
            o.bool("deleted", true).finish()
        } else {
            match std::str::from_utf8(e.value) {
                Ok(v) => o.str("v", v).finish(),
                Err(_) => bail!(
                    "nilai biner tak terduga di blok {} slot {} (bukan UTF-8, bukan tombstone 0xFF): {:?}",
                    bi,
                    e.slot,
                    &e.value[..e.value.len().min(32)]
                ),
            }
        };
        s.push_str(&line);
        s.push_str(NL);
    }
    Ok((s, deleted))
}

pub fn dump(src: &Path, out: &Path, root: &Path) -> Result<Code> {
    let (_, blocks) = read_container_ex(src, root)?;
    let parts = blocks
        .par_iter()
        .enumerate()
        .skip(1)
        .map(|(bi, b)| dump_block(bi, b))
        .collect::<Result<Vec<_>>>()?;
    let f = std::fs::File::create(out).with_context(|| format!("{}", out.display()))?;
    let mut w = std::io::BufWriter::with_capacity(1 << 20, f);
    let mut deleted = 0;
    for (s, d) in &parts {
        w.write_all(s.as_bytes())?;
        deleted += d;
    }
    w.flush()?;
    println!("ditulis: {}", out.display());
    if deleted > 0 {
        println!("entri terhapus (tombstone): {deleted}");
    }
    Ok(0)
}

/// One patch edit: (keyHash if given, new value, JSONL line number).
type Edit = (Option<String>, Vec<u8>, usize);

pub fn patch(src: &Path, jsonl: &Path, out: &Path, level: i32, root: &Path) -> Result<Code> {
    let (hdr, blocks) = read_container_ex(src, root)?;
    // block -> slot -> (keyHash, value, line); later lines win, like the Python dict
    let mut edits: IndexMap<i64, IndexMap<i64, Edit>> = IndexMap::new();
    let name = jsonl.display().to_string();
    for_each_jsonl(jsonl, |ln, r: DumpRec| {
        let v = if r.is_deleted() {
            TOMBSTONE.to_vec()
        } else if let Some(v) = r.v {
            v.into_bytes()
        } else {
            bail!("{name} baris {ln}: tidak ada field 'v' maupun 'deleted'");
        };
        edits.entry(r.b).or_default().insert(r.s, (r.h, v, ln));
        Ok(())
    })?;
    let mut bad: Vec<i64> = edits
        .keys()
        .copied()
        .filter(|&b| !(1 <= b && (b as usize) < blocks.len()))
        .collect();
    bad.sort();
    if !bad.is_empty() {
        bad.truncate(10);
        bail!(
            "{name}: block di luar jangkauan 1..{}: {:?}",
            blocks.len() - 1,
            bad
        );
    }
    // validate in file order first (same first error as Python), then rebuild in parallel
    for (bi, blk) in blocks.iter().enumerate().skip(1) {
        let Some(ed) = edits.get(&(bi as i64)) else {
            continue;
        };
        let (_, _, ents) = parse_shard(blk)?;
        let hashes: HashMap<usize, u64> = ents.iter().map(|e| (e.slot, e.hash)).collect();
        for (&s, (h, _, ln)) in ed {
            let Some(&have) = usize::try_from(s).ok().and_then(|s| hashes.get(&s)) else {
                bail!("{name} baris {ln}: blok {bi} slot {s} bukan slot terisi");
            };
            if let Some(h) = h {
                let want = u64::from_str_radix(h, 16)
                    .with_context(|| format!("{name} baris {ln}: keyHash {h}"))?;
                if want != have {
                    bail!(
                        "{name} baris {ln}: keyHash {h} != {have:016x} di blok {bi} slot {s} \
                         (JSONL di-dump dari versi file yang berbeda?)"
                    );
                }
            }
        }
    }
    let mut out_blocks = vec![blocks[0].clone()];
    let rebuilt = blocks[1..]
        .par_iter()
        .enumerate()
        .map(|(i, blk)| {
            let bi = (i + 1) as i64;
            match edits.get(&bi) {
                None => Ok(blk.clone()),
                Some(ed) => {
                    let (_, _, ents) = parse_shard(blk)?;
                    let mut vals: HashMap<usize, &[u8]> =
                        ents.iter().map(|e| (e.slot, e.value)).collect();
                    for (&s, (_, v, _)) in ed {
                        vals.insert(s as usize, v);
                    }
                    rebuild_shard(blk, |slot| vals.get(&slot).copied())
                }
            }
        })
        .collect::<wwm_locmap::Result<Vec<_>>>()?;
    out_blocks.extend(rebuilt);
    write_container_ex(out, &hdr, &out_blocks, level)?;
    println!("ditulis: {}", out.display());
    Ok(0)
}
