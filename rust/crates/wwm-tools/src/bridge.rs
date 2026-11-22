//! Global -> CN bridge — port of `tools/bridge.py`.
//!
//! 1. per key: a CN keyHash that exists in global gets global's Indonesian text for it;
//! 2. per text: a Mandarin text seen on shared keys gets the Indonesian translation used
//!    most often for it (ties: first seen, like `Counter.most_common`).

use crate::dict::load_translated_map;
use crate::dump;
use crate::profiles::{self, Profile};
use anyhow::Result;
use indexmap::IndexMap;
use std::collections::HashMap;

/// keyHash (hex) -> English text for every live key of the global dumps; `_diff`
/// overrides base, a `_diff` tombstone removes the key.
pub fn global_keys(root: &std::path::Path) -> Result<HashMap<String, String>> {
    let g = profiles::get(root, "global")?;
    let mut keys = HashMap::new();
    for (name, d) in &g.dumps {
        let hint = format!(
            "{} tidak ada — jembatan butuh dump-nya: python wwm_locmap.py dump {} {}",
            g.path(d).display(),
            name,
            d
        );
        for r in dump::load(&g, name, d, &hint)? {
            match r.v {
                None => {
                    keys.remove(&r.h);
                }
                Some(v) => {
                    keys.insert(r.h, v);
                }
            }
        }
    }
    Ok(keys)
}

/// Counter with insertion order (Python `Counter`).
pub type Counter<K> = IndexMap<K, u64>;

/// `Counter.most_common()`: count descending, ties in insertion order.
pub fn most_common<K: Clone>(c: &Counter<K>) -> Vec<(K, u64)> {
    let mut v: Vec<(K, u64)> = c.iter().map(|(k, n)| (k.clone(), *n)).collect();
    v.sort_by_key(|e| std::cmp::Reverse(e.1)); // stable
    v
}

/// Profile text -> Counter(global English text) over entries whose key exists in global.
pub fn shared_texts(
    p: &Profile,
    gkeys: &HashMap<String, String>,
) -> Result<IndexMap<String, Counter<String>>> {
    let mut out: IndexMap<String, Counter<String>> = IndexMap::new();
    for (name, d) in &p.dumps {
        let hint = format!(
            "{} tidak ada — jembatan butuh dump-nya: python wwm_locmap.py dump {} {}",
            p.path(d).display(),
            p.rel(name),
            p.rel(d)
        );
        for r in dump::load(p, name, d, &hint)? {
            if let (Some(e), Some(v)) = (gkeys.get(&r.h), r.v) {
                *out.entry(v).or_default().entry(e.clone()).or_insert(0) += 1;
            }
        }
    }
    Ok(out)
}

/// (per_key, per_text); both empty when the profile does not bridge from global.
pub fn load_bridge(p: &Profile) -> Result<(HashMap<String, String>, HashMap<String, String>)> {
    if p.bridge_from.as_deref() != Some("global") {
        return Ok((HashMap::new(), HashMap::new()));
    }
    let g = profiles::get(&p.root, "global")?;
    let (translated, _) = load_translated_map(&g)?;
    let gkeys = global_keys(&p.root)?;
    let per_key = gkeys
        .iter()
        .filter_map(|(h, e)| translated.get(e).map(|t| (h.clone(), t.clone())))
        .collect();
    Ok((per_key, per_text(&shared_texts(p, &gkeys)?, &translated)))
}

pub fn per_text(
    shared: &IndexMap<String, Counter<String>>,
    translated: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for (text, ens) in shared {
        let mut ids: Counter<&str> = IndexMap::new();
        for (e, n) in ens {
            if let Some(t) = translated.get(e) {
                *ids.entry(t.as_str()).or_insert(0) += n;
            }
        }
        if let Some((best, _)) = most_common(&ids).first() {
            out.insert(text.clone(), best.to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::testutil;

    #[test]
    fn most_common_ties_keep_insertion_order() {
        let mut c: Counter<&str> = IndexMap::new();
        c.insert("b", 2);
        c.insert("a", 3);
        c.insert("c", 3);
        c.insert("d", 1);
        assert_eq!(
            most_common(&c),
            vec![("a", 3), ("c", 3), ("b", 2), ("d", 1)]
        );
    }

    #[test]
    fn global_keys_diff_overrides_and_deletes() {
        let d = testutil::repo();
        testutil::write(
            d.path(),
            "strings.jsonl",
            "{\"b\": 1, \"s\": 0, \"h\": \"01\", \"v\": \"Old\"}\n{\"b\": 1, \"s\": 1, \"h\": \"02\", \"v\": \"Gone\"}\n{\"b\": 1, \"s\": 2, \"h\": \"03\", \"v\": \"Kept\"}\n",
        );
        testutil::write(
            d.path(),
            "strings_diff.jsonl",
            "{\"b\": 1, \"s\": 0, \"h\": \"01\", \"v\": \"New\"}\n{\"b\": 1, \"s\": 1, \"h\": \"02\", \"deleted\": true}\n",
        );
        let k = global_keys(d.path()).unwrap();
        assert_eq!(k.len(), 2);
        assert_eq!(k["01"], "New");
        assert_eq!(k["03"], "Kept");
    }

    #[test]
    fn per_text_picks_most_used_translation() {
        let mut shared: IndexMap<String, Counter<String>> = IndexMap::new();
        let mut c = IndexMap::new();
        c.insert("Sword".to_string(), 1);
        c.insert("Blade".to_string(), 1);
        c.insert("Untranslated".to_string(), 5);
        shared.insert("剑".to_string(), c);
        let tr: HashMap<String, String> = [
            ("Sword".to_string(), "Pedang".to_string()),
            ("Blade".to_string(), "Bilah".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(per_text(&shared, &tr)["剑"], "Pedang");
    }
}
