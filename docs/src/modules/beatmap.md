# Beatmap

Reads beatmap data. Available in `SongSelect`, `Editor`, `Playing`, and `ResultScreen` states.

## Structs

The beatmap module exposes several sub-structs that can be read independently or via the top-level `BeatmapInfo`:

```rust
pub struct BeatmapInfo {
    pub metadata: BeatmapMetadata,
    pub location: BeatmapLocation,
    pub stats: BeatmapStats,
    pub technical: BeatmapTechnicalInfo,
}
```

### `BeatmapMetadata`

| Field | Offset | Type |
|-------|--------|------|
| `author` | `0x18` | String |
| `creator` | `0x7C` | String |
| `title_romanized` | `0x24` | String |
| `title_original` | `0x28` | String |
| `difficulty` | `0xAC` | String |
| `tags` | `0x20` | String |

### `BeatmapLocation`

| Field | Offset | Type |
|-------|--------|------|
| `folder` | `0x78` | String |
| `filename` | `0x90` | String |
| `audio` | `0x64` | String |
| `cover` | `0x68` | String |

Helper methods: `get_file_path()`, `get_audio_path()`, `get_cover_path()`.

### `BeatmapStats`

| Field | Offset | Type |
|-------|--------|------|
| `ar` | `0x2C` | f32 |
| `cs` | `0x30` | f32 |
| `hp` | `0x34` | f32 |
| `od` | `0x38` | f32 |
| `length` | `0x134` | i32 |
| `object_count` | `0xF8` | i32 |
| `slider_count` | `0x146` | i32 |
| `star_rating` | — | `BeatmapStarRating` (skipped, computed separately) |

### `BeatmapTechnicalInfo`

| Field | Offset | Type |
|-------|--------|------|
| `md5` | `0x6C` | String |
| `id` | `0xC8` | i32 |
| `set_id` | `0xCC` | i32 |
| `mode` | `0x11C` (via i32) | `GameMode` |
| `ranked_status` | `0x12C` (via i32) | `BeatmapStatus` |

## Pointer Chain

```
state.addresses.base - 0xC
  → read_i32   (ptr1)
  → read_i32   (beatmap_addr)  ← all nested structs use this as base
```

## Usage

```rust
let mut reader = BeatmapReader::new(&p, &mut state, OsuClientKind::Stable)?;
let info = reader.info()?;
println!("{} — {}", info.metadata.title_romanized, info.metadata.difficulty);
println!("AR: {} CS: {} HP: {} OD: {}", info.stats.ar, info.stats.cs, info.stats.hp, info.stats.od);
```

Individual sub-structs can also be read directly:

```rust
let meta = BeatmapMetadata::read(&p, &mut state)?;
let loc  = BeatmapLocation::read(&p, &mut state)?;
let path = loc.get_file_path();
```
