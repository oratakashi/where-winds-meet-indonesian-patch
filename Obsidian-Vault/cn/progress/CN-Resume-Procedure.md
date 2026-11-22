# CN Resume Procedure

How to pick up a **WWM China** translation session. Same shape as the global
[[Resume-Procedure]] (read its "Minimize round-trips" section — it applies unchanged); only
paths and the `--profile cn` flag differ.

## One-time setup per machine

Commands go through `sh tools/wwm.sh` (Rust `wwm` when built, Python scripts otherwise —
`WWM_TOOLCHAIN=python` forces Python; see `docs/Rust-Toolchain.md`).

```bash
cargo build --release --manifest-path rust/Cargo.toml   # Rust toolchain (optional)
pip install -r requirements.txt                         # Python failsafe
# CN keystream -> cn/work/keystream_v2.bin (gitignored). Decrypt the committed copy with the
# passphrase (repo secret CN_KEYSTREAM_PASSPHRASE, kept by the owner in cn/work/keystream_passphrase.txt):
gpg --batch --pinentry-mode loopback --passphrase-file cn/work/keystream_passphrase.txt -o cn/work/keystream_v2.bin -d cn/keystream_v2.bin.gpg
# ...or recover it from the client at the login screen, ADMIN terminal (the client runs elevated):
sh tools/wwm.sh cn find-dict --pid <yysls.exe PID> --v2-dir "D:\yysls\yysls_fast\LocalData\Patch\HD\v2"
# dumps (gitignored): CN + global (the Python bridge and term_lookup need both; the Rust
# binary decodes a missing dump from its binary, keystream permitting)
sh tools/wwm.sh dump cn/translate_words_map_zh_cn cn/work/strings_zh_cn.jsonl
sh tools/wwm.sh dump cn/translate_words_map_zh_cn__small cn/work/strings_zh_cn__small.jsonl
sh tools/wwm.sh dump translate_words_map_en strings.jsonl
sh tools/wwm.sh dump translate_words_map_en_diff strings_diff.jsonl
```

Translating and validating only needs `cn/translation_work/unique_strings.jsonl` and
`cn/locale/` (tracked). The keystream (and, on Python, the dumps) are needed for
`cn term-lookup`, `cn name-check`, `patch-all --profile cn` and `rebuild-unique --profile cn`.

## Session steps

1. Read [[CN-Current-Status]] (next idx per phase) and [[CN-Quick-Reference]].
2. Read the next batch from `cn/translation_work/unique_strings.jsonl` (idx order).
3. Look up every proper noun with `sh tools/wwm.sh cn term-lookup …` (many terms per call).
4. Append the translations to `cn/locale/phaseN.jsonl` (`{"idx": N, "v": "…"}`, no gaps).
5. Validate: `sh tools/wwm.sh qa --profile cn --locale "cn/locale/*.jsonl"` → 0 findings,
   then `sh tools/wwm.sh cn name-check --fix` and `sh tools/wwm.sh cn name-check --idx <batch range>`
   — fix by hand every place/NPC/Inner Way name it still lists ([[CN-Quick-Reference]] §2a).
6. Update the phase log `Obsidian-Vault/cn/translation_logs/CN-Phase-N.md` (create it on first
   use: range, output file, `## Batches` entry with idx range + date, new CN-only names).
7. `sh tools/wwm.sh progress --profile cn --write` and quote its overall line in the session
   log and the final message.

## Building a playable CN patch

Every push to `main` that touches `cn/` (or the global translation, which CN reuses) rebuilds
the rolling GitHub Release **`cn-latest`** via `.github/workflows/release-patch-cn.yml` — players
download from there. To build locally:

```bash
sh tools/wwm.sh patch-all --profile cn --level 12   # -> cn/work/patched/translate_words_map_zh_cn*
```

Each entry gets: global translation for its key (bridge) → else the CN translation of its
Mandarin text → else the original. The output is a normal v2 container (encrypted dictionary
copied verbatim), installed over
`D:\yysls\yysls_fast\LocalData\Patch\HD\v2\locale\` (back up the originals first). That folder
is the patcher's overlay — whether the CN launcher reverts it is not tested yet.

`sh tools/wwm.sh expand --profile cn` writes the same result as a patch JSONL
(`cn/work/strings.translated.jsonl`) for `sh tools/wwm.sh patch`, if one file needs it.
