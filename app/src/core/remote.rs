//! Where builds come from: the repo's rolling GitHub releases (`latest`, `cn-latest`).
//! Plain asset URLs — no GitHub API, so no rate limit and no token.
//!
//! A release carries the assets the app reads: `manifest.json`, its minisign signature
//! `manifest.json.sig` (checked before anything in the manifest is trusted) and one zip
//! (`Manifest::archive`) holding every patched file flat, by name. The zip is trusted through
//! its sha256 in the signed manifest.
use crate::model::{Archive, MANIFEST_SCHEMA, Manifest, Profile};
use crate::update;
use std::fs;
use std::io;
use std::path::Path;
use std::time::Duration;

pub trait Remote {
    fn manifest(&self, profile: Profile) -> Result<Manifest, String>;
    /// Download the release zip `archive` to `dest`. The caller verifies the hash.
    fn download(&self, profile: Profile, archive: &Archive, dest: &Path) -> Result<(), String>;
}

/// The release Patch Guard updates itself from.
pub const LATEST_TAG: &str = "latest";
const MANIFEST_MAX: u64 = 16 << 20;
const SIG_MAX: u64 = 4096;

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

    fn url(&self, tag: &str, name: &str) -> String {
        format!("{}/{}/{}", self.base, tag, name)
    }

    /// A small asset, at most `limit` bytes. `Err(None)` = the release has no such asset.
    pub fn get_bytes(&self, tag: &str, name: &str, limit: u64) -> Result<Vec<u8>, Option<String>> {
        let resp = self.agent.get(&self.url(tag, name)).call().map_err(|e| match e {
            ureq::Error::Status(404, _) => None,
            e => Some(e.to_string()),
        })?;
        let mut out = Vec::new();
        io::Read::read_to_end(&mut io::Read::take(resp.into_reader(), limit + 1), &mut out)
            .map_err(|e| Some(e.to_string()))?;
        if out.len() as u64 > limit {
            return Err(Some(format!("{name} terlalu besar")));
        }
        Ok(out)
    }

    /// Download asset `name` to `dest`, reading at most `size + 1` bytes (the caller verifies).
    pub fn download_to(&self, tag: &str, name: &str, size: u64, dest: &Path) -> Result<(), String> {
        let resp = self.agent.get(&self.url(tag, name)).call().map_err(|e| e.to_string())?;
        let mut out = fs::File::create(dest).map_err(|e| e.to_string())?;
        // Never trust the server for the length: stop a little past what we expect.
        let mut body = io::Read::take(resp.into_reader(), size + 1);
        io::copy(&mut body, &mut out).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl Remote for HttpRemote {
    fn manifest(&self, profile: Profile) -> Result<Manifest, String> {
        let tag = profile.release_tag();
        let json = self.get_bytes(tag, "manifest.json", MANIFEST_MAX).map_err(|e| {
            e.unwrap_or_else(|| "rilis belum punya manifest.json".to_string())
        })?;
        let sig = self.get_bytes(tag, "manifest.json.sig", SIG_MAX).map_err(|e| {
            e.unwrap_or_else(|| {
                "rilis belum bertanda tangan (manifest.json.sig tidak ada) — tunggu build berikutnya"
                    .to_string()
            })
        })?;
        verified_manifest(&json, &String::from_utf8_lossy(&sig), profile)
    }

    fn download(&self, profile: Profile, archive: &Archive, dest: &Path) -> Result<(), String> {
        self.download_to(profile.release_tag(), &archive.name, archive.size, dest)
    }
}

/// Only a manifest signed with the release key is read at all.
fn verified_manifest(json: &[u8], sig: &str, profile: Profile) -> Result<Manifest, String> {
    update::verify(json, sig).map_err(|e| format!("manifest.json tidak bisa dipercaya: {e}"))?;
    let v: serde_json::Value =
        serde_json::from_slice(json).map_err(|e| format!("manifest.json tidak terbaca: {e}"))?;
    parse_manifest(v, profile)
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
    fn unsigned_or_tampered_manifest_is_rejected() {
        let json = br#"{"schema": 2, "profile": "global", "files": []}"#;
        let e = verified_manifest(json, "", Profile::Global).unwrap_err();
        assert!(e.contains("tidak bisa dipercaya"), "{e}");
        // Signed by some other key: rejected too.
        let (_, sign) = crate::update::tests::test_key();
        assert!(verified_manifest(json, &sign(json), Profile::Global).is_err());
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
