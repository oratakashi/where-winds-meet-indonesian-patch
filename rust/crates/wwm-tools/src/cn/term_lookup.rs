//! `wwm cn term-lookup` — port of `cn/tools/term_lookup.py`: how global translated a
//! Mandarin term, through keys shared by the CN and global files.

use crate::Code;
use crate::bridge::{Counter, global_keys, most_common};
use crate::dict::load_translated_map;
use crate::dump;
use crate::profiles;
use anyhow::Result;
use indexmap::IndexMap;
use std::path::Path;

pub fn run(root: &Path, terms: &[String], contains: bool, limit: usize) -> Result<Code> {
    let cn = profiles::get(root, "cn")?;
    let (bin, d) = (&cn.dumps[0].0, &cn.dumps[0].1);
    let hint = format!(
        "{} tidak ada — python wwm_locmap.py dump cn/{} {}",
        cn.path(d).display(),
        bin,
        cn.rel(d)
    );
    let recs = dump::load(&cn, bin, d, &hint)?;
    let en = global_keys(root)?;
    let (idn, _) = load_translated_map(&profiles::get(root, "global")?)?;
    // Mandarin text -> Counter((English, Indonesian or None))
    let mut by_text: IndexMap<String, Counter<(String, Option<String>)>> = IndexMap::new();
    for r in recs {
        if let (Some(e), Some(v)) = (en.get(&r.h), r.v) {
            let key = (e.clone(), idn.get(e).cloned());
            *by_text.entry(v).or_default().entry(key).or_insert(0) += 1;
        }
    }
    let id_or = |i: &Option<String>| match i.as_deref() {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => "(belum diterjemahkan)".to_string(),
    };
    for term in terms {
        println!("== {term}");
        if contains {
            let mut hits: Vec<&String> = by_text
                .keys()
                .filter(|t| t.contains(term.as_str()))
                .collect();
            hits.sort_by_key(|t| t.chars().count()); // stable: dict order among equal lengths
            hits.truncate(limit);
            if hits.is_empty() {
                println!("   (tidak ada di key bersama — istilah khusus CN)");
            }
            for t in hits {
                let ((e, i), _) = most_common(&by_text[t]).into_iter().next().unwrap();
                println!("   {}\n     EN: {}\n     ID: {}", t, e, id_or(&i));
            }
        } else {
            let Some(c) = by_text.get(term) else {
                println!("   (tidak ada di key bersama — coba --contains)");
                continue;
            };
            for ((e, i), n) in most_common(c).into_iter().take(limit) {
                println!("   {n:5}x  EN: {}\n          ID: {}", e, id_or(&i));
            }
        }
    }
    Ok(0)
}
