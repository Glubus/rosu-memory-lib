# Changelog

All notable changes to `rosu-memory-lib` are documented here.

## [Unreleased] — rewrite-using-proc-macros

### Added
- `#[derive(ReadMemory)]` proc macro in `rosu-memory-macros` crate
  - Field attributes: `#[offset]`, `#[nested]`, `#[ptr_chain]`, `#[computed]`, `#[skip]`
  - Struct attributes: `init_base`, `chain`, `guard`
  - Automatic batching of consecutive `#[offset(LITERAL)]` primitive fields into a single `readProcessMemory` call
- `Hit` struct now derives `ReadMemory` — memory layout with offsets for all six hit types
- 10 unit tests for `calculate_accuracy` covering all game modes and edge cases
- mdbook documentation under `docs/` with architecture overview, macro reference, and per-module pages

### Changed
- **gameplay**: rewritten from imperative `stable/{offset,memory}.rs` to single `stable.rs` using `ReadMemory`
  - `mods` decoded via `#[computed]` XOR trick
  - `hp` and `retries` via `#[ptr_chain]` (different pointer branches)
  - `ig_time` via `#[computed(TimingInfo::read(...)?.game_time)]`
- **resultscreen**: rewritten to single `stable.rs`
  - `accuracy` via `#[computed(calculate_accuracy(&mode, &hits)?)]`
- **user**: rewritten to single `stable.rs`
  - macro batches `playcount`/`playmode`/`rank`/`pp`/`bancho_status` (5 × i32) automatically
- **overlay**: consolidated from `stable/{memory,mod,offset}.rs` to single `stable.rs` (stays imperative — dynamic array walk cannot be expressed with current macro attributes)
- All `XxxReader` structs reduced to a single `fn info() -> XxxInfo` method (individual field accessors removed)
- `helpers.rs`: removed `generate_reader_fn!` and `generate_offset_getter!` legacy macros

### Removed
- `gameplay/common.rs`, `gameplay/stable/{offset,memory,mod}.rs`
- `resultscreen/common.rs`, `resultscreen/stable/{offset,memory,mod}.rs`
- `user/common.rs`, `user/stable/{offset,memory,mod}.rs`
- `overlay/stable/{memory,mod,offset}.rs`
- `REFACTORING.md` (superseded by mdbook docs)

## Previous changes

See git log for history prior to the proc-macro rewrite.
