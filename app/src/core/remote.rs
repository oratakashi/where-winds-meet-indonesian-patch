//! Where builds come from: the repo's rolling GitHub releases (`latest`, `cn-latest`).
//! Plain asset URLs — no GitHub API, so no rate limit and no token.
//!
//! A release carries the assets the app reads: `manifest.json`, its minisign signature
//! `manifest.json.sig` (checked before anything in the manifest is trusted) and one zip
//! (`Manifest::archive`) holding every patched file flat, by name. The zip is trusted through
//! its sha256 in the signed manifest.
use crate::model::{Archive, MANIFEST_SCHEMA, Manifest, Profile};
use crate::store::is_sha256_hex;
use crate::update;
use std::fs;
use std::io;
use std::path::Path;
use std::time::Duration;

pub trait Remote {
    fn manifest(&self, profile: Profile) -> Result<Manifest, String>;
    /// Download the release zip `archive` to `dest`, calling `on_progress` with the bytes
    /// written so far. The caller verifies the hash.
    fn download(
        &self,
        profile: Profile,
        archive: &Archive,
        dest: &Path,
        on_progress: &mut dyn FnMut(u64),
    ) -> Result<(), String>;
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
    /// Debug builds: `WWM_PATCH_GUARD_BASE_URL` points the app at another server (local
    /// testing, plain http allowed); it must serve `<base>/<tag>/manifest.json` and
    /// `<base>/<tag>/<archive name>`. Release builds always use GitHub, over https only,
    /// redirects included (GitHub sends downloads on to its asset host).
    pub fn new() -> HttpRemote {
        #[cfg(debug_assertions)]
        let base = std::env::var("WWM_PATCH_GUARD_BASE_URL")
            .ok()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| DEFAULT_BASE.to_string());
        #[cfg(not(debug_assertions))]
        let base = DEFAULT_BASE.to_string();
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(15))
            .timeout_read(Duration::from_secs(60))
            .https_only(!cfg!(debug_assertions))
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
            return Err(Some(tf!("remote.is_too_large", name = name)));
        }
        Ok(out)
    }

    /// Download asset `name` to `dest`, reading at most `size + 1` bytes (the caller verifies).
    /// `on_progress` gets the bytes written so far after every chunk.
    pub fn download_to(
        &self,
        tag: &str,
        name: &str,
        size: u64,
        dest: &Path,
        on_progress: &mut dyn FnMut(u64),
    ) -> Result<(), String> {
        let resp = self.agent.get(&self.url(tag, name)).call().map_err(|e| e.to_string())?;
        let mut out = fs::File::create(dest).map_err(|e| e.to_string())?;
        // Never trust the server for the length: stop a little past what we expect.
        copy_with_progress(resp.into_reader(), &mut out, size + 1, on_progress)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl HttpRemote {
    /// `manifest.json` and its `.sig`, as served right now.
    fn manifest_pair(&self, tag: &str) -> Result<(Vec<u8>, String), String> {
        let json = self.get_bytes(tag, "manifest.json", MANIFEST_MAX).map_err(|e| {
            e.unwrap_or_else(|| t!("common.release_no_manifest").to_string())
        })?;
        let sig = self.get_bytes(tag, "manifest.json.sig", SIG_MAX).map_err(|e| {
            e.unwrap_or_else(|| {
                t!("remote.the_release_is_not").to_string()
            })
        })?;
        Ok((json, String::from_utf8_lossy(&sig).into_owned()))
    }
}

impl Remote for HttpRemote {
    fn manifest(&self, profile: Profile) -> Result<Manifest, String> {
        let tag = profile.release_tag();
        let (json, signed) = with_sig_retry(SIG_TRIES, SIG_RETRY_WAIT, || self.manifest_pair(tag))?;
        let mut m = parse_manifest_json(&json, profile)?;
        m.signed_at = signed.at;
        Ok(m)
    }

    fn download(
        &self,
        profile: Profile,
        archive: &Archive,
        dest: &Path,
        on_progress: &mut dyn FnMut(u64),
    ) -> Result<(), String> {
        self.download_to(profile.release_tag(), &archive.name, archive.size, dest, on_progress)
    }
}

/// Copy at most `limit` bytes from `from` to `to`, calling `on_progress` with the total
/// written after every chunk. Returns the bytes copied.
pub fn copy_with_progress(
    from: impl io::Read,
    to: &mut impl io::Write,
    limit: u64,
    on_progress: &mut dyn FnMut(u64),
) -> io::Result<u64> {
    let mut from = io::Read::take(from, limit);
    let mut buf = vec![0u8; 64 * 1024];
    let mut done = 0u64;
    loop {
        let n = match io::Read::read(&mut from, &mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        to.write_all(&buf[..n])?;
        done += n as u64;
        on_progress(done);
    }
    Ok(done)
}

/// A release is replaced asset by asset, so `manifest.json` and its `.sig` can briefly belong
/// to different builds. A signature mismatch is retried a few times before it counts.
const SIG_TRIES: u32 = 3;
const SIG_RETRY_WAIT: Duration = Duration::from_secs(3);

/// The name `manifest.json`'s signature must have been made for.
const MANIFEST_JSON: &str = "manifest.json";

/// Fetch (manifest, sig) with `fetch` until the signature verifies, at most `tries` times.
/// Network errors are returned at once; only a bad signature is retried.
fn with_sig_retry(
    tries: u32,
    wait: Duration,
    mut fetch: impl FnMut() -> Result<(Vec<u8>, String), String>,
) -> Result<(Vec<u8>, update::Signed), String> {
    let mut last = String::new();
    for i in 0..tries.max(1) {
        if i > 0 {
            std::thread::sleep(wait);
        }
        let (json, sig) = fetch()?;
        match update::verify(&json, &sig, MANIFEST_JSON) {
            Ok(signed) => return Ok((json, signed)),
            Err(e) => last = e,
        }
    }
    Err(tf!("remote.manifest_untrusted_retry", last = last))
}

/// Only a manifest signed with the release key is read at all.
#[cfg(test)]
fn verified_manifest(json: &[u8], sig: &str, profile: Profile) -> Result<Manifest, String> {
    update::verify(json, sig, MANIFEST_JSON).map_err(|e| tf!("remote.manifest_untrusted", e = e))?;
    parse_manifest_json(json, profile)
}

fn parse_manifest_json(json: &[u8], profile: Profile) -> Result<Manifest, String> {
    let v: serde_json::Value =
        serde_json::from_slice(json).map_err(|e| tf!("remote.manifest_unreadable", e = e))?;
    parse_manifest(v, profile)
}

/// Check the schema before the shape, so an older/newer release gets a clear message
/// instead of a "missing field" error.
fn parse_manifest(v: serde_json::Value, profile: Profile) -> Result<Manifest, String> {
    let schema = v.get("schema").and_then(serde_json::Value::as_u64).unwrap_or(0);
    if schema != MANIFEST_SCHEMA as u64 {
        return Err(tf!("remote.manifest_schema_is_not", schema = schema));
    }
    let m: Manifest =
        serde_json::from_value(v).map_err(|e| tf!("remote.manifest_unreadable", e = e))?;
    if m.profile != profile.key() {
        let profile = &m.profile;
        return Err(tf!("remote.manifest_is_for_another", profile = profile));
    }
    // Hashes become file names in the data folder and the zip name part of a URL: even a
    // signed manifest must not carry anything else.
    let bad = |field: &str| Err(tf!("remote.manifest_bad_field", field = field));
    if !is_sha256_hex(&m.archive.sha256) {
        return bad("archive.sha256");
    }
    if !plain_asset_name(&m.archive.name) {
        return bad("archive.name");
    }
    for f in &m.files {
        if !is_sha256_hex(&f.original_sha256) || !is_sha256_hex(&f.patched_sha256) {
            return bad(&f.name);
        }
    }
    Ok(m)
}

fn plain_asset_name(s: &str) -> bool {
    !s.is_empty()
        && !s.contains("..")
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_'))
}

/// Copy entry `name` of the zip at `zip` to `dest`, reading at most `limit + 1` bytes
/// (the caller compares the hash against the manifest).
pub fn extract(zip: &Path, name: &str, limit: u64, dest: &Path) -> Result<(), String> {
    let f = fs::File::open(zip).map_err(|e| e.to_string())?;
    let mut z = zip::ZipArchive::new(io::BufReader::new(f))
        .map_err(|e| tf!("remote.release_zip_is_damaged", e = e))?;
    let entry = z.by_name(name).map_err(|_| tf!("remote.is_not_in_the", name = name))?;
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
    fn copy_reports_progress_and_stops_at_the_limit() {
        let data = vec![7u8; 200 * 1024];
        let mut out = Vec::new();
        let mut seen = Vec::new();
        let n = copy_with_progress(&data[..], &mut out, 150 * 1024, &mut |d| seen.push(d)).unwrap();
        assert_eq!(n, 150 * 1024);
        assert_eq!(out.len(), 150 * 1024);
        assert_eq!(seen.last(), Some(&(150 * 1024)));
        assert!(seen.windows(2).all(|w| w[0] < w[1]), "{seen:?}");
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
        assert!(verified_manifest(json, &sign(json, "manifest.json"), Profile::Global).is_err());
    }

    #[test]
    fn signature_mismatch_is_retried_then_reported() {
        let json = br#"{"schema": 2}"#.to_vec();
        // Always unsigned: every try fails, the error says the release may be mid-update.
        let mut calls = 0;
        let e = with_sig_retry(3, Duration::ZERO, || {
            calls += 1;
            Ok((json.clone(), String::new()))
        })
        .unwrap_err();
        assert_eq!(calls, 3);
        assert!(e.contains("tidak bisa dipercaya") && e.contains("sedang diperbarui"), "{e}");
        // A network error is not retried.
        let mut calls = 0;
        let e = with_sig_retry(3, Duration::ZERO, || {
            calls += 1;
            Err::<(Vec<u8>, String), _>("offline".to_string())
        })
        .unwrap_err();
        assert_eq!((calls, e.as_str()), (1, "offline"));
    }

    #[test]
    fn old_schema_asks_for_an_app_update() {
        let v = json!({"schema": 1, "profile": "global", "files": []});
        let e = parse_manifest(v, Profile::Global).unwrap_err();
        assert!(e.contains("skema 1") && e.contains("perbarui"), "{e}");
    }

    #[test]
    fn schema_2_manifest_parses() {
        let (a, o, p) = ("a".repeat(64), "0".repeat(64), "f".repeat(64));
        let v = json!({
            "schema": 2, "profile": "global",
            "archive": {"name": "p.zip", "sha256": a, "size": 3},
            "files": [{"name": "x", "original_sha256": o, "original_size": 1,
                       "patched_sha256": p, "patched_size": 2}]
        });
        let m = parse_manifest(v.clone(), Profile::Global).unwrap();
        assert_eq!(m.archive.name, "p.zip");
        assert!(parse_manifest(v.clone(), Profile::Cn).is_err());

        // Anything that is not a plain sha256 / file name is refused, signed or not.
        let with = |set: &dyn Fn(&mut serde_json::Value)| {
            let mut v = v.clone();
            set(&mut v);
            parse_manifest(v, Profile::Global)
        };
        assert!(with(&|v| v["archive"]["sha256"] = json!(r"..\..\x")).is_err());
        assert!(with(&|v| v["archive"]["sha256"] = json!("A".repeat(64))).is_err());
        assert!(with(&|v| v["archive"]["name"] = json!("../p.zip")).is_err());
        assert!(with(&|v| v["files"][0]["patched_sha256"] = json!(r"..\config.json")).is_err());
        assert!(with(&|v| v["files"][0]["original_sha256"] = json!("o")).is_err());
    }
}
