//! Procedural macros for `rosu-memory-lib`.
//!
//! This crate provides the `ReadMemory` derive macro, which greatly simplifies
//! reading structured data from another process's memory. Instead of manually
//! calculating pointers and offsets, you can simply annotate your struct fields
//! and let the macro generate the memory-reading code.

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod attr;
mod expand;
mod utils;

/// Derive macro that generates a `read_from_memory` method on a struct.
///
/// # Struct-level Attributes
///
/// Use `#[read_memory(...)]` on the struct to configure the base address and pointer chains.
///
/// | Key     | Example Value                | Description                                                                 |
/// |---------|------------------------------|-----------------------------------------------------------------------------|
/// | `init_base` | `p.read_i32(0x1234)?`     | Sets the initial base address expression. May reference `p` and `state`.    |
/// | `chain` | `[0x10, 0x20]`               | Translates to consecutive `addr = p.read_i32(addr + offset)?` pointer walks.|
/// | `guard` | `Playing`                    | Ensures the game is in `GameState::Playing` before reading memory.         |
///
/// If `init_base` is provided, the macro also generates a `read` method that
/// computes the base before delegating to `read_from_memory`:
/// ```rust,ignore
/// pub fn read(p: &Process, state: &mut State) -> Result<Self, Error>
/// ```
///
/// If `base` is omitted, the method instead takes an `i32` parameter for nested components:
/// ```rust,ignore
/// pub fn read_from_memory(p: &Process, state: &mut State, base: i32) -> Result<Self, Error>
/// ```
///
/// # Field-level Attributes
///
/// Every field in the struct must have one of the following annotations:
///
/// ## `#[offset(expr)]` / `#[offset(expr, via = Type)]`
/// Reads a primitive or `String` from `__base + expr`. The tool automagically infers the `p.read_X` method
/// based on the field's type. Use optionally `via = Type` when you need a custom type that implements `From<Type>`.
///
/// ```rust,ignore
/// #[offset(0x1C)]          // Reads an i32 exactly out of `base + 0x1C`.
/// pub beatmap_id: i32,
///
/// #[offset(0x20, via = i32)] // Reads an i32 and calls `GameMode::from(val)`.
/// pub mode: GameMode,
/// ```
///
/// ## `#[nested(expr)]`
/// Delegates reading to `FieldType::read_from_memory(p, state, __base + expr)`. Useful
/// for composing memory structures!
///
/// ## `#[ptr_chain(base_expr, d1, d2, ..., final_offset)]`
/// Declares a custom pointer chain *independent* of the struct's base address.
/// Useful when a field resides somewhere else entirely (e.g. from a static pointer).
///
/// ```rust,ignore
/// #[ptr_chain(state.addresses.rulesets, 0x4, 0x14, 0x2C, 0x0)]
/// pub ruleset_ptr: i32,
/// ```
/// Also supports `via = Type` at the end: `#[ptr_chain(..., 0x10, via = u8)]`.
///
/// ## `#[computed(expr)]`
/// Evaluates `expr` inline, which might refer to previous fields that are already read! Let's you derive
/// flags out of existing members seamlessly.
///
/// ## `#[skip]`
/// Initializes using `Default::default()`. Memory is ignored.
///
/// # Batching (Performance)
/// Consecutive `#[offset(LITERAL)]` primitive fields are **batched**. The macro groups them into
/// a single `p.read(..., byte_slice)` operation resulting in significantly fewer `readProcessMemory` calls!
#[proc_macro_derive(
    ReadMemory,
    attributes(read_memory, offset, nested, ptr_chain, computed, skip, init_base)
)]
pub fn derive_read_memory(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive_read_memory(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
