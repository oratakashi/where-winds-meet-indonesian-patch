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

/// Every `t!("key")` / `tf!("key", name = …)` outside `//` comments.
fn macro_calls(src: &str) -> Vec<Call> {
    let b = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let (formatted, len) = if b[i..].starts_with(b"tf!(") {
            (true, 4)
        } else if b[i..].starts_with(b"t!(") {
            (false, 3)
        } else {
            i += 1;
            continue;
        };
        let ident_before = i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
        let line_start = src[..i].rfind('\n').map_or(0, |p| p + 1);
        let comment = src[line_start..i].trim_start().starts_with("//");
        let mut j = i + len;
        while j < b.len() && b[j].is_ascii_whitespace() {
            j += 1;
        }
        if ident_before || comment || j >= b.len() || b[j] != b'"' {
            i += len;
            continue;
        }
        let Some(end) = src[j + 1..].find('"') else { break };
        let key = src[j + 1..j + 1 + end].to_string();
        let mut k = j + 1 + end + 1;
        // The arguments up to the matching `)`, split at top-level commas.
        let (mut depth, mut part, mut parts) = (0i32, String::new(), Vec::new());
        while k < b.len() {
            let c = b[k] as char;
            match c {
                '"' => {
                    let close = src[k + 1..].find('"').map_or(b.len(), |p| k + 1 + p);
                    part.push_str(&src[k..=close.min(b.len() - 1)]);
                    k = close + 1;
                    continue;
                }
                '(' | '[' | '{' => depth += 1,
                ')' if depth == 0 => break,
                ')' | ']' | '}' => depth -= 1,
                ',' if depth == 0 => {
                    parts.push(std::mem::take(&mut part));
                    k += 1;
                    continue;
                }
                _ => {}
            }
            part.push(c);
            k += 1;
        }
        parts.push(part);
        let args = parts
            .iter()
            .filter_map(|p| p.split_once('=').map(|(n, _)| n.trim().to_string()))
            .filter(|n| is_ident(n))
            .collect();
        out.push(Call { line: src[..i].matches('\n').count() + 1, key, formatted, args });
        i = k;
    }
    out
}
