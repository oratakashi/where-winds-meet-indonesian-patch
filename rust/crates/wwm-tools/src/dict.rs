//! Translation dictionary (`unique_strings.jsonl` + `locale/phase*.jsonl` + `locale/update*.jsonl`)
//! and `wwm expand` — port of `tools/expand_locale.py`.

use crate::Code;
use crate::bridge::load_bridge;
use crate::dump;
use crate::profiles::{LOCALE_PATTERNS, Profile};
use crate::py::{NL, Obj, basename, for_each_jsonl, glob, repr_list};
use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize)]
pub struct IdxRec {
    pub idx: i64,
    pub v: String,
}

#[derive(Deserialize)]
pub struct UniqueRec {
    pub idx: i64,
    #[serde(default)]
    pub freq: Option<u64>,
    pub v: String,
}

/// idx -> source text of `unique_strings.jsonl`.
pub fn load_unique(p: &Profile) -> Result<HashMap<i64, String>> {
    let mut m = HashMap::new();
    for_each_jsonl(&p.unique, |_, r: UniqueRec| {
        m.insert(r.idx, r.v);
        Ok(())
    })?;
    Ok(m)
}

/// Sorted locale files of the profile (`phase*.jsonl` + `update*.jsonl`).
pub fn phase_files(p: &Profile) -> Result<Vec<String>> {
    let mut files = Vec::new();
    for pat in LOCALE_PATTERNS {
        files.extend(glob(&p.locale_dir.join(pat).to_string_lossy())?);
    }
    files.sort();
    Ok(files)
}

/// `load_translated_map(profile)`: source text -> translation, plus the locale files read.
pub fn load_translated_map(p: &Profile) -> Result<(HashMap<String, String>, Vec<String>)> {
    let idx_to_src = load_unique(p)?;
    let files = phase_files(p)?;
    let mut src_to_translated = HashMap::with_capacity(idx_to_src.len());
    let mut idx_file: HashMap<i64, String> = HashMap::new();
    for path in &files {
        let name = basename(path);
        for_each_jsonl(Path::new(path), |_, r: IdxRec| {
            let Some(src) = idx_to_src.get(&r.idx) else {
                bail!("{name}: idx {} tidak ada di unique_strings.jsonl", r.idx);
            };
            if let Some(prev) = idx_file.get(&r.idx) {
                bail!(
                    "idx {} diterjemahkan dua kali: {} dan {}",
                    r.idx,
                    prev,
                    name
                );
            }
            idx_file.insert(r.idx, name.clone());
            src_to_translated.insert(src.clone(), r.v);
            Ok(())
        })?;
    }
    Ok((src_to_translated, files))
}

#[derive(Deserialize)]
struct OverrideRec {
    h: String,
    src: String,
    v: String,
}

/// keyHash (hex) -> (source text when written, translation), from the profile's overrides.
pub fn load_overrides(p: &Profile) -> Result<HashMap<String, (String, String)>> {
    let mut out = HashMap::new();
    let Some(path) = p.overrides.as_ref().filter(|o| o.exists()) else {
        return Ok(out);
    };
    for_each_jsonl(path, |ln, d: OverrideRec| {
        if out.contains_key(&d.h) {
            bail!("{} baris {}: key {} dua kali", path.display(), ln, d.h);
        }
        out.insert(d.h, (d.src, d.v));
        Ok(())
    })?;
    Ok(out)
}

/// Python `a or b or c ...`: first non-empty string, else the last value.
fn py_or<'a>(vals: &[Option<&'a str>]) -> Option<&'a str> {
    for v in vals {
        if let Some(s) = v
            && !s.is_empty()
        {
            return Some(s);
        }
    }
    *vals.last().unwrap_or(&None)
}

pub fn expand(p: &Profile, strings: Option<&str>, out: Option<&str>) -> Result<Code> {
    let strings = match strings {
        Some(s) => s.into(),
        None => p.path(&p.dumps[0].1),
    };
    let out = match out {
        Some(o) => o.into(),
        None => p.translated_out.clone(),
    };
    let (src_to_translated, phase_files) = load_translated_map(p)?;
    let (bridge, text_bridge) = load_bridge(p)?;
    let overrides = load_overrides(p)?;
    println!("string unik diterjemahkan: {}", src_to_translated.len());
    if !overrides.is_empty() {
        println!("override per key: {}", overrides.len());
    }
    if !bridge.is_empty() {
        println!(
            "jembatan global: {} key + {} teks",
            bridge.len(),
            text_bridge.len()
        );
    }
    let names: Vec<String> = phase_files.iter().map(|f| basename(f)).collect();
    println!("dari file fase: {}", repr_list(&names));

    let recs = dump::read_dump(&strings).with_context(|| format!("{}", strings.display()))?;
    let f = std::fs::File::create(&out).with_context(|| format!("{}", out.display()))?;
    let mut w = std::io::BufWriter::with_capacity(1 << 20, f);
    let mut written = 0usize;
    for d in &recs {
        let Some(v) = d.v.as_deref() else { continue }; // tombstone
        let mut o = overrides.get(&d.h);
        if let Some((src, _)) = o
            && src != v
        {
            println!(
                "PERINGATAN: override {} dilewati, teks sumber sudah berubah",
                d.h
            );
            o = None;
        }
        let t = py_or(&[
            o.map(|x| x.1.as_str()),
            bridge.get(&d.h).map(String::as_str),
            src_to_translated.get(v).map(String::as_str),
            text_bridge.get(v).map(String::as_str),
        ]);
        let Some(t) = t else { continue };
        let line = Obj::new()
            .int("b", d.b as i64)
            .int("s", d.s as i64)
            .str("h", &d.h)
            .str("v", t)
            .finish();
        w.write_all(line.as_bytes())?;
        w.write_all(NL.as_bytes())?;
        written += 1;
    }
    w.flush()?;
    println!("baris ditulis ke {}: {}", out.display(), written);
    Ok(0)
}

/// Test fixture: a tiny unique_strings.jsonl + phase0/phase1 for the test profile.
#[cfg(test)]
pub(crate) fn tests_seed(root: &Path) {
    use crate::profiles::testutil;
    testutil::write(
        root,
        "translation_work/unique_strings.jsonl",
        "{\"idx\": 0, \"freq\": 5, \"v\": \"Hello\"}\n{\"idx\": 1, \"freq\": 2, \"v\": \"World\"}\n{\"idx\": 2, \"freq\": 1, \"v\": \"#Y%s#E\"}\n{\"idx\": 3, \"freq\": 1, \"v\": \"Bye\"}\n",
    );
    testutil::write(
        root,
        "locale/phase0.jsonl",
        "{\"idx\": 0, \"v\": \"Halo\"}\n{\"idx\": 1, \"v\": \"Dunia\"}\n",
    );
    testutil::write(
        root,
        "locale/phase1.jsonl",
        "{\"idx\": 3, \"v\": \"Dah\"}\n",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{get, testutil};

    fn seed(root: &Path) {
        tests_seed(root)
    }

    #[test]
    fn translated_map() {
        let d = testutil::repo();
        seed(d.path());
        let p = get(d.path(), "global").unwrap();
        let (m, files) = load_translated_map(&p).unwrap();
        assert_eq!(m.len(), 3);
        assert_eq!(m["Hello"], "Halo");
        assert_eq!(m["Bye"], "Dah");
        assert_eq!(
            files.iter().map(|f| basename(f)).collect::<Vec<_>>(),
            ["phase0.jsonl", "phase1.jsonl"]
        );
    }

    #[test]
    fn duplicate_and_unknown_idx_are_errors() {
        let d = testutil::repo();
        seed(d.path());
        testutil::write(
            d.path(),
            "locale/update1a.jsonl",
            "{\"idx\": 0, \"v\": \"X\"}\n",
        );
        let p = get(d.path(), "global").unwrap();
        assert_eq!(
            load_translated_map(&p).unwrap_err().to_string(),
            "idx 0 diterjemahkan dua kali: phase0.jsonl dan update1a.jsonl"
        );
        testutil::write(
            d.path(),
            "locale/update1a.jsonl",
            "{\"idx\": 99, \"v\": \"X\"}\n",
        );
        assert_eq!(
            load_translated_map(&p).unwrap_err().to_string(),
            "update1a.jsonl: idx 99 tidak ada di unique_strings.jsonl"
        );
    }

    #[test]
    fn py_or_semantics() {
        assert_eq!(py_or(&[None, Some("a"), Some("b")]), Some("a"));
        assert_eq!(py_or(&[Some(""), None, Some("b")]), Some("b"));
        assert_eq!(py_or(&[None, None, Some("")]), Some(""));
        assert_eq!(py_or(&[Some(""), None, None]), None);
    }

    #[test]
    fn expand_writes_partial_patch() {
        let d = testutil::repo();
        seed(d.path());
        testutil::write(
            d.path(),
            "strings.jsonl",
            "{\"b\": 1, \"s\": 0, \"h\": \"0000000000000001\", \"v\": \"Hello\"}\n\
             {\"b\": 1, \"s\": 2, \"h\": \"0000000000000002\", \"v\": \"Untranslated\"}\n\
             {\"b\": 2, \"s\": 5, \"h\": \"0000000000000003\", \"deleted\": true}\n\
             {\"b\": 2, \"s\": 7, \"h\": \"0000000000000004\", \"v\": \"Bye\"}\n",
        );
        let p = get(d.path(), "global").unwrap();
        expand(&p, None, None).unwrap();
        let out = std::fs::read_to_string(d.path().join("strings.translated.jsonl")).unwrap();
        assert_eq!(
            out.replace("\r\n", "\n"),
            "{\"b\": 1, \"s\": 0, \"h\": \"0000000000000001\", \"v\": \"Halo\"}\n{\"b\": 2, \"s\": 7, \"h\": \"0000000000000004\", \"v\": \"Dah\"}\n"
        );
    }
}
