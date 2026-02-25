use syn::{Expr, Type};

/// Parsed representation of a single field attribute.
pub enum FieldAttr {
    /// `#[offset(expr)]` or `#[offset(expr, via = Type)]`
    ///
    /// Reads `p.read_TYPE(__base + expr)` where TYPE is inferred from the
    /// field's Rust type (or forced by `via`).
    Offset { expr: Expr, via: Option<Type> },

    /// `#[nested(expr)]`
    ///
    /// Calls `FieldType::read_from_memory(p, state, __base + expr)`.
    Nested(Expr),

    /// `#[ptr_chain(base_expr, d1, d2, ..., final_offset)]`
    /// optionally followed by `, via = Type`
    ///
    /// First element = starting address expression.
    /// Middle elements = intermediate `read_i32` dereferences.
    /// Last numeric element = the final read offset.
    PtrChain {
        /// Initial address expression (e.g. `state.addresses.rulesets`)
        base: Expr,
        /// All offsets: intermediates are read_i32 derefs, last is the read offset.
        /// Must have at least one element (the final read offset).
        offsets: Vec<Expr>,
        via: Option<Type>,
    },

    /// `#[computed(expr)]`
    ///
    /// Evaluates `expr` which may reference fields defined above this one.
    Computed(Expr),

    /// `#[skip]`
    ///
    /// Uses `Default::default()` for this field.
    Skip,
}
