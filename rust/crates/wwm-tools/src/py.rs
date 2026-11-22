//! Python-compatible formatting, so files and output stay byte-identical to the
//! Python scripts: `json.dumps`, `repr`, `"{:,}"`, `"%.2f"`, `glob.glob`, text-mode newlines.

use anyhow::{Context, Result};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// Newline Python writes for `open(path, "w")` without `newline=` (text mode).
pub const NL: &str = if cfg!(windows) { "\r\n" } else { "\n" };

/// Append `s` as a JSON string literal the way `json.dumps` does
/// (`ensure_ascii=False`: only `"` `\` and control characters are escaped).
pub fn json_str(out: &mut String, s: &str, ensure_ascii: bool) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c if ensure_ascii && (c as u32) > 0x7e => {
                let mut buf = [0u16; 2];
                for u in c.encode_utf16(&mut buf) {
                    let _ = write!(out, "\\u{:04x}", u);
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Builder for one `json.dumps(dict)` line: `{"k": v, "k2": v2}` (key order kept).
pub struct Obj {
    buf: String,
    first: bool,
    ascii: bool,
}

impl Obj {
    pub fn new() -> Self {
        Obj {
            buf: String::from("{"),
            first: true,
            ascii: false,
        }
    }
    /// `json.dumps` default (`ensure_ascii=True`).
    pub fn ascii() -> Self {
        Obj {
            buf: String::from("{"),
            first: true,
            ascii: true,
        }
    }
    fn key(&mut self, k: &str) {
        if !self.first {
            self.buf.push_str(", ");
        }
        self.first = false;
        json_str(&mut self.buf, k, self.ascii);
        self.buf.push_str(": ");
    }
    pub fn int(mut self, k: &str, v: i64) -> Self {
        self.key(k);
        let _ = write!(self.buf, "{v}");
        self
    }
    pub fn str(mut self, k: &str, v: &str) -> Self {
        self.key(k);
        json_str(&mut self.buf, v, self.ascii);
        self
    }
    pub fn bool(mut self, k: &str, v: bool) -> Self {
        self.key(k);
        self.buf.push_str(if v { "true" } else { "false" });
        self
    }
    /// Already-encoded JSON value (nested object).
    pub fn raw(mut self, k: &str, v: &str) -> Self {
        self.key(k);
        self.buf.push_str(v);
        self
    }
    pub fn finish(mut self) -> String {
        self.buf.push('}');
        self.buf
    }
}

impl Default for Obj {
    fn default() -> Self {
        Self::new()
    }
}

/// `repr(str)` for the strings this repo prints (file names, hashes, Indonesian text).
pub fn repr_str(s: &str) -> String {
    let q = if s.contains('\'') && !s.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut o = String::with_capacity(s.len() + 2);
    o.push(q);
    for c in s.chars() {
        match c {
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if c == q => {
                o.push('\\');
                o.push(c);
            }
            c if (c as u32) < 0x20 || c as u32 == 0x7f => {
                let _ = write!(o, "\\x{:02x}", c as u32);
            }
            c => o.push(c),
        }
    }
    o.push(q);
    o
}

/// `repr(list_of_str)`.
pub fn repr_list<S: AsRef<str>>(items: &[S]) -> String {
    let parts: Vec<String> = items.iter().map(|s| repr_str(s.as_ref())).collect();
    format!("[{}]", parts.join(", "))
}

/// A value of a Python dict used as a finding key: `{'idx': 5, 'file': 'x'}`.
pub enum KeyVal {
    Int(i64),
    Str(String),
}

pub fn repr_dict(items: &[(&str, KeyVal)]) -> String {
    let parts: Vec<String> = items
        .iter()
        .map(|(k, v)| {
            let v = match v {
                KeyVal::Int(i) => i.to_string(),
                KeyVal::Str(s) => repr_str(s),
            };
            format!("{}: {}", repr_str(k), v)
        })
        .collect();
    format!("{{{}}}", parts.join(", "))
}

/// `"{:,}".format(n)`.
pub fn num<N: Into<i128>>(n: N) -> String {
    let n: i128 = n.into();
    let s = n.unsigned_abs().to_string();
    let mut o = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            o.push(',');
        }
        o.push(c);
    }
    if n < 0 { format!("-{o}") } else { o }
}

/// `"%.2f%%" % (100.0 * a / b)` or `"—"` when `b == 0` (progress.py `pct`).
pub fn pct(a: u64, b: u64) -> String {
    if b == 0 {
        "—".to_string()
    } else {
        format!("{:.2}%", 100.0 * a as f64 / b as f64)
    }
}

/// `glob.has_magic(pattern)`.
pub fn has_magic(p: &str) -> bool {
    p.contains(['*', '?', '['])
}

/// `glob.glob(pattern)`: matches, hidden files only when the pattern says so, in
/// directory order (callers sort, as the Python scripts do).
pub fn glob(pattern: &str) -> Result<Vec<String>> {
    let opts = glob::MatchOptions {
        case_sensitive: !cfg!(windows),
        require_literal_separator: true,
        require_literal_leading_dot: true,
    };
    let mut out = Vec::new();
    for e in glob::glob_with(pattern, opts)
        .with_context(|| format!("pola glob tidak valid: {pattern}"))?
    {
        let p = e?;
        out.push(p.to_string_lossy().into_owned());
    }
    Ok(out)
}

pub fn basename(p: &str) -> String {
    Path::new(p)
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Read a whole text file (UTF-8).
pub fn read_text(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))
}

/// Non-blank lines of a JSONL text, with their 1-based line number.
pub fn jsonl_lines(text: &str) -> impl Iterator<Item = (usize, &str)> {
    text.split('\n')
        .enumerate()
        .map(|(i, l)| (i + 1, l.trim()))
        .filter(|(_, l)| !l.is_empty())
}

/// Parse every non-blank line of a JSONL file into `T`, calling `f` per record.
pub fn for_each_jsonl<T, F>(path: &Path, mut f: F) -> Result<()>
where
    T: serde::de::DeserializeOwned,
    F: FnMut(usize, T) -> Result<()>,
{
    let text = read_text(path)?;
    for (ln, line) in jsonl_lines(&text) {
        let rec: T = serde_json::from_str(line)
            .with_context(|| format!("{} baris {}", path.display(), ln))?;
        f(ln, rec)?;
    }
    Ok(())
}

/// `os.path.join(root, *rel.split("/"))` — native separators, like the Python paths.
pub fn join(root: &Path, rel: &str) -> PathBuf {
    let mut p = root.to_path_buf();
    for c in rel.split('/') {
        if c == "." || c.is_empty() {
            continue;
        }
        p.push(c);
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dumps(s: &str) -> String {
        let mut o = String::new();
        json_str(&mut o, s, false);
        o
    }

    #[test]
    fn json_matches_python_dumps() {
        // values checked against `json.dumps(x, ensure_ascii=False)`
        assert_eq!(dumps("The leaf?"), r#""The leaf?""#);
        assert_eq!(dumps("a\"b\\c"), r#""a\"b\\c""#);
        assert_eq!(dumps("x\ny\r\tz\u{8}\u{c}"), r#""x\ny\r\tz\b\f""#);
        assert_eq!(dumps("\u{1}\u{1f}\u{7f}"), "\"\\u0001\\u001f\u{7f}\"");
        assert_eq!(dumps("Ünïcødé 燕云 ✓ /"), "\"Ünïcødé 燕云 ✓ /\"");
        let mut a = String::new();
        json_str(&mut a, "é燕😀", true);
        let bs = '\\';
        assert_eq!(a, format!("\"{bs}u00e9{bs}u71d5{bs}ud83d{bs}ude00\""));
        let o = Obj::new()
            .int("b", 1)
            .int("s", 3)
            .str("h", "c5cadbb857eee8b4")
            .str("v", "The leaf?")
            .finish();
        assert_eq!(
            o,
            r#"{"b": 1, "s": 3, "h": "c5cadbb857eee8b4", "v": "The leaf?"}"#
        );
        let o = Obj::new()
            .int("b", 5)
            .int("s", 14)
            .str("h", "924f075a96b1ccf6")
            .bool("deleted", true)
            .finish();
        assert_eq!(
            o,
            r#"{"b": 5, "s": 14, "h": "924f075a96b1ccf6", "deleted": true}"#
        );
    }

    #[test]
    fn repr_matches_python() {
        assert_eq!(repr_str("phase1.jsonl"), "'phase1.jsonl'");
        assert_eq!(repr_str("it's"), "\"it's\"");
        assert_eq!(repr_str("a'\"b"), "'a\\'\"b'");
        assert_eq!(repr_str("a\\b\n"), "'a\\\\b\\n'");
        assert_eq!(repr_list(&["a.jsonl", "b.jsonl"]), "['a.jsonl', 'b.jsonl']");
        assert_eq!(repr_list::<&str>(&[]), "[]");
        assert_eq!(
            repr_dict(&[
                ("idx", KeyVal::Int(5)),
                ("file", KeyVal::Str("phase1.jsonl".into()))
            ]),
            "{'idx': 5, 'file': 'phase1.jsonl'}"
        );
    }

    #[test]
    fn number_formats_match_python() {
        assert_eq!(num(0u32), "0");
        assert_eq!(num(999u32), "999");
        assert_eq!(num(1000u32), "1,000");
        assert_eq!(num(733661u32), "733,661");
        assert_eq!(num(1234567890u64), "1,234,567,890");
        assert_eq!(num(-1234i64), "-1,234");
        assert_eq!(pct(0, 0), "—");
        assert_eq!(pct(1, 3), "33.33%");
        assert_eq!(pct(2, 3), "66.67%");
        assert_eq!(pct(30000, 30000), "100.00%");
        // '%.2f' % 0.125 == '0.12' (ties to even on the exact binary value)
        assert_eq!(format!("{:.2}", 0.125f64), "0.12");
        assert_eq!(format!("{:.2}", 0.375f64), "0.38");
        assert_eq!(format!("{:.3}", 100.0 * 1.0 / 8.0), "12.500");
    }

    #[test]
    fn glob_magic() {
        assert!(has_magic("locale/*.jsonl"));
        assert!(!has_magic("locale/phase1.jsonl"));
    }

    #[test]
    fn jsonl_lines_skip_blank_and_crlf() {
        let t = "{\"a\": 1}\r\n\r\n  \n{\"a\": 2}";
        let v: Vec<_> = jsonl_lines(t).collect();
        assert_eq!(v, vec![(1, "{\"a\": 1}"), (4, "{\"a\": 2}")]);
    }
}
