//! `wwm qa` — port of `tools/qa_check.py`.
//!
//! PROMPT_LEAK / EMPTY / MARKUP per entry; `--locale` mode also checks idx (present in
//! unique_strings.jsonl, contiguous inside a file, not duplicated across files).

use crate::Code;
use crate::dict::{IdxRec, load_unique};
use crate::dump::DumpRec;
use crate::profiles::Profile;
use crate::py::{KeyVal, NL, Obj, basename, for_each_jsonl, glob, has_magic, repr_dict};
use anyhow::{Context, Result};
use indexmap::IndexMap;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::sync::LazyLock;

/// The engine's format tokens — keep identical to `TOKEN` in tools/qa_check.py.
/// The 6-digit hex alternative must stay before the 1-letter code (`#e9a35f` vs `#e`).
pub static TOKEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"#[0-9a-fA-F]{6}|#[A-Za-z]|%[sd]|\{[^}]*\}|<[^>]*>|\$[A-Z][A-Z0-9_]+(?::[^$\s]*)?\$",
    )
    .unwrap()
});

/// Prompt-leak signatures — keep identical to `LEAK_SIGNATURES` in tools/qa_check.py.
pub const LEAK_SIGNATURES: &[&str] = &[
    "Penanda format",
    "Penanda pemformatan",
    "Formatting markers",
    "tidak perlu diterjemahkan",
    "Jangan terjemahkan konten",
    "Jangan menerjemahkan konten",
    "<P_*>",
];

pub fn tokens(s: &str) -> HashMap<&str, usize> {
    let mut m = HashMap::new();
    for t in TOKEN.find_iter(s) {
        *m.entry(t.as_str()).or_insert(0) += 1;
    }
    m
}

fn py_strip_empty(s: &str) -> bool {
    s.trim().is_empty()
}

/// Kind of finding for a source/translation pair, or `None`.
pub fn check_pair(s: &str, t: &str) -> Option<&'static str> {
    if LEAK_SIGNATURES.iter().any(|p| t.contains(p)) {
        return Some("PROMPT_LEAK");
    }
    if !py_strip_empty(s) && py_strip_empty(t) {
        return Some("EMPTY");
    }
    if tokens(s) != tokens(t) {
        return Some("MARKUP");
    }
    None
}

pub struct Finding {
    pub key: Vec<(&'static str, KeyVal)>,
    pub kind: &'static str,
    pub src: String,
    pub dst: String,
}

pub fn summarize(findings: &[Finding], total: usize, report: Option<&str>) -> Result<Code> {
    println!("diperiksa : {total} entri");
    for kind in ["PROMPT_LEAK", "MARKUP", "EMPTY", "IDX"] {
        let n = findings.iter().filter(|f| f.kind == kind).count();
        if n > 0 || kind != "IDX" {
            let p = if total > 0 {
                100.0 * n as f64 / total as f64
            } else {
                0.0
            };
            println!("{kind:<12}: {n:>6}  ({p:.3}%)");
        }
    }
    if let (Some(report), false) = (report, findings.is_empty()) {
        let f = std::fs::File::create(report).with_context(|| report.to_string())?;
        let mut w = std::io::BufWriter::new(f);
        for fd in findings {
            let mut o = Obj::new();
            for (k, v) in &fd.key {
                o = match v {
                    KeyVal::Int(i) => o.int(k, *i),
                    KeyVal::Str(s) => o.str(k, s),
                };
            }
            let line = o
                .str("kind", fd.kind)
                .str("src", &fd.src)
                .str("dst", &fd.dst)
                .finish();
            w.write_all(line.as_bytes())?;
            w.write_all(NL.as_bytes())?;
        }
        w.flush()?;
        println!("laporan ditulis: {report}");
    } else if !findings.is_empty() {
        for fd in findings.iter().take(20) {
            println!(
                "  {} {}\n    src: {}\n    dst: {}",
                fd.kind,
                repr_dict(&fd.key),
                fd.src,
                fd.dst
            );
        }
        if findings.len() > 20 {
            println!(
                "  ... {} lagi (pakai --report untuk daftar lengkap)",
                findings.len() - 20
            );
        }
    }
    Ok(if findings.is_empty() { 0 } else { 1 })
}

/// (b, s) -> text of a dump, tombstones skipped; insertion order kept.
fn load_pairs(path: &Path) -> Result<IndexMap<(i64, i64), String>> {
    let mut d = IndexMap::new();
    for_each_jsonl(path, |_, r: DumpRec| {
        if r.is_deleted() {
            return Ok(());
        }
        let v =
            r.v.with_context(|| format!("{}: entri tanpa 'v'", path.display()))?;
        d.insert((r.b, r.s), v);
        Ok(())
    })?;
    Ok(d)
}

pub fn run_dump(original: &Path, translated: &Path, report: Option<&str>) -> Result<Code> {
    let src = load_pairs(original)?;
    let dst = load_pairs(translated)?;
    let missing = dst.keys().filter(|k| !src.contains_key(*k)).count();
    if missing > 0 {
        println!("FATAL: {missing} entri tidak ada di file original (block/slot salah?)");
        return Ok(2);
    }
    let mut findings = Vec::new();
    for (k, s) in &src {
        let Some(t) = dst.get(k) else { continue };
        if let Some(kind) = check_pair(s, t) {
            findings.push(Finding {
                key: vec![("b", KeyVal::Int(k.0)), ("s", KeyVal::Int(k.1))],
                kind,
                src: s.clone(),
                dst: t.clone(),
            });
        }
    }
    summarize(&findings, dst.len(), report)
}

#[derive(Deserialize)]
struct OverrideRec {
    h: String,
    src: String,
    v: String,
}

pub fn run_locale(p: &Profile, patterns: &[String], report: Option<&str>) -> Result<Code> {
    // A glob with no match is skipped (e.g. cn/locale/ before its first phase);
    // a plain path that does not exist still fails when opened.
    let mut paths: Vec<String> = Vec::new();
    for pat in patterns {
        let hits = glob(pat)?;
        if hits.is_empty() {
            if !has_magic(pat) {
                paths.push(pat.clone());
            }
        } else {
            paths.extend(hits);
        }
    }
    paths.sort();
    paths.dedup();
    let overrides = p.overrides.as_ref().filter(|o| o.exists());
    if paths.is_empty() && overrides.is_none() {
        println!("tidak ada file locale yang cocok: {}", patterns.join(" "));
        return Ok(0);
    }
    let src = load_unique(p)?;
    let mut findings = Vec::new();
    let mut seen: HashMap<i64, String> = HashMap::new();
    let mut total = 0usize;
    for path in &paths {
        let name = basename(path);
        let mut prev: Option<i64> = None;
        for_each_jsonl(Path::new(path), |_, r: IdxRec| {
            total += 1;
            let i = r.idx;
            let key = || vec![("idx", KeyVal::Int(i)), ("file", KeyVal::Str(name.clone()))];
            if let Some(other) = seen.get(&i) {
                findings.push(Finding {
                    key: key(),
                    kind: "IDX",
                    src: format!("duplikat, juga ada di {other}"),
                    dst: String::new(),
                });
            }
            seen.insert(i, name.clone());
            if let Some(pv) = prev
                && i != pv + 1
            {
                findings.push(Finding {
                    key: key(),
                    kind: "IDX",
                    src: format!("tidak berurutan setelah idx {pv}"),
                    dst: String::new(),
                });
            }
            prev = Some(i);
            match src.get(&i) {
                None => findings.push(Finding {
                    key: key(),
                    kind: "IDX",
                    src: "tidak ada di unique_strings.jsonl".into(),
                    dst: String::new(),
                }),
                Some(s) => {
                    if let Some(kind) = check_pair(s, &r.v) {
                        findings.push(Finding {
                            key: key(),
                            kind,
                            src: s.clone(),
                            dst: r.v,
                        });
                    }
                }
            }
            Ok(())
        })?;
    }
    if let Some(ov) = overrides {
        let name = ov.file_name().unwrap().to_string_lossy().into_owned();
        for_each_jsonl(ov, |_, r: OverrideRec| {
            total += 1;
            if let Some(kind) = check_pair(&r.src, &r.v) {
                findings.push(Finding {
                    key: vec![("h", KeyVal::Str(r.h)), ("file", KeyVal::Str(name.clone()))],
                    kind,
                    src: r.src,
                    dst: r.v,
                });
            }
            Ok(())
        })?;
        paths.push(ov.to_string_lossy().into_owned());
    }
    let names: Vec<String> = paths.iter().map(|p| basename(p)).collect();
    println!("file: {}", names.join(", "));
    summarize(&findings, total, report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{get, testutil};

    fn toks(s: &str) -> Vec<&str> {
        TOKEN.find_iter(s).map(|m| m.as_str()).collect()
    }

    #[test]
    fn token_regex_matches_python() {
        assert_eq!(toks("#e9a35fWarna#E"), ["#e9a35f", "#E"]);
        assert_eq!(
            toks("#Yhalo#E %s %d {0} {name}"),
            ["#Y", "#E", "%s", "%d", "{0}", "{name}"]
        );
        assert_eq!(
            toks("<Atk|12|#C|3> naik $STEADY_CRI:.0f$"),
            ["<Atk|12|#C|3>", "$STEADY_CRI:.0f$"]
        );
        assert_eq!(toks("$S pilih #painter$wanderer# $E"), ["#p"]);
        assert_eq!(toks("$A$ $AB$ $AB_1$"), ["$AB$", "$AB_1$"]);
        assert_eq!(toks("100% aman %x"), Vec::<&str>::new());
    }

    #[test]
    fn check_pair_order_and_kinds() {
        assert_eq!(check_pair("#YHi#E", "#YHai#E"), None);
        assert_eq!(check_pair("#E#Y", "#YHai#E"), None); // order is not checked
        assert_eq!(check_pair("#e9a35fHi#E", "#eHi#E"), Some("MARKUP")); // damaged color
        assert_eq!(check_pair("%s x", "x"), Some("MARKUP"));
        assert_eq!(check_pair("Hi", "  "), Some("EMPTY"));
        assert_eq!(check_pair("", ""), None);
        assert_eq!(
            check_pair("%s", "Formatting markers: %s"),
            Some("PROMPT_LEAK")
        );
        assert_eq!(check_pair("Hi", ""), Some("EMPTY"));
        assert_eq!(check_pair("#Y", ""), Some("EMPTY")); // EMPTY wins over MARKUP
    }

    #[test]
    fn locale_mode_finds_idx_problems() {
        let d = testutil::repo();
        crate::dict::tests_seed(d.path());
        testutil::write(
            d.path(),
            "locale/phase1.jsonl",
            "{\"idx\": 3, \"v\": \"Dah\"}\n{\"idx\": 1, \"v\": \"Dunia\"}\n{\"idx\": 2, \"v\": \"tanpa\"}\n{\"idx\": 9, \"v\": \"?\"}\n",
        );
        let p = get(d.path(), "global").unwrap();
        let pat = d
            .path()
            .join("locale")
            .join("*.jsonl")
            .to_string_lossy()
            .into_owned();
        let report = d.path().join("qa.jsonl");
        let code = run_locale(&p, &[pat], Some(report.to_str().unwrap())).unwrap();
        assert_eq!(code, 1);
        let lines: Vec<String> = std::fs::read_to_string(&report)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();
        assert_eq!(
            lines,
            [
                r#"{"idx": 1, "file": "phase1.jsonl", "kind": "IDX", "src": "duplikat, juga ada di phase0.jsonl", "dst": ""}"#,
                r#"{"idx": 1, "file": "phase1.jsonl", "kind": "IDX", "src": "tidak berurutan setelah idx 3", "dst": ""}"#,
                r##"{"idx": 2, "file": "phase1.jsonl", "kind": "MARKUP", "src": "#Y%s#E", "dst": "tanpa"}"##,
                r#"{"idx": 9, "file": "phase1.jsonl", "kind": "IDX", "src": "tidak berurutan setelah idx 2", "dst": ""}"#,
                r#"{"idx": 9, "file": "phase1.jsonl", "kind": "IDX", "src": "tidak ada di unique_strings.jsonl", "dst": ""}"#,
            ]
        );
    }

    #[test]
    fn unmatched_glob_is_skipped() {
        let d = testutil::repo();
        let p = get(d.path(), "global").unwrap();
        let pat = d
            .path()
            .join("nothing")
            .join("*.jsonl")
            .to_string_lossy()
            .into_owned();
        assert_eq!(run_locale(&p, &[pat], None).unwrap(), 0);
    }

    #[test]
    fn dump_mode() {
        let d = testutil::repo();
        testutil::write(
            d.path(),
            "a.jsonl",
            "{\"b\": 1, \"s\": 0, \"v\": \"%s ok\"}\n{\"b\": 1, \"s\": 1, \"v\": \"x\"}\n",
        );
        testutil::write(
            d.path(),
            "b.jsonl",
            "{\"b\": 1, \"s\": 0, \"v\": \"oke\"}\n",
        );
        assert_eq!(
            run_dump(&d.path().join("a.jsonl"), &d.path().join("b.jsonl"), None).unwrap(),
            1
        );
        testutil::write(
            d.path(),
            "c.jsonl",
            "{\"b\": 9, \"s\": 0, \"v\": \"oke\"}\n",
        );
        assert_eq!(
            run_dump(&d.path().join("a.jsonl"), &d.path().join("c.jsonl"), None).unwrap(),
            2
        );
    }
}
