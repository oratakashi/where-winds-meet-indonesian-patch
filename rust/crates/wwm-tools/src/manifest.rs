//! `wwm manifest` — port of `tools/make_manifest.py`.
//!
//! For each variant in --outdir: sha256 + size of the ORIGINAL (repo binary) and the
//! PATCHED file, plus the release zip (patched files, flat, stored, fixed timestamp).
//! Patch Guard (app/) trusts `original_sha256` as the only proof that a build matches
//! the installed game files — never fill it from anything but the binary patch-all read.

use crate::Code;
use crate::profiles::Profile;
use anyhow::{Context, Result};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub const SCHEMA: u64 = 2;
pub const NAME: &str = "manifest.json";

pub fn digest(path: &Path) -> Result<(String, u64)> {
    let mut f = std::fs::File::open(path).with_context(|| format!("{}", path.display()))?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    let mut size = 0u64;
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
        size += n as u64;
    }
    Ok((format!("{:x}", h.finalize()), size))
}

pub fn current_commit(root: &Path) -> String {
    if let Some(sha) = std::env::var_os("GITHUB_SHA").filter(|s| !s.is_empty()) {
        return sha.to_string_lossy().into_owned();
    }
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .stderr(std::process::Stdio::null())
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

/// Write the release zip: exactly the patched files listed, flat, uncompressed
/// (they are zstd already), fixed 1980-01-01 timestamp, zip64.
pub fn write_archive(path: &Path, outdir: &Path, names: &[String]) -> Result<Value> {
    use zip::write::SimpleFileOptions;
    let f = std::fs::File::create(path).with_context(|| format!("{}", path.display()))?;
    let mut z = zip::ZipWriter::new(std::io::BufWriter::new(f));
    let opts = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(zip::DateTime::default())
        .unix_permissions(0o644)
        .large_file(true);
    for name in names {
        z.start_file(name.as_str(), opts)?;
        let mut src = std::fs::File::open(outdir.join(name))?;
        std::io::copy(&mut src, &mut z)?;
    }
    z.finish()?.flush()?;
    let (sha, size) = digest(path)?;
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    Ok(json!({"name": file_name, "sha256": sha, "size": size}))
}

pub fn utc_now() -> String {
    let fmt = time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");
    time::OffsetDateTime::now_utc()
        .format(&fmt)
        .unwrap_or_default()
}

pub fn build(
    p: &Profile,
    outdir: &Path,
    commit: &str,
    archive: Option<&Path>,
) -> Result<Map<String, Value>> {
    let mut files = Vec::new();
    let mut names = Vec::new();
    for name in &p.files {
        let (src, out) = (p.path(name), outdir.join(name));
        if !(src.exists() && out.exists()) {
            continue; // variant not shipped (e.g. _mobile)
        }
        let (o_sha, o_size) = digest(&src)?;
        let (p_sha, p_size) = digest(&out)?;
        files.push(
            json!({"name": name, "original_sha256": o_sha, "original_size": o_size,
                          "patched_sha256": p_sha, "patched_size": p_size}),
        );
        names.push(name.clone());
    }
    let mut m = Map::new();
    m.insert("schema".into(), json!(SCHEMA));
    m.insert("profile".into(), json!(p.name));
    m.insert("commit".into(), json!(commit));
    m.insert("built_at".into(), json!(utc_now()));
    if let (false, Some(a)) = (files.is_empty(), archive) {
        m.insert("archive".into(), write_archive(a, outdir, &names)?);
    }
    m.insert("files".into(), Value::Array(files));
    Ok(m)
}

pub fn run(p: &Profile, outdir: Option<&str>, commit: Option<&str>, archive: &str) -> Result<Code> {
    let outdir: PathBuf = outdir
        .map(PathBuf::from)
        .unwrap_or_else(|| p.patched_dir.clone());
    let commit = commit
        .map(String::from)
        .unwrap_or_else(|| current_commit(&p.root));
    let m = build(p, &outdir, &commit, Some(Path::new(archive)))?;
    let files = m["files"].as_array().cloned().unwrap_or_default();
    if files.is_empty() {
        crate::exit!(
            "tidak ada file hasil patch di {}, jalankan tools/patch_all.py dulu",
            outdir.display()
        );
    }
    let path = outdir.join(NAME);
    // json.dump(m, f, indent=2) + "\n" (ASCII only, which serde's pretty output is here)
    let text = serde_json::to_string_pretty(&Value::Object(m.clone()))? + "\n";
    std::fs::write(&path, text).with_context(|| format!("{}", path.display()))?;
    for e in &files {
        let s = |k: &str| e[k].as_str().unwrap_or_default().to_string();
        println!(
            "{:<36} original={}  patched={}",
            s("name"),
            &s("original_sha256")[..12],
            &s("patched_sha256")[..12]
        );
    }
    let a = &m["archive"];
    println!(
        "{:<36} sha256={}  size={}",
        a["name"].as_str().unwrap_or_default(),
        &a["sha256"].as_str().unwrap_or_default()[..12],
        a["size"]
    );
    println!("-> {}", path.display());
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{get, testutil};

    #[test]
    fn manifest_and_deterministic_zip() {
        let d = testutil::repo();
        testutil::write(d.path(), "translate_words_map_en", "orig");
        testutil::write(d.path(), "patched/translate_words_map_en", "patched!");
        testutil::write(
            d.path(),
            "patched/translate_words_map_en_diff",
            "no original -> skipped",
        );
        let p = get(d.path(), "global").unwrap();
        let out = d.path().join("patched");
        let z1 = d.path().join("a.zip");
        let m = build(&p, &out, "abc", Some(&z1)).unwrap();
        let keys: Vec<&str> = m.keys().map(String::as_str).collect();
        assert_eq!(
            keys,
            [
                "schema", "profile", "commit", "built_at", "archive", "files"
            ]
        );
        assert_eq!(m["files"].as_array().unwrap().len(), 1);
        let f = &m["files"][0];
        assert_eq!(
            f["original_sha256"],
            format!("{:x}", Sha256::digest(b"orig"))
        );
        assert_eq!(f["patched_sha256"].as_str().unwrap().len(), 64);
        assert_eq!(f["original_size"], 4);
        assert_eq!(f["patched_size"], 8);
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let z2 = d.path().join("b.zip");
        build(&p, &out, "abc", Some(&z2)).unwrap();
        assert_eq!(
            digest(&z1).unwrap(),
            digest(&z2).unwrap(),
            "zip must not depend on build time"
        );
        // zip content: one flat, stored entry
        let mut za = zip::ZipArchive::new(std::fs::File::open(&z1).unwrap()).unwrap();
        assert_eq!(za.len(), 1);
        let mut e = za.by_index(0).unwrap();
        assert_eq!(e.name().unwrap(), "translate_words_map_en");
        assert_eq!(e.compression(), zip::CompressionMethod::Stored);
        let mut s = String::new();
        e.read_to_string(&mut s).unwrap();
        assert_eq!(s, "patched!");
    }

    #[test]
    fn pretty_json_matches_python_indent_2() {
        let mut m = Map::new();
        m.insert("schema".into(), json!(2));
        m.insert("files".into(), json!([]));
        m.insert("archive".into(), json!({"name": "x.zip", "size": 3}));
        assert_eq!(
            serde_json::to_string_pretty(&Value::Object(m)).unwrap(),
            "{\n  \"schema\": 2,\n  \"files\": [],\n  \"archive\": {\n    \"name\": \"x.zip\",\n    \"size\": 3\n  }\n}"
        );
    }
}
