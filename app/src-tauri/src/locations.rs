//! Where the game is: find installed copies on this machine (`discover`), and find the locale
//! folders to guard from a folder (`detect`) — a launcher's install folder, a game folder
//! (`wwm_lite`, `yysls_fast`, the Steam install folder), or anything inside one.
use crate::model::{Client, OVERLAY_LOCALE, PACKAGE_LOCALE, Profile};
use crate::store::GuardDir;
use std::fs;
use std::path::{Path, PathBuf};

/// Where Winds Meet on Steam.
const STEAM_APP_ID: &str = "3564740";

pub fn detect(picked: &Path) -> Vec<GuardDir> {
    // Candidate game folders: the picked folder, its parents (the player picked
    // something inside the game), and its children (the player picked the launcher folder).
    let mut bases: Vec<PathBuf> = picked.ancestors().map(Path::to_path_buf).collect();
    if let Ok(rd) = fs::read_dir(picked) {
        bases.extend(rd.flatten().map(|e| e.path()).filter(|p| p.is_dir()));
    }
    let mut found: Vec<GuardDir> = Vec::new();
    for base in &bases {
        let client = Client::of(base);
        for profile in Profile::ALL {
            for rel in profile.locale_dirs(client) {
                let dir = base.join(rel);
                if has_locale_file(profile, &dir) && !found.iter().any(|g| g.path == dir) {
                    found.push(GuardDir { profile, path: dir });
                }
            }
        }
    }
    found
}

fn has_locale_file(profile: Profile, dir: &Path) -> bool {
    profile.files().iter().any(|f| dir.join(f).is_file())
}

/// Locale folders of every copy of the game found on this machine (Steam libraries and the
/// NetEase launchers' uninstall entries).
pub fn discover() -> Vec<GuardDir> {
    let mut found: Vec<GuardDir> = Vec::new();
    for root in install_roots() {
        for g in detect(&root) {
            if !found.iter().any(|f| same_path(&f.path, &g.path)) {
                found.push(g);
            }
        }
    }
    found
}

/// Windows paths differ in case between sources (the registry says `d:/steam`).
pub fn same_path(a: &Path, b: &Path) -> bool {
    a.to_string_lossy().eq_ignore_ascii_case(&b.to_string_lossy())
}

/// Versions up to 0.1 guarded the Steam `LocalData\Patch` overlay, which the patcher
/// re-downloads on every launch: guard the `Package` folder next to it instead.
/// Returns whether anything changed.
pub fn migrate(dirs: &mut Vec<GuardDir>) -> bool {
    let mut out: Vec<GuardDir> = Vec::new();
    for d in dirs.iter() {
        let mut d = d.clone();
        if d.profile == Profile::Global
            && Client::of(&d.path) == Client::Steam
            && d.path.ends_with(OVERLAY_LOCALE)
        {
            // <game>\LocalData\Patch\HD\oversea\locale -> <game>
            if let Some(game) = d.path.ancestors().nth(5) {
                d.path = game.join(PACKAGE_LOCALE);
            }
        }
        if !out.contains(&d) {
            out.push(d);
        }
    }
    let changed = out != *dirs;
    *dirs = out;
    changed
}

/// `<library>\steamapps\common\<installdir>` for every Steam library that has the game.
fn steam_roots(steam: &Path) -> Vec<PathBuf> {
    let apps = steam.join("steamapps");
    // The library list first: it spells paths the way Explorer does.
    let mut libs: Vec<PathBuf> = Vec::new();
    if let Ok(vdf) = fs::read_to_string(apps.join("libraryfolders.vdf")) {
        libs.extend(vdf.lines().filter_map(|l| vdf_value(l, "path")).map(PathBuf::from));
    }
    if !libs.iter().any(|l| same_path(l, steam)) {
        libs.push(steam.to_path_buf());
    }
    let mut roots = Vec::new();
    for lib in libs {
        let apps = lib.join("steamapps");
        let Ok(acf) = fs::read_to_string(apps.join(format!("appmanifest_{STEAM_APP_ID}.acf")))
        else {
            continue;
        };
        if let Some(dir) = acf.lines().find_map(|l| vdf_value(l, "installdir")) {
            roots.push(apps.join("common").join(dir));
        }
    }
    roots
}

/// The value of a `"key"  "value"` line of a Valve KeyValues file (`.vdf`, `.acf`).
fn vdf_value(line: &str, key: &str) -> Option<String> {
    let rest = line.trim().strip_prefix('"')?.strip_prefix(key)?.strip_prefix('"')?;
    let value = rest.trim().strip_prefix('"')?.strip_suffix('"')?;
    Some(value.replace(r"\\", r"\"))
}

/// The install folder an uninstall registry entry points at. The NetEase launchers leave
/// `InstallLocation` empty, so fall back to the folder of the uninstaller.
fn uninstall_root(install_location: &str, uninstall_string: &str) -> Option<PathBuf> {
    let loc = install_location.trim().trim_matches('"');
    if !loc.is_empty() {
        return Some(PathBuf::from(loc));
    }
    let exe = Path::new(uninstall_string.trim().trim_matches('"'));
    let is_exe = exe.extension().is_some_and(|e| e.eq_ignore_ascii_case("exe"));
    if is_exe { exe.parent().map(Path::to_path_buf) } else { None }
}

#[cfg(windows)]
fn install_roots() -> Vec<PathBuf> {
    use winreg::RegKey;
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};

    const UNINSTALL: [&str; 2] = [
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
    ];
    let hives = [RegKey::predef(HKEY_CURRENT_USER), RegKey::predef(HKEY_LOCAL_MACHINE)];
    let value = |key: &RegKey, name: &str| key.get_value::<String, _>(name).unwrap_or_default();
    let mut roots: Vec<PathBuf> = Vec::new();

    let steam_keys = [
        (&hives[0], r"Software\Valve\Steam", "SteamPath"),
        (&hives[1], r"SOFTWARE\WOW6432Node\Valve\Steam", "InstallPath"),
        (&hives[1], r"SOFTWARE\Valve\Steam", "InstallPath"),
    ];
    for (hive, key, name) in steam_keys {
        if let Ok(k) = hive.open_subkey(key) {
            let path = value(&k, name);
            if !path.is_empty() {
                // SteamPath is written with forward slashes.
                roots.extend(steam_roots(Path::new(&path.replace('/', r"\"))));
            }
        }
    }

    let entries = [
        "wwm_launcher_uninstall".to_string(),
        "yysls_launcher_uninstall".to_string(),
        format!("Steam App {STEAM_APP_ID}"),
    ];
    for hive in &hives {
        for base in UNINSTALL {
            for entry in &entries {
                let Ok(k) = hive.open_subkey(format!(r"{base}\{entry}")) else { continue };
                roots.extend(uninstall_root(
                    &value(&k, "InstallLocation"),
                    &value(&k, "UninstallString"),
                ));
            }
        }
    }
    roots
}

#[cfg(not(windows))]
fn install_roots() -> Vec<PathBuf> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game(root: &Path, variant: &str, rel: &str, file: &str) -> PathBuf {
        let dir = root.join(variant).join(rel);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(file), b"x").unwrap();
        dir
    }

    #[test]
    fn finds_both_global_folders_from_the_launcher_folder() {
        let tmp = tempfile::tempdir().unwrap();
        let a = game(tmp.path(), "wwm_lite", OVERLAY_LOCALE, "translate_words_map_en");
        let b = game(tmp.path(), "wwm_lite", PACKAGE_LOCALE, "translate_words_map_en_diff");
        let got = detect(tmp.path());
        assert_eq!(got.len(), 2);
        assert!(got.iter().all(|g| g.profile == Profile::Global));
        assert!(got.iter().any(|g| g.path == a) && got.iter().any(|g| g.path == b));
    }

    #[test]
    fn finds_cn_from_inside_the_locale_folder() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = game(tmp.path(), "yysls_fast", r"LocalData\Patch\HD\v2\locale", "translate_words_map_zh_cn");
        let got = detect(&dir);
        assert_eq!(got, vec![GuardDir { profile: Profile::Cn, path: dir }]);
    }

    #[test]
    fn ignores_folders_without_locale_files() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(r"wwm_lite\LocalData\Patch\HD\oversea\locale")).unwrap();
        assert!(detect(tmp.path()).is_empty());
    }

    /// A Steam library with the game in it; returns (steam folder, game folder).
    fn steam(tmp: &Path) -> (PathBuf, PathBuf) {
        let steam = tmp.join("Steam");
        let lib = tmp.join("Games");
        fs::create_dir_all(steam.join("steamapps")).unwrap();
        fs::create_dir_all(lib.join("steamapps")).unwrap();
        let vdf = format!(
            "\"libraryfolders\"\n{{\n\t\"0\"\n\t{{\n\t\t\"path\"\t\t\"{}\"\n\t}}\n\t\"1\"\n\t{{\n\t\t\"path\"\t\t\"{}\"\n\t}}\n}}\n",
            steam.display().to_string().replace('\\', r"\\"),
            lib.display().to_string().replace('\\', r"\\"),
        );
        fs::write(steam.join(r"steamapps\libraryfolders.vdf"), vdf).unwrap();
        fs::write(
            lib.join(format!(r"steamapps\appmanifest_{STEAM_APP_ID}.acf")),
            "\"AppState\"\n{\n\t\"name\"\t\t\"Where Winds Meet\"\n\t\"installdir\"\t\t\"Where Winds Meet\"\n}\n",
        )
        .unwrap();
        let common = lib.join(r"steamapps\common");
        game(&common, "Where Winds Meet", PACKAGE_LOCALE, "translate_words_map_en");
        game(&common, "Where Winds Meet", OVERLAY_LOCALE, "translate_words_map_en_diff");
        (steam, common.join("Where Winds Meet"))
    }

    #[test]
    fn steam_install_is_found_through_the_library_list() {
        let tmp = tempfile::tempdir().unwrap();
        let (steam, game) = steam(tmp.path());
        assert_eq!(steam_roots(&steam), vec![game]);
    }

    #[test]
    fn steam_guards_only_the_package_folder() {
        let tmp = tempfile::tempdir().unwrap();
        let (_, game) = steam(tmp.path());
        let want = vec![GuardDir { profile: Profile::Global, path: game.join(PACKAGE_LOCALE) }];
        assert_eq!(detect(&game), want);
        // Even when the player picks the overlay folder itself.
        assert_eq!(detect(&game.join(OVERLAY_LOCALE)), want);
    }

    #[test]
    fn old_steam_overlay_entry_moves_to_package() {
        let steam_game = Path::new(r"D:\Steam\steamapps\common\Where Winds Meet");
        let launcher = GuardDir {
            profile: Profile::Global,
            path: Path::new(r"D:\wwm\wwm_lite").join(OVERLAY_LOCALE),
        };
        let package = GuardDir { profile: Profile::Global, path: steam_game.join(PACKAGE_LOCALE) };
        let mut dirs = vec![
            launcher.clone(),
            GuardDir { profile: Profile::Global, path: steam_game.join(OVERLAY_LOCALE) },
            package.clone(),
        ];
        assert!(migrate(&mut dirs));
        assert_eq!(dirs, vec![launcher, package]);
        assert!(!migrate(&mut dirs));
    }

    #[test]
    fn reads_valve_key_values() {
        assert_eq!(vdf_value("\t\t\"path\"\t\t\"D:\\\\Steam\"", "path").as_deref(), Some(r"D:\Steam"));
        assert_eq!(vdf_value("\t\"installdir\"\t\t\"Where Winds Meet\"", "installdir").as_deref(), Some("Where Winds Meet"));
        assert_eq!(vdf_value("\t\"pathology\"\t\t\"x\"", "path"), None);
    }

    #[test]
    fn uninstall_entry_gives_the_install_folder() {
        let p = |s: &str| Some(PathBuf::from(s));
        assert_eq!(uninstall_root("", r"D:\wwm\uninstaller.exe"), p(r"D:\wwm"));
        assert_eq!(uninstall_root(r"D:\Steam\steamapps\common\Where Winds Meet", "\"D:\\Steam\\steam.exe\" steam://uninstall/3564740"), p(r"D:\Steam\steamapps\common\Where Winds Meet"));
        assert_eq!(uninstall_root("", "\"D:\\Steam\\steam.exe\" steam://uninstall/1"), None);
    }
}
