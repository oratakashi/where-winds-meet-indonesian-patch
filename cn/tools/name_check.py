#!/usr/bin/env python3
"""
name_check.py — cek nama resmi global di terjemahan CN (cn/locale/*.jsonl).

Nama (tempat, NPC, sekte, Inner Way, Path, item bernama) di CN harus sama persis dengan
bentuk yang dipakai baris global hasil jembatan. Kamus nama dibangun dari key bersama:
teks Mandarin utuh (2–8 aksara Han) di key yang juga ada di global, yang terjemahan
Indonesia globalnya == teks Inggrisnya (artinya global sengaja tidak menerjemahkannya).
Baris CN yang sumbernya memuat istilah itu tapi terjemahannya tidak memuat nama Inggris
tersebut dilaporkan.

    python cn/tools/name_check.py                       # ringkasan + istilah terbanyak
    python cn/tools/name_check.py --report names.jsonl  # semua temuan, per baris
    python cn/tools/name_check.py --idx 90000-93999     # hanya rentang idx (cek per batch)
    python cn/tools/name_check.py --fix                 # ganti varian Pinyin otomatis

--fix hanya mengganti tulisan Pinyin dari istilahnya (mis. "Gunung Bujian",
"Bujianshan" -> "Hidden Mountain"; "Lembah Hui" -> "Mirkvale"), dengan kata generik
Indonesia di depan atau kata generik Inggris di belakang. Terjemahan harfiah
("Gunung Tak Terlihat") tidak bisa ditebak dan tetap dilaporkan: perbaiki manual.

Butuh dump yang sama dengan term_lookup.py (cn/work/strings_zh_cn.jsonl +
strings.jsonl/strings_diff.jsonl). Lihat Obsidian-Vault/cn/knowladge/CN-Quick-Reference.md.
"""
import argparse, glob, json, os, re, sys, time
from collections import Counter, defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, os.path.join(ROOT, "tools"))
import profiles
from bridge import global_keys
from expand_locale import load_translated_map

HAN = re.compile(r"[一-鿿·]{2,8}")
# kata generik yang biasa menempel pada nama tempat yang ditulis Pinyin
PRE = ("Gunung|Lembah|Menara|Paviliun|Kota|Desa|Hutan|Aula|Balai|Kuil|Gua|Jembatan|Stasiun|"
       "Benteng|Kediaman|Pasar|Puncak|Lereng|Bukit|Tebing|Danau|Kolam|Sungai|Tepi|Dermaga|"
       "Gerbang|Jalan|Jalur|Distrik|Taman|Kebun|Istana|Akademi|Sekte|Geng|"
       "Perkemahan|Markas|Pos|Celah|Ngarai|Jurang|Lapangan|Lautan|Pulau|Makam|Gudang|Kantor|"
       "Rumah|Kampung|Biara|Pagoda|Lumbung|Teras|Kedai|Penginapan|Bengkel|Altar|"
       "Lonceng|Perahu|Kapal|Padang|Gurun|Kamp")
POST = ("Mountain|Mountains|Valley|Tower|Pavilion|Town|Village|Forest|Hall|Temple|Cave|"
        "Cavern|Bridge|Station|Fort|Fortress|Manor|Market|Bazaar|Peak|Slope|Hill|Ridge|Cliff|"
        "Crag|Lake|Pool|River|Bank|Dock|Pier|Gate|Road|Path|Ward|District|Garden|Palace|"
        "Academy|Sect|Gang|Camp|Base|Pass|Ferry|Crossing|Gorge|Abyss|Field|Sea|Island|Tomb|"
        "Barrow|Office|House|Monastery|Pagoda|Lane|Street|Avenue|Terrace|Inn|Shrine|Grove|"
        "Shan|Lou|Cun|Zhen|Gu|Lin|Ge|Si|Dian|Ting|Tai|Fang")


def pinyin_of(ch):
    from pypinyin import pinyin, Style
    out = set()
    for r in pinyin(ch, style=Style.NORMAL, heteronym=True)[0]:
        out.add(r)
        out.add(r.replace("ü", "u").replace("v", "u"))
        out.add(r.replace("ü", "v"))
    return sorted((x for x in out if x.isalpha()), key=len, reverse=True)


def build_names():
    """teks Mandarin -> nama Inggris yang global biarkan tetap Inggris."""
    CN = profiles.CN
    gk = global_keys()
    idn, _ = load_translated_map("global")
    by = defaultdict(Counter)
    with open(CN.path(CN.dumps[0][1]), encoding="utf-8") as f:
        for line in f:
            d = json.loads(line)
            e = gk.get(d["h"])
            if e is not None and "v" in d and HAN.fullmatch(d["v"]):
                by[d["v"]][e] += 1
    names = {}
    for t, c in by.items():
        e, n = c.most_common(1)[0]
        if n >= 0.6 * sum(c.values()) and idn.get(e) == e and re.search(r"[A-Z]", e) \
                and t not in NOT_NAMES:
            names[t] = e
    # Global juga membiarkan beberapa kata biasa tetap Inggris (Chat, Organization, ...).
    # Nama asli ditulis kapital di tengah kalimat Inggris; kata biasa lebih sering kecil.
    targets = set(names.values())
    targets |= {e.lower() for e in targets}
    seen = Counter()
    with open(profiles.GLOBAL.unique, encoding="utf-8") as f:
        for line in f:
            w = WORD.findall(json.loads(line)["v"])
            for i in range(len(w)):
                g = w[i]
                for j in range(i + 1, min(i + 5, len(w) + 1)):
                    if j > i + 1:
                        g += " " + w[j - 1]
                    if g in targets:
                        seen[g] += 1
    return {t: e for t, e in names.items() if seen[e.lower()] < seen[e] or e.lower() == e}


WORD = re.compile(r"[A-Za-z][\w'’-]*")
NOT_NAMES = {"工匠"}    # kata biasa yang di key bersama kebetulan jadi nama NPC


def terms_in(s, names, maxlen):
    """istilah di teks sumber, cocok terpanjang dulu, tanpa tumpang tindih."""
    i, hits = 0, []
    while i < len(s):
        for L in range(min(maxlen, len(s) - i), 1, -1):
            if s[i:i + L] in names:
                hits.append(s[i:i + L]); i += L; break
        else:
            i += 1
    return hits


def _anycase(words):
    return "|".join("[%s%s]%s" % (w[0].upper(), w[0].lower(), w[1:]) for w in words)

PRE_RX, POST_RX = _anycase(PRE.split("|")), _anycase(POST.split("|"))

_rx = {}
def variant_rx(term):
    """regex untuk tulisan Pinyin dari `term` (atau sebagian berurutan, paling banyak dua
    aksara terlewat — itu bagian yang jadi kata generik), plus kata generik di sekitarnya.
    Suku pertama harus berhuruf kapital: nama ditulis kapital, kata biasa tidak."""
    if term in _rx:
        return _rx[term]
    chars = [c for c in term if c != "·"]
    syl = [pinyin_of(c) for c in chars]
    pats = []
    for L in range(len(chars), 0, -1):
        if L < len(chars) - 2:
            break
        for a in range(0, len(chars) - L + 1):
            run = "(?:%s)" % "|".join(x.capitalize() for x in syl[a])
            for s in syl[a + 1:a + L]:
                run += "[ '’]?(?:%s)" % _anycase(s)
            # Pinyin sebagian (mis. "Hangzhou" dari 杭州百姓) hanya diganti kalau aksara sisanya
            # jelas menjadi kata generik yang menempel; tanpa itu sisanya ("Warga") tertinggal
            if L < len(chars):
                pats.append(r"(?:(?:%s) ){1,2}%s(?: (?:%s))?|%s (?:%s)"
                            % (PRE_RX, run, POST_RX, run, POST_RX))
            else:
                pats.append(r"(?:(?:%s) ){0,2}%s(?: (?:%s))?" % (PRE_RX, run, POST_RX))
    rx = re.compile(r"(?<![\w'’])(?:%s)(?![\w'’])" % "|".join(pats)) if pats else None
    _rx[term] = rx
    return rx


def fix_segments(s, v, names):
    """Label editor `A-B-C`: kalau jumlah segmen sumber dan terjemahan sama, segmen yang
    sumbernya persis sebuah nama diganti nama globalnya (posisinya pasti sama)."""
    ss, vs = s.split("-"), v.split("-")
    if len(ss) < 2 or len(ss) != len(vs):
        return v
    for k, seg in enumerate(ss):
        e = names.get(seg.strip())
        if e and e.lower() not in vs[k].lower():
            lead = vs[k][:len(vs[k]) - len(vs[k].lstrip())]
            tail = vs[k][len(vs[k].rstrip()):]
            vs[k] = lead + e + tail
    return "-".join(vs)


# Varian yang bukan Pinyin, dikumpulkan dari cn/locale (2026-10-25). (aksara di sumber,
# regex varian, pengganti). Path: "Bellstrike·Bayangan" / "Bellstrike Ying" -> "Bellstrike - Umbra".
_PATH = {"鸣金": ("Bellstrike", "Ming ?jin"), "裂石": ("Stonesplit", "(?:Pemecah|Pecah) Batu|Lie ?shi"),
         "牵丝": ("Silkbind", "Qian ?si|Tarikan Sutra|Penarik Sutra"),
         "破竹": ("Bamboocut", "(?:Pemecah|Pecah) Bambu|Po ?zhu")}
_SUFFIX = {
    "影": ("Umbra", "Bayangan|Ying|Shadow"), "虹": ("Splendor", "Pelangi|Hong|Rainbow"),
    "霖": ("Deluge", "Hujan|Lin"), "威": ("Might", "Kekuatan|Wei"), "玉": ("Jade", "Giok|Yu"),
    "尘": ("Dust", "Debu|Chen"), "钧": ("Strength", "Jun"), "风": ("Wind", "Angin|Feng"),
    "鸢": ("Kite", "Layang-layang|Layang|Elang|Yuan"), "樽": ("Draught", "Cawan|Zun|Cask"),
    "翊": ("Wings", "Sayap|Yi"), "韵": ("Yun", "Irama|Rhythm"),
}
CURATED = []
for _zh, (_en, _xalts) in _PATH.items():
    for _ch, (_y, _alts) in _SUFFIX.items():
        _prx = re.compile(r"(?:%s|%s)\s*(?:[·\-–]\s*|\s)(?:%s|%s)(?![\w'’])"
                         % (_en, _xalts, _alts, _y), re.I)
        CURATED.append((_zh + "·" + _ch, _prx, "%s - %s" % (_en, _y)))
        # tanpa titik tengah (流派塔牵丝霖) global menulis "Silkbind Deluge"
        CURATED.append((_zh + _ch, _prx, "%s %s" % (_en, _y)))
CURATED += [
    ("流派塔", re.compile(r"\b[Mm]enara [Aa]liran\b|\b[Mm]enara Path\b"), "Path Tower"),
    ("心法心得", re.compile(r"\b(?:[Cc]atatan|[Ww]awasan|[Pp]emahaman|[Pp]engalaman) Inner Way\b"
                         r"(?! Notes)"), "Inner Way Notes"),
    ("心法", re.compile(r"\b[Tt]eknik [Bb]atin\b"), "Inner Way"),
]
# Nama tempat yang diterjemahkan harfiah (ditambang dari cn/locale 2026-10-25).
_LITERAL = [
    ("皇宫", "Imperial Palace", r"[Ii]stana (?:Kekaisaran|Kaisar)"),
    ("千年渡", "Millennium Crossing", r"Persimpangan Milenium|Penyeberangan Seribu Tahun"),
    ("天工地窟", "Divinecraft Dungeon", r"Gua (?:Tiangong|Divinecraft)|Divinecraft Gua"),
    ("南门大街", "South Gate Avenue", r"[Jj]alan [Gg]erbang [Ss]elatan|South Gate Street"),
    ("官署", "Hall of State", r"[Kk]antor [Pp]emerintah(?:an)?"),
    ("不见山", "Hidden Mountain", r"Gunung Tak Terlihat"),
    ("墨山道", "Mohist Hill", r"Jalur Gunung Tinta"),
    ("栖鸟岩", "Spiritroost Crag", r"(?:(?:Tebing|Batu) )?Burung Bertengger"),
    ("滹沱南岸", "South Hutuo Bank", r"[Tt]epi [Ss]elatan(?: Sungai)? Hutuo"),
    ("绣金楼", "Aureate Pavilion", r"(?:(?:Menara|Paviliun) )?Sulam Emas"),
    ("御苑南", "South Imperial Garden",
     r"(?:Taman (?:Kekaisaran|Istana)|Imperial Garden) Selatan|Selatan Taman (?:Kekaisaran|Istana)"),
    ("御苑北", "North Imperial Garden",
     r"(?:Taman (?:Kekaisaran|Istana)|Imperial Garden) Utara|Utara Taman (?:Kekaisaran|Istana)"),
    ("嗟叹崖", "Gasping Cliff", r"Tebing (?:yang Terengah|Ratapan|Keluh(?: Kesah)?)"),
    ("佛光顶", "Halo Peak", r"Puncak Cahaya(?: Buddha)?"),
    ("花海", "Flower Expanse", r"[Ll]autan [Bb]unga"),
    ("将军祠", "General's Shrine", r"Kuil (?:Sang )?Jenderal"),
    ("鬼樊楼", "Ghost Revelry Hall", r"Revelry Hall Hantu|Menara Fan Hantu"),
    ("西湖", "West Lake", r"Danau Barat"),
    ("飞天残垣", "Skyward City Ruins", r"Reruntuhan (?:Terbang|Feitian)"),
    ("菩提苦海", "Bodhi Sea", r"(?:Bodhi )?Lautan (?:Derita|Pahit)(?: Bodhi)?"),
    ("不羡仙", "Blissful Retreat",
     r"Tanpa Pamrih Mendambakan Dewa|Tak (?:Iri|Mengidam|Mendambakan) Dewa"),
    ("封桩库", "Sealed Treasury", r"Gudang (?:Tiang Penyegel|Fengzhuang)"),
    ("太岳台", "Skyreach Terrace", r"Teras (?:Skyreach|Taiyue)|Taiyue Terrace"),
    ("碧水云涛", "Jadestream Valley",
     r"Biyun Yuntao|Air Hijau Ombak Awan|Ombak Awan (?:Air Hijau|Biru)"),
    ("佛爷寨", "Buddha Fort", r"(?:Benteng|Perkemahan) Buddha"),
    ("角门里", "Forsaken Quarter", r"[Dd]alam [Gg]erbang [Ss]amping|[Gg]erbang [Ss]amping"),
    ("心湖", "Heart Lake", r"Danau Hati"),
    ("饮马隘", "Steed's Pass", r"Celah Minum Kuda|Yinma Pass"),
    ("演武场", "Training Grounds", r"[Ll]apangan [Ll]atihan(?: [Bb]ela [Dd]iri)?|[Aa]rena [Ll]atihan"),
    ("太清楼", "Purity Tower", r"Menara Kesucian"),
    ("花船集", "Flower Boat Market", r"Pasar Perahu Bunga"),
    ("千佛村", "Thousand-Buddha Village", r"Desa Seribu Buddha|Thousand Buddha Vale"),
    ("后苑", "Rear Garden", r"[Tt]aman [Bb]elakang"),
    ("封丘村", "Verdant Hill Village", r"Desa Verdant Hill"),
    ("天上来", "Heavenfall", r"Datang dari Langit"),
    ("来生岸", "Floral Expanse Beyond",
     r"Laborious Shore|Shore of Rebirth|Tepi Kehidupan(?: Mendatang| Berikutnya)?"),
    ("思危园", "Vigilance Garden", r"Taman Kewaspadaan"),
    ("灵隐新寺", "New Lingyin Temple", r"Kuil Baru Lingyin"),
    ("玉露台", "Cleardew Terrace", r"Teras (?:Embun )?Giok"),
    ("时楼", "Temporal Pavilion", r"Menara Waktu"),
    ("陷城湖", "Sunken City Lake", r"Danau Kota(?: (?:Runtuh|Tenggelam))?"),
    ("觉障林", "Perception Forest",
     r"Awakening Forest|Hutan (?:Penyadar|Juezhang)|Grove of Barriers"),
    ("黄河", "Yellow River", r"Sungai Kuning"),
    ("酒肉山林", "Gourmand's Grove", r"Hutan(?: Gunung)? Arak(?: dan)? Daging|Arak Daging"),
    ("狮子坟", "Lion Barrow", r"Makam Singa"),
    ("武庙", "Martial Temple", r"Kuil (?:Bela Diri|Militer)"),
    ("酒香塔", "Sweetwine Tower", r"Menara (?:Aroma|Wangi) Arak"),
    ("鬼市", "Ghostlight Market", r"(?:Anak )?Pasar Hantu|Ghost Market"),
    ("北盟遗址", "Northern Vow Ruins", r"(?:(?:Situs|Reruntuhan) )?Aliansi Utara"),
    ("博浪沙", "Roaring Sands", r"Bolang ?[Ss]ands|Bolangsha"),
    ("万古一人殿", "Eternal Solitude Hall", r"Aula Satu[- ]Satunya Sepanjang Masa"),
    ("隐月山", "Moonveil Mountain", r"Gunung Bulan Tersembunyi"),
    ("常平仓", "Ever-Normal Granary", r"Changping Granary|Lumbung Changping"),
    ("雾谷", "Misty Valley", r"Lembah Kabut(?: Tipis)?"),
    ("赤龙堂", "Bloodscale Hall", r"Aula Naga Merah|Red Dragon Hall"),
    ("骆驼驿", "Camel Station", r"Stasiun Unta"),
    ("封龙山", "Dragonbend Mountain", r"Gunung (?:Dragonbend|Fenglong)"),
    ("神仙渡", "Heaven's Pier", r"Dermaga Dewa|Shenxian (?:Ferry|Crossing)"),
    ("九流门驻地", "Nine Mortal Ways Base", r"Markas Nine Mortal Ways"),
    ("九流门", "Nine Mortal Ways", r"(?:Gerbang|Sekte) Sembilan Aliran|Sekte Jiuliu"),
    ("望淮南崖", "Dreamfall Cliff", r"Tebing Memandang Huainan"),
    ("太仓粟", "Granary of Plenty", r"Lumbung Taicang"),
    ("三馆", "Reserve Archives", r"Tiga (?:Balai|Paviliun|Gedung)"),
    ("前广场", "Front Plaza", r"[Aa]lun-alun [Dd]epan"),
]
CURATED += [(t, re.compile(r"(?<![\w'’])(?:%s)(?![\w'’])" % p), e) for t, e, p in _LITERAL]

# Sisa ganda setelah penggantian: "Gunung Hidden Mountain", "Tepi Selatan South Hutuo Bank".
_DUP = {"Mountain": "Gunung", "Village": "Desa", "Town": "Kota", "Hall": "Aula|Balai",
        "Tower": "Menara", "Pavilion": "Paviliun", "Lake": "Danau", "Cliff": "Tebing",
        "Crag": "Tebing|Batu", "Temple": "Kuil", "Market": "Pasar", "Garden": "Taman",
        "Valley": "Lembah", "Forest": "Hutan", "Bridge": "Jembatan", "Station": "Stasiun",
        "Fort": "Benteng", "Manor": "Kediaman", "Peak": "Puncak", "River": "Sungai",
        "Bank": "Tepi(?: [Ss]elatan| [Uu]tara)?", "Pass": "Celah", "Ruins": "Reruntuhan",
        "Academy": "Akademi", "Cavern": "Gua", "Dungeon": "Gua", "Terrace": "Teras",
        "Shrine": "Kuil", "Grove": "Hutan", "Sea": "Lautan", "Gate": "Gerbang",
        "Avenue": "Jalan", "Granary": "Lumbung", "Treasury": "Gudang", "Base": "Markas"}
DUP_RX = [(re.compile(r"(?<![\w'’])(?:%s) ((?:[A-Z][\w'’-]* ){1,3}%s)(?![\w'’])" % (i, e)), r"\1")
          for e, i in _DUP.items()]


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--locale", default="cn/locale/*.jsonl")
    ap.add_argument("--idx", help="rentang idx, mis. 90000-93999")
    ap.add_argument("--report", help="tulis semua temuan ke JSONL ini")
    ap.add_argument("--fix", action="store_true", help="ganti varian Pinyin di file locale")
    ap.add_argument("--top", type=int, default=40)
    a = ap.parse_args()
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8")
    lo, hi = (map(int, a.idx.split("-")) if a.idx else (0, 1 << 62))

    names = build_names()
    maxlen = max(map(len, names))
    CN = profiles.CN
    src = {}
    with open(CN.unique, encoding="utf-8") as f:
        for line in f:
            if line.strip():
                d = json.loads(line); src[d["idx"]] = d["v"]

    miss, report, fixed = Counter(), [], 0
    for path in sorted(glob.glob(os.path.join(ROOT, a.locale))):
        rows = [json.loads(l) for l in open(path, encoding="utf-8") if l.strip()]
        changed = False
        for d in rows:
            if not lo <= d["idx"] <= hi:
                continue
            s = src[d["idx"]]
            if a.fix:
                v2 = fix_segments(s, d["v"], names)
                for need, rx, rep in CURATED:
                    if need in s and rep.lower() not in v2.lower():
                        v2 = rx.sub(rep, v2)
                for rx, rep in DUP_RX:
                    v2 = rx.sub(rep, v2)
                if v2 != d["v"]:
                    d["v"] = v2; changed = True; fixed += 1
            bad = []
            for t in terms_in(s, names, maxlen):
                e = names[t]
                if e.lower() in d["v"].lower():
                    continue
                rx = variant_rx(t) if a.fix else None
                if rx is not None:
                    v2, n = rx.subn(lambda m: m.group(0) if e.lower() in m.group(0).lower()
                                    else e, d["v"])
                    if n and v2 != d["v"]:
                        d["v"] = v2; changed = True; fixed += 1
                        continue
                bad.append((t, e))
            for t, e in bad:
                miss[(t, e)] += 1
            if bad:
                report.append({"file": os.path.basename(path), "idx": d["idx"], "src": s,
                               "v": d["v"], "names": {t: e for t, e in bad}})
        if changed:
            # tulis ke file sementara lalu ganti: di Windows file yang sedang dipindai
            # (antivirus/Obsidian) kadang menolak dibuka untuk ditulis (Errno 22)
            tmp = path + ".tmp"
            with open(tmp, "w", encoding="utf-8", newline="\n") as f:
                for d in rows:
                    f.write(json.dumps(d, ensure_ascii=False) + "\n")
            for attempt in range(10):
                try:
                    os.replace(tmp, path); break
                except OSError:
                    if attempt == 9:
                        raise
                    time.sleep(0.5)

    if a.fix:
        print("diganti otomatis: %d" % fixed)
    print("baris dengan nama tidak sesuai global: %d" % len(report))
    for (t, e), n in miss.most_common(a.top):
        print("%6d  %s -> %s" % (n, t, e))
    if a.report:
        with open(a.report, "w", encoding="utf-8") as f:
            for r in report:
                f.write(json.dumps(r, ensure_ascii=False) + "\n")


if __name__ == "__main__":
    main()
