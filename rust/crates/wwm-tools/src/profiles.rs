//! Profiles (`global`, `cn`) from `tools/profiles.json` — the same file `tools/profiles.py` reads.

use crate::py::join;
use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// (label, file in locale dir, first idx, last idx inclusive)
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Phase(pub String, pub String, pub i64, pub i64);

/// (region, phase tag, [(phase label, part name)])
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Region(pub String, pub String, pub Vec<(String, String)>);

#[derive(Debug, Clone, Deserialize)]
struct Raw {
    base: String,
    phases: Vec<Phase>,
    #[serde(default)]
    regions: Vec<Region>,
    files: Vec<String>,
    dumps: Vec<(String, String)>,
    status: String,
    coverage: String,
    corpus_label: String,
    patched_dir: String,
    translated_out: String,
    bridge_from: Option<String>,
    overrides: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub root: PathBuf,
    pub base: PathBuf,
    pub unique: PathBuf,
    pub locale_dir: PathBuf,
    pub phases: Vec<Phase>,
    pub regions: Vec<Region>,
    pub files: Vec<String>,
    /// (binary, dump) relative to `base` ('/'-separated)
    pub dumps: Vec<(String, String)>,
    pub status: PathBuf,
    pub status_rel: String,
    pub coverage: String,
    pub corpus_label: String,
    pub patched_dir: PathBuf,
    pub translated_out: PathBuf,
    pub bridge_from: Option<String>,
    pub overrides: Option<PathBuf>,
    /// repo-relative '/' path of `base` ("" for the root)
    pub base_rel: String,
}

pub const LOCALE_PATTERNS: [&str; 2] = ["phase*.jsonl", "update*.jsonl"];

impl Profile {
    /// `P.path(rel)`.
    pub fn path(&self, rel: &str) -> PathBuf {
        join(&self.base, rel)
    }
    /// `P.rel(P.path(rel))`: repo-relative, '/'-separated.
    pub fn rel(&self, rel: &str) -> String {
        if self.base_rel.is_empty() {
            rel.to_string()
        } else {
            format!("{}/{}", self.base_rel, rel)
        }
    }
    pub fn locale_rel(&self) -> String {
        self.rel("locale")
    }
}

/// Repo root: `$WWM_ROOT`, else the first ancestor of the current directory (then of
/// the executable) that holds `tools/profiles.json`.
pub fn root() -> Result<PathBuf> {
    if let Some(r) = std::env::var_os("WWM_ROOT").filter(|r| !r.is_empty()) {
        return Ok(PathBuf::from(r));
    }
    let mut starts = vec![std::env::current_dir()?];
    if let Ok(exe) = std::env::current_exe() {
        starts.push(exe);
    }
    for s in starts {
        for a in s.ancestors() {
            if a.join("tools").join("profiles.json").is_file() {
                return Ok(a.to_path_buf());
            }
        }
    }
    bail!(
        "root repo tidak ditemukan (tools/profiles.json) — jalankan dari dalam repo atau set WWM_ROOT"
    )
}

pub fn load_all(root: &Path) -> Result<Vec<Profile>> {
    let path = root.join("tools").join("profiles.json");
    let text = std::fs::read_to_string(&path).with_context(|| format!("{}", path.display()))?;
    let map: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(&text).with_context(|| format!("{}", path.display()))?;
    let mut out = Vec::new();
    for (name, v) in map {
        if name.starts_with('_') {
            continue;
        }
        let r: Raw = serde_json::from_value(v)
            .with_context(|| format!("{}: profil {name}", path.display()))?;
        let base = join(root, &r.base);
        let base_rel = if r.base == "." {
            String::new()
        } else {
            r.base.trim_end_matches('/').to_string()
        };
        out.push(Profile {
            name,
            root: root.to_path_buf(),
            unique: base.join("translation_work").join("unique_strings.jsonl"),
            locale_dir: base.join("locale"),
            phases: r.phases,
            regions: r.regions,
            files: r.files,
            dumps: r.dumps,
            status: join(root, &r.status),
            status_rel: r.status,
            coverage: r.coverage,
            corpus_label: r.corpus_label,
            patched_dir: join(root, &r.patched_dir),
            translated_out: join(root, &r.translated_out),
            bridge_from: r.bridge_from,
            overrides: r.overrides.map(|o| join(root, &o)),
            base,
            base_rel,
        });
    }
    Ok(out)
}

pub fn get(root: &Path, name: &str) -> Result<Profile> {
    let all = load_all(root)?;
    let names: Vec<String> = all.iter().map(|p| p.name.clone()).collect();
    all.into_iter()
        .find(|p| p.name == name)
        .with_context(|| format!("profil tak dikenal: {name} (pilihan: {})", names.join(", ")))
}

#[cfg(test)]
pub(crate) mod testutil {
    //! A throw-away repo with a minimal profiles.json, for unit tests.
    use std::path::Path;

    pub const PROFILES: &str = r#"{
  "_comment": ["test"],
  "global": {
    "base": ".",
    "phases": [["0", "phase0.jsonl", 0, 2], ["1", "phase1.jsonl", 3, 5], ["Update-1A", "update1a.jsonl", 6, 7], ["Update-1B", "update1b.jsonl", 8, 9]],
    "regions": [["Start", "Phase 0–1", [["0", "Phase 0"], ["1", "Phase 1"]]], ["Next", "Update-1", [["Update-1A", "Part A"], ["Update-1B", "Part B"]]]],
    "files": ["translate_words_map_en", "translate_words_map_en_diff"],
    "dumps": [["translate_words_map_en", "strings.jsonl"], ["translate_words_map_en_diff", "strings_diff.jsonl"]],
    "status": "Obsidian-Vault/progress/Current-Status.md",
    "coverage": "dump",
    "corpus_label": "original corpus (Phases 0–1)",
    "patched_dir": "patched",
    "translated_out": "strings.translated.jsonl",
    "bridge_from": null,
    "overrides": null
  },
  "cn": {
    "base": "cn",
    "phases": [["1", "phase1.jsonl", 0, 2]],
    "regions": [],
    "files": ["translate_words_map_zh_cn"],
    "dumps": [["translate_words_map_zh_cn", "work/strings_zh_cn.jsonl"]],
    "status": "Obsidian-Vault/cn/progress/CN-Current-Status.md",
    "coverage": "freq",
    "corpus_label": "CN-only corpus (Phases 1–1)",
    "patched_dir": "cn/work/patched",
    "translated_out": "cn/work/strings.translated.jsonl",
    "bridge_from": "global",
    "overrides": "cn/overrides.jsonl"
  }
}"#;

    pub fn write(root: &Path, rel: &str, text: &str) {
        let p = crate::py::join(root, rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, text).unwrap();
    }

    pub fn repo() -> tempfile::TempDir {
        let d = tempfile::tempdir().unwrap();
        write(d.path(), "tools/profiles.json", PROFILES);
        d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_profiles_json() {
        let d = testutil::repo();
        let g = get(d.path(), "global").unwrap();
        assert_eq!(g.base, d.path());
        assert_eq!(g.phases[1], Phase("1".into(), "phase1.jsonl".into(), 3, 5));
        assert_eq!(g.regions[0].2[1], ("1".to_string(), "Phase 1".to_string()));
        assert_eq!(
            g.unique,
            d.path()
                .join("translation_work")
                .join("unique_strings.jsonl")
        );
        assert_eq!(g.rel("strings.jsonl"), "strings.jsonl");
        assert_eq!(g.locale_rel(), "locale");
        let c = get(d.path(), "cn").unwrap();
        assert_eq!(
            c.path("work/strings_zh_cn.jsonl"),
            d.path().join("cn").join("work").join("strings_zh_cn.jsonl")
        );
        assert_eq!(
            c.rel("work/strings_zh_cn.jsonl"),
            "cn/work/strings_zh_cn.jsonl"
        );
        assert_eq!(
            c.overrides.as_deref(),
            Some(d.path().join("cn").join("overrides.jsonl").as_path())
        );
        assert!(get(d.path(), "jp").is_err());
    }

    #[test]
    fn repo_profiles_json_parses() {
        // the real tools/profiles.json must always load
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let all = load_all(&root).unwrap();
        assert!(all.iter().any(|p| p.name == "global"));
        assert!(all.iter().any(|p| p.name == "cn"));
        for p in &all {
            for w in p.phases.windows(2) {
                assert_eq!(
                    w[0].3 + 1,
                    w[1].2,
                    "{}: phases {} -> {} not contiguous",
                    p.name,
                    w[0].0,
                    w[1].0
                );
            }
        }
    }
}
