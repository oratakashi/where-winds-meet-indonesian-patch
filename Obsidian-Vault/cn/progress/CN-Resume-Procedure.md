# CN Resume Procedure

How to pick up a **WWM China** translation session. Same shape as the global
[[Resume-Procedure]] (read its "Minimize round-trips" section — it applies unchanged); only
paths and the `--profile cn` flag differ.

## One-time setup per machine

```bash
pip install -r requirements.txt
# CN keystream -> cn/work/keystream_v2.bin (gitignored). Decrypt the committed copy with the
# passphrase (repo secret CN_KEYSTREAM_PASSPHRASE, kept by the owner in cn/work/keystream_passphrase.txt):
gpg --batch --pinentry-mode loopback --passphrase-file cn/work/keystream_passphrase.txt -o cn/work/keystream_v2.bin -d cn/keystream_v2.bin.gpg
# ...or recover it from the client at the login screen, ADMIN terminal (the client runs elevated):
python cn/tools/find_dict_in_dump.py --pid <yysls.exe PID> --v2-dir "D:\yysls\yysls_fast\LocalData\Patch\HD\v2"
# dumps (gitignored): CN + global (the bridge and term_lookup need both)
python wwm_locmap.py dump cn/translate_words_map_zh_cn cn/work/strings_zh_cn.jsonl
python wwm_locmap.py dump cn/translate_words_map_zh_cn__small cn/work/strings_zh_cn__small.jsonl
python wwm_locmap.py dump translate_words_map_en strings.jsonl
python wwm_locmap.py dump translate_words_map_en_diff strings_diff.jsonl
```

Translating and validating only needs `cn/translation_work/unique_strings.jsonl` and
`cn/locale/` (tracked). The keystream and dumps are needed for `term_lookup.py`,
`patch_all.py --profile cn` and `rebuild_unique_strings.py --profile cn`.

## Session steps

1. Read [[CN-Current-Status]] (next idx per phase) and [[CN-Quick-Reference]].
2. Read the next batch from `cn/translation_work/unique_strings.jsonl` (idx order).
3. Look up every proper noun with `python cn/tools/term_lookup.py …` (many terms per call).
4. Append the translations to `cn/locale/phaseN.jsonl` (`{"idx": N, "v": "…"}`, no gaps).
5. Validate: `python tools/qa_check.py --profile cn --locale "cn/locale/*.jsonl"` → 0 findings,
   then `python cn/tools/name_check.py --fix` and `python cn/tools/name_check.py --idx <batch range>`
   — fix by hand every place/NPC/Inner Way name it still lists ([[CN-Quick-Reference]] §2a).
6. Update the phase log `Obsidian-Vault/cn/translation_logs/CN-Phase-N.md` (create it on first
   use: range, output file, `## Batches` entry with idx range + date, new CN-only names).
7. `python tools/progress.py --profile cn --write` and quote its overall line in the session
   log and the final message.

## Building a playable CN patch

Every push to `main` that touches `cn/` (or the global translation, which CN reuses) rebuilds
the rolling GitHub Release **`cn-latest`** via `.github/workflows/release-patch-cn.yml` — players
download from there. To build locally:

```bash
python tools/patch_all.py --profile cn --level 12   # -> cn/work/patched/translate_words_map_zh_cn*
```

Each entry gets: global translation for its key (bridge) → else the CN translation of its
Mandarin text → else the original. The output is a normal v2 container (encrypted dictionary
copied verbatim), installed over
`D:\yysls\yysls_fast\LocalData\Patch\HD\v2\locale\` (back up the originals first). That folder
is the patcher's overlay — whether the CN launcher reverts it is not tested yet.

`python tools/expand_locale.py --profile cn` writes the same result as a patch JSONL
(`cn/work/strings.translated.jsonl`) for `wwm_locmap.py patch`, if one file needs it.
