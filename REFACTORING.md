# Refactoring: Removal of External Dependencies

## Overview
This document details the removal of unnecessary external dependencies (`rosu-pp`, `rosu-map`, and `rayon`) from the rosu-memory-lib project. The library now has minimal external dependencies and is more focused on its core mission: reading osu! game memory.

## Removed Dependencies

### 1. **rosu-pp** (Performance Points Calculator)
- **Reason**: This library was only used for star rating calculations in `beatmap/stable/file.rs`. Calculating star ratings is a specialized task that goes beyond the scope of memory reading.
- **Impact**: 
  - Removed from workspace dependencies
  - Removed from rosu-memory-lib dependencies
  - The `star_rating()` function now returns an error directing users to implement this separately
  - Deleted `examples/pp.rs` which heavily depended on rosu-pp

### 2. **rosu-map** (Beatmap File Parser)
- **Reason**: This library provided comprehensive beatmap parsing but created tight coupling with a specific implementation. The library now implements its own lightweight parser for the .osu format.
- **Impact**:
  - Removed from workspace dependencies
  - Removed from rosu-memory-lib dependencies
  - Replaced macro-based code generation with explicit parsing functions
  - Implemented custom .osu file parsers for all required metadata

### 3. **rayon** (Parallel Processing)
- **Reason**: Only used for parallel signature reading via the `parallel-read` feature. Sequential reading is sufficient and removes a heavy dependency.
- **Impact**:
  - Removed from workspace dependencies
  - Removed the `parallel-read` feature flag
  - Removed optional dependency on rayon
  - Kept only sequential signature reading in `StaticAddresses::new_sequential()`

## Changes Made

### Cargo.toml Files

#### Workspace Cargo.toml
**Before:**
```toml
[workspace.dependencies]
rosu-mem = "2.0.0"
rosu-map = "0.2.1"
rosu-pp = "3.1.0"
rosu-mods = "0.3.1"
rayon = { version = "1.10.0" }
syn = { version = "2", features = ["full", "extra-traits"] }
quote = "1"
proc-macro2 = "1"

[profile.dev.package.rosu-mem]
opt-level = 3
```

**After:**
```toml
[workspace.dependencies]
rosu-mem = "2.0.0"
rosu-mods = "0.3.1"
syn = { version = "2", features = ["full", "extra-traits"] }
quote = "1"
proc-macro2 = "1"
```

#### rosu-memory-lib Cargo.toml
**Before:**
```toml
[features]
default = []
parallel-read = ["rayon"]

[dependencies]
rosu-mem.workspace = true
rosu-map.workspace = true
rosu-pp.workspace = true
rosu-mods.workspace = true
rayon = { workspace = true, optional = true }
rosu-memory-macros = { path = "../rosu-memory-macros" }
```

**After:**
```toml
[dependencies]
rosu-mem.workspace = true
rosu-mods.workspace = true
rosu-memory-macros = { path = "../rosu-memory-macros" }
```

### Source Code Changes

#### `libs/rosu-memory-lib/src/reader/structs.rs`
- Removed `#[cfg(feature = "parallel-read")]` guards
- Removed rayon imports and parallel processing logic
- Removed `new_parallel()` method
- Kept only `new_sequential()` method (now called directly from `new()`)

#### `libs/rosu-memory-lib/src/reader/beatmap/stable/file.rs`
Major refactoring to remove rosu-map dependency:

**Removed:**
- rosu-map imports and usage
- Macro-based code generation for beatmap field getters

**Added:**
- Pure Rust .osu file parser functions:
  - `read_osu_file_content()` - File reader
  - `find_section()` - .osu file section locator
  - `parse_metadata_string()` - String metadata parser
  - `parse_metadata_float()` - Float metadata parser
  - `parse_metadata_int()` - Integer metadata parser
  - `read_beatmap_id_from_file()` - BeatmapID extraction
  - `read_beatmap_set_id_from_file()` - BeatmapSetID extraction
  - `read_artist_from_file()` - Artist extraction
  - `read_creator_from_file()` - Creator extraction
  - `read_title_from_file()` - Title extraction
  - `read_title_unicode_from_file()` - Unicode title extraction
  - `read_difficulty_from_file()` - Difficulty version extraction
  - `read_tags_from_file()` - Tags extraction
  - `read_overall_difficulty_from_file()` - OD extraction
  - `read_approach_rate_from_file()` - AR extraction
  - `read_circle_size_from_file()` - CS extraction
  - `read_hp_drain_rate_from_file()` - HP extraction
  - `read_object_count_from_file()` - Hit object counting
  - `read_slider_count_from_file()` - Slider detection and counting
  - `read_length_from_file()` - Beatmap length calculation
  - `read_drain_time_from_file()` - Drain time calculation
  - `read_mode_from_file()` - Game mode extraction

**Changed Functions:**
- All metadata getters (beatmap_id, author, creator, etc.) now use the internal parsers
- `star_rating()` now returns an informative error message
- `stats()` and `info()` use individual function calls instead of macro expansion

#### Removed Files
- `examples/pp.rs` - Example demonstrating PP calculation with rosu-pp

## Benefits

1. **Reduced Binary Size**: Fewer dependencies mean smaller compiled binaries
2. **Faster Compilation**: Less code to compile, especially rayon's parallel infrastructure
3. **Reduced Maintenance**: Fewer dependency updates to track
4. **Clearer Scope**: Library focuses on memory reading, not feature calculation
5. **No Feature Flags**: Simpler codebase without conditional compilation
6. **Zero-Copy Internal Parsing**: The new parsers are lightweight and efficient

## Migration Guide for Users

### Star Rating Calculation
If your code previously relied on `star_rating()`:

**Old code (would have needed rosu-pp):**
```rust
// This is now removed - implement it yourself or add rosu-pp
let rating = rosu_memory_lib::reader::beatmap::stable::file::star_rating(&process, &mut state)?;
```

**New approach - use rosu-pp directly (if needed):**
```rust
use rosu_pp::Beatmap;

let beatmap_path = rosu_memory_lib::reader::beatmap::stable::file::path(&process, &mut state)?;
let beatmap = Beatmap::from_path(&beatmap_path)?;
let diff_attrs = rosu_pp::Difficulty::new().calculate(&beatmap);
let stars = diff_attrs.stars();
```

### Parallel Signature Reading
If your code used the `parallel-read` feature:

**Old code:**
```toml
rosu-memory-lib = { version = "1.3.1", features = ["parallel-read"] }
```

**New code:**
```toml
rosu-memory-lib = "1.3.1"
```

The sequential implementation is efficient enough for most use cases.

## Stability

All public APIs remain unchanged except for:
- Removal of the `parallel-read` feature flag
- `star_rating()` now returns `Error::Other` instead of a calculation result

The rest of the library functions identically to before.

## Future Improvements

The custom .osu file parser can be further optimized:
- Add caching for frequently accessed files
- Implement error recovery for malformed files
- Add support for additional metadata fields
- Create helper functions for common parsing patterns

The `rosu-memory-macros` crate remains unchanged and provides robust procedural macro support for memory structure generation.