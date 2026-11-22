# Plan: WWM Patch Guard — Tauri → GPUI

Status: **diimplementasikan di branch `gpui-port`** (dibuat dan dikerjakan 2026-11-22). Target rilis: Patch Guard **v0.4.0**.
Lihat bagian "Hasil implementasi" di akhir dokumen untuk apa yang sudah diuji dan apa yang belum.

## Tujuan

- Lepas dari WebView2: tidak ada HTML/CSS/WASM, Trunk, Tailwind, atau `tauri-cli`. Aplikasi menjadi
  satu binary Rust native yang menggambar UI-nya sendiri lewat [GPUI](https://gpui.rs) (DirectX 11
  di Windows).
- Fitur dan tampilan **tetap sama** dengan v0.3.0: navigasi samping ala WinUI `NavigationView`,
  halaman Home / Settings / History, backdrop Mica, warna aksen Windows, dan mode terang/gelap.
- Pengguna yang sudah memasang v0.3.0 (Tauri) bisa upgrade di tempat tanpa kehilangan config,
  riwayat, atau entri autostart.

### Di luar cakupan

- Mengubah aturan guard (`guard.rs`), engine, lokasi folder, atau format `manifest.json`.
- Dukungan macOS/Linux (Windows-only, sama seperti sekarang).
- Redesign tampilan.

## Kondisi sekarang (yang harus dipertahankan)

| Bagian | File | Tergantung Tauri? |
|---|---|---|
| Aturan guard (unit-tested) | `src-tauri/src/guard.rs` | tidak |
| Watcher/poll loop, thread sendiri, callback `on_change` | `src-tauri/src/engine.rs` | tidak |
| Deteksi Steam/launcher | `src-tauri/src/locations.rs` | tidak |
| Download release zip | `src-tauri/src/remote.rs` | tidak |
| Config + log di data dir | `src-tauri/src/store.rs`, `model.rs` | tidak |
| Shell: tray, window, 12 command, notifikasi, autostart, single-instance, Mica | `src-tauri/src/lib.rs` (303 baris) | **ya**, satu-satunya |
| UI Leptos (WASM) + komponen Rust/UI + Tailwind Fluent theme | `src/` (~590 baris + `components/ui`), `style/tailwind.css` | ya (via `window.__TAURI__`) |

Jadi yang ditulis ulang hanya `lib.rs` (shell) dan `src/` (UI). Sekitar 1.700 baris backend
dipindahkan apa adanya.

Command yang sekarang dipanggil UI dan harus punya padanan langsung (pemanggilan fungsi Rust
biasa, tanpa IPC): `get_status`, `add_folder`, `detect_installs`, `add_detected`, `clear_history`,
`remove_folder`, `check_now`, `set_paused`, `set_auto_update`, `set_autostart`,
`restore_originals`, `force_install`, plus event `status`.

## Keputusan

### 1. "Mirip native WinUI 3" → GPUI polos + modul `fluent` buatan sendiri

Kontrol WinUI 3 yang **benar-benar native** hanya bisa didapat lewat XAML / Windows App SDK, dan
itu bukan GPUI. Di GPUI semua kontrol digambar sendiri, jadi hasilnya **meniru** Fluent, bukan
kontrol sistem. Dengan pendekatan di bawah, hasilnya bisa dibuat sangat mirip, setara dengan
tampilan versi Tailwind sekarang yang juga meniru Fluent:

- **Backdrop**: `WindowBackgroundAppearance::MicaBackdrop` (tersedia di `gpui-pre` 0.3.x). Kalau
  OS tidak mendukung (Windows 10), pakai `Opaque` + warna latar `#f3f3f3` / `#202020`, sama seperti
  kelas `html.mica` sekarang.
- **Font**: `Segoe UI Variable` (fallback `Segoe UI`). Ikon dari font sistem `Segoe Fluent Icons`
  (fallback `Segoe MDL2 Assets`) memakai codepoint glyph, jadi tidak perlu aset SVG.
- **Warna aksen**: dibaca dari Windows (`UISettings::GetColorValue(Accent*)` lewat crate
  `windows`, atau `HKCU\Software\Microsoft\Windows\DWM\AccentColor`). Diperbarui saat
  `WM_SETTINGCHANGE` / `observe_window_appearance`.
- **Terang/gelap**: `window.appearance()` + `observe_window_appearance` dari GPUI.
- **Token**: palet di `style/tailwind.css` dipindahkan 1:1 ke `struct Theme { light, dark, mica }`.

Komponen yang dibutuhkan (sedikit, karena hanya mengikuti yang dipakai sekarang):

| Fluent | Pengganti dari | Catatan |
|---|---|---|
| `NavigationView` (pane kiri + item terpilih dengan indikator aksen) | `app.rs` | 3 item + footer versi |
| `Button` (Accent / Standard / Subtle) | `components/ui/button.rs` | hover/pressed/disabled, fokus keyboard |
| `ToggleSwitch` | `switch.rs` | animasi knob via `with_animation` |
| `InfoBar` (Info/Success/Warning/Error) | `alert.rs` | |
| `InfoBadge` / status badge | `badge.rs` | |
| `SettingsCard` / card | `card.rs` | latar transparan saat Mica aktif |
| `ContentDialog` | `window.confirm()` | konfirmasi restore / force install / hapus |
| `ProgressRing` (opsional) | – | saat "Periksa sekarang" berjalan |

**Kenapa bukan `gpui-component`?** Gayanya shadcn, bukan Fluent, jadi ujung-ujungnya hampir semua
harus di-restyle. Selain itu ia mem-pin `gpui-pre` di versi yang ditentukannya sendiri
(`=0.3.12` di v0.10) dan menarik banyak dependensi. Untuk ±8 komponen, modul sendiri lebih kecil
dan bisa dikontrol penuh. Source-nya tetap berguna sebagai referensi pola (focus handle, animasi).

### 2. Versi GPUI: `gpui-pre`, di-pin exact

- `gpui` 0.2.2 di crates.io sudah lama dan belum punya `MicaBackdrop` (hanya `Blurred`).
- `gpui-pre` (snapshot rutin repo Zed, sekarang 0.3.13) punya `MicaBackdrop`/`MicaAltBackdrop`
  dan backend Windows yang lebih baru.
- Pin exact (`gpui = { package = "gpui-pre", version = "=0.3.13" }`) karena API GPUI masih
  pra-1.0 dan sering berubah. Upgrade hanya dilakukan dengan sengaja.

### 3. Installer: `cargo-packager` (NSIS)

Nama file rilis tetap `wwm-patch-guard-setup.exe`. Identifier `com.oratakashi.wwm`, product name
`WWM Patch Guard`, dan mode install per-user disamakan dengan konfigurasi Tauri supaya installer
baru menimpa instalasi lama. Template NSIS cargo-packager diturunkan dari template Tauri, jadi
lokasi install dan uninstall key-nya kemungkinan besar sama. **Ini wajib diverifikasi di fase 0.**
Bootstrapper WebView2 tidak lagi dibutuhkan.

### 4. Pengganti tiap plugin Tauri

| Tauri | Pengganti | Kompatibilitas yang harus dijaga |
|---|---|---|
| `WebviewWindowBuilder` + Mica effect | `cx.open_window(WindowOptions { window_background: MicaBackdrop, window_min_size, .. })` | ukuran 900×660, min 720×520, judul sama |
| `tray-icon` feature | crate `tray-icon` + `muda` (dipakai Tauri di balik layar) | menu: Buka / Periksa sekarang / Keluar, klik kiri membuka window |
| `RunEvent::ExitRequested` + `prevent_exit` | tetap hidup setelah window terakhir ditutup (lihat risiko R2) | menutup window ≠ keluar |
| `tauri-plugin-single-instance` | named mutex `Local\WWMPatchGuard` + named event "show". Instance kedua men-set event lalu keluar; thread di instance pertama menunggu event itu dan membuka window | – |
| `tauri-plugin-autostart` | crate `auto-launch` (backend plugin tsb.), argumen `--minimized` | **nama entri `HKCU\…\Run` harus sama** dengan yang dibuat v0.3.0, kalau tidak bisa muncul dua entri |
| `tauri-plugin-dialog` (pilih folder) | `cx.prompt_for_paths(PathPromptOptions { directories: true, .. })` (IFileOpenDialog) | judul dialog |
| `tauri-plugin-notification` | crate `tauri-winrt-notification` (tanpa Tauri) | AUMID dari shortcut installer, teks notifikasi sama |
| `app.path().app_local_data_dir()` | `dirs::data_local_dir()/com.oratakashi.wwm` | **path harus identik** supaya config/riwayat lama terbaca. `WWM_PATCH_GUARD_DATA_DIR` tetap didukung |
| `emit("status")` + `listen` | callback engine → `async_channel` → task foreground GPUI → update `Entity<AppState>` + `cx.notify()` | – |
| icon exe + manifest | `build.rs` dengan `winresource` (icon.ico). Fitur `windows-manifest` GPUI dipertahankan atau digabung, jangan dobel | DPI awareness, Common Controls v6 |
| `windows_subsystem` | `#![windows_subsystem = "windows"]` di `main.rs` | tidak ada jendela konsol |

## Struktur target

```
app/
  Cargo.toml          # satu package `wwm-patch-guard` (lib + bin); profile release dipertahankan
  build.rs            # winresource: icon + version info
  icons/              # dipindahkan dari src-tauri/icons (hanya .ico/.png yang dipakai)
  src/
    main.rs           # entry: single-instance, --minimized, Application::new().run(...)
    lib.rs            # pub mod core::*, re-export untuk test
    core/             # git mv dari src-tauri/src, isi TIDAK diubah
      engine.rs guard.rs locations.rs model.rs remote.rs store.rs
    shell/
      tray.rs  instance.rs  autostart.rs  notify.rs  backdrop.rs (accent + Mica)
    state.rs          # AppState (snapshot, found, page, error) + aksi = pengganti 12 command
    fluent/           # theme.rs, icons.rs, button.rs, toggle.rs, info_bar.rs, badge.rs,
                      # card.rs, dialog.rs, nav_view.rs
    pages/            # home.rs, settings.rs, history.rs (port 1:1 dari Leptos)
```

Yang dihapus: `src-tauri/`, `index.html`, `Trunk.toml`, `.taurignore`, `style/`,
`src/api.rs`, `src/components/ui/` (Rust/UI), `capabilities/`, `tauri.conf.json`. Versi aplikasi
cukup dibaca dari `Cargo.toml` (`env!("CARGO_PKG_VERSION")`).

`cargo test -p wwm-patch-guard --lib` tetap menjadi perintah test (nama package tidak berubah).

## Fase

### Fase 0 — Spike go/no-go (branch terpisah, kode dibuang setelahnya)

Satu binary kecil di Windows 11 yang membuktikan hal-hal berikut. Kalau ada yang gagal, plan
direvisi dulu sebelum lanjut.

1. Window `gpui-pre =0.3.13` dengan `MicaBackdrop` + teks Segoe UI Variable + satu glyph Segoe
   Fluent Icons. Juga dicek di Windows 10 / VM tanpa Mica (fallback opaque).
2. `tray-icon` jalan bersamaan dengan event loop GPUI di thread utama: klik kiri dan menu
   terkirim ke task GPUI.
3. Window ditutup → proses tetap hidup. Tray "Buka" membuat window baru.
4. `prompt_for_paths` (folder) dan notifikasi `tauri-winrt-notification` muncul.
5. Ukuran binary release dan RAM idle (dengan dan tanpa window) dicatat dan dibandingkan dengan
   v0.3.0 (WebView2).
6. `cargo packager` NSIS: install di atas v0.3.0 yang sudah terpasang → hanya ada satu entri di
   Apps & Features, satu shortcut, dan entri autostart yang sama.
7. Build release dicoba di mesin lokal (lihat catatan Kaspersky "Access is denied" pada build
   release) dan di CI.

### Fase 1 — Restrukturisasi crate

- `git mv src-tauri/src/{engine,guard,locations,model,remote,store}.rs app/src/core/`, ganti nama
  modul, buang dependensi `tauri*` dari `Cargo.toml`.
- `cargo test -p wwm-patch-guard --lib` hijau tanpa ada perubahan logika.
- Pada titik ini UI Leptos lama sudah tidak bisa di-build. Kerjakan di branch `gpui-port` sampai
  fase 5 selesai, jangan merge ke `main` di tengah jalan.

### Fase 2 — Shell tanpa UI

- `main.rs`: single-instance, parsing `--minimized`, data dir, `Engine::start`, tray, notifikasi,
  autostart.
- `state.rs`: `AppState` sebagai `Entity`, metode untuk 12 aksi di atas. Aksi yang memblokir
  (`detect_installs`, dialog folder) dijalankan di `cx.background_spawn`.
- Acceptance: tray berfungsi penuh tanpa window (Periksa sekarang → engine jalan → notifikasi),
  instance kedua hanya memunculkan window kosong.

### Fase 3 — Design system `fluent/`

- `Theme` dari token Tailwind (light/dark/mica), aksen sistem, font, glyph ikon.
- Komponen di tabel keputusan 1, masing-masing dengan state hover/pressed/disabled/focus dan
  navigasi keyboard (Tab/Enter/Space).
- Halaman "gallery" sementara (fitur debug, `--gallery`) untuk membandingkan langsung dengan
  screenshot v0.3.0.

### Fase 4 — Port halaman

- `NavigationView` + `home.rs` (243 baris Leptos), `settings.rs` (77), `history.rs` (87). Port
  1:1: teks Indonesia, urutan, dan kondisi tampil dibiarkan sama.
- `confirm()` diganti `ContentDialog`.
- Acceptance: screenshot berdampingan v0.3.0 vs GPUI untuk tiap halaman, light + dark, Mica on/off.

### Fase 5 — Packaging & CI

- Konfigurasi `[package.metadata.packager]` di `app/Cargo.toml` (NSIS, per-user, icon,
  identifier). Kalau ada kebutuhan Tauri NSIS yang tidak didukung (misalnya menutup proses yang
  sedang jalan sebelum install), tambahkan hook NSIS-nya.
- `app-ci.yml`: hapus `trunk`, `tauri-cli`, target `wasm32-unknown-unknown`. Tambahkan
  `cargo-packager`, lalu `cargo test` + `cargo packager --release`.
- `release-patch.yml`: ganti `cargo tauri build` → `cargo packager --release --formats nsis`, path
  output `target/release/bundle/nsis/*.exe` → folder output packager, dan sumber versi
  `jq … tauri.conf.json` → versi dari `cargo metadata` / `app/Cargo.toml`. Nama aset tetap
  `wwm-patch-guard-setup.exe`.

### Fase 6 — QA, dokumentasi, rilis

Checklist manual di Windows 11 dan Windows 10:

- [ ] Upgrade dari v0.3.0: config, folder yang dijaga, riwayat, status pause/auto-update, dan
      autostart tetap ada.
- [ ] Mulai via autostart (`--minimized`) → hanya tray, tanpa window.
- [ ] Deteksi Steam + launcher, tambah folder manual, hapus folder.
- [ ] Patcher NetEase mengembalikan file → dipasang lagi otomatis + notifikasi.
- [ ] Force install (dengan dialog), restore originals (guard ter-pause), clear history.
- [ ] Tutup window → tray tetap ada. "Keluar" → proses berhenti. Instance kedua → window lama
      muncul.
- [ ] Ganti tema/aksen Windows saat app terbuka → UI ikut berubah.
- [ ] Uninstall bersih.
- [ ] `WWM_PATCH_GUARD_BASE_URL` / `WWM_PATCH_GUARD_DATA_DIR` masih berfungsi.

Dokumentasi: bagian `app/` di `CLAUDE.md` (Tauri 2 + Leptos + Trunk → GPUI, perintah build), README
(persyaratan WebView2 dihapus), `docs/Rust-Toolchain.md` kalau menyebut app. Rilis sebagai v0.4.0.

## Risiko

| # | Risiko | Mitigasi |
|---|---|---|
| R1 | API GPUI pra-1.0 berubah antarversi, dan `gpui-pre` hanya snapshot | pin exact, upgrade sengaja, kode GPUI terkumpul di `fluent/` + `pages/` |
| R2 | GPUI di Windows mungkin keluar saat window terakhir ditutup | spike #3. Kalau benar, pakai window tersembunyi atau hook quit. Kalau mentok, fallback: sembunyikan window alih-alih menutupnya (tapi RAM idle lebih besar) |
| R3 | `tray-icon` butuh pump pesan Win32 di thread yang sama | GPUI memakai `GetMessage` loop di thread utama, jadi tray dibuat di sana (spike #2) |
| R4 | Aksesibilitas: GPUI belum menyediakan UI Automation (screen reader) seperti WebView2 | regresi yang diterima secara sadar. Dicatat di README. Navigasi keyboard tetap didukung |
| R5 | Butuh GPU DirectX 11 (VM/driver lama) | uji di VM tanpa GPU (WARP). WebView2 juga punya persyaratan serupa |
| R6 | Antivirus (Kaspersky) memblokir exe release baru | uji lokal pakai build debug dan installer dari CI (sama seperti sekarang) |
| R7 | Installer baru tidak menimpa instalasi Tauri lama → dua entri | spike #6. Kalau beda, tambahkan langkah uninstall v0.3.0 di NSIS |
| R8 | Kualitas teks/ikon kurang "native" | Segoe UI Variable + Segoe Fluent Icons dari sistem, lalu dibandingkan screenshot di fase 4 |

## Keputusan owner (2026-11-22)

- **Windows 10 tetap didukung**: tanpa Mica, latar opaque dengan palet yang sama.
- **Tauri di-drop seluruhnya**: tidak ada masa transisi, `latest` langsung membawa versi GPUI.
- **Versi naik ke 0.4.x** (mulai 0.4.0).

## Hasil implementasi (2026-11-22)

- Struktur sesuai plan: `app/src/{core,shell,fluent,pages}`, `state.rs`, `app.rs`. Modul `core/`
  dipindah tanpa perubahan logika, dan 29 unit test lulus.
- **R2 benar terjadi.** Secara default, GPUI di Windows memakai `QuitMode::LastWindowClosed`.
  `app.rs` sekarang memanggil `cx.set_quit_mode(QuitMode::Explicit)`. Sudah dites: window
  ditutup → proses tetap jalan, launch kedua → window muncul lagi.
- **R4 lebih ringan dari perkiraan.** `gpui-pre` 0.3.13 sudah punya AccessKit (`role`,
  `aria_label`, `aria_toggled`). Kontrol di `fluent/` dan navigasi sudah diberi role.
- Notifikasi memakai `cx.show_system_notification`, dan AppUserModelID di-set lewat
  `cx.set_app_identity`. Dialog folder memakai `cx.prompt_for_paths`. Jadi tidak perlu crate
  terpisah untuk keduanya.
- Sudah diuji di Windows 11 (debug build): ketiga halaman, Mica, warna aksen, dialog konfirmasi,
  single instance, start `--minimized`, tutup ke tray, dan buka lagi. RAM sekitar 35 MB tanpa
  window dan 54 MB dengan window.
- **Belum diuji:**
  - klik menu tray, tombol Esc pada dialog, dan notifikasi toast;
  - tampilan di Windows 10;
  - upgrade installer di atas v0.3.0.
- Belum ada scrollbar yang terlihat, dan teks path belum bisa diseleksi. Keduanya regresi kecil
  dibanding versi WebView.
