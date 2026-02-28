# Result Screen

Reads result screen data while osu! is in the `ResultScreen` state.

## Struct: `ResultScreenInfo`

```rust
pub struct ResultScreenInfo {
    pub username: String,
    pub mode: GameMode,
    pub max_combo: i16,
    pub score: i32,
    pub hits: Hit,
    pub accuracy: f64,
}
```

| Field | Offset (from result_screen_base) | Notes |
|-------|----------------------------------|-------|
| `username` | `0x28` | |
| `mode` | `0x64` (via i32) | |
| `max_combo` | `0x68` | |
| `score` | `0x78` | |
| `hits` | `0x88` (nested `Hit`) | Same layout as gameplay |
| `accuracy` | — | Computed from `mode` + `hits` |

## Pointer Chain

```
state.addresses.rulesets - 0xb
  → read_i32 + 0x4    (ruleset_addr)
  → read_i32 + 0x38   (result_screen_base)  ← struct base
```

## Usage

```rust
let mut reader = ResultScreenReader::new(&p, &mut state, OsuClientKind::Stable);
let info = reader.info()?;
println!("Score: {} | Acc: {:.2}% | Max combo: {}x", info.score, info.accuracy, info.max_combo);
```
