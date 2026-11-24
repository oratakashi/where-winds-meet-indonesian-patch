//! Checks the UI locale files (`locales/*.json`) against each other and against the code, and
//! embeds the app icon and version info into the Windows executable.
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    check_locales();
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("icons/icon.ico");
        res.set("ProductName", "WWM Patch Guard");
        res.set("FileDescription", "WWM Patch Guard");
        res.compile().expect("embed Windows resources");
    }
}

/// Fail the build when
/// - `id.json` and `en.json` do not have the same keys,
/// - a key's `{placeholders}` differ between the two languages,
/// - the code uses `t!("key")` / `tf!("key", …)` with a key that is in neither file,
/// - `t!` is used for a text with placeholders, or `tf!` passes other names than the text has.
fn check_locales() {
    println!("cargo:rerun-if-changed=locales");
    println!("cargo:rerun-if-changed=src");
    let load = |name: &str| -> BTreeMap<String, String> {
        let path = format!("locales/{name}.json");
        let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path}: JSON tidak valid: {e}"))
    };
    let id = load("id");
    let en = load("en");
    let mut errors = Vec::new();

    for k in id.keys().filter(|k| !en.contains_key(*k)) {
        errors.push(format!("locales: key \"{k}\" ada di id.json tapi tidak ada di en.json"));
    }
    for k in en.keys().filter(|k| !id.contains_key(*k)) {
        errors.push(format!("locales: key \"{k}\" ada di en.json tapi tidak ada di id.json"));
    }
    for (k, v) in &id {
        if let Some(e) = en.get(k) {
            let (a, b) = (placeholders(v), placeholders(e));
            if a != b {
                errors.push(format!("locales: placeholder \"{k}\" beda — id {a:?}, en {b:?}"));
            }
        }
    }

    let mut sources = Vec::new();
    rs_files(Path::new("src"), &mut sources);
    let mut all_code = String::new();
    for path in &sources {
        let src = fs::read_to_string(path).unwrap_or_default();
        for call in macro_calls(&src) {
            let at = format!("{}:{}", path.display(), call.line);
            let Some(text) = id.get(&call.key) else {
                errors.push(format!("{at}: key \"{}\" tidak ada di locales/*.json", call.key));
                continue;
            };
            let want = placeholders(text);
            if !call.formatted && !want.is_empty() {
                errors.push(format!("{at}: \"{}\" punya placeholder {want:?} — pakai tf!", call.key));
            }
            if call.formatted && call.args != want {
                errors.push(format!(
                    "{at}: tf!(\"{}\") mengirim {:?}, teksnya butuh {want:?}",
                    call.key, call.args
                ));
            }
        }
        all_code.push_str(&src);
    }
    for k in id.keys().filter(|k| !all_code.contains(&format!("\"{k}\""))) {
        println!("cargo:warning=locales: key \"{k}\" tidak dipakai di kode");
    }
    if !errors.is_empty() {
        panic!("pemeriksaan locale gagal:\n  {}", errors.join("\n  "));
    }
}

fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            rs_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// `{name}` placeholders of a text.
fn placeholders(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut rest = text;
    while let Some(a) = rest.find('{') {
        let Some(b) = rest[a..].find('}') else { break };
        let name = &rest[a + 1..a + b];
        if is_ident(name) {
            out.insert(name.to_string());
        }
        rest = &rest[a + b + 1..];
    }
    out
}

fn is_ident(s: &str) -> bool {
    let mut c = s.chars();
    c.next().is_some_and(|f| f.is_ascii_alphabetic() || f == '_')
        && c.all(|x| x.is_ascii_alphanumeric() || x == '_')
}

struct Call {
    line: usize,
    key: String,
    formatted: bool,
    args: BTreeSet<String>,
}

/// Every `t!("key")` / `tf!("key", name = …)` in code: string, raw-string and char literals
/// and `//` / `/* */` comments are skipped, so a `t!(…)` written inside one is not a call. A
/// `t!` nested in a `tf!` argument is a call of its own and is checked too.
fn macro_calls(src: &str) -> Vec<Call> {
    let b = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if let Some(next) = skip_non_code(b, i) {
            i = next;
            continue;
        }
        let ident_before = i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
        let (formatted, len) = if ident_before {
            (false, 0)
        } else if b[i..].starts_with(b"tf!(") {
            (true, 4)
        } else if b[i..].starts_with(b"t!(") {
            (false, 3)
        } else {
            (false, 0)
        };
        if len == 0 {
            i += 1;
            continue;
        }
        let mut j = i + len;
        while j < b.len() && b[j].is_ascii_whitespace() {
            j += 1;
        }
        if j >= b.len() || b[j] != b'"' {
            i += len;
            continue;
        }
        let key_end = skip_str(b, j);
        let key = src[j + 1..key_end.saturating_sub(1).max(j + 1)].to_string();
        // The arguments up to the matching `)`, split at top-level commas.
        let (mut k, mut depth, mut part, mut parts) = (key_end, 0i32, String::new(), Vec::new());
        while k < b.len() {
            if let Some(next) = skip_non_code(b, k) {
                part.push_str(&src[k..next]);
                k = next;
                continue;
            }
            let c = b[k];
            match c {
                b'(' | b'[' | b'{' => depth += 1,
                b')' if depth == 0 => break,
                b')' | b']' | b'}' => depth -= 1,
                b',' if depth == 0 => {
                    parts.push(std::mem::take(&mut part));
                    k += 1;
                    continue;
                }
                _ => {}
            }
            // Only ASCII matters here (argument names); keep the rest char-safe.
            let ch_len = src[k..].chars().next().map_or(1, char::len_utf8);
            part.push_str(&src[k..k + ch_len]);
            k += ch_len;
        }
        parts.push(part);
        let args = parts
            .iter()
            .filter_map(|p| p.split_once('=').map(|(n, _)| n.trim().to_string()))
            .filter(|n| is_ident(n))
            .collect();
        out.push(Call { line: src[..i].matches('\n').count() + 1, key, formatted, args });
        i = key_end;
    }
    out
}

/// If a comment or a string / raw-string / char literal starts at `i`, the index right after
/// it; `None` when `i` is ordinary code.
fn skip_non_code(b: &[u8], i: usize) -> Option<usize> {
    let ident_before = i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
    match b[i] {
        b'/' if b.get(i + 1) == Some(&b'/') => {
            Some(b[i..].iter().position(|&c| c == b'\n').map_or(b.len(), |p| i + p))
        }
        b'/' if b.get(i + 1) == Some(&b'*') => {
            Some(b[i + 2..].windows(2).position(|w| w == b"*/").map_or(b.len(), |p| i + 2 + p + 2))
        }
        b'"' => Some(skip_str(b, i)),
        // r"…", r#"…"#, br"…" (the `b` is an ident char before the `r`, so allow it)
        b'r' if !ident_before || (i > 0 && b[i - 1] == b'b' && (i < 2 || !is_ident_byte(b[i - 2]))) => {
            let mut j = i + 1;
            while j < b.len() && b[j] == b'#' {
                j += 1;
            }
            if j >= b.len() || b[j] != b'"' {
                return None;
            }
            let hashes = j - i - 1;
            let mut close = Vec::with_capacity(hashes + 1);
            close.push(b'"');
            close.extend(std::iter::repeat_n(b'#', hashes));
            Some(b[j + 1..].windows(close.len()).position(|w| w == close.as_slice()).map_or(b.len(), |p| j + 1 + p + close.len()))
        }
        // A char literal ('x', '\n', '"'); a lifetime ('a, 'static) is left as code.
        b'\'' => {
            if b.get(i + 1) == Some(&b'\\') {
                let end = b[i + 2..].iter().position(|&c| c == b'\'')?;
                Some(i + 2 + end + 1)
            } else {
                // Length of the UTF-8 char after the quote, from its lead byte.
                let lead = *b.get(i + 1)?;
                let len = match lead {
                    0x00..=0x7f => 1,
                    0xc0..=0xdf => 2,
                    0xe0..=0xef => 3,
                    _ => 4,
                };
                let after = i + 1 + len;
                (b.get(after) == Some(&b'\'')).then_some(after + 1)
            }
        }
        _ => None,
    }
}

fn is_ident_byte(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

/// `b[i]` is the opening `"` of a normal string literal: the index right after its closing `"`.
fn skip_str(b: &[u8], i: usize) -> usize {
    let mut j = i + 1;
    while j < b.len() {
        match b[j] {
            b'\\' => j += 2,
            b'"' => return j + 1,
            _ => j += 1,
        }
    }
    b.len()
}
