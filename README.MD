# rosu-memory-lib

A modular Rust library for reading osu! memory, forked from [rosu-memory](https://github.com/486c/rosu-memory).

> **Full documentation:** see the [mdbook docs](./docs/) for architecture overview, module reference, and macro documentation.

## Overview

While rosu_memory was originally designed as a binary application, this fork transforms it into a flexible library that can be integrated into any Rust project. The library uses a `#[derive(ReadMemory)]` proc macro to generate type-safe, batched memory reads from annotated structs — eliminating pointer-chain boilerplate entirely.

## Features

- Memory reading for osu!stable (lazer support may come in the future)
- `#[derive(ReadMemory)]` proc macro — declare offsets on struct fields, get a `read_from_memory` method
- Automatic batching of consecutive primitive reads into single `readProcessMemory` calls
- Modular design: one `Reader` struct per domain (`BeatmapReader`, `GameplayReader`, `ResultScreenReader`, `UserReader`, `OverlayReader`)
- Type-safe memory access with full Rust error propagation

## Quick Start

```rust
use rosu_memory_lib::reader::{
    common::{OsuClientKind, stable::GameStateInfo},
    gameplay::GameplayReader,
    structs::{State, StaticAddresses},
};
use rosu_mem::process::Process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = Process::connect("osu!")?;
    let mut state = State { addresses: StaticAddresses::new(&p)? };

    let mut gameplay = GameplayReader::new(&p, &mut state, OsuClientKind::Stable);
    let info = gameplay.info()?;
    println!("Score: {} | Combo: {}x | HP: {:.2}", info.score, info.combo, info.hp);
    Ok(())
}
```

## Module Overview

| Module | Struct | Available when |
|--------|--------|----------------|
| `beatmap` | `BeatmapInfo` | SongSelect, Editor, Playing, ResultScreen |
| `gameplay` | `GameplayInfo` | Playing |
| `resultscreen` | `ResultScreenInfo` | ResultScreen |
| `user` | `UserInfo` | always |
| `overlay` | `KeyOverlay` | Playing + osu!Standard mode |

See the [module docs](./docs/src/modules/) for field tables, pointer chains, and usage examples.

## Examples

Check out the `examples/` directory for various implementations and use cases.

## Projects Using It

- [rosu-snippets](https://github.com/Glubus/rosu-snippets) — Create your own snippets for efficient mapping in mania

*If you're using rosu-memory-lib in your project, feel free to submit a PR to add it to this list!*

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

## Credits

- Original [rosu_memory](https://github.com/486c/rosu-memory) by 486c
- All contributors to the original project

## License

Same as original rosu_memory
