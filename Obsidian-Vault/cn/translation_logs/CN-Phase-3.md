# CN Phase 3 Translation Log (idx 60,000–89,999)

Session history and terminology for `cn/locale/phase3.jsonl`. Rules:
[[CN-Quick-Reference]]; live progress: [[CN-Current-Status]]; precedents: [[CN-Phase-2]].

## Batches

- **2026-10-23 — batch 1, idx 60,000–63,999 (4,000 rows).** Mandarin → Indonesian,
  translated in three parallel ranges (60,000–61,333, 61,334–62,666,
  62,667–63,999). Output: `cn/locale/phase3.jsonl`. Next idx: **64,000**.
  Exact contiguous range verified (4,000 rows, no gaps or duplicates); digit differences
  reviewed as Chinese-numeral conversions or date/word-order changes. Newline/tab counts
  and runtime markers checked. Full CN QA: **52,002 entries** including 2 overrides,
  **0 findings**. Phase 3: **4,000 / 30,000 (13.33%)**.
  **Overall: 52,000 / 159,176 unique strings (32.67%); CN-only in-game coverage
  670,416 / 823,139 (81.45%).** Generated with
  `python tools/progress.py --profile cn --write`; includes internal editor labels,
  excludes the global bridge.

## Batch 1 decisions

- Mostly internal editor labels (NPC/spawn, reward tables, activity and quest labels):
  translate briefly and literally; developer and CN-only person names use Pinyin.
- Global proper nouns follow the shared-key lookup; tokens, numeric identifiers,
  poem wrappers, newlines and tabs retain their source structure.

- Confirmed global names include Mohist Hill, Jadebrook Mountain, South Imperial Garden,
  Granary of Plenty, Wheatwind Bazaar, Skyward City Ruins, Aureate Pavilion, Eternal
  Solitude Hall, Buddha Fort, The Void King, Bodhi Sea, Jadestream Valley, Coffin Master,
  Perception Forest, Well of Heaven, Unbound Cavern, Sky Shuttle, Hollow Vale, An Liuli,
  Velvet Shade, Derndale, Mirkvale, Cloud Steps, Mirage, Hexi and Jade Gate Pass.
- Lookup labels: 文津馆 → Perintah Terikat Tinta; 太医署 → Imperial Medical Academy;
  太清楼 → Menara Kesucian; 天水集 → Skybrim Market; 丝竹雅韵 → Melodi yang Anggun;
  聆音 → Gema; 万相集 → Gallery; 金蟾 → Kodok Emas.
- CN-only forms: 封龙书院 → Fenglong Academy; 月桂林 → Yuegui Forest;
  千年渡 → Qiannian Ferry; 寒刃寸拳 → Bilah Dingin Pukulan Satu Inci;
  守势寒刃 → Bilah Dingin Bertahan; 怒虎无双 → Harimau Murka Tiada Tara;
  玄金甲 → Zirah Emas Hitam. These remain provisional until an official global match exists.
- Bare hashtags with Chinese text (61799, 63116) use a space after `#` before the
  translated text, preventing an accidental `#S`/`#K` markup token. Existing actual
  tokens remain verbatim. An extra trailing newline at 60138 was removed after review.
