# Update-3A Translation Log (idx 535,919–565,918)

Strings new in **game v2.4 [Across Autumn Waters]** (2026-11-13), documented in [[Update-3]]. Output: `locale/update3a.jsonl` (created by the first translation session). 30,000 strings. Standard [[Quick-Reference]] conventions apply.

## Batches

- 2026-11-13 (session 151): idx 535,919–536,199 (281 rows) -> `locale/update3a.jsonl`. QA clean. Update-3A 281/30,000.
- 2026-11-14 (session 151, continued): idx 536,200–565,918 (29,719 rows) translated by 15 subagents in disjoint ranges of ~2,000 rows (user-authorized), merged in order and re-validated by the primary agent. **Update-3A complete: 30,000/30,000.**

## Decisions & notes
- Stat labels (`Minimum/Maximum <X> Attack`), Path/Set names, `Thunder Enlightenment`, `Skill` names stay English; `Set`/`Path` words kept as English loanwords. `Legacy`/`Homestead` stay English; `Hangzhou Resident/Commoner/Citizen` variants -> `Warga Hangzhou`/`Rakyat Jelata Hangzhou`; `Section N` -> `Bagian N`, `Stage` -> `Tahap`, `segment` -> `Segmen`; `Login for N Days` -> `Login selama N Hari`; `%sd` countdown -> `%sh`.
- Subagent-batch conventions: `Layer`->`Lapisan N`, `Segment`->`segmen`, `Rank N`->`Rank Kesebelas` etc., `Phase N`->`Tahap N`, `Part N`->`Bagian N`, `Floor N`->`Lantai N`, `Chapter`->`Bab N`, `Prosperity`->`Kemakmuran`, `Countermeasure`->`Cara menghadapi:` (unified; r05 had `Cara melawan`), `Strategy`->`Strategi:`, `Account:` story prefix->`Kisah:`, `Resident X`->`Warga X`, `Village Men`->`Pria Desa`, `Great Hero`->`Pahlawan Agung`, `Eldest Young Master`->`Tuan Muda Sulung`, `1 PM - 3 PM`->`13.00 - 15.00`.
- Kept English: Thunder Enlightenment/Heaven's Will buff names, Void King/Formless Emperor boss names, `· Trial` outfit/weapon suffixes, `Lone Cloud Sect` (like Mohist Sect), `Gift of Gab`, `Ludo`, internal animation labels (`Chicken Simulator - ...`), whole-sentence `<...>` tags.
- Leftover Chinese developer notes translated (e.g. 537413, 540167, 557626); stray CJK inside tags kept.
