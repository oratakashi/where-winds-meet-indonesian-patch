# Contributing

Contributions are welcome — from a one-line typo report to format research.

## Report a translation problem

Found a wrong, awkward or broken line in-game? Open an
[issue](https://github.com/oratakashi/where-winds-meet-indonesian-patch/issues) with:

- the text as shown in-game (a screenshot helps),
- where it appears (menu, quest, item, NPC dialogue),
- your client (Steam, official launcher, Android; the China client is no longer maintained) and the patch release date.

## Work on the code or translations

1. Set up the toolkit: [Developer Guide](Developer-Guide.md#prerequisites).
2. Enable the pre-commit hook once per clone — it runs the QA check and the progress check on any
   commit that touches translation data:
   ```bash
   git config core.hooksPath .githooks
   ```
3. Follow the [Translation Guidelines](Translation-Guidelines.md) and the
   [`Quick-Reference.md`](../Obsidian-Vault/knowladge/Quick-Reference.md) glossary.
4. Before opening a pull request, run:
   ```bash
   sh tools/wwm.sh qa --locale "locale/*.jsonl"
   sh tools/wwm.sh progress --write
   ```
   CI runs the same checks on every PR that touches `locale/` (and `cn/` for the China client).
5. Changing the tooling? Every tool exists in Rust (`rust/`) **and** Python — change both, then run
   `cd rust && cargo test` and `sh tools/parity_check.sh` (CI's `rust-ci` does the same). See
   [Rust Toolchain](Rust-Toolchain.md).

## Open research areas

1. **Hash derivation** — the H1/H2 split derived from `keyHash` and `seed`
   ([Format-Spec §4.4](../Obsidian-Vault/knowladge/Format-Spec.md)) would allow inserting new keys.
2. **QA signatures** — report new LLM prompt-leak patterns for `LEAK_SIGNATURES` in
   `tools/qa_check.py` (and its Rust twin in `rust/crates/wwm-tools/src/qa.rs`).
3. **Other locales** — check parser compatibility with non-English packages (`_de`, `_fr`, `_ja`, …).
4. **Glossary polish** — refine Wuxia terminology in
   [`Obsidian-Vault/knowladge/glossary/`](../Obsidian-Vault/knowladge/glossary/).
