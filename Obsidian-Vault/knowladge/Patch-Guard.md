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
| `manifest.json.sig` | `sign` job | **required** from v0.5: a manifest without a valid signature is not read at all |
| `wwm-patch-guard-setup.exe` | `build-app` job (cargo-packager NSIS) | the installer, only rebuilt when `app/` changed or `build_app` is dispatched |
| `wwm-patch-guard-setup.exe.sig` | `sign` job | installer signature |
| `patch-guard-update.json` (+ `.sig`) | `sign` job | `{version, name, sha256, size, signature, notes, pub_date}` for the self-update |

Jobs (since v0.6.1): `changes` → `build-app` (Windows, optional) and `build-patch` (Ubuntu) build
**without any secret** and upload unsigned artifacts → `sign` (environment `release`, no checkout,
no cache, pinned `cargo-packager@0.11.8`) signs them → `publish` (`contents: write`) uploads.

The "Remove stale release assets" step deletes everything else; when the app was not rebuilt,
the installer, its `.sig` and `patch-guard-update.json(.sig)` from the previous run are kept.

## Signing (minisign)

- Key pairs made with `cargo packager signer generate` (minisign format, base64-wrapped the
  cargo-packager/Tauri way). Since v0.6.1 the app trusts a **list** (`KEYS` in `core/update.rs`,
  `include_str!`): `app/signing/patch-guard.pub` (**active**, CI signs with it) and
  `app/signing/patch-guard-backup.pub` (**backup**, generated 2026-11-24; its private key is only
  on the owner's PC in `~\.wwm-patch-guard\patch-guard-backup.key` and must be copied to offline
  storage — it is never put in GitHub until it is needed). Private keys never enter the repo.
- CI secrets: **`PATCH_GUARD_SIGN_KEY`** (the private key file's content) and
  **`PATCH_GUARD_SIGN_KEY_PASSWORD`** (optional: GitHub rejects empty secrets, so leave it unset for a
  key without a password). Keep them as secrets of the **environment `release`** (Settings →
  Environments → `release`: deployment branches = `main` only; a required reviewer is possible but
  would hold every translation release until approved), not as repository secrets. Only the
  `sign` job (and the CN workflow's single job) use that environment. They reach cargo-packager as
  `CARGO_PACKAGER_SIGN_PRIVATE_KEY` / `CARGO_PACKAGER_SIGN_PRIVATE_KEY_PASSWORD` environment
  variables, never on the command line. A missing secret fails the run.
- Signed: `manifest.json` (global and CN), the installer, `patch-guard-update.json`.
  `make_manifest.py` / `wwm manifest` stay unsigned and unchanged, so the parity check is unaffected.
- **What a signature vouches for** (v0.6.1+): besides the bytes, the minisign *trusted comment*,
  which the global signature covers. cargo-packager writes `timestamp:<unix secs>\tfile:<name>`;
  `update::verify(data, sig, file)` requires `file` to be the asset it was fetched as
  (`manifest.json`, `patch-guard-update.json`, the installer's `name`), so one signed asset cannot
  be passed off as another, and returns the timestamp. **Anti-rollback**: the guard keeps the
  newest manifest timestamp per profile in `state.json` (`manifest_signed_at`) and refuses an
  older one ("manifest.json … lebih lama"), so an old signed release served again cannot roll the
  translation back; the self-update was already monotonic (only a newer `version` installs).
  There is deliberately **no expiry date**: `cn-latest` is frozen and app releases can be months
  apart, so an expiring signature would break working installs; the player's clock is not used
  either (a wrong clock must not block the guard).
- `cn-latest` has **no `manifest.json.sig`** (last built 2026-10-21, before signing): v0.5+ cannot
  read it. Dispatch `release-patch-cn.yml` once if CN guarding should work again.
- **Rotation (planned)**: (1) every shipped build already trusts the backup key; (2) generate a new
  backup pair, put its `.pub` in `app/signing/` as the new backup and the old backup as active,
  drop the old active `.pub`; (3) replace the `release` environment's `PATCH_GUARD_SIGN_KEY` with
  the old backup's private key; (4) push `app/` (version bump) — that release is signed with a key
  every copy trusts, and the build it installs no longer trusts the retired key.
- **Revocation (key leaked)**: same steps, immediately. Removing a key from `KEYS` is the
  revocation; copies that have not updated yet still trust it until they install that release,
  so also delete the leaked secret and re-run the release at once. If *both* keys are lost, every
  installed copy must be reinstalled by hand from a build with new keys.
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
   to `<data>\updates\<version>\` and stores the signed `patch-guard-update.json` + `.sig` next
   to it, exactly as served. There is no `pending.json` any more (v0.6.1+): every reader
   (`Updates::pending`, the helper) re-verifies the folder each time — JSON signature, folder
   name = signed `version`, version newer than the running build, installer size, sha256 and
   installer signature, the installer read through a handle that denies writes and deletes.
   Any failure deletes `updates\`; nothing unverified is ever run. `version` and `name` must be
   plain file names (`[A-Za-z0-9.+_-]`, no `..`, the name ending in `.exe`).
4. The window shows an info bar **"Patch Guard vX siap dipasang — Restart sekarang"**, the tray
   menu gets **Restart untuk update ke vX**, and History logs it.
5. Installing (`update::apply`): copy the running exe to a fresh folder
   `<data>\updater\<pid>-<nanos>\wwm-patch-guard-updater.exe` (not `%TEMP%`: an exe there would
   load any DLL dropped next to it), start it detached with
   `--apply-update --wait-pid <pid> [--relaunch [--minimized]]`, then quit. **The helper takes no
   paths** (v0.6.1+; any other argument → exit 2, nothing done): it finds the data folder itself
   (`system::data_dir`), waits for the app to exit, verifies the pending update again (step 3)
   and keeps the installer locked while it runs it with `/S` (per-user NSIS install, no UAC),
   writes `<data>\update-result.json` (`{version, ok, code, at}`, v0.6+), deletes `updates\`,
   and — with `--relaunch` — starts **the installed exe only**: `InstallLocation` of
   `HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\WWM Patch Guard` +
   `wwm-patch-guard.exe` (`system::installed_exe`; a dev build that was never installed is not
   relaunched). So starting the exe with `--apply-update` by hand can only install a signed,
   newer Patch Guard. The copy is needed because the installer replaces the exe the app runs
   from. Leftover helper folders (and v0.6.0's `%TEMP%` copies) are deleted on the next normal
   start. `main` also calls `SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_DEFAULT_DIRS)` first.
6. When it installs:
   - **Restart sekarang** (window) or the tray item → install, then start again. Always.
   - Only with **Pasang update Patch Guard otomatis** on (Settings; `Config::auto_install_app`,
     v0.6.1+, **default off**, a config without the field = off):
     - **Keluar** from the tray with an update waiting → install, no restart.
     - Start with an update still waiting (e.g. the PC was shut down) → install first, then start
       the installed copy the same way (`--minimized` from autostart stays).
   - With it off, a downloaded update just waits for "Restart sekarang" (the info bar says so).
     Downloading in the background stays on by default (`auto_update_app`).
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
  `WWM_PATCH_GUARD_PUBKEY=<content of test.key.pub>`. Since v0.6.1 **all three overrides exist
  only in debug builds** (release builds ignore them, always use GitHub and refuse plain http,
  redirects included — `https_only`); the helper relaunches only an installed copy.
  Manifest hashes must be 64 lowercase hex digits and the zip name a plain name, and
  `state.json` entries with other hashes are dropped on load (they become file names).
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
| 0.6.1 (in progress) | Update helper takes no paths and re-verifies the signed update before installing; signatures bound to their file name, manifest anti-rollback, active + backup key; installing on quit/start is opt-in (`auto_install_app`); CI signs in a separate `sign` job (environment `release`) |
