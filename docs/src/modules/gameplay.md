# Gameplay

Reads live gameplay data while osu! is in the `Playing` state.

## Struct: `GameplayInfo`

```rust
pub struct GameplayInfo {
    pub score: i32,
    pub mods: u32,
    pub combo: i16,
    pub max_combo: i16,
    pub hp: f64,
    pub username: String,
    pub ig_time: i32,
    pub retries: i32,
    pub hits: Hit,
}
```

| Field | Offset (from score_base) | Notes |
|-------|--------------------------|-------|
| `score` | `0x78` | |
| `mods` | `0x1C` → XOR | Decoded via XOR of two u64 values |
| `combo` | `0x94` | |
| `max_combo` | `0x68` | |
| `hp` | ptr_chain: rulesets→+0x4→+0x68→+0x40, `+0x1C` | Different pointer branch than score |
| `username` | `0x28` | |
| `ig_time` | — | Delegated to `TimingInfo` |
| `retries` | `base - 0x33` → `+0x8` | Different base pointer |
| `hits` | `0x88` (nested `Hit`) | 6× i16 batched read |

## Pointer Chain

```
state.addresses.rulesets - 0xb
  → read_i32 + 0x4    (ruleset_addr)
  → read_i32 + 0x68   (gameplay_base)
  → read_i32 + 0x38   (score_base)  ← struct base
```

## Usage

```rust
let mut reader = GameplayReader::new(&p, &mut state, OsuClientKind::Stable);
let info = reader.info()?;
println!("Score: {} | Combo: {}x | HP: {:.2}", info.score, info.combo, info.hp);
```
