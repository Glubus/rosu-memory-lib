# Proc Macros

The `rosu-memory-macros` crate provides `#[derive(ReadMemory)]`.

## Struct-Level Attributes

Use `#[read_memory(...)]` on the struct:

| Key | Description |
|-----|-------------|
| `init_base = expr` | Expression that computes the starting base address. Generates a `read(p, state)` convenience method. |
| `chain = [o1, o2, …]` | Pointer dereference chain applied to the base: `addr = p.read_i32(addr + oN)?` for each offset. |
| `guard = Variant` | Checks `GameState::Variant` before reading (requires `check_game_state` helper). |

The alternate `#[init_base(expr)]` attribute (without `read_memory`) is also supported.

## Field-Level Attributes

Every field must have one of:

### `#[offset(expr)]` / `#[offset(expr, via = Type)]`

Reads a primitive from `base + expr`. Type is inferred from the Rust field type.
Use `via = Type` when the field type needs a `From` conversion (e.g. enum from i32).

```rust
#[offset(0x78)]
pub score: i32,

#[offset(0x64, via = i32)]
pub mode: GameMode,
```

### `#[nested(expr)]`

Delegates to `FieldType::read_from_memory(p, state, base + expr)`.

```rust
#[nested(0x88)]
pub hits: Hit,
```

### `#[ptr_chain(base_expr, d1, d2, …, final_offset)]`

Walks a custom pointer chain independent of the struct base.

```rust
#[ptr_chain(state.addresses.rulesets - 0xb, 0x4, 0x68, 0x40, 0x1C)]
pub hp: f64,
```

### `#[computed(expr)]`

Inline expression; can reference fields declared earlier in the struct.

```rust
#[computed(crate::reader::helpers::calculate_accuracy(&mode, &hits)?)]
pub accuracy: f64,
```

### `#[skip]`

Initialises with `Default::default()`.

## Batching

Consecutive `#[offset(LITERAL)]` fields with primitive types are grouped into a single `p.read()` call, minimising `readProcessMemory` syscalls.
