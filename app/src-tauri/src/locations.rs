//! Find the locale folders to guard from whatever folder the player picked: the launcher's
//! install folder, a game variant folder (`wwm_lite`, `yysls_fast`), or anything inside one.
use crate::model::Profile;
use crate::store::GuardDir;
use std::fs;
use std::path::{Path, PathBuf};

pub fn detect(picked: &Path) -> Vec<GuardDir> {
    // Candidate variant folders: the picked folder, its parents (the player picked
    // something inside the game), and its children (the player picked the launcher folder).
    let mut bases: Vec<PathBuf> = picked.ancestors().map(Path::to_path_buf).collect();
    if let Ok(rd) = fs::read_dir(picked) {
        bases.extend(rd.flatten().map(|e| e.path()).filter(|p| p.is_dir()));
    }
    let mut found: Vec<GuardDir> = Vec::new();
    for base in &bases {
        for profile in Profile::ALL {
            for rel in profile.locale_dirs() {
                let dir = base.join(rel);
                let has_file = profile.files().iter().any(|f| dir.join(f).is_file());
                if has_file && !found.iter().any(|g| g.path == dir) {
                    found.push(GuardDir { profile, path: dir });
                }
            }
        }
    }
    found
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
        let a = game(tmp.path(), "wwm_lite", r"LocalData\Patch\HD\oversea\locale", "translate_words_map_en");
        let b = game(tmp.path(), "wwm_lite", r"Package\HD\oversea\locale", "translate_words_map_en_diff");
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
}
