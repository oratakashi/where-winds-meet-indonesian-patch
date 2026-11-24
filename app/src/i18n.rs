//! UI language: Indonesian (default) or English, chosen in Settings and kept in `config.json`.
//!
//! Every user-facing text lives in one place per language, `app/locales/id.json` and
//! `app/locales/en.json` (flat, namespaced keys). The code only names the key:
//!
//! ```ignore
//! let label = t!("common.check_now");               // &'static str
//! let line = tf!("common.version", v = version);    // String, `{v}` filled in
//! ```
//!
//! Both files are embedded in the exe. `build.rs` fails the build when the two files have
//! different keys or placeholders, when the code names a key that does not exist, or when a
//! `tf!` passes other names than the text's `{placeholders}` — so a missing translation is
//! still a build error.
//!
//! The language is process-wide (read by the guard thread too, so status details and history
//! lines come out in the chosen language). Tests override it per thread, so tests that switch
//! language never race the ones that assert on Indonesian text.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    #[default]
    Id,
    En,
}

impl Lang {
    pub const ALL: [Lang; 2] = [Lang::Id, Lang::En];
}

const ID_JSON: &str = include_str!("../locales/id.json");
const EN_JSON: &str = include_str!("../locales/en.json");

type Table = HashMap<String, String>;

/// Indexed by `Lang as usize`. `build.rs` has already checked both files.
static TABLES: LazyLock<[Table; 2]> = LazyLock::new(|| {
    let parse = |json: &str| serde_json::from_str::<Table>(json).unwrap_or_default();
    [parse(ID_JSON), parse(EN_JSON)]
});

static LANG: AtomicU8 = AtomicU8::new(0);

#[cfg(test)]
thread_local! {
    static TEST_LANG: std::cell::Cell<Option<Lang>> = const { std::cell::Cell::new(None) };
}

pub fn get() -> Lang {
    #[cfg(test)]
    if let Some(l) = TEST_LANG.with(|c| c.get()) {
        return l;
    }
    match LANG.load(Ordering::Relaxed) {
        1 => Lang::En,
        _ => Lang::Id,
    }
}

pub fn set(lang: Lang) {
    #[cfg(test)]
    {
        TEST_LANG.with(|c| c.set(Some(lang)));
    }
    #[cfg(not(test))]
    LANG.store(lang as u8, Ordering::Relaxed);
}

/// The text of `key` in `lang`; Indonesian when that language lacks it, else the key itself.
pub fn text_in(lang: Lang, key: &'static str) -> &'static str {
    let tables: &'static [Table; 2] = &TABLES;
    tables[lang as usize]
        .get(key)
        .or_else(|| tables[Lang::Id as usize].get(key))
        .map_or(key, String::as_str)
}

/// The text of `key` in the current language.
pub fn text(key: &'static str) -> &'static str {
    text_in(get(), key)
}

/// The text of `key` with every `{name}` replaced by the matching argument.
pub fn format(key: &'static str, args: &[(&str, &dyn Display)]) -> String {
    let mut out = text(key).to_string();
    for (name, value) in args {
        out = out.replace(&format!("{{{name}}}"), &value.to_string());
    }
    out
}

/// `t!("namespace.key")`: the text in the current language.
#[macro_export]
macro_rules! t {
    ($key:literal $(,)?) => {
        $crate::i18n::text($key)
    };
}

/// `tf!("namespace.key", name = value, …)`: the text with each `{name}` filled in.
#[macro_export]
macro_rules! tf {
    ($key:literal $(, $name:ident = $value:expr)* $(,)?) => {
        $crate::i18n::format(
            $key,
            &[$((stringify!($name), &$value as &dyn ::std::fmt::Display)),*],
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(json: &str) -> Vec<String> {
        let mut k: Vec<String> = serde_json::from_str::<Table>(json).unwrap().into_keys().collect();
        k.sort();
        k
    }

    #[test]
    fn both_locale_files_parse_and_have_the_same_keys() {
        assert!(!keys(ID_JSON).is_empty());
        assert_eq!(keys(ID_JSON), keys(EN_JSON));
    }

    #[test]
    fn texts_follow_the_language() {
        assert_eq!(get(), Lang::Id);
        assert_eq!(t!("common.check_now"), "Periksa sekarang");
        set(Lang::En);
        assert_eq!(t!("common.check_now"), "Check now");
        let v = "0.6.0";
        assert_eq!(tf!("common.version", v = v), "Version 0.6.0");
        set(Lang::Id);
        assert_eq!(tf!("common.version", v = 7), "Versi 7");
        assert_eq!(text_in(Lang::En, "lang.name"), "English");
        assert_eq!(text_in(Lang::Id, "lang.name"), "Indonesia");
    }

    #[test]
    fn unknown_key_shows_the_key() {
        assert_eq!(text("does.not.exist"), "does.not.exist");
    }

    #[test]
    fn serde_names() {
        assert_eq!(serde_json::to_string(&Lang::En).unwrap(), "\"en\"");
        assert_eq!(serde_json::from_str::<Lang>("\"id\"").unwrap(), Lang::Id);
    }
}
