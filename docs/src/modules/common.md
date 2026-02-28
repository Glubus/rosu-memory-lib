# Common

Shared types and stable-client helpers used across all modules.

## Enums

### `GameState`

Represents the current osu! game state read from memory.

| Variant | Value |
|---------|-------|
| `MainMenu` | 0 |
| `Editor` | 1 |
| `Playing` | 2 |
| `SongSelect` | 5 |
| `ResultScreen` | 7 |
| … | … |

### `GameMode`

| Variant | Value |
|---------|-------|
| `Osu` | 0 |
| `Taiko` | 1 |
| `Catch` | 2 |
| `Mania` | 3 |

### `OsuClientKind`

| Variant | Description |
|---------|-------------|
| `Stable` | osu!stable (fully supported) |
| `Lazer` | osu!lazer (unsupported, returns `Error::Unsupported`) |

## Stable Structs

### `GameStateInfo`

```rust
pub struct GameStateInfo {
    pub state: GameState,
}
```

`init_base = p.read_i32(state.addresses.status - 0x4)?`, reads `GameState` via `u32` at offset `0x0`.

### `TimingInfo`

```rust
pub struct TimingInfo {
    pub game_time: i32,
}
```

`init_base = p.read_i32(state.addresses.playtime + 0x5)?`, game time in milliseconds at offset `0x0`.

### `MenuInfo`

```rust
pub struct MenuInfo {
    pub mods: u32,
}
```

Active mods in the song select menu. `init_base = p.read_i32(state.addresses.menu_mods + 0x9)?`.

### `PauseInfo`

```rust
pub struct PauseInfo {
    pub pause_val: i8,
}
impl PauseInfo {
    pub fn is_paused(&self) -> bool { self.pause_val == 1 }
}
```

### `ReplayInfo`

```rust
pub struct ReplayInfo {
    pub is_watching: bool,
}
```

### `PathInfo`

```rust
pub struct PathInfo {
    pub songs_folder: PathBuf,
}
```

Reads the `Songs` folder path from osu! settings. On Linux may return a relative path.
