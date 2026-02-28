# Architecture

## Rewrite Pattern

All modules follow the same structure, modelled after the `beatmap` module:

```
reader/
├── beatmap/
│   ├── mod.rs       — BeatmapReader with impl_osu_accessor!
│   └── stable.rs    — BeatmapInfo + sub-structs + pub mod memory
├── gameplay/
│   ├── mod.rs       — GameplayReader
│   └── stable.rs    — GameplayInfo + pub mod memory
├── resultscreen/
│   ├── mod.rs       — ResultScreenReader
│   └── stable.rs    — ResultScreenInfo + pub mod memory
├── user/
│   ├── mod.rs       — UserReader
│   └── stable.rs    — UserInfo + pub mod memory
├── overlay/
│   ├── common.rs    — Key, KeyOverlay structs
│   ├── mod.rs       — OverlayReader
│   └── stable.rs    — imperative pointer walk + pub mod memory
└── common/
    ├── mod.rs       — GameState, GameMode, OsuClientKind enums
    └── stable.rs    — GameStateInfo, TimingInfo, PauseInfo, …
```

## ReadMemory Derive Macro

Structs derive `#[derive(ReadMemory)]` which generates a `read_from_memory(p, state, base)` method.

Field attributes control how each field is read:

| Attribute | Description |
|-----------|-------------|
| `#[offset(0xNN)]` | Read primitive at `base + offset` |
| `#[offset(0xNN, via = T)]` | Read as `T`, convert with `From` |
| `#[nested(0xNN)]` | Delegate to `FieldType::read_from_memory(p, state, base + offset)` |
| `#[ptr_chain(base, d1, …, final)]` | Walk a pointer chain independent of the struct base |
| `#[computed(expr)]` | Evaluate an expression (can reference earlier fields) |
| `#[skip]` | Use `Default::default()` |

Consecutive `#[offset(LITERAL)]` primitive fields are **batched** into a single `p.read()` syscall.

## Reader Pattern

Each module exposes a `XxxReader<'a>` struct with an `info()` method that returns the complete data struct:

```rust
pub struct GameplayReader<'a> {
    pub process: &'a Process,
    pub state: &'a mut State,
    pub osu_type: OsuClientKind,
}

impl<'a> GameplayReader<'a> {
    impl_osu_accessor! {
        fn info() -> GameplayInfo => stable::memory::info,
    }
}
```

The `impl_osu_accessor!` macro generates the dispatch: Stable delegates to the stable implementation; other client types return `Error::Unsupported`.
