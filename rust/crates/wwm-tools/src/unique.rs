//! `wwm rebuild-unique` (port of `tools/rebuild_unique_strings.py`) and `wwm build-unique`
//! (port of the one-time bootstrap `translation_work/build_unique.py`).
//!
//! rebuild-unique appends texts never seen before to `unique_strings.jsonl` at new idx
//! values (max idx + 1 ...), ordered by frequency in the main dump, and never touches
//! existing idx — locale/*.jsonl stays valid. When a default dump is missing, the
//! tracked binary is decoded in memory instead (same records).

use crate::Code;
use crate::bridge::{global_keys, shared_texts};
use crate::dict::UniqueRec;
use crate::dump::{self, Rec};
use crate::profiles::Profile;
use crate::py::{NL, Obj, for_each_jsonl};
use anyhow::{Context, Result};
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// (seen texts, max idx, row count) of an existing unique_strings.jsonl (empty if absent).
pub fn load_existing(path: &Path) -> Result<(HashSet<String>, i64, usize)> {
    let (mut seen, mut max_idx, mut count) = (HashSet::new(), -1i64, 0usize);
    if !path.exists() {
        return Ok((seen, max_idx, count));
    }
    for_each_jsonl(path, |_, d: UniqueRec| {
        seen.insert(d.v);
        max_idx = max_idx.max(d.idx);
        count += 1;
        Ok(())
    })?;
    Ok((seen, max_idx, count))
}

/// Where texts come from: an explicit dump path, or a profile dump (falls back to its binary).
enum Source {
    Path(PathBuf),
    Profile(String, String),
}

fn records(p: &Profile, src: &Source, required: bool) -> Result<Vec<Rec>> {
    let (path, binary) = match src {
        Source::Path(path) => (path.clone(), None),
        Source::Profile(bin, d) => (p.path(d), Some(p.path(bin))),
    };
    if path.exists() {
        return dump::read_dump(&path);
    }
    if let Some(b) = binary.filter(|b| b.exists()) {
        return dump::decode_binary(&b, &p.root);
    }
    if required {
        crate::exit!(
            "{} tidak ada — dump dulu binary-nya dengan wwm_locmap.py dump",
            path.display()
        );
    }
    Ok(Vec::new())
}

fn live<'a>(
    recs: &'a [Rec],
    skip: &'a HashMap<String, String>,
    skip_texts: &'a HashSet<String>,
) -> impl Iterator<Item = &'a str> + 'a {
    recs.iter().filter_map(move |r| {
        let v = r.v.as_deref()?;
        if skip.contains_key(&r.h) || skip_texts.contains(v) {
            None
        } else {
            Some(v)
        }
    })
}

pub fn rebuild(
    p: &Profile,
    strings: Option<&str>,
    extra: Option<&[String]>,
    dry_run: bool,
) -> Result<Code> {
    let main = match strings {
        Some(s) => Source::Path(s.into()),
        None => Source::Profile(p.dumps[0].0.clone(), p.dumps[0].1.clone()),
    };
    let extras: Vec<Source> = match extra {
        Some(e) => e.iter().map(|s| Source::Path(s.into())).collect(),
        None => p.dumps[1..]
            .iter()
            .map(|(b, d)| Source::Profile(b.clone(), d.clone()))
            .collect(),
    };
    let main_recs = records(p, &main, true)?;

    let (mut skip, mut skip_texts) = (HashMap::new(), HashSet::new());
    if p.bridge_from.as_deref() == Some("global") {
        skip = global_keys(&p.root)?;
        skip_texts = shared_texts(p, &skip)?.into_keys().collect();
        println!(
            "jembatan global: {} key global + {} teks di key bersama dilewati (diterjemahkan lewat profil global)",
            skip.len(),
            skip_texts.len()
        );
    }

    let (seen, max_idx, old_count) = load_existing(&p.unique)?;
    println!("unique_strings.jsonl lama: {old_count} entri, idx maksimum: {max_idx}");

    let mut freq: HashMap<&str, u64> = HashMap::new();
    for v in live(&main_recs, &skip, &skip_texts) {
        *freq.entry(v).or_insert(0) += 1;
    }
    let extra_recs: Vec<Vec<Rec>> = extras
        .iter()
        .map(|s| records(p, s, false))
        .collect::<Result<_>>()?;
    // first_seen order across main + extras
    let mut new_texts: IndexMap<String, usize> = IndexMap::new();
    for recs in std::iter::once(&main_recs).chain(extra_recs.iter()) {
        for v in live(recs, &skip, &skip_texts) {
            if !seen.contains(v) && !new_texts.contains_key(v) {
                let n = new_texts.len();
                new_texts.insert(v.to_string(), n);
            }
        }
    }
    let mut order: Vec<(String, usize)> = new_texts.into_iter().collect();
    order.sort_by_key(|(v, first)| {
        (
            std::cmp::Reverse(freq.get(v.as_str()).copied().unwrap_or(0)),
            *first,
        )
    });

    println!("string baru (belum pernah tercatat): {}", order.len());
    if dry_run {
        println!("--dry-run: tidak ada yang ditulis");
        return Ok(0);
    }
    if order.is_empty() {
        return Ok(0);
    }
    if let Some(dir) = p.unique.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(&p.unique)?;
    // a last line without newline would glue onto the first appended row
    let size = f.seek(SeekFrom::End(0))?;
    let mut needs_nl = false;
    if size > 0 {
        f.seek(SeekFrom::End(-1))?;
        let mut b = [0u8; 1];
        f.read_exact(&mut b)?;
        needs_nl = b[0] != b'\n';
    }
    let mut w = std::io::BufWriter::new(f);
    if needs_nl {
        w.write_all(b"\n")?;
    }
    let mut next = max_idx + 1;
    for (v, _) in &order {
        let fq = freq.get(v.as_str()).copied().unwrap_or(0).max(1);
        let line = Obj::new()
            .int("idx", next)
            .int("freq", fq as i64)
            .str("v", v)
            .finish();
        w.write_all(line.as_bytes())?;
        w.write_all(b"\n")?;
        next += 1;
    }
    w.flush()?;
    println!(
        "ditambahkan idx {}..{} ke {}",
        max_idx + 1,
        next - 1,
        p.unique.display()
    );
    Ok(0)
}

/// One-time bootstrap. Refuses to run when unique_strings.jsonl exists: renumbering
/// idx would silently break every locale/*.jsonl.
pub fn build_unique(root: &Path) -> Result<Code> {
    let out = root.join("translation_work").join("unique_strings.jsonl");
    if out.exists() {
        crate::exit!(
            "{} sudah ada — menimpanya akan merusak mapping idx di locale/*.jsonl.\n\
             Pakai tools/rebuild_unique_strings.py untuk menambah string baru.",
            out.display()
        );
    }
    let src = root.join("strings.jsonl");
    let text = std::fs::read_to_string(&src).with_context(|| format!("{}", src.display()))?;
    let mut cnt: IndexMap<String, (u64, usize)> = IndexMap::new();
    for (i, line) in text.split('\n').enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let d: dump::DumpRec = serde_json::from_str(line.trim())?;
        if d.is_deleted() {
            continue;
        }
        let v = d.v.context("entri tanpa 'v'")?;
        cnt.entry(v).or_insert((0, i)).0 += 1;
    }
    let mut u: Vec<(&String, &(u64, usize))> = cnt.iter().collect();
    u.sort_by_key(|(_, (n, first))| (std::cmp::Reverse(*n), *first));
    let mut w = std::io::BufWriter::new(std::fs::File::create(&out)?);
    for (idx, (v, (n, _))) in u.iter().enumerate() {
        let line = Obj::new()
            .int("idx", idx as i64)
            .int("freq", *n as i64)
            .str("v", v)
            .finish();
        w.write_all(line.as_bytes())?;
        w.write_all(NL.as_bytes())?;
    }
    w.flush()?;
    println!("total unique: {}", u.len());
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{get, testutil};

    fn setup() -> tempfile::TempDir {
        let d = testutil::repo();
        // existing file without trailing newline
        testutil::write(
            d.path(),
            "translation_work/unique_strings.jsonl",
            "{\"idx\": 0, \"freq\": 3, \"v\": \"Old\"}",
        );
        testutil::write(
            d.path(),
            "strings.jsonl",
            "{\"b\": 1, \"s\": 0, \"h\": \"01\", \"v\": \"Old\"}\n\
             {\"b\": 1, \"s\": 1, \"h\": \"02\", \"v\": \"Rare\"}\n\
             {\"b\": 1, \"s\": 2, \"h\": \"03\", \"v\": \"Common\"}\n\
             {\"b\": 1, \"s\": 3, \"h\": \"04\", \"v\": \"Common\"}\n\
             {\"b\": 1, \"s\": 4, \"h\": \"05\", \"deleted\": true}\n",
        );
        testutil::write(
            d.path(),
            "strings_diff.jsonl",
            "{\"b\": 1, \"s\": 0, \"h\": \"06\", \"v\": \"Diff only\"}\n{\"b\": 1, \"s\": 1, \"h\": \"07\", \"v\": \"Rare\"}\n",
        );
        d
    }

    #[test]
    fn appends_by_frequency_keeps_old_idx() {
        let d = setup();
        let p = get(d.path(), "global").unwrap();
        rebuild(&p, None, None, true).unwrap();
        assert_eq!(
            std::fs::read_to_string(&p.unique).unwrap(),
            "{\"idx\": 0, \"freq\": 3, \"v\": \"Old\"}"
        );
        rebuild(&p, None, None, false).unwrap();
        assert_eq!(
            std::fs::read_to_string(&p.unique).unwrap(),
            "{\"idx\": 0, \"freq\": 3, \"v\": \"Old\"}\n\
             {\"idx\": 1, \"freq\": 2, \"v\": \"Common\"}\n\
             {\"idx\": 2, \"freq\": 1, \"v\": \"Rare\"}\n\
             {\"idx\": 3, \"freq\": 1, \"v\": \"Diff only\"}\n"
        );
        // second run: nothing new
        rebuild(&p, None, None, false).unwrap();
        assert_eq!(
            std::fs::read_to_string(&p.unique).unwrap().lines().count(),
            4
        );
    }

    #[test]
    fn build_unique_refuses_existing_file() {
        let d = setup();
        let e = build_unique(d.path()).unwrap_err().to_string();
        assert!(e.contains("sudah ada"), "{e}");
        std::fs::remove_file(
            d.path()
                .join("translation_work")
                .join("unique_strings.jsonl"),
        )
        .unwrap();
        build_unique(d.path()).unwrap();
        let t = std::fs::read_to_string(
            d.path()
                .join("translation_work")
                .join("unique_strings.jsonl"),
        )
        .unwrap();
        let rows: Vec<&str> = t.lines().collect();
        assert_eq!(rows[0], "{\"idx\": 0, \"freq\": 2, \"v\": \"Common\"}");
        assert_eq!(rows.len(), 3);
    }
}
