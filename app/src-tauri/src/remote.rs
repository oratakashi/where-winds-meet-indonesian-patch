//! Where builds come from: the repo's rolling GitHub releases (`latest`, `cn-latest`).
//! Plain asset URLs — no GitHub API, so no rate limit and no token.
use crate::model::{FileEntry, MANIFEST_SCHEMA, Manifest, Profile};
use std::fs;
use std::io;
use std::path::Path;
use std::time::Duration;

pub trait Remote {
    fn manifest(&self, profile: Profile) -> Result<Manifest, String>;
    /// Download the patched build of `entry` to `dest`. The caller verifies the hash.
    fn download(&self, profile: Profile, entry: &FileEntry, dest: &Path) -> Result<(), String>;
}

const DEFAULT_BASE: &str =
    "https://github.com/oratakashi/where-winds-meet-indonesian-patch/releases/download";

pub struct HttpRemote {
    base: String,
    agent: ureq::Agent,
}

impl HttpRemote {
    /// `WWM_PATCH_GUARD_BASE_URL` points the app at another server (local testing):
    /// it must serve `<base>/<tag>/manifest.json` and `<base>/<tag>/<file name>`.
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
        let m: Manifest = serde_json::from_reader(resp.into_reader())
            .map_err(|e| format!("manifest.json tidak terbaca: {e}"))?;
        if m.schema != MANIFEST_SCHEMA {
            return Err(format!(
                "manifest skema {} tidak didukung — perbarui Patch Guard",
                m.schema
            ));
        }
        if m.profile != profile.key() {
            return Err(format!("manifest untuk profil lain ({})", m.profile));
        }
        Ok(m)
    }

    fn download(&self, profile: Profile, entry: &FileEntry, dest: &Path) -> Result<(), String> {
        let resp = self.agent.get(&self.url(profile, &entry.name)).call().map_err(|e| e.to_string())?;
        let mut out = fs::File::create(dest).map_err(|e| e.to_string())?;
        // Never trust the server for the length: stop a little past what the manifest says.
        let mut body = io::Read::take(resp.into_reader(), entry.patched_size + 1);
        io::copy(&mut body, &mut out).map_err(|e| e.to_string())?;
        Ok(())
    }
}
