# Overlay

Reads key/mouse press state and counts for the input overlay.

Only available when osu! is `Playing` in osu!Standard mode (playmode == 0, beatmap mode == Osu).

## Struct: `KeyOverlay`

```rust
pub struct KeyOverlay {
    pub key_1: Key,
    pub key_2: Key,
    pub mouse_1: Key,
    pub mouse_2: Key,
}

pub struct Key {
    pub pressed: bool,
    pub count: i32,
}
```

## Implementation Note

The overlay data lives in a dynamic array structure that requires walking through multiple pointer levels at runtime. This cannot be expressed with the `ReadMemory` derive macro, so the implementation remains imperative in `stable.rs`.

## Pointer Chain

```
state.addresses.rulesets - 0xb → +0x4 (ruleset_addr)
  → ruleset_addr + 0xb0 (key_ptr)
  → key_ptr + 0x10 → + 0x4 (key_array_addr)
  → key_array_addr[0x8/0xC/0x10/0x14] → +0x1C (pressed), +0x14 (count)
```

## Usage

```rust
let mut reader = OverlayReader::new(&p, &mut state, OsuClientKind::Stable);
let info = reader.info()?;
println!("K1: {} ({})", info.key_1.pressed, info.key_1.count);
```
