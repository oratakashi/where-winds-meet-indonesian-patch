<div align="center">

# Where Winds Meet — Indonesian Patch

**A community Indonesian translation for *Where Winds Meet*, plus the open-source toolkit that builds it.**

[![Latest patch](https://img.shields.io/github/release-date/oratakashi/where-winds-meet-indonesian-patch?label=latest%20patch)](../../releases/tag/latest)
[![Downloads](https://img.shields.io/github/downloads/oratakashi/where-winds-meet-indonesian-patch/total?label=downloads)](../../releases)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Python 3.8+](https://img.shields.io/badge/python-3.8%2B-blue.svg)](https://www.python.org/)

[**⬇ Download — Global client**](../../releases/tag/latest) &nbsp;·&nbsp;
[⬇ China client (燕云十六声) — **no longer maintained**](../../releases/tag/cn-latest) &nbsp;·&nbsp;
[Installation guide](#player-installation-guide)

</div>

> [!NOTE]
> This is an unofficial fan project, not affiliated with NetEase or Everstone Studio. It only
> changes in-game text — no gameplay changes. On the global client the game language must stay on
> **English** for the patch to show.

> [!WARNING]
> **The China client (燕云十六声) patch is no longer maintained.** The last build stays online
> as-is, but it will not receive new translations or updates for new CN game versions. Reasons:
> - **Small Indonesian player base** on the China client.
> - **Limited translation scope** — story and cutscene subtitles can't be translated (they come
>   from integrity-checked data packs).
> - **CN updates ship too fast** to keep up with the translation phases.
>
> For a complete Indonesian experience, play the **global client**, which stays fully supported.

## Contents

- [Translation Progress](#translation-progress)
- [Player Installation Guide](#player-installation-guide)
- [Auto-reinstall app (Windows)](#auto-reinstall--wwm-patch-guard-windows)
- [Updating & Uninstalling](#updating--uninstalling)
- [FAQ](#faq)
- [Support the Project](#support-the-project)
- [For Contributors & Developers](#for-contributors--developers)
- [Disclaimer](#disclaimer) · [License](#license)

---

## Translation Progress

Translation is ongoing, so some text may still appear in English (or Chinese on the China client, which is no longer maintained).
New releases are built automatically whenever translations are added.

- **Global client:** [`Current-Status.md`](Obsidian-Vault/progress/Current-Status.md)
- **China client (no longer maintained, frozen):** [`CN-Current-Status.md`](Obsidian-Vault/cn/progress/CN-Current-Status.md)

---

## Player Installation Guide

No Python or tools needed — just copy two files into the game folder.

1. Download the patch for your client:
   - Global: **[`wwm-indonesian-patch.zip`](../../releases/tag/latest)** — current target: game v2.4 [Across Autumn Waters]
   - China: **[`wwm-cn-indonesian-patch.zip`](../../releases/tag/cn-latest)** — ⚠️ no longer maintained (last build only)
2. Follow the steps for your platform below.

> [!IMPORTANT]
> **Before you start**
> - **Back up the original files** to a folder **outside** the game folder. You'll need them to
>   uninstall, and the Android patcher deletes extra files it finds in the game folder.
> - **Reinstall the patch after every game update**, even small hotfixes — updates put the
>   original files back.

### Which platform works best?

| Platform | Locale folder | Full patch (`_diff` text) | After a game update |
|---|---|---|---|
| [PC — Official launcher](#pc--official-wwm-launcher) | `<InstallDir>\<variant>\LocalData\Patch\HD\oversea\locale\` | ✅ Yes | Reinstall |
| [Android](#android) | `Android/data/com.netease.yysls/files/LocalData/Patch/HD/oversea/locale/` | ✅ Yes | Reinstall |
| [PC — Steam](#pc--steam) | `<SteamLibrary>\steamapps\common\Where Winds Meet\Package\HD\oversea\locale\` | ⚠️ Partial — `_diff` text stays English | Usually survives; reinstall after a Steam update |
| [PC — China client](#pc--china-client-燕云十六声) ⚠️ unmaintained | `<InstallDir>\yysls_fast\LocalData\Patch\HD\v2\locale\` | ✅ Yes (except cutscene subtitles) | Reinstall |

### Auto-reinstall — WWM Patch Guard (Windows)

Game updates and in-game hot-patches put the original files back, sometimes without the launcher
showing any update. **WWM Patch Guard** is a small tray app that watches the game's locale folder
and puts the patch back by itself. It is built for the **official launcher (global)** and also
guards a **Steam** install — there only the `Package\HD\oversea\locale` folder, since Steam's
in-game patcher re-downloads the `LocalData\Patch` overlay on every launch. It can
also guard a China client folder, but only while the frozen `cn-latest` release stays unchanged
and the original files still match it (that release has no `manifest.json`, so in practice it
will report "waiting for a release" there).

1. Download the `WWM Patch Guard` installer from the [releases page](../../releases) and run it.
2. Installed copies of the game (Steam and the official launcher) are found automatically and
   listed under **Game terdeteksi** on the Home page: click **Jaga** (or **Jaga semua**) and the
   patch is downloaded and installed. Only if nothing is found, click **Pilih folder manual** and
   pick your game (or launcher) install folder.
3. Leave it in the tray (turn on **Jalankan saat Windows menyala**). When a file is restored while
   the game is running, restart the game to see the Indonesian text again.

How it decides what to do:

- File is the original the patch was built from → the patch is reinstalled (from a local cache,
  no download).
- File is a **new** original from a real game update → the app waits for a patch release built
  for that exact file, then installs it. It never installs an outdated patch over a newer game.
- **Kembalikan file asli** in the app uninstalls the patch (backups are kept outside the game
  folder, in `%LOCALAPPDATA%\com.oratakashi.wwm\`).

It only copies files — it does not touch the game process, memory or network traffic. It is not
available for Steam (see the note there) or Android.

### PC — Official WWM Launcher

1. Open the launcher's install folder (right-click the launcher shortcut → **Open file location**).
   The game lives in a subfolder named after the variant you downloaded, e.g. `wwm_lite`.
2. Open `<InstallDir>\<variant>\LocalData\Patch\HD\oversea\locale\`.
3. Back up the `translate_words_map_en*` files, then extract the ZIP here and overwrite them.
4. Do the same in `<InstallDir>\<variant>\Package\HD\oversea\locale\`.
5. Start the game and set **Settings → Language → Game Language = English**.

### Android

The locale folder is
`Internal Storage/Android/data/com.netease.yysls/files/LocalData/Patch/HD/oversea/locale/`
(the package name may vary by region — look for the folder with `translate_words_map_en*` files).
**No root needed**, but Android 11+ hides `Android/data` from most file managers.

> [!TIP]
> Try the easy way first: connect the phone to a PC in **File Transfer (MTP)** mode. On many devices
> `Android/data` is visible and writable from Windows Explorer — if so, just back up and copy.

<details>
<summary><b>Method A — File manager on the phone (no PC needed)</b></summary>

<br>

[**Files** by Marc apps & software](https://play.google.com/store/apps/details?id=com.marc.files)
(free) can open `Android/data` even on Android 13+.

1. Install **Files** and grant the storage permission it asks for.
2. Download the patch ZIP on the phone and extract it (e.g. to `Download/wwm-indonesian-patch/`).
3. In the app, go to **Internal Storage → Android → data** → the game's locale folder above. Allow
   folder access if prompted.
4. Back up the existing `translate_words_map_en*` files to another folder, e.g. `Download/wwm-backup/`.
5. Copy the extracted files into the locale folder, overwriting the originals.
6. Force-close the game, relaunch it and set **Game Language = English**.

Some OEM skins (MIUI/HyperOS, One UI, ColorOS…) are stricter — if this doesn't work, use Method B.

</details>

<details>
<summary><b>Method B — ADB from a Windows PC (step by step)</b></summary>

<br>

**One-time setup on the PC**

1. Download **SDK Platform-Tools for Windows**:
   <https://developer.android.com/tools/releases/platform-tools>
2. Extract it somewhere easy, e.g. `C:\platform-tools\` (it contains `adb.exe`).
3. Extract the patch ZIP **into that same folder**, so the `translate_words_map_en*` files sit next
   to `adb.exe`.

**One-time setup on the phone**

4. **Settings → About phone** → tap **Build number** 7 times until it says *"You are now a
   developer"* (Xiaomi: **MIUI/OS version**; Samsung: under **Software information**).
5. **Settings → System → Developer options** → turn on **USB debugging**.

**Copying the files**

6. Connect the phone with a USB **data** cable.
7. Open `C:\platform-tools\` in Explorer, type `cmd` in the address bar and press **Enter**.
8. Run:
   ```bat
   adb devices
   ```
   Tap **Allow** on the *"Allow USB debugging?"* prompt (tick *Always allow*). Run it again — you
   should see a line ending in `device`. `unauthorized` → check the phone; nothing listed → try
   another cable/port or set USB mode to *File Transfer*.
9. Back up the originals to the PC:
   ```bat
   mkdir backup
   adb pull /sdcard/Android/data/com.netease.yysls/files/LocalData/Patch/HD/oversea/locale/ backup
   ```
10. Push the patched files (one line at a time):
    ```bat
    adb push translate_words_map_en /sdcard/Android/data/com.netease.yysls/files/LocalData/Patch/HD/oversea/locale/
    adb push translate_words_map_en_diff /sdcard/Android/data/com.netease.yysls/files/LocalData/Patch/HD/oversea/locale/
    ```
    Each should report `1 file pushed`. On *"No such file or directory"*, find your package folder
    with `adb shell ls /sdcard/Android/data/ | findstr netease` and adjust the path.
11. Force-close the game, relaunch it and set **Game Language = English**.
12. (Optional) Turn **USB debugging** off again.

To restore the originals, repeat step 10 with the files from `backup\locale\`.

</details>

### PC — Steam

1. In Steam, right-click **Where Winds Meet** → **Manage → Browse Local Files**.
2. Open `Package\HD\oversea\locale\`.
3. Back up `translate_words_map_en` and `translate_words_map_en_diff`, then extract the ZIP here
   and overwrite them.
4. Launch the game and set **Settings → Language → Game Language = English**.

> [!NOTE]
> On Steam, NetEase's patcher restores the `_diff` overlay file on every launch, so text from that
> layer (mostly recently changed or added lines) stays in English. The base file stays patched.
> This is outside the patch's control — the [official launcher](#pc--official-wwm-launcher)
> doesn't do this.

### PC — China Client (燕云十六声)

> [!CAUTION]
> **No longer maintained.** The patch below is the final build and may not match the current CN
> game version. Don't install it over a newer game version, and don't expect updates. Reasons:
> few Indonesian players on the CN client, limited scope (no cutscene subtitles), and CN updates
> that move faster than the translation phases can follow. Use the global client instead.

The China client had its own patch and release. Lines shared with the global game use the global
translation; CN-only lines are still being translated.

1. Close the game **and** the launcher.
2. Open `<InstallDir>\yysls_fast\LocalData\Patch\HD\v2\locale\` (e.g. `D:\yysls\yysls_fast\...`).
3. Back up `translate_words_map_zh_cn` and `translate_words_map_zh_cn__small`.
4. Extract **[`wwm-cn-indonesian-patch.zip`](../../releases/tag/cn-latest)** here and overwrite
   both files. Leave the small `*_diff` files untouched.
5. Launch through the launcher, keeping the game language on **Chinese (简体中文)** — the CN patch
   replaces the Chinese text itself.

> [!NOTE]
> **Not translated on the China client:** story and cutscene subtitles (they come from data packs
> the launcher verifies on every start, and modifying them could put your account at risk), the
> main-menu "退出游戏" button artwork and the NetEase login dialog. For a fully translated story,
> play the global client. Don't install an older patch over a newer game version.

---

## Updating & Uninstalling

- **After a game update:** download the latest release and copy the files in again. If the update
  added new text, the current release still works — the new lines just stay in English until the
  next release.
- **To uninstall:** copy your backed-up original files back over the patched ones.

---

## FAQ

<details>
<summary><b>Why must the game language be English?</b></summary>

The game's translation files don't contain the original text — only a map from a hash of the
English text to the displayed text. The patch replaces the displayed values, so the game only finds
them while it looks up English. ([Technical details](docs/Technical-Overview.md#why-the-game-must-be-set-to-english))
The China client is the exception: keep it on Chinese.
</details>

<details>
<summary><b>Some text is still in English (or Chinese). Is the patch broken?</b></summary>

No. Translation is still in progress, new game updates add new text, and on Steam the `_diff`
layer is restored on every launch. Check [Translation Progress](#translation-progress) and the
platform table above.
</details>

<details>
<summary><b>The patch disappeared after the game updated.</b></summary>

Expected — every update (even hotfixes) restores the original files on the official launcher,
Android and the China client (the latter is no longer maintained, so no new build will come).
Reinstall the latest release, or on Windows let
[WWM Patch Guard](#auto-reinstall--wwm-patch-guard-windows) do it automatically.
</details>

<details>
<summary><b>I can't see the <code>Android/data</code> folder.</b></summary>

That's an Android 11+ restriction, not a root requirement. Use the
[Files app (Method A)](#android) or [ADB (Method B)](#android).
</details>

<details>
<summary><b>Is this a cheat? Can I get banned?</b></summary>

The patch only changes display text and gives no gameplay advantage. Still, modifying client files
may conflict with the game's Terms of Service — see the [Disclaimer](#disclaimer) and use it at your
own risk.
</details>

<details>
<summary><b>I found a bad translation.</b></summary>

Please [open an issue](../../issues) with a screenshot and where it appears.
</details>

---

## Support the Project

This patch is and will always be **free**. Keeping it up to date has real costs: AI translation of
hundreds of thousands of lines, rebuilding the patch after every NetEase update, and the time of a
single maintainer.

If the patch made the game more enjoyable for you, please consider a donation — every amount goes
toward translation costs and keeping the patch current. Can't donate? Starring the repo, sharing
it with other players or reporting bad translations helps too.

<p align="center">
  <img src=".donation/saweria.png" alt="Saweria QR code" width="240"><br>
  <b>Donate via Saweria:</b> <a href="https://saweria.co/oratakashi">saweria.co/oratakashi</a>
</p>

---

## For Contributors & Developers

This repo also contains a reverse-engineering toolkit (`wwm_locmap.py` + `tools/`) to dump,
translate, validate and repack the game's `translate_words_map_*` localization files.

| Guide | What's inside |
|---|---|
| [Developer Guide](docs/Developer-Guide.md) | Setup, dump → translate → QA → repack, daily workflow, game updates, China profile, repo layout |
| [Technical Overview](docs/Technical-Overview.md) | Binary format, why English is required, known limitations |
| [Translation Guidelines](docs/Translation-Guidelines.md) | Engine format tokens, proper nouns, LLM QA |
| [Contributing](docs/Contributing.md) | Reporting issues, pull requests, open research areas |
| [Format-Spec.md](Obsidian-Vault/knowladge/Format-Spec.md) | The full reverse-engineered file format specification |

---

## Disclaimer

- This is an independent fan translation and research project, **not affiliated with or endorsed
  by** NetEase Inc., Everstone Studio or their subsidiaries.
- *Where Winds Meet* is an online game. Modifying client files may conflict with its Terms of
  Service and could trigger integrity checks. The authors accept no liability for account
  penalties or instability — use it at your own risk.
- The patch changes **display text only**: no cheats, exploits or gameplay advantages.
- This repository does not distribute the game itself or commercial translation packs.

## License

Licensed under the [GNU General Public License v3.0](LICENSE). The license covers the source code
and documentation in this repository only; it grants no rights to the game's proprietary content.
