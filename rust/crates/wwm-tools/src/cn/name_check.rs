//! `wwm cn name-check` — port of `cn/tools/name_check.py`: CN rows (cn/locale/*.jsonl)
//! whose source holds a name global keeps in English (place, NPC, sect, Inner Way, ...)
//! but whose translation does not use it. `--fix` rewrites Pinyin / known variants.
//!
//! Known difference: syllables come from the `pinyin` crate instead of `pypinyin`, so a
//! rare heteronym may differ (only affects which Pinyin spellings `--fix` recognises).

use crate::Code;
use crate::bridge::{Counter, global_keys, most_common};
use crate::dict::{UniqueRec, load_translated_map, load_unique};
use crate::dump;
use crate::profiles;
use crate::py::{NL, Obj, basename, for_each_jsonl, glob, join, read_text};
use anyhow::{Context, Result};
use fancy_regex::{Captures, Regex};
use indexmap::IndexMap;
use std::collections::{BTreeSet, HashMap};
use std::io::Write;
use std::path::Path;
use std::sync::LazyLock;

static HAN: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new("^[一-鿿·]{2,8}$").unwrap());
static WORD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[A-Za-z][\w'’-]*").unwrap());
/// common words that happen to be NPC names on shared keys
const NOT_NAMES: &[&str] = &["工匠"];

/// generic words glued to place names written in Pinyin
const PRE: &str = "Gunung|Lembah|Menara|Paviliun|Kota|Desa|Hutan|Aula|Balai|Kuil|Gua|Jembatan|Stasiun|\
Benteng|Kediaman|Pasar|Puncak|Lereng|Bukit|Tebing|Danau|Kolam|Sungai|Tepi|Dermaga|\
Gerbang|Jalan|Jalur|Distrik|Taman|Kebun|Istana|Akademi|Sekte|Geng|\
Perkemahan|Markas|Pos|Celah|Ngarai|Jurang|Lapangan|Lautan|Pulau|Makam|Gudang|Kantor|\
Rumah|Kampung|Biara|Pagoda|Lumbung|Teras|Kedai|Penginapan|Bengkel|Altar|\
Lonceng|Perahu|Kapal|Padang|Gurun|Kamp";
const POST: &str = "Mountain|Mountains|Valley|Tower|Pavilion|Town|Village|Forest|Hall|Temple|Cave|\
Cavern|Bridge|Station|Fort|Fortress|Manor|Market|Bazaar|Peak|Slope|Hill|Ridge|Cliff|\
Crag|Lake|Pool|River|Bank|Dock|Pier|Gate|Road|Path|Ward|District|Garden|Palace|\
Academy|Sect|Gang|Camp|Base|Pass|Ferry|Crossing|Gorge|Abyss|Field|Sea|Island|Tomb|\
Barrow|Office|House|Monastery|Pagoda|Lane|Street|Avenue|Terrace|Inn|Shrine|Grove|\
Shan|Lou|Cun|Zhen|Gu|Lin|Ge|Si|Dian|Ting|Tai|Fang";

fn upper_first(w: &str) -> (String, String, String) {
    let mut cs = w.chars();
    let f = cs.next().unwrap_or_default();
    (
        f.to_uppercase().collect(),
        f.to_lowercase().collect(),
        cs.collect(),
    )
}

/// `_anycase(words)`: `[Ww]ord|...`
pub fn anycase<S: AsRef<str>>(words: &[S]) -> String {
    words
        .iter()
        .map(|w| {
            let (u, l, rest) = upper_first(w.as_ref());
            format!("[{u}{l}]{rest}")
        })
        .collect::<Vec<_>>()
        .join("|")
}

/// `str.capitalize()`
fn capitalize(w: &str) -> String {
    let (u, _, rest) = upper_first(w);
    format!("{u}{}", rest.to_lowercase())
}

static PRE_RX: LazyLock<String> = LazyLock::new(|| anycase(&PRE.split('|').collect::<Vec<_>>()));
static POST_RX: LazyLock<String> = LazyLock::new(|| anycase(&POST.split('|').collect::<Vec<_>>()));

/// Toneless Pinyin spellings of one character, longest first (ü written as u and v).
pub fn pinyin_of(ch: char) -> Vec<String> {
    use pinyin::ToPinyinMulti;
    let mut raw: Vec<String> = Vec::new();
    match ch.to_pinyin_multi() {
        Some(multi) => raw.extend(multi.into_iter().map(|p| p.plain().replace('ü', "v"))),
        None => raw.push(ch.to_string()), // pypinyin returns the character itself
    }
    let mut out: BTreeSet<String> = BTreeSet::new();
    for r in raw {
        out.insert(r.replace(['ü', 'v'], "u"));
        out.insert(r.replace('ü', "v"));
        out.insert(r);
    }
    let mut v: Vec<String> = out
        .into_iter()
        .filter(|x| !x.is_empty() && x.chars().all(char::is_alphabetic))
        .collect();
    v.sort_by_key(|x| std::cmp::Reverse(x.chars().count())); // stable: alphabetical among equals
    v
}

/// Regex for the Pinyin spelling of `term` (or a contiguous part of it, at most two
/// characters missing — those became a generic word), plus surrounding generic words.
pub fn variant_rx(term: &str) -> Option<Regex> {
    let chars: Vec<char> = term.chars().filter(|&c| c != '·').collect();
    let syl: Vec<Vec<String>> = chars.iter().map(|&c| pinyin_of(c)).collect();
    let n = chars.len();
    let mut pats = Vec::new();
    for l in (1..=n).rev() {
        if l + 2 < n {
            break;
        }
        for a in 0..=n - l {
            let caps: Vec<String> = syl[a].iter().map(|x| capitalize(x)).collect();
            let mut run = format!("(?:{})", caps.join("|"));
            for s in &syl[a + 1..a + l] {
                run += &format!("[ '’]?(?:{})", anycase(s));
            }
            let (pre, post) = (&*PRE_RX, &*POST_RX);
            if l < n {
                pats.push(format!(
                    "(?:(?:{pre}) ){{1,2}}{run}(?: (?:{post}))?|{run} (?:{post})"
                ));
            } else {
                pats.push(format!("(?:(?:{pre}) ){{0,2}}{run}(?: (?:{post}))?"));
            }
        }
    }
    if pats.is_empty() {
        return None;
    }
    Some(Regex::new(&format!("(?<![\\w'’])(?:{})(?![\\w'’])", pats.join("|"))).expect("variant_rx"))
}

/// Terms of `names` in the source text, longest match first, no overlap.
pub fn terms_in(s: &str, names: &HashMap<String, String>, maxlen: usize) -> Vec<String> {
    let cs: Vec<char> = s.chars().collect();
    let (mut i, mut hits) = (0usize, Vec::new());
    while i < cs.len() {
        let mut matched = false;
        for l in (2..=maxlen.min(cs.len() - i)).rev() {
            let t: String = cs[i..i + l].iter().collect();
            if names.contains_key(&t) {
                hits.push(t);
                i += l;
                matched = true;
                break;
            }
        }
        if !matched {
            i += 1;
        }
    }
    hits
}

/// Editor labels `A-B-C`: with as many segments in source and translation, a segment
/// whose source is exactly a name gets that name's global form (same position).
pub fn fix_segments(s: &str, v: &str, names: &HashMap<String, String>) -> String {
    let ss: Vec<&str> = s.split('-').collect();
    let mut vs: Vec<String> = v.split('-').map(String::from).collect();
    if ss.len() < 2 || ss.len() != vs.len() {
        return v.to_string();
    }
    for (k, seg) in ss.iter().enumerate() {
        if let Some(e) = names.get(seg.trim())
            && !e.is_empty()
            && !vs[k].to_lowercase().contains(&e.to_lowercase())
        {
            let x = &vs[k];
            let lead = &x[..x.len() - x.trim_start().len()];
            let tail = &x[x.trim_end().len()..];
            vs[k] = format!("{lead}{e}{tail}");
        }
    }
    vs.join("-")
}

const PATH: &[(&str, &str, &str)] = &[
    ("鸣金", "Bellstrike", "Ming ?jin"),
    ("裂石", "Stonesplit", "(?:Pemecah|Pecah) Batu|Lie ?shi"),
    ("牵丝", "Silkbind", "Qian ?si|Tarikan Sutra|Penarik Sutra"),
    ("破竹", "Bamboocut", "(?:Pemecah|Pecah) Bambu|Po ?zhu"),
];
const SUFFIX: &[(&str, &str, &str)] = &[
    ("影", "Umbra", "Bayangan|Ying|Shadow"),
    ("虹", "Splendor", "Pelangi|Hong|Rainbow"),
    ("霖", "Deluge", "Hujan|Lin"),
    ("威", "Might", "Kekuatan|Wei"),
    ("玉", "Jade", "Giok|Yu"),
    ("尘", "Dust", "Debu|Chen"),
    ("钧", "Strength", "Jun"),
    ("风", "Wind", "Angin|Feng"),
    ("鸢", "Kite", "Layang-layang|Layang|Elang|Yuan"),
    ("樽", "Draught", "Cawan|Zun|Cask"),
    ("翊", "Wings", "Sayap|Yi"),
    ("韵", "Yun", "Irama|Rhythm"),
];
/// Place names translated literally (mined from cn/locale 2026-10-25): (source, name, variants)
const LITERAL: &[(&str, &str, &str)] = &[
    (
        "皇宫",
        "Imperial Palace",
        r"[Ii]stana (?:Kekaisaran|Kaisar)",
    ),
    (
        "千年渡",
        "Millennium Crossing",
        r"Persimpangan Milenium|Penyeberangan Seribu Tahun",
    ),
    (
        "天工地窟",
        "Divinecraft Dungeon",
        r"Gua (?:Tiangong|Divinecraft)|Divinecraft Gua",
    ),
    (
        "南门大街",
        "South Gate Avenue",
        r"[Jj]alan [Gg]erbang [Ss]elatan|South Gate Street",
    ),
    ("官署", "Hall of State", r"[Kk]antor [Pp]emerintah(?:an)?"),
    ("不见山", "Hidden Mountain", r"Gunung Tak Terlihat"),
    ("墨山道", "Mohist Hill", r"Jalur Gunung Tinta"),
    (
        "栖鸟岩",
        "Spiritroost Crag",
        r"(?:(?:Tebing|Batu) )?Burung Bertengger",
    ),
    (
        "滹沱南岸",
        "South Hutuo Bank",
        r"[Tt]epi [Ss]elatan(?: Sungai)? Hutuo",
    ),
    (
        "绣金楼",
        "Aureate Pavilion",
        r"(?:(?:Menara|Paviliun) )?Sulam Emas",
    ),
    (
        "御苑南",
        "South Imperial Garden",
        r"(?:Taman (?:Kekaisaran|Istana)|Imperial Garden) Selatan|Selatan Taman (?:Kekaisaran|Istana)",
    ),
    (
        "御苑北",
        "North Imperial Garden",
        r"(?:Taman (?:Kekaisaran|Istana)|Imperial Garden) Utara|Utara Taman (?:Kekaisaran|Istana)",
    ),
    (
        "嗟叹崖",
        "Gasping Cliff",
        r"Tebing (?:yang Terengah|Ratapan|Keluh(?: Kesah)?)",
    ),
    ("佛光顶", "Halo Peak", r"Puncak Cahaya(?: Buddha)?"),
    ("花海", "Flower Expanse", r"[Ll]autan [Bb]unga"),
    ("将军祠", "General's Shrine", r"Kuil (?:Sang )?Jenderal"),
    (
        "鬼樊楼",
        "Ghost Revelry Hall",
        r"Revelry Hall Hantu|Menara Fan Hantu",
    ),
    ("西湖", "West Lake", r"Danau Barat"),
    (
        "飞天残垣",
        "Skyward City Ruins",
        r"Reruntuhan (?:Terbang|Feitian)",
    ),
    (
        "菩提苦海",
        "Bodhi Sea",
        r"(?:Bodhi )?Lautan (?:Derita|Pahit)(?: Bodhi)?",
    ),
    (
        "不羡仙",
        "Blissful Retreat",
        r"Tanpa Pamrih Mendambakan Dewa|Tak (?:Iri|Mengidam|Mendambakan) Dewa",
    ),
    (
        "封桩库",
        "Sealed Treasury",
        r"Gudang (?:Tiang Penyegel|Fengzhuang)",
    ),
    (
        "太岳台",
        "Skyreach Terrace",
        r"Teras (?:Skyreach|Taiyue)|Taiyue Terrace",
    ),
    (
        "碧水云涛",
        "Jadestream Valley",
        r"Biyun Yuntao|Air Hijau Ombak Awan|Ombak Awan (?:Air Hijau|Biru)",
    ),
    ("佛爷寨", "Buddha Fort", r"(?:Benteng|Perkemahan) Buddha"),
    (
        "角门里",
        "Forsaken Quarter",
        r"[Dd]alam [Gg]erbang [Ss]amping|[Gg]erbang [Ss]amping",
    ),
    ("心湖", "Heart Lake", r"Danau Hati"),
    ("饮马隘", "Steed's Pass", r"Celah Minum Kuda|Yinma Pass"),
    (
        "演武场",
        "Training Grounds",
        r"[Ll]apangan [Ll]atihan(?: [Bb]ela [Dd]iri)?|[Aa]rena [Ll]atihan",
    ),
    ("太清楼", "Purity Tower", r"Menara Kesucian"),
    ("花船集", "Flower Boat Market", r"Pasar Perahu Bunga"),
    (
        "千佛村",
        "Thousand-Buddha Village",
        r"Desa Seribu Buddha|Thousand Buddha Vale",
    ),
    ("后苑", "Rear Garden", r"[Tt]aman [Bb]elakang"),
    ("封丘村", "Verdant Hill Village", r"Desa Verdant Hill"),
    ("天上来", "Heavenfall", r"Datang dari Langit"),
    (
        "来生岸",
        "Floral Expanse Beyond",
        r"Laborious Shore|Shore of Rebirth|Tepi Kehidupan(?: Mendatang| Berikutnya)?",
    ),
    ("思危园", "Vigilance Garden", r"Taman Kewaspadaan"),
    ("灵隐新寺", "New Lingyin Temple", r"Kuil Baru Lingyin"),
    ("玉露台", "Cleardew Terrace", r"Teras (?:Embun )?Giok"),
    ("时楼", "Temporal Pavilion", r"Menara Waktu"),
    (
        "陷城湖",
        "Sunken City Lake",
        r"Danau Kota(?: (?:Runtuh|Tenggelam))?",
    ),
    (
        "觉障林",
        "Perception Forest",
        r"Awakening Forest|Hutan (?:Penyadar|Juezhang)|Grove of Barriers",
    ),
    ("黄河", "Yellow River", r"Sungai Kuning"),
    (
        "酒肉山林",
        "Gourmand's Grove",
        r"Hutan(?: Gunung)? Arak(?: dan)? Daging|Arak Daging",
    ),
    ("狮子坟", "Lion Barrow", r"Makam Singa"),
    ("武庙", "Martial Temple", r"Kuil (?:Bela Diri|Militer)"),
    ("酒香塔", "Sweetwine Tower", r"Menara (?:Aroma|Wangi) Arak"),
    (
        "鬼市",
        "Ghostlight Market",
        r"(?:Anak )?Pasar Hantu|Ghost Market",
    ),
    (
        "北盟遗址",
        "Northern Vow Ruins",
        r"(?:(?:Situs|Reruntuhan) )?Aliansi Utara",
    ),
    ("博浪沙", "Roaring Sands", r"Bolang ?[Ss]ands|Bolangsha"),
    (
        "万古一人殿",
        "Eternal Solitude Hall",
        r"Aula Satu[- ]Satunya Sepanjang Masa",
    ),
    ("隐月山", "Moonveil Mountain", r"Gunung Bulan Tersembunyi"),
    (
        "常平仓",
        "Ever-Normal Granary",
        r"Changping Granary|Lumbung Changping",
    ),
    ("雾谷", "Misty Valley", r"Lembah Kabut(?: Tipis)?"),
    (
        "赤龙堂",
        "Bloodscale Hall",
        r"Aula Naga Merah|Red Dragon Hall",
    ),
    ("骆驼驿", "Camel Station", r"Stasiun Unta"),
    (
        "封龙山",
        "Dragonbend Mountain",
        r"Gunung (?:Dragonbend|Fenglong)",
    ),
    (
        "神仙渡",
        "Heaven's Pier",
        r"Dermaga Dewa|Shenxian (?:Ferry|Crossing)",
    ),
    (
        "九流门驻地",
        "Nine Mortal Ways Base",
        r"Markas Nine Mortal Ways",
    ),
    (
        "九流门",
        "Nine Mortal Ways",
        r"(?:Gerbang|Sekte) Sembilan Aliran|Sekte Jiuliu",
    ),
    ("望淮南崖", "Dreamfall Cliff", r"Tebing Memandang Huainan"),
    ("太仓粟", "Granary of Plenty", r"Lumbung Taicang"),
    (
        "三馆",
        "Reserve Archives",
        r"Tiga (?:Balai|Paviliun|Gedung)",
    ),
    ("前广场", "Front Plaza", r"[Aa]lun-alun [Dd]epan"),
];
/// Leftover doubles after a replacement: "Gunung Hidden Mountain" -> "Hidden Mountain".
const DUP: &[(&str, &str)] = &[
    ("Mountain", "Gunung"),
    ("Village", "Desa"),
    ("Town", "Kota"),
    ("Hall", "Aula|Balai"),
    ("Tower", "Menara"),
    ("Pavilion", "Paviliun"),
    ("Lake", "Danau"),
    ("Cliff", "Tebing"),
    ("Crag", "Tebing|Batu"),
    ("Temple", "Kuil"),
    ("Market", "Pasar"),
    ("Garden", "Taman"),
    ("Valley", "Lembah"),
    ("Forest", "Hutan"),
    ("Bridge", "Jembatan"),
    ("Station", "Stasiun"),
    ("Fort", "Benteng"),
    ("Manor", "Kediaman"),
    ("Peak", "Puncak"),
    ("River", "Sungai"),
    ("Bank", "Tepi(?: [Ss]elatan| [Uu]tara)?"),
    ("Pass", "Celah"),
    ("Ruins", "Reruntuhan"),
    ("Academy", "Akademi"),
    ("Cavern", "Gua"),
    ("Dungeon", "Gua"),
    ("Terrace", "Teras"),
    ("Shrine", "Kuil"),
    ("Grove", "Hutan"),
    ("Sea", "Lautan"),
    ("Gate", "Gerbang"),
    ("Avenue", "Jalan"),
    ("Granary", "Lumbung"),
    ("Treasury", "Gudang"),
    ("Base", "Markas"),
];

/// (source term, regex, replacement) — curated non-Pinyin variants.
pub static CURATED: LazyLock<Vec<(String, Regex, String)>> = LazyLock::new(|| {
    let mut v = Vec::new();
    for (zh, en, xalts) in PATH {
        for (ch, y, alts) in SUFFIX {
            let prx = Regex::new(&format!(
                r"(?i)(?:{en}|{xalts})\s*(?:[·\-–]\s*|\s)(?:{alts}|{y})(?![\w'’])"
            ))
            .unwrap();
            v.push((format!("{zh}·{ch}"), prx.clone(), format!("{en} - {y}")));
            // without the middle dot (流派塔牵丝霖) global writes "Silkbind Deluge"
            v.push((format!("{zh}{ch}"), prx, format!("{en} {y}")));
        }
    }
    v.push((
        "流派塔".into(),
        Regex::new(r"\b[Mm]enara [Aa]liran\b|\b[Mm]enara Path\b").unwrap(),
        "Path Tower".into(),
    ));
    v.push((
        "心法心得".into(),
        Regex::new(r"\b(?:[Cc]atatan|[Ww]awasan|[Pp]emahaman|[Pp]engalaman) Inner Way\b(?! Notes)")
            .unwrap(),
        "Inner Way Notes".into(),
    ));
    v.push((
        "心法".into(),
        Regex::new(r"\b[Tt]eknik [Bb]atin\b").unwrap(),
        "Inner Way".into(),
    ));
    for (t, e, p) in LITERAL {
        v.push((
            t.to_string(),
            Regex::new(&format!(r"(?<![\w'’])(?:{p})(?![\w'’])")).unwrap(),
            e.to_string(),
        ));
    }
    v
});

pub static DUP_RX: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    DUP.iter()
        .map(|(e, i)| {
            Regex::new(&format!(
                r"(?<![\w'’])(?:{i}) ((?:[A-Z][\w'’-]* ){{1,3}}{e})(?![\w'’])"
            ))
            .unwrap()
        })
        .collect()
});

/// Mandarin text -> English name that global keeps untranslated.
pub fn build_names(root: &Path) -> Result<HashMap<String, String>> {
    let cn = profiles::get(root, "cn")?;
    let g = profiles::get(root, "global")?;
    let gk = global_keys(root)?;
    let (idn, _) = load_translated_map(&g)?;
    let (bin, d) = (&cn.dumps[0].0, &cn.dumps[0].1);
    let hint = format!(
        "{} tidak ada — python wwm_locmap.py dump cn/{} {}",
        cn.path(d).display(),
        bin,
        cn.rel(d)
    );
    let mut by: IndexMap<String, Counter<String>> = IndexMap::new();
    for r in dump::load(&cn, bin, d, &hint)? {
        if let (Some(e), Some(v)) = (gk.get(&r.h), r.v)
            && HAN.is_match(&v)
        {
            *by.entry(v).or_default().entry(e.clone()).or_insert(0) += 1;
        }
    }
    let mut names: IndexMap<String, String> = IndexMap::new();
    for (t, c) in &by {
        let (e, n) = most_common(c).into_iter().next().unwrap();
        let sum: u64 = c.values().sum();
        if n as f64 >= 0.6 * sum as f64
            && idn.get(&e) == Some(&e)
            && e.chars().any(|ch| ch.is_ascii_uppercase())
            && !NOT_NAMES.contains(&t.as_str())
        {
            names.insert(t.clone(), e);
        }
    }
    // Global also keeps some common words in English (Chat, Organization, ...). Real
    // names are capitalised mid-sentence; common words are lower-case more often.
    let mut targets: std::collections::HashSet<String> = names.values().cloned().collect();
    targets.extend(names.values().map(|e| e.to_lowercase()).collect::<Vec<_>>());
    let mut seen: HashMap<String, u64> = HashMap::new();
    for_each_jsonl(&g.unique, |_, r: UniqueRec| {
        let w: Vec<&str> = WORD.find_iter(&r.v).map(|m| m.as_str()).collect();
        for i in 0..w.len() {
            let mut gram = w[i].to_string();
            for j in i + 1..(i + 5).min(w.len() + 1) {
                if j > i + 1 {
                    gram.push(' ');
                    gram.push_str(w[j - 1]);
                }
                if targets.contains(&gram) {
                    *seen.entry(gram.clone()).or_insert(0) += 1;
                }
            }
        }
        Ok(())
    })?;
    let cnt = |k: &str| seen.get(k).copied().unwrap_or(0);
    Ok(names
        .into_iter()
        .filter(|(_, e)| cnt(&e.to_lowercase()) < cnt(e) || e.to_lowercase() == *e)
        .collect())
}

/// Apply the curated + DUP fixes and editor-label segments (the `--fix` text pass).
pub fn fix_text(s: &str, v: &str, names: &HashMap<String, String>) -> String {
    let mut v2 = fix_segments(s, v, names);
    for (need, rx, rep) in CURATED.iter() {
        if s.contains(need.as_str()) && !v2.to_lowercase().contains(&rep.to_lowercase()) {
            v2 = rx.replace_all(&v2, fancy_regex::NoExpand(rep)).into_owned();
        }
    }
    for rx in DUP_RX.iter() {
        v2 = rx.replace_all(&v2, "${1}").into_owned();
    }
    v2
}

/// One locale row: the full JSON object (other fields kept, in order).
type Row = serde_json::Map<String, serde_json::Value>;

/// `json.dumps(obj, ensure_ascii=False)` for a generic value.
pub fn py_dumps(v: &serde_json::Value) -> String {
    use serde_json::Value as V;
    match v {
        V::Null => "null".into(),
        V::Bool(b) => b.to_string(),
        V::Number(n) => n.to_string(),
        V::String(s) => {
            let mut o = String::new();
            crate::py::json_str(&mut o, s, false);
            o
        }
        V::Array(a) => format!(
            "[{}]",
            a.iter().map(py_dumps).collect::<Vec<_>>().join(", ")
        ),
        V::Object(m) => {
            let parts: Vec<String> = m
                .iter()
                .map(|(k, v)| {
                    let mut o = String::new();
                    crate::py::json_str(&mut o, k, false);
                    format!("{o}: {}", py_dumps(v))
                })
                .collect();
            format!("{{{}}}", parts.join(", "))
        }
    }
}

pub fn run(
    root: &Path,
    locale: &str,
    idx: Option<&str>,
    report: Option<&str>,
    fix: bool,
    top: usize,
) -> Result<Code> {
    let (lo, hi) = match idx {
        Some(r) => {
            let (a, b) = r.split_once('-').context("--idx harus berbentuk A-B")?;
            (a.trim().parse::<i64>()?, b.trim().parse::<i64>()?)
        }
        None => (0, 1i64 << 62),
    };
    let names = build_names(root)?;
    let maxlen = names
        .keys()
        .map(|k| k.chars().count())
        .max()
        .context("kamus nama kosong")?;
    let cn = profiles::get(root, "cn")?;
    let src = load_unique(&cn)?;
    let mut rx_cache: HashMap<String, Option<Regex>> = HashMap::new();

    let mut miss: Counter<(String, String)> = IndexMap::new();
    let mut report_rows: Vec<String> = Vec::new();
    let mut fixed = 0usize;
    let mut paths = glob(&join(root, locale).to_string_lossy())?;
    paths.sort();
    for path in &paths {
        let text = read_text(Path::new(path))?;
        let mut rows: Vec<Row> = Vec::new();
        for (ln, line) in crate::py::jsonl_lines(&text) {
            rows.push(serde_json::from_str(line).with_context(|| format!("{path} baris {ln}"))?);
        }
        let mut changed = false;
        for d in rows.iter_mut() {
            let i = d
                .get("idx")
                .and_then(|x| x.as_i64())
                .context("baris tanpa idx")?;
            if !(lo <= i && i <= hi) {
                continue;
            }
            let s = src
                .get(&i)
                .with_context(|| format!("idx {i} tidak ada di unique_strings.jsonl"))?;
            let mut v = d
                .get("v")
                .and_then(|x| x.as_str())
                .unwrap_or_default()
                .to_string();
            if fix {
                let v2 = fix_text(s, &v, &names);
                if v2 != v {
                    v = v2;
                    changed = true;
                    fixed += 1;
                }
            }
            let mut bad: Vec<(String, String)> = Vec::new();
            for t in terms_in(s, &names, maxlen) {
                let e = &names[&t];
                let el = e.to_lowercase();
                if v.to_lowercase().contains(&el) {
                    continue;
                }
                let rx = if fix {
                    rx_cache
                        .entry(t.clone())
                        .or_insert_with(|| variant_rx(&t))
                        .as_ref()
                } else {
                    None
                };
                if let Some(rx) = rx {
                    let mut n = 0usize;
                    let v2 = rx
                        .replace_all(&v, |c: &Captures<'_, str>| {
                            n += 1;
                            let m = &c[0];
                            if m.to_lowercase().contains(&el) {
                                m.to_string()
                            } else {
                                e.clone()
                            }
                        })
                        .into_owned();
                    if n > 0 && v2 != v {
                        v = v2;
                        changed = true;
                        fixed += 1;
                        continue;
                    }
                }
                bad.push((t, e.clone()));
            }
            d.insert("v".into(), serde_json::Value::String(v.clone()));
            for b in &bad {
                *miss.entry(b.clone()).or_insert(0) += 1;
            }
            if !bad.is_empty() {
                // {t: e for t, e in bad}: a repeated term keeps its first position
                let dedup: IndexMap<&str, &str> =
                    bad.iter().map(|(t, e)| (t.as_str(), e.as_str())).collect();
                let mut nm = Obj::new();
                for (t, e) in dedup {
                    nm = nm.str(t, e);
                }
                report_rows.push(
                    Obj::new()
                        .str("file", &basename(path))
                        .int("idx", i)
                        .str("src", s)
                        .str("v", &v)
                        .raw("names", &nm.finish())
                        .finish(),
                );
            }
        }
        if changed {
            // write a temp file and rename: on Windows a file being scanned
            // (antivirus/Obsidian) sometimes refuses to open for writing
            let tmp = format!("{path}.tmp");
            let mut out = String::new();
            for d in &rows {
                out.push_str(&py_dumps(&serde_json::Value::Object(d.clone())));
                out.push('\n');
            }
            std::fs::write(&tmp, out)?;
            let mut attempt = 0;
            loop {
                match std::fs::rename(&tmp, path) {
                    Ok(()) => break,
                    Err(e) if attempt == 9 => return Err(e.into()),
                    Err(_) => {
                        attempt += 1;
                        std::thread::sleep(std::time::Duration::from_millis(500));
                    }
                }
            }
        }
    }
    if fix {
        println!("diganti otomatis: {fixed}");
    }
    println!(
        "baris dengan nama tidak sesuai global: {}",
        report_rows.len()
    );
    for ((t, e), n) in most_common(&miss).into_iter().take(top) {
        println!("{n:6}  {t} -> {e}");
    }
    if let Some(rp) = report {
        let mut w = std::io::BufWriter::new(std::fs::File::create(rp)?);
        for r in &report_rows {
            w.write_all(r.as_bytes())?;
            w.write_all(NL.as_bytes())?;
        }
        w.flush()?;
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names() -> HashMap<String, String> {
        [
            ("不见山", "Hidden Mountain"),
            ("开封", "Kaifeng"),
            ("不见", "Hidden"),
        ]
        .into_iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
    }

    #[test]
    fn pinyin_variants() {
        let p = pinyin_of('山');
        assert!(p.contains(&"shan".to_string()), "{p:?}");
        let l = pinyin_of('绿');
        assert!(
            l.contains(&"lv".to_string()) && l.contains(&"lu".to_string()),
            "{l:?}"
        );
        assert_eq!(pinyin_of('A'), vec!["A".to_string()]);
        assert!(pinyin_of('1').is_empty());
    }

    #[test]
    fn anycase_and_capitalize() {
        assert_eq!(anycase(&["Gunung", "lembah"]), "[Gg]unung|[Ll]embah");
        assert_eq!(capitalize("shAN"), "Shan");
    }

    #[test]
    fn terms_longest_first_without_overlap() {
        let n = names();
        assert_eq!(terms_in("前往不见山和开封", &n, 3), ["不见山", "开封"]);
        assert_eq!(terms_in("没有", &n, 3), Vec::<String>::new());
    }

    #[test]
    fn variant_rx_replaces_pinyin_spelling() {
        let rx = variant_rx("不见山").unwrap();
        let v = "Pergi ke Gunung Bujian sekarang, lalu Bujianshan.";
        let out = rx.replace_all(v, "Hidden Mountain").into_owned();
        assert_eq!(
            out,
            "Pergi ke Hidden Mountain sekarang, lalu Hidden Mountain."
        );
        // not inside another word
        assert!(!rx.is_match("XBujian").unwrap());
    }

    #[test]
    fn segments_and_curated_fixes() {
        let n = names();
        assert_eq!(
            fix_segments("开封-任务", "Kota Kai-Misi", &n),
            "Kaifeng-Misi"
        );
        assert_eq!(
            fix_segments("开封-任务", "Kaifeng-Misi", &n),
            "Kaifeng-Misi"
        );
        assert_eq!(fix_segments("开封", "x", &n), "x");
        assert_eq!(
            fix_text("前往不见山", "Ke Gunung Tak Terlihat", &n),
            "Ke Hidden Mountain"
        );
        assert_eq!(
            fix_text("鸣金·影", "bellstrike · bayangan", &n),
            "Bellstrike - Umbra"
        );
        assert_eq!(
            fix_text("x", "Gunung Hidden Mountain", &n),
            "Hidden Mountain"
        );
    }

    #[test]
    fn py_dumps_generic_object() {
        let v: serde_json::Value =
            serde_json::from_str(r#"{"idx": 5, "v": "Halo \"x\"", "n": null, "t": [true, 1]}"#)
                .unwrap();
        assert_eq!(
            py_dumps(&v),
            r#"{"idx": 5, "v": "Halo \"x\"", "n": null, "t": [true, 1]}"#
        );
    }
}
