# Introduction

`rosu-memory-lib` is a Rust library for reading osu!stable process memory.

It exposes typed structs for each game data domain (beatmap, gameplay, result screen, user profile, key overlay) and uses a derive macro (`ReadMemory`) to eliminate boilerplate pointer-chain traversal code.

## Quick Start

Each domain has a `Reader` struct that wraps a `Process` reference and a mutable `State`:

```rust
let p = Process::connect("osu!")?;
let mut state = State { addresses: StaticAddresses::new(&p)? };

let mut gameplay = GameplayReader::new(&p, &mut state, OsuClientKind::Stable);
let info = gameplay.info()?;
println!("Score: {}", info.score);
```

## Module Overview

| Module | Struct | Available when |
|--------|--------|----------------|
| `beatmap` | `BeatmapInfo` | SongSelect, Editor, Playing, ResultScreen |
| `gameplay` | `GameplayInfo` | Playing |
| `resultscreen` | `ResultScreenInfo` | ResultScreen |
| `user` | `UserInfo` | always (once process is running) |
| `overlay` | `KeyOverlay` | Playing + osu!Standard mode |
| `common` | `GameStateInfo`, `TimingInfo`, … | varies |
