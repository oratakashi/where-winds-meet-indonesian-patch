//! `wwm compare-update` — port of `.claude/skills/wwm-game-update/scripts/compare_update.py`.
//! Read-only report for a new game version: compare every translate_words_map_* in the
//! repo root with its `.before` copy and count strings missing from unique_strings.jsonl.

use crate::Code;
use crate::dict::UniqueRec;
use crate::manifest::digest;
use crate::py::{basename, for_each_jsonl, glob, num};
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use wwm_locmap::{TOMBSTONE, parse_index, parse_shard, read_container_ex};

const GITHUB_WARN: f64 = 50.0 * 1048576.0;
const GITHUB_LIMIT: f64 = 100.0 * 1048576.0;

/// (index total, keyHash -> text or None for a tombstone, block count)
fn load(path: &Path, root: &Path) -> Result<(u64, HashMap<u64, Option<String>>, usize)> {
    let (_, blocks) = read_container_ex(path, root)?;
    let total = parse_index(&blocks[0])?.0;
    let mut ents = HashMap::new();
    for blk in &blocks[1..] {
        for e in parse_shard(blk)?.2 {
            let v = if e.value == TOMBSTONE {
                None
            } else {
                Some(String::from_utf8(e.value.to_vec())?)
            };
            ents.insert(e.hash, v);
        }
    }
    Ok((total, ents, blocks.len()))
}

fn chars(set: &HashSet<&str>) -> usize {
    set.iter().map(|s| s.chars().count()).sum()
}

pub fn run(root: &Path) -> Result<Code> {
    let unique = root.join("translation_work").join("unique_strings.jsonl");
    let mut seen: HashSet<String> = HashSet::new();
    for_each_jsonl(&unique, |_, r: UniqueRec| {
        seen.insert(r.v);
        Ok(())
    })?;
    println!("unique_strings.jsonl: {} baris", seen.len());

    let all = glob(&root.join("translate_words_map_*").to_string_lossy())?;
    let mut names: Vec<String> = all
        .iter()
        .filter(|p| !p.ends_with(".before"))
        .map(|p| basename(p))
        .collect();
    names.sort();
    let befores: HashSet<String> =
        glob(&root.join("translate_words_map_*.before").to_string_lossy())?
            .iter()
            .map(|p| {
                let b = basename(p);
                b[..b.len() - 7].to_string()
            })
            .collect();
    let mut gone: Vec<&String> = befores.iter().filter(|b| !names.contains(b)).collect();
    gone.sort();
    for g in gone {
        println!("\n!! {g} ada di .before tapi TIDAK ada di versi baru (variant dihapus?)");
    }

    let mut loaded: Vec<HashMap<u64, Option<String>>> = Vec::new();
    for name in &names {
        let path = root.join(name);
        let before = root.join(format!("{name}.before"));
        println!(
            "\n== {} ({} bytes)",
            name,
            num(std::fs::metadata(&path)?.len())
        );
        let old = if !before.exists() {
            println!(
                "   !! tidak ada .before — variant BARU (cek format, tambahkan ke files di tools/profiles.json)"
            );
            None
        } else if digest(&path)?.0 == digest(&before)?.0 {
            println!("   byte-identik dengan .before — tidak berubah");
            continue;
        } else {
            Some(load(&before, root)?)
        };
        let (total, ents, nblk) = load(&path, root)?;
        let live = ents.values().filter(|v| v.is_some()).count();
        println!(
            "   blocks {}, index total {}, parsed {}, tombstone {}",
            nblk,
            num(total),
            num(ents.len() as u64),
            num((ents.len() - live) as u64)
        );
        if let Some((o_total, o_ents, o_nblk)) = old {
            let added = ents.keys().filter(|h| !o_ents.contains_key(h)).count();
            let removed = o_ents.keys().filter(|h| !ents.contains_key(h)).count();
            let changed = ents
                .iter()
                .filter(|(h, v)| o_ents.get(h).is_some_and(|o| o != *v))
                .count();
            println!(
                "   sebelumnya: blocks {}, index total {}, parsed {}",
                o_nblk,
                num(o_total),
                num(o_ents.len() as u64)
            );
            println!(
                "   key +{} / -{}, nilai berubah {}",
                num(added as u64),
                num(removed as u64),
                num(changed as u64)
            );
        }
        let new: HashSet<&str> = ents
            .values()
            .flatten()
            .map(String::as_str)
            .filter(|v| !seen.contains(*v))
            .collect();
        println!(
            "   teks unik belum ada di dictionary: {} ({} karakter)",
            num(new.len() as u64),
            num(chars(&new) as u64)
        );
        loaded.push(ents);
    }
    let mut new_all: HashSet<&str> = HashSet::new();
    for ents in &loaded {
        new_all.extend(
            ents.values()
                .flatten()
                .map(String::as_str)
                .filter(|v| !seen.contains(*v)),
        );
    }
    println!(
        "\n== TOTAL string baru (gabungan, dedup): {} ({} karakter)",
        num(new_all.len() as u64),
        num(chars(&new_all) as u64)
    );
    if new_all.is_empty() {
        println!("   tidak ada yang perlu di-append");
    } else {
        println!(
            "   akan jadi idx {}..{} bila di-append",
            seen.len(),
            seen.len() + new_all.len() - 1
        );
    }
    let size = std::fs::metadata(&unique)?.len() as f64;
    let est = size * (seen.len() + new_all.len()) as f64 / seen.len().max(1) as f64;
    let flag = if est > GITHUB_LIMIT {
        "!! MELEWATI 100 MB"
    } else if est > GITHUB_WARN {
        "peringatan GitHub >50 MB"
    } else {
        "ok"
    };
    println!(
        "\nukuran unique_strings.jsonl: {:.1} MB -> perkiraan {:.1} MB ({})",
        size / 1048576.0,
        est / 1048576.0,
        flag
    );
    Ok(0)
}
