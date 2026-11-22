//! `wwm hook stop` — port of `tools/hooks/claude_stop_check.py` (Claude Code Stop hook).
//!
//! If locale/ (or cn/locale/) has uncommitted changes and that profile's progress block
//! is stale, print `{"decision": "block", ...}` so Claude runs `progress --write` and
//! reports the overall line before ending the turn.

use crate::Code;
use crate::profiles;
use crate::progress::is_stale;
use crate::py::Obj;
use anyhow::Result;
use std::io::Read;
use std::path::Path;

/// The block reason for one profile, or None when nothing is pending.
pub fn check(root: &Path, payload: &str) -> Result<Option<String>> {
    let v: serde_json::Value = serde_json::from_str(payload).unwrap_or(serde_json::Value::Null);
    // already blocked once this turn: never lock Claude in a loop
    if v.get("stop_hook_active")
        .and_then(|x| x.as_bool())
        .unwrap_or(false)
    {
        return Ok(None);
    }
    for (name, folder) in [("global", "locale"), ("cn", "cn/locale")] {
        let out = std::process::Command::new("git")
            .args(["status", "--porcelain", "--", folder])
            .current_dir(root)
            .output();
        let changed = out
            .map(|o| !String::from_utf8_lossy(&o.stdout).trim().is_empty())
            .unwrap_or(false);
        if !changed {
            continue;
        }
        let p = profiles::get(root, name)?;
        if !is_stale(&p)? {
            continue;
        }
        let flag = if name == "global" {
            String::new()
        } else {
            format!(" --profile {name}")
        };
        let reason = format!(
            "{folder}/ changed this session but the generated progress block in {} is stale. \
             Run `sh tools/wwm.sh progress{flag} --write` (or `python tools/progress.py{flag} --write`), \
             then include its overall line (unique strings done/total/% and in-game coverage %) \
             in the status/session write-up and in your final message to the user \
             (Resume-Procedure step 8).",
            p.status_rel
        );
        return Ok(Some(
            Obj::ascii()
                .str("decision", "block")
                .str("reason", &reason)
                .finish(),
        ));
    }
    Ok(None)
}

pub fn stop(root: &Path) -> Result<Code> {
    let mut payload = String::new();
    let _ = std::io::stdin().read_to_string(&mut payload);
    if let Some(out) = check(root, &payload)? {
        println!("{out}");
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_flag_passes_immediately() {
        // no repo needed: returns before touching git
        assert_eq!(
            check(Path::new("does-not-exist"), r#"{"stop_hook_active": true}"#).unwrap(),
            None
        );
    }

    #[test]
    fn no_locale_changes_passes() {
        let d = crate::profiles::testutil::repo();
        // not a git repo -> git prints nothing on stdout -> unchanged
        assert_eq!(check(d.path(), "{}").unwrap(), None);
        assert_eq!(check(d.path(), "not json").unwrap(), None);
    }
}
