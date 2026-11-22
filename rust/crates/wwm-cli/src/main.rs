//! `wwm` — Rust port of the repo's Python tooling. Every subcommand takes the same
//! arguments as its Python script, so `tools/wwm.sh` can switch between the two.
//!
//! | wwm | Python |
//! |---|---|
//! | `info` / `dump` / `patch` | `wwm_locmap.py info/dump/patch` |
//! | `qa` | `tools/qa_check.py` |
//! | `expand` | `tools/expand_locale.py` |
//! | `rebuild-unique` | `tools/rebuild_unique_strings.py` |
//! | `progress` | `tools/progress.py` |
//! | `patch-all` | `tools/patch_all.py` |
//! | `manifest` | `tools/make_manifest.py` |
//! | `build-unique` | `translation_work/build_unique.py` |
//! | `compare-update` | `.claude/skills/wwm-game-update/scripts/compare_update.py` |
//! | `hook stop` | `tools/hooks/claude_stop_check.py` |
//! | `cn term-lookup` / `cn name-check` / `cn find-dict` | `cn/tools/*.py` |

use clap::{Parser, Subcommand};
use std::path::Path;
use wwm_tools::Exit;

#[derive(Parser)]
#[command(
    name = "wwm",
    version,
    about = "WWM translate_words_map toolkit (Rust port of the Python tools)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(clap::Args)]
struct Prof {
    /// global (WWM global, default) atau cn (WWM China)
    #[arg(long, default_value = "global", value_parser = ["global", "cn"])]
    profile: String,
}

#[derive(Subcommand)]
enum Cmd {
    /// print header/entry counts (wwm_locmap.py info)
    Info { src: String },
    /// decode container -> JSONL (wwm_locmap.py dump)
    Dump { src: String, out: String },
    /// overlay edits.jsonl onto src (wwm_locmap.py patch)
    Patch {
        src: String,
        jsonl: String,
        out: String,
        #[arg(long, default_value_t = 19)]
        level: i32,
    },
    /// validate translations (tools/qa_check.py)
    Qa {
        original: Option<String>,
        translated: Option<String>,
        /// validasi file locale per-idx terhadap unique_strings.jsonl
        #[arg(long, num_args = 1.., value_name = "FILE_OR_GLOB")]
        locale: Option<Vec<String>>,
        #[arg(long)]
        report: Option<String>,
        #[command(flatten)]
        p: Prof,
    },
    /// build a patch JSONL for one dump (tools/expand_locale.py)
    Expand {
        #[command(flatten)]
        p: Prof,
        /// default: dump pertama profil
        #[arg(long)]
        strings: Option<String>,
        /// default: strings.translated.jsonl profil
        #[arg(long)]
        out: Option<String>,
    },
    /// append new strings to unique_strings.jsonl (tools/rebuild_unique_strings.py)
    RebuildUnique {
        #[command(flatten)]
        p: Prof,
        #[arg(long)]
        strings: Option<String>,
        #[arg(long, num_args = 0..)]
        extra: Option<Vec<String>>,
        /// hitung & laporkan saja, jangan tulis ke unique_strings.jsonl
        #[arg(long)]
        dry_run: bool,
    },
    /// translation progress block (tools/progress.py)
    Progress {
        /// perbarui blok di Current-Status.md
        #[arg(long, conflicts_with = "check")]
        write: bool,
        /// exit 1 kalau blok basi
        #[arg(long)]
        check: bool,
        #[command(flatten)]
        p: Prof,
    },
    /// apply the dictionary to every variant (tools/patch_all.py)
    PatchAll {
        #[command(flatten)]
        p: Prof,
        #[arg(long)]
        outdir: Option<String>,
        #[arg(long, default_value_t = 12)]
        level: i32,
        #[arg(long, num_args = 0..)]
        files: Option<Vec<String>>,
    },
    /// release zip + manifest.json (tools/make_manifest.py)
    Manifest {
        #[command(flatten)]
        p: Prof,
        #[arg(long)]
        outdir: Option<String>,
        #[arg(long)]
        commit: Option<String>,
        #[arg(long)]
        archive: String,
    },
    /// one-time bootstrap of unique_strings.jsonl; refuses when it exists (translation_work/build_unique.py)
    BuildUnique,
    /// compare translate_words_map_* with their .before copies (wwm-game-update skill)
    CompareUpdate,
    /// Claude Code hooks
    Hook {
        #[command(subcommand)]
        which: HookCmd,
    },
    /// WWM China helpers (cn/tools/*.py)
    Cn {
        #[command(subcommand)]
        which: CnCmd,
    },
}

#[derive(Subcommand)]
enum HookCmd {
    /// Stop hook (tools/hooks/claude_stop_check.py)
    Stop,
}

#[derive(Subcommand)]
enum CnCmd {
    /// how global translated a Mandarin term (cn/tools/term_lookup.py)
    TermLookup {
        #[arg(required = true)]
        terms: Vec<String>,
        /// cari teks Mandarin yang MEMUAT istilah (default: teks persis)
        #[arg(long)]
        contains: bool,
        #[arg(long, default_value_t = 8)]
        limit: usize,
    },
    /// CN rows missing a global name (cn/tools/name_check.py)
    NameCheck {
        #[arg(long, default_value = "cn/locale/*.jsonl")]
        locale: String,
        /// rentang idx, mis. 90000-93999
        #[arg(long)]
        idx: Option<String>,
        /// tulis semua temuan ke JSONL ini
        #[arg(long)]
        report: Option<String>,
        /// ganti varian Pinyin di file locale
        #[arg(long)]
        fix: bool,
        #[arg(long, default_value_t = 40)]
        top: usize,
    },
    /// recover the v2 dictionary keystream (cn/tools/find_dict_in_dump.py)
    FindDict {
        dumps: Vec<String>,
        #[arg(long)]
        pid: Option<u32>,
        #[arg(long = "v2-dir")]
        v2_dir: Vec<String>,
        #[arg(long)]
        out: Option<String>,
    },
}

fn run(cli: Cli) -> anyhow::Result<i32> {
    use wwm_tools::*;
    let root = profiles::root;
    let prof =
        |p: &Prof| -> anyhow::Result<profiles::Profile> { profiles::get(&root()?, &p.profile) };
    // codec commands need the root only for the v2 keystream default path
    let codec_root = || root().unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
    match cli.cmd {
        Cmd::Info { src } => locmap::info(Path::new(&src), &codec_root()),
        Cmd::Dump { src, out } => locmap::dump(Path::new(&src), Path::new(&out), &codec_root()),
        Cmd::Patch {
            src,
            jsonl,
            out,
            level,
        } => locmap::patch(
            Path::new(&src),
            Path::new(&jsonl),
            Path::new(&out),
            level,
            &codec_root(),
        ),
        Cmd::Qa {
            original,
            translated,
            locale,
            report,
            p,
        } => match (locale, original, translated) {
            (Some(l), _, _) => qa::run_locale(&prof(&p)?, &l, report.as_deref()),
            (None, Some(o), Some(t)) => {
                qa::run_dump(Path::new(&o), Path::new(&t), report.as_deref())
            }
            _ => {
                eprintln!(
                    "wwm qa: error: butuh original.jsonl dan translated.jsonl, atau --locale"
                );
                Ok(2)
            }
        },
        Cmd::Expand { p, strings, out } => {
            dict::expand(&prof(&p)?, strings.as_deref(), out.as_deref())
        }
        Cmd::RebuildUnique {
            p,
            strings,
            extra,
            dry_run,
        } => unique::rebuild(&prof(&p)?, strings.as_deref(), extra.as_deref(), dry_run),
        Cmd::Progress { write, check, p } => {
            let mode = if write {
                progress::Mode::Write
            } else if check {
                progress::Mode::Check
            } else {
                progress::Mode::Print
            };
            progress::run(&prof(&p)?, mode)
        }
        Cmd::PatchAll {
            p,
            outdir,
            level,
            files,
        } => patch_all::run(&prof(&p)?, outdir.as_deref(), level, files.as_deref()),
        Cmd::Manifest {
            p,
            outdir,
            commit,
            archive,
        } => manifest::run(&prof(&p)?, outdir.as_deref(), commit.as_deref(), &archive),
        Cmd::BuildUnique => unique::build_unique(&root()?),
        Cmd::CompareUpdate => compare::run(&root()?),
        Cmd::Hook {
            which: HookCmd::Stop,
        } => hook::stop(&root()?),
        Cmd::Cn { which } => match which {
            CnCmd::TermLookup {
                terms,
                contains,
                limit,
            } => cn::term_lookup::run(&root()?, &terms, contains, limit),
            CnCmd::NameCheck {
                locale,
                idx,
                report,
                fix,
                top,
            } => cn::name_check::run(
                &root()?,
                &locale,
                idx.as_deref(),
                report.as_deref(),
                fix,
                top,
            ),
            CnCmd::FindDict {
                dumps,
                pid,
                v2_dir,
                out,
            } => cn::find_dict::run(&root()?, &dumps, pid, &v2_dir, out.as_deref()),
        },
    }
}

fn main() {
    let code = match run(Cli::parse()) {
        Ok(c) => c,
        Err(e) => {
            match e.downcast_ref::<Exit>() {
                Some(x) => eprintln!("{x}"),
                None => eprintln!("error: {e:#}"),
            }
            1
        }
    };
    std::process::exit(code);
}
