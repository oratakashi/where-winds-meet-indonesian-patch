# Patch Guard — `app/`

**WWM Patch Guard** is the Windows tray app players run to keep the Indonesian patch
installed. NetEase's patcher puts the original `translate_words_map_*` files back after a
game update (see "Deployment gotcha" in `CLAUDE.md`), and Patch Guard reinstalls the patch on
its own. It needs no codec: it compares sha256 hashes against the release `manifest.json` and
copies files.

Related: [[Format-Spec]] (the files it copies), [[Current-Status]] (translation progress the
releases carry). Developer quick start: `docs/Developer-Guide.md` §"WWM Patch Guard".

## Which folders it guards

| Client | Folder guarded | Why |
| --- | --- | --- |
| Official launcher (global, `wwm_lite`) | `LocalData\Patch\HD\oversea\locale` | The patch holds here between updates; every server update restores the originals (whole `pkg_en` re-hashed). |
| Steam | `Package\HD\oversea\locale` **only** | Steam's in-game patcher re-downloads the `LocalData\Patch` overlay on every launch. `Package\…` is only replaced by a Steam depot update. |
| China client (frozen) | `LocalData\Patch\HD\v2\locale` | Only while its originals still match the frozen `cn-latest` release. |

`core/locations.rs` finds installs: Steam via registry `SteamPath` → `libraryfolders.vdf` →
`appmanifest_3564740.acf`; the NetEase launchers via their uninstall registry entries.

Rules (`core/guard.rs`, unit-tested):

- The file is the original the release was built from → reinstall the cached patched build.
- The file is a **new** original (real game update) → wait for a release built for exactly that
  file. Never install an outdated patch over a newer game.
- The file matches no known original → leave it alone; installing anyway is a user-confirmed
  "force install".
- Backups of replaced originals live outside the game folder (the patcher deletes unknown files
  there, Android even deleted `.ori` backups).

## Architecture (GPUI, since v0.4)

```text
app/src/
  main.rs        update helper first (--apply-update), then app::run()
  app.rs         single instance → pending update? → engine → GPUI window + command loop (Cmd)
  state.rs       window state + actions (AppState)
  core/          no UI code
    guard.rs     install/restore rules
    engine.rs    guard thread (watch + 30 s poll) and app-update thread (every 6 h)
    remote.rs    release downloads: signed manifest.json, zip, small assets
    update.rs    self-update: signature check, version compare, download, helper
    store.rs     %LOCALAPPDATA%\com.oratakashi.wwm (config, history, state, caches, updates/)
    locations.rs installed-game discovery
  shell/         tray (tray-icon), single instance (named mutex + event), autostart, system
  fluent/        WinUI 3 look-alike controls (Button, Toggle, badge, error/info bar)
  pages/         Home, Settings, History
```

v0.3 and earlier were Tauri 2 + Leptos; `docs/Patch-Guard-GPUI-Plan.md` records the port. The data
folder name kept Tauri's identifier, so config and history carried over.

## Release assets

Built by `.github/workflows/release-patch.yml` on every push to `main` (rolling tag `latest`);
`release-patch-cn.yml` (manual only) builds `cn-latest`.

| Asset | Written by | Read by Patch Guard as |
| --- | --- | --- |
| `wwm-indonesian-patch.zip` | `wwm manifest --archive` | the patched files, flat, by name; trusted through its sha256 in the manifest |
| `manifest.json` (schema 2) | `wwm manifest` | original + patched sha256/size per file, zip sha256/size |
| `manifest.json.sig` | workflow "Sign manifest.json" | **required** from v0.5: a manifest without a valid signature is not read at all |
| `wwm-patch-guard-setup.exe` | `build-app` job (cargo-packager NSIS) | the installer, only rebuilt when `app/` changed or `build_app` is dispatched |
| `wwm-patch-guard-setup.exe.sig` | `build-app` job | installer signature |
| `patch-guard-update.json` (+ `.sig`) | `build-app` job | `{version, name, sha256, size, signature, notes, pub_date}` for the self-update |

The "Remove stale release assets" step deletes everything else; when the app was not rebuilt,
the installer, its `.sig` and `patch-guard-update.json(.sig)` from the previous run are kept.

## Signing (minisign)

- One key pair, made with `cargo packager signer generate` (minisign format, base64-wrapped the
  cargo-packager/Tauri way). Public key: `app/signing/patch-guard.pub`, compiled into the app
  (`include_str!` in `core/update.rs`). The private key never enters the repo.
- CI secrets: **`PATCH_GUARD_SIGN_KEY`** (the private key file's content) and
  **`PATCH_GUARD_SIGN_KEY_PASSWORD`** (optional: GitHub rejects empty secrets, so leave it unset for a key without a password — an unset secret reads as an empty string). Signing runs
  `cargo packager signer sign -k "$KEY" --password "$PW" <file>`; the password is always passed
  explicitly, because minisign prompts for one when it is missing. A missing secret fails the run.
- Signed: `manifest.json` (global and CN), the installer, `patch-guard-update.json`.
  `make_manifest.py` / `wwm manifest` stay unsigned and unchanged, so the parity check is unaffected.
- After the secret is first added, dispatch `release-patch-cn.yml` once so `cn-latest` gets
  its `manifest.json.sig` too: v0.5+ reads `cn-latest` and rejects it otherwise.
- **Rotating the key**: ship an app version with the new `patch-guard.pub` first (signed with the
  old key, so installed copies can still update to it), then switch the secrets. Releases signed
  only with the new key are rejected by every older copy.
- Why it is required, not optional: the manifest's hashes decide what gets written into the game
  folder, and the installer runs on the player's PC. A signature ties both to the owner's key.

## Self-update (v0.5+)

VS Code style: download in the background, install when the user restarts.

1. The update thread (`engine.rs` `Updater`) checks 20 s after start, then every 6 h, when
   **Perbarui Patch Guard otomatis** is on (Settings, default on; `Config::auto_update_app`).
   **Periksa update** in Settings checks right away, even when it is off.
2. `update::check` reads `patch-guard-update.json` + `.sig` from `latest`, verifies the signature,
   compares `version` with `CARGO_PKG_VERSION` (`major.minor.patch`).
3. `Updates::fetch` downloads the installer (by name from the same release, capped at `size + 1`)
   to `<data>\updates\<version>\`, checks size, sha256 and the installer `.sig`, then writes
   `updates\pending.json`. Any failure deletes the folder; nothing unverified is ever run.
4. The window shows an info bar **"Patch Guard vX siap dipasang — Restart sekarang"**, the tray
   menu gets **Restart untuk update ke vX**, and History logs it.
5. Installing (`update::apply`): copy the running exe to
   `%TEMP%\wwm-patch-guard-updater-<pid>.exe`, start it detached with
   `--apply-update <installer> --wait-pid <pid> --pending <updates dir> --version <v> [--relaunch <exe> -- <args>]`,
   then quit. The helper waits for the app to exit, runs the installer with `/S` (per-user NSIS
   install, no UAC), writes `<data>\update-result.json` (`{version, ok, code, at}`, v0.6+),
   deletes `updates\`, and starts Patch Guard again. The copy is needed because
   the installer replaces the exe the app runs from. Leftover helper copies are deleted on the
   next normal start.
6. When it installs:
   - **Restart sekarang** (window) or the tray item → install, then start again.
   - **Keluar** from the tray with an update waiting → install, no restart.
   - Start with an update still waiting (e.g. the PC was shut down) → install first, then start
     with the same arguments (`--minimized` from autostart stays).
   - It first waits (up to 1 min) for a guard pass that may be writing game files.
7. If the installer fails, the helper still clears `updates\` and starts the old version, so it
   cannot loop. Since v0.6 the next start reads `update-result.json` (then deletes it), logs
   "Patch Guard diperbarui ke vX" or "Pembaruan … gagal (kode N)" in History, shows the failure
   in Settings, and the **automatic** check does not download that version again — only
   **Periksa update** retries it. (A helper from v0.5 writes no result file.)

## Robustness (v0.6)

- A failed manifest fetch (offline, or the release mid-replacement) is cached for only 60 s
  (`ERROR_TTL` in `guard.rs`), not the 10 min / 6 h TTL of a good manifest.
- A signature mismatch between `manifest.json` and `.sig` is retried 3× with a 3 s pause
  (`remote.rs` `with_sig_retry`) before it is reported as "rilis mungkin sedang diperbarui".
- A cached patched build (`patched/<sha>`) is re-hashed before it is installed (`Guard::cached`,
  hash cached per size + mtime). A damaged copy is deleted and downloaded again, so it never
  turns the game file into "unknown".

## Language (v0.6)

The UI is **Indonesian by default**, English on request: Settings → **Bahasa / Language**
(`Config::language`, `"id"` / `"en"`; a config without it is Indonesian).

- **All UI text lives in `app/locales/id.json` and `app/locales/en.json`**: flat, namespaced
  keys sorted alphabetically (`common.*`, `home.*`, `settings.*`, `history.*`, `tray.*`,
  `notify.*`, `dialog.*`, `guard.*`, `remote.*`, `update.*`, `engine.*`, `log.*`, `date.month.NN`,
  `lang.name`). Placeholders are named: `"Versi {v} siap dipasang"`. Both files are embedded in
  the exe (`include_str!`), so players cannot break the app by editing them.
- Code names keys only: `t!("home.restart_now")` (`&'static str`) and
  `tf!("common.version", v = version)` (`String`, fills `{v}`). `app/src/i18n.rs` holds the
  tables and macros; a key missing in the current language falls back to Indonesian, then to
  the key itself.
- `app/build.rs` fails the build when the two files have different keys, a key's placeholders
  differ between languages, the code uses a key that does not exist, `t!` is used for a text with
  placeholders, or `tf!` passes other names than the text has. It warns about unused keys.
- **Adding a text:** add the key to both JSON files, use `t!`/`tf!` in the code, build.
- The language is process-wide (an atomic), set from the config in `Engine::start` before any
  thread runs.
- Switching (`AppState::set_language`) saves the config, relabels the tray
  (`Cmd::LanguageChanged` → `Tray::relabel`), refreshes the window and runs a guard pass so the
  status details are phrased again. That pass logs no "new" problems (`problem_lines`).
- History: `LogLine.msg` (`LogMsg`) stores what happened, so those lines follow the language;
  free-form lines (OS/network errors) and history from before v0.6 show their saved `text`.
  `Snapshot.app_update_note` is an `AppNote`, phrased when shown.
- The installer (NSIS, run silently) and the README are not translated.

Players on **v0.4.x or older** must install v0.5 by hand once: they cannot update themselves, and
from v0.5 releases carry signed manifests anyway.

## Build, test, release

```bash
cd app
cargo test -p wwm-patch-guard --lib   # guard rules, signatures, versions, pending/helper args
cargo run                             # debug build with a console
cargo packager --release --formats nsis
```

- On the owner's PC the release exe gets "Access is denied" (Kaspersky), so test with the debug
  build; the installer comes from CI.
- Local end-to-end test without a real release: generate a test key
  (`cargo packager signer generate --ci --path <tmp>\test.key`), serve
  `<srv>/latest/{patch-guard-update.json(.sig), wwm-patch-guard-setup.exe(.sig), manifest.json(.sig)}`
  with `python -m http.server`, and start the debug build with
  `WWM_PATCH_GUARD_BASE_URL=http://127.0.0.1:<port>`, `WWM_PATCH_GUARD_DATA_DIR=<tmp>\data` and
  `WWM_PATCH_GUARD_PUBKEY=<content of test.key.pub>` (the key override only exists in debug builds).
  Checked on 2026-11-23: download and verification → `pending.json`; start with a pending update →
  helper → relaunch; a tampered installer or a JSON signed by another key is rejected and logged.
- Releasing a new version: bump `version` in `app/Cargo.toml`, push to `main` →
  `release-patch.yml` rebuilds the installer, signs it, and writes `patch-guard-update.json`.
  Check that the `latest` release has all assets from the table above. Installed copies pick
  it up within 6 h.

## Version history

| Version | Change |
| --- | --- |
| 0.3.x | Tauri 2 + Leptos; release zip + manifest schema 2 |
| 0.4.0 | Native GPUI port (no WebView2), WinUI-style UI |
| 0.5.0 | Self-update (VS Code style) and minisign signatures; unsigned manifests are rejected |
| 0.6.0 | Indonesian/English UI; short error TTL, signature retry, cache re-verification; failed self-updates are reported |
