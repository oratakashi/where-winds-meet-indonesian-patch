//! `wwm patch-all` — port of `tools/patch_all.py`: apply the translation dictionary to
//! every `translate_words_map_*` variant of a profile, shards processed in parallel.

use crate::Code;
use crate::bridge::load_bridge;
use crate::dict::{load_overrides, load_translated_map};
use crate::profiles::Profile;
use anyhow::{Context, Result};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use wwm_locmap::{
    Header, TOMBSTONE, parse_shard, read_container_ex, rebuild_shard, write_container_ex,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    pub total: usize,
    pub matched: usize,
    pub tombstones: usize,
    pub bridged: usize,
}

pub struct Dict<'a> {
    pub src_to_translated: &'a HashMap<String, String>,
    /// keyHash (hex) -> translation, tried before the dictionary
    pub bridge: &'a HashMap<String, String>,
    /// source text -> translation, tried after it
    pub text_bridge: &'a HashMap<String, String>,
    /// keyHash (hex) -> (source text, translation), wins over everything
    pub overrides: &'a HashMap<String, (String, String)>,
}

/// Patch one shard: returns the rebuilt shard, its stats and override warnings.
fn patch_shard(blk: &[u8], d: &Dict) -> Result<(Vec<u8>, Stats, Vec<String>)> {
    let (_, _, ents) = parse_shard(blk)?;
    let mut st = Stats::default();
    let mut warn = Vec::new();
    let mut vals: HashMap<usize, &[u8]> = HashMap::with_capacity(ents.len());
    for e in &ents {
        st.total += 1;
        if e.value == TOMBSTONE {
            vals.insert(e.slot, e.value);
            st.tombstones += 1;
            continue;
        }
        let Ok(text) = std::str::from_utf8(e.value) else {
            vals.insert(e.slot, e.value); // unexpected binary value: keep as is
            continue;
        };
        let hx = format!("{:016x}", e.hash);
        let mut o = d.overrides.get(&hx);
        if let Some((src, _)) = o
            && src != text
        {
            warn.push(format!(
                "PERINGATAN: override {hx} dilewati, teks sumber sudah berubah"
            ));
            o = None;
        }
        let t: Option<&String> = if let Some((_, t)) = o {
            Some(t)
        } else if let Some(t) = d.bridge.get(&hx) {
            st.bridged += 1;
            Some(t)
        } else if let Some(t) = d.src_to_translated.get(text) {
            Some(t)
        } else if let Some(t) = d.text_bridge.get(text) {
            st.bridged += 1;
            Some(t)
        } else {
            None
        };
        match t {
            Some(t) => {
                vals.insert(e.slot, t.as_bytes());
                st.matched += 1;
            }
            None => {
                vals.insert(e.slot, e.value);
            }
        }
    }
    let out = rebuild_shard(blk, |s| vals.get(&s).copied())?;
    Ok((out, st, warn))
}

/// `patch_file()`: header, patched blocks, stats (warnings are printed in shard order).
pub fn patch_file(src: &Path, root: &Path, d: &Dict) -> Result<(Header, Vec<Vec<u8>>, Stats)> {
    let (hdr, blocks) =
        read_container_ex(src, root).with_context(|| format!("{}", src.display()))?;
    let parts = blocks[1..]
        .par_iter()
        .map(|b| patch_shard(b, d))
        .collect::<Result<Vec<_>>>()?;
    let mut out = Vec::with_capacity(blocks.len());
    let mut st = Stats::default();
    let mut blocks = blocks;
    out.push(std::mem::take(&mut blocks[0]));
    for (b, s, w) in parts {
        for line in w {
            println!("{line}");
        }
        st.total += s.total;
        st.matched += s.matched;
        st.tombstones += s.tombstones;
        st.bridged += s.bridged;
        out.push(b);
    }
    Ok((hdr, out, st))
}

pub fn run(
    p: &Profile,
    outdir: Option<&str>,
    level: i32,
    files: Option<&[String]>,
) -> Result<Code> {
    let outdir = outdir
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| p.patched_dir.clone());
    let files: Vec<String> = files
        .filter(|f| !f.is_empty())
        .map(|f| f.to_vec())
        .unwrap_or_else(|| p.files.clone());
    let (src_to_translated, phase_files) = load_translated_map(p)?;
    println!(
        "dictionary terjemahan: {} string unik, dari {} file fase",
        src_to_translated.len(),
        phase_files.len()
    );
    let (bridge, text_bridge) = load_bridge(p)?;
    let overrides = load_overrides(p)?;
    if !overrides.is_empty() {
        println!("override per key: {}", overrides.len());
    }
    if !bridge.is_empty() {
        println!(
            "jembatan global: {} key + {} teks sudah diterjemahkan",
            bridge.len(),
            text_bridge.len()
        );
    }
    std::fs::create_dir_all(&outdir).with_context(|| format!("{}", outdir.display()))?;
    let d = Dict {
        src_to_translated: &src_to_translated,
        bridge: &bridge,
        text_bridge: &text_bridge,
        overrides: &overrides,
    };
    for name in &files {
        let src = p.path(name);
        if !src.exists() {
            println!("skip (tidak ada): {name}");
            continue;
        }
        let (hdr, blocks, st) = patch_file(&src, &p.root, &d)?;
        let out_path = outdir.join(name);
        write_container_ex(&out_path, &hdr, &blocks, level)?;
        let live = st.total - st.tombstones;
        let pctv = if live > 0 {
            100.0 * st.matched as f64 / live as f64
        } else {
            0.0
        };
        let via = if bridge.is_empty() {
            String::new()
        } else {
            format!(" (jembatan={}, cn={})", st.bridged, st.matched - st.bridged)
        };
        println!(
            "{:<40} entri={:<8} tombstone={:<6} cocok={:<7} ({:5.2}%){} -> {}",
            name,
            st.total,
            st.tombstones,
            st.matched,
            pctv,
            via,
            out_path.display()
        );
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{get, testutil};
    use wwm_locmap::{build_index, build_shard};

    #[test]
    fn patches_by_text_and_keeps_the_rest() {
        let d = testutil::repo();
        crate::dict::tests_seed(d.path());
        let blocks = vec![
            build_index(4, 2),
            build_shard(
                16,
                0,
                &[(0, 1, b"Hello"), (5, 2, b"Untranslated"), (9, 3, TOMBSTONE)],
            ),
            build_shard(8, 0, &[(3, 4, b"Bye"), (4, 5, b"\xfe\xfe")]),
        ];
        write_container_ex(
            &d.path().join("translate_words_map_en"),
            &Header::v1(),
            &blocks,
            1,
        )
        .unwrap();
        let p = get(d.path(), "global").unwrap();
        assert_eq!(run(&p, None, 3, None).unwrap(), 0);
        let (_, got) = read_container_ex(
            &d.path().join("patched").join("translate_words_map_en"),
            d.path(),
        )
        .unwrap();
        let v = |b: usize| {
            parse_shard(&got[b])
                .unwrap()
                .2
                .iter()
                .map(|e| e.value.to_vec())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            v(1),
            vec![
                b"Halo".to_vec(),
                b"Untranslated".to_vec(),
                TOMBSTONE.to_vec()
            ]
        );
        assert_eq!(v(2), vec![b"Dah".to_vec(), b"\xfe\xfe".to_vec()]);
        assert_eq!(got[0], blocks[0]);
        assert!(
            !d.path()
                .join("patched")
                .join("translate_words_map_en_diff")
                .exists()
        );
    }

    #[test]
    fn override_wins_and_stale_override_is_skipped() {
        let tr: HashMap<String, String> = [("Hello".to_string(), "Halo".to_string())]
            .into_iter()
            .collect();
        let empty = HashMap::new();
        let ov: HashMap<String, (String, String)> = [
            (
                format!("{:016x}", 1),
                ("Hello".to_string(), "Hai".to_string()),
            ),
            (
                format!("{:016x}", 2),
                ("Old text".to_string(), "Teks".to_string()),
            ),
        ]
        .into_iter()
        .collect();
        let dict = Dict {
            src_to_translated: &tr,
            bridge: &empty,
            text_bridge: &empty,
            overrides: &ov,
        };
        let blk = build_shard(8, 0, &[(0, 1, b"Hello"), (1, 2, b"Hello")]);
        let (out, st, warn) = patch_shard(&blk, &dict).unwrap();
        let vals: Vec<_> = parse_shard(&out)
            .unwrap()
            .2
            .iter()
            .map(|e| e.value.to_vec())
            .collect();
        assert_eq!(vals, vec![b"Hai".to_vec(), b"Halo".to_vec()]);
        assert_eq!(
            st,
            Stats {
                total: 2,
                matched: 2,
                tombstones: 0,
                bridged: 0
            }
        );
        assert_eq!(
            warn,
            vec![format!(
                "PERINGATAN: override {:016x} dilewati, teks sumber sudah berubah",
                2
            )]
        );
    }

    #[test]
    fn bridge_order_key_then_dict_then_text() {
        let tr: HashMap<String, String> = [("A".to_string(), "dict".to_string())]
            .into_iter()
            .collect();
        let bridge: HashMap<String, String> = [(format!("{:016x}", 1), "key".to_string())]
            .into_iter()
            .collect();
        let tb: HashMap<String, String> = [
            ("A".to_string(), "text".to_string()),
            ("B".to_string(), "text".to_string()),
        ]
        .into_iter()
        .collect();
        let empty = HashMap::new();
        let dict = Dict {
            src_to_translated: &tr,
            bridge: &bridge,
            text_bridge: &tb,
            overrides: &empty,
        };
        let blk = build_shard(
            8,
            0,
            &[(0, 1, b"A"), (1, 2, b"A"), (2, 3, b"B"), (3, 4, b"C")],
        );
        let (out, st, _) = patch_shard(&blk, &dict).unwrap();
        let vals: Vec<_> = parse_shard(&out)
            .unwrap()
            .2
            .iter()
            .map(|e| e.value.to_vec())
            .collect();
        assert_eq!(
            vals,
            vec![
                b"key".to_vec(),
                b"dict".to_vec(),
                b"text".to_vec(),
                b"C".to_vec()
            ]
        );
        assert_eq!(
            st,
            Stats {
                total: 4,
                matched: 3,
                tombstones: 0,
                bridged: 2
            }
        );
    }
}
