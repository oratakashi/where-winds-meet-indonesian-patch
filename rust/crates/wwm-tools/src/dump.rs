//! Reading `wwm_locmap.py dump` output — or, when the dump is missing, decoding the
//! tracked binary directly. Both give exactly the same records (a dump is a
//! deterministic function of the binary), so CI on the Rust path skips the dump step.

use crate::profiles::Profile;
use crate::py::for_each_jsonl;
use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::path::Path;
use wwm_locmap::{TOMBSTONE, parse_shard, read_container_ex};

/// One line of a dump JSONL (`{"b","s","h","v"}` or `{"b","s","h","deleted": true}`).
#[derive(Debug, Deserialize)]
pub struct DumpRec {
    pub b: i64,
    pub s: i64,
    #[serde(default)]
    pub h: Option<String>,
    #[serde(default)]
    pub v: Option<String>,
    #[serde(default)]
    pub deleted: Option<bool>,
}

impl DumpRec {
    pub fn is_deleted(&self) -> bool {
        self.deleted == Some(true)
    }
}

/// A dump entry: keyHash as 16 hex digits and its text (`None` = tombstone).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rec {
    pub b: usize,
    pub s: usize,
    pub h: String,
    pub v: Option<String>,
}

/// Decode every entry of a container file, in dump order (block, then slot).
pub fn decode_binary(path: &Path, root: &Path) -> Result<Vec<Rec>> {
    let (_, blocks) =
        read_container_ex(path, root).with_context(|| format!("{}", path.display()))?;
    let mut out = Vec::new();
    for (bi, blk) in blocks.iter().enumerate().skip(1) {
        let (_, _, ents) =
            parse_shard(blk).with_context(|| format!("{} blok {bi}", path.display()))?;
        for e in ents {
            let v = if e.value == TOMBSTONE {
                None
            } else {
                match std::str::from_utf8(e.value) {
                    Ok(s) => Some(s.to_string()),
                    Err(_) => bail!(
                        "nilai biner tak terduga di blok {} slot {} (bukan UTF-8, bukan tombstone 0xFF): {:?}",
                        bi,
                        e.slot,
                        &e.value[..e.value.len().min(32)]
                    ),
                }
            };
            out.push(Rec {
                b: bi,
                s: e.slot,
                h: format!("{:016x}", e.hash),
                v,
            });
        }
    }
    Ok(out)
}

/// Read a dump JSONL file.
pub fn read_dump(path: &Path) -> Result<Vec<Rec>> {
    let mut out = Vec::new();
    for_each_jsonl(path, |ln, r: DumpRec| {
        let deleted = r.is_deleted();
        if !deleted && r.v.is_none() {
            bail!(
                "{} baris {ln}: tidak ada field 'v' maupun 'deleted'",
                path.display()
            );
        }
        out.push(Rec {
            b: r.b as usize,
            s: r.s as usize,
            h: r.h.unwrap_or_default(),
            v: if deleted { None } else { r.v },
        });
        Ok(())
    })?;
    Ok(out)
}

/// Entries of profile dump `dump` (relative to the profile base): the dump file if it
/// exists, else the tracked `binary` decoded in memory. `hint` is the error shown when
/// neither exists.
pub fn load(p: &Profile, binary: &str, dump: &str, hint: &str) -> Result<Vec<Rec>> {
    let dp = p.path(dump);
    if dp.exists() {
        return read_dump(&dp);
    }
    let bp = p.path(binary);
    if bp.exists() {
        return decode_binary(&bp, &p.root);
    }
    crate::exit!("{hint}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{get, testutil};
    use wwm_locmap::{Header, build_index, build_shard, write_container_ex};

    pub fn write_bin(root: &Path, name: &str, entries: &[(usize, u64, &[u8])]) {
        let blocks = vec![
            build_index(entries.len() as u64, 1),
            build_shard(16, 0, entries),
        ];
        write_container_ex(&root.join(name), &Header::v1(), &blocks, 1).unwrap();
    }

    #[test]
    fn binary_and_dump_agree() {
        let d = testutil::repo();
        write_bin(
            d.path(),
            "translate_words_map_en",
            &[(1, 0xab, b"Hi"), (4, 0xcd, TOMBSTONE)],
        );
        let p = get(d.path(), "global").unwrap();
        let from_bin = load(&p, "translate_words_map_en", "strings.jsonl", "x").unwrap();
        assert_eq!(
            from_bin,
            vec![
                Rec {
                    b: 1,
                    s: 1,
                    h: "00000000000000ab".into(),
                    v: Some("Hi".into())
                },
                Rec {
                    b: 1,
                    s: 4,
                    h: "00000000000000cd".into(),
                    v: None
                },
            ]
        );
        testutil::write(
            d.path(),
            "strings.jsonl",
            "{\"b\": 1, \"s\": 1, \"h\": \"00000000000000ab\", \"v\": \"Hi\"}\n{\"b\": 1, \"s\": 4, \"h\": \"00000000000000cd\", \"deleted\": true}\n",
        );
        assert_eq!(
            load(&p, "translate_words_map_en", "strings.jsonl", "x").unwrap(),
            from_bin
        );
        let e = load(&p, "nope", "nope.jsonl", "hint text").unwrap_err();
        assert_eq!(e.to_string(), "hint text");
    }
}
