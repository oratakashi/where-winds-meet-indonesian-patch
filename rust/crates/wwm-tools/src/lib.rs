//! Rust port of the repo's Python tooling. Every public `run_*` function mirrors one
//! script (same arguments, same output, same exit code); `wwm-cli` only parses argv.
//!
//! | module | Python |
//! |---|---|
//! | [`locmap`] | `wwm_locmap.py` info / dump / patch |
//! | [`qa`] | `tools/qa_check.py` |
//! | [`dict`] | `tools/expand_locale.py` (dictionary + `expand`) |
//! | [`bridge`] | `tools/bridge.py` |
//! | [`progress`] | `tools/progress.py` |
//! | [`patch_all`] | `tools/patch_all.py` |
//! | [`manifest`] | `tools/make_manifest.py` |
//! | [`unique`] | `tools/rebuild_unique_strings.py`, `translation_work/build_unique.py` |
//! | [`compare`] | `.claude/skills/wwm-game-update/scripts/compare_update.py` |
//! | [`hook`] | `tools/hooks/claude_stop_check.py` |
//! | [`cn`] | `cn/tools/term_lookup.py`, `name_check.py`, `find_dict_in_dump.py` |

pub mod bridge;
pub mod cn;
pub mod compare;
pub mod dict;
pub mod dump;
pub mod hook;
pub mod locmap;
pub mod manifest;
pub mod patch_all;
pub mod profiles;
pub mod progress;
pub mod py;
pub mod qa;
pub mod unique;

/// Exit code of a command (0 = ok), like the Python scripts' `sys.exit(main())`.
pub type Code = i32;

/// `sys.exit("message")` in Python: message on stderr, exit code 1.
#[derive(Debug)]
pub struct Exit(pub String);

impl std::fmt::Display for Exit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Exit {}

/// Shorthand for `return Err(Exit(...).into())`.
#[macro_export]
macro_rules! exit {
    ($($t:tt)*) => { return Err($crate::Exit(format!($($t)*)).into()) };
}
