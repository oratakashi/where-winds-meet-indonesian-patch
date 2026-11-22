//! Where builds come from: the repo's rolling GitHub releases (`latest`, `cn-latest`).
//! Plain asset URLs — no GitHub API, so no rate limit and no token.
//!
//! A release carries two assets the app reads: `manifest.json` and one zip
//! (`Manifest::archive`) holding every patched file flat, by name.
use crate::model::{Archive, MANIFEST_SCHEMA, Manifest, Profile};
use std::fs;
use std::io;
use std::path::Path;
use std::time::Duration;

pub trait Remote {
    fn manifest(&self, profile: Profile) -> Result<Manifest, String>;
    /// Download the release zip `archive` to `dest`. The caller verifies the hash.
    fn download(&self, profile: Profile, archive: &Archive, dest: &Path) -> Result<(), String>;
}

const DEFAULT_BASE: &str =
    "https://github.com/oratakashi/where-winds-meet-indonesian-patch/releases/download";

pub struct HttpRemote {
    base: String,
    agent: ureq::Agent,
}

impl HttpRemote {
    /// `WWM_PATCH_GUARD_BASE_URL` points the app at another server (local testing):
    /// it must serve `<base>/<tag>/manifest.json` and `<base>/<tag>/<archive name>`.
    pub fn new() -> HttpRemote {
        let base = std::env::var("WWM_PATCH_GUARD_BASE_URL")
            .ok()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| DEFAULT_BASE.to_string());
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(15))
            .timeout_read(Duration::from_secs(60))
            .user_agent(concat!("wwm-patch-guard/", env!("CARGO_PKG_VERSION")))
            .build();
        HttpRemote { base: base.trim_end_matches('/').to_string(), agent }
    }

    fn url(&self, profile: Profile, name: &str) -> String {
        format!("{}/{}/{}", self.base, profile.release_tag(), name)
    }
}

impl Remote for HttpRemote {
    fn manifest(&self, profile: Profile) -> Result<Manifest, String> {
        let resp = self
            .agent
            .get(&self.url(profile, "manifest.json"))
            .call()
            .map_err(|e| match e {
                ureq::Error::Status(404, _) => "rilis belum punya manifest.json".to_string(),
                e => e.to_string(),
            })?;
        let v: serde_json::Value = serde_json::from_reader(resp.into_reader())
            .map_err(|e| format!("manifest.json tidak terbaca: {e}"))?;
        parse_manifest(v, profile)
    }

    fn download(&self, profile: Profile, archive: &Archive, dest: &Path) -> Result<(), String> {
        let resp =
            self.agent.get(&self.url(profile, &archive.name)).call().map_err(|e| e.to_string())?;
        let mut out = fs::File::create(dest).map_err(|e| e.to_string())?;
        // Never trust the server for the length: stop a little past what the manifest says.
        let mut body = io::Read::take(resp.into_reader(), archive.size + 1);
        io::copy(&mut body, &mut out).map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// Check the schema before the shape, so an older/newer release gets a clear message
/// instead of a "missing field" error.
fn parse_manifest(v: serde_json::Value, profile: Profile) -> Result<Manifest, String> {
    let schema = v.get("schema").and_then(serde_json::Value::as_u64).unwrap_or(0);
    if schema != MANIFEST_SCHEMA as u64 {
        return Err(format!("manifest skema {schema} tidak didukung — perbarui Patch Guard"));
    }
    let m: Manifest =
        serde_json::from_value(v).map_err(|e| format!("manifest.json tidak terbaca: {e}"))?;
    if m.profile != profile.key() {
        return Err(format!("manifest untuk profil lain ({})", m.profile));
    }
    Ok(m)
}

/// Copy entry `name` of the zip at `zip` to `dest`, reading at most `limit + 1` bytes
/// (the caller compares the hash against the manifest).
pub fn extract(zip: &Path, name: &str, limit: u64, dest: &Path) -> Result<(), String> {
    let f = fs::File::open(zip).map_err(|e| e.to_string())?;
    let mut z = zip::ZipArchive::new(io::BufReader::new(f))
        .map_err(|e| format!("zip rilis rusak: {e}"))?;
    let entry = z.by_name(name).map_err(|_| format!("{name} tidak ada di zip rilis"))?;
    let mut out = fs::File::create(dest).map_err(|e| e.to_string())?;
    io::copy(&mut io::Read::take(entry, limit + 1), &mut out).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
pub(crate) fn zip_of(files: &[(&str, &[u8])]) -> Vec<u8> {
    use std::io::Write;
    let mut w = zip::ZipWriter::new(io::Cursor::new(Vec::new()));
    for (name, data) in files {
        w.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
        w.write_all(data).unwrap();
    }
    w.finish().unwrap().into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_only_the_named_entry() {
        let tmp = tempfile::tempdir().unwrap();
        let z = tmp.path().join("p.zip");
        fs::write(&z, zip_of(&[("a", b"alpha"), ("b", b"bravo")])).unwrap();
        let out = tmp.path().join("out");
        extract(&z, "b", 5, &out).unwrap();
        assert_eq!(fs::read(&out).unwrap(), b"bravo");
        assert!(extract(&z, "missing", 5, &out).unwrap_err().contains("tidak ada"));
    }

    #[test]
    fn extraction_stops_past_the_expected_size() {
        let tmp = tempfile::tempdir().unwrap();
        let z = tmp.path().join("p.zip");
        fs::write(&z, zip_of(&[("a", &[7u8; 1000])])).unwrap();
        let out = tmp.path().join("out");
        extract(&z, "a", 10, &out).unwrap();
        assert_eq!(fs::read(&out).unwrap().len(), 11);
    }

    #[test]
    fn not_a_zip_is_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        let z = tmp.path().join("p.zip");
        fs::write(&z, b"<html>not found</html>").unwrap();
        assert!(extract(&z, "a", 5, &tmp.path().join("out")).is_err());
    }

    #[test]
    fn old_schema_asks_for_an_app_update() {
        let v = json!({"schema": 1, "profile": "global", "files": []});
        let e = parse_manifest(v, Profile::Global).unwrap_err();
        assert!(e.contains("skema 1") && e.contains("perbarui"), "{e}");
    }

    #[test]
    fn schema_2_manifest_parses() {
        let v = json!({
            "schema": 2, "profile": "global",
            "archive": {"name": "p.zip", "sha256": "ab", "size": 3},
            "files": [{"name": "x", "original_sha256": "o", "original_size": 1,
                       "patched_sha256": "p", "patched_size": 2}]
        });
        let m = parse_manifest(v.clone(), Profile::Global).unwrap();
        assert_eq!(m.archive.name, "p.zip");
        assert!(parse_manifest(v, Profile::Cn).is_err());
    }
}
