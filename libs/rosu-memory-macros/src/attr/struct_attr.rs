use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Ident, Result, Token,
};

/// Everything parsed from `#[read_memory(...)]` on the struct itself.
pub struct ReadMemoryStructAttr {
    /// Starting address expression (may reference `p` and `state`).
    /// If `None`, the generated function receives `base: i32` as a parameter.
    pub base: Option<Expr>,
    /// Each element becomes `addr = p.read_i32(addr + element)?`
    pub chain: Vec<Expr>,
    /// Optional `GameState` variant ident used to generate a guard check.
    pub guard: Option<Ident>,
}

impl Default for ReadMemoryStructAttr {
    fn default() -> Self {
        Self {
            base: None,
            chain: vec![],
            guard: None,
        }
    }
}

impl Parse for ReadMemoryStructAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut base: Option<Expr> = None;
        let mut chain: Vec<Expr> = vec![];
        let mut guard: Option<Ident> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "base" => base = Some(input.parse::<Expr>()?),
                "chain" => {
                    let content;
                    bracketed!(content in input);
                    let exprs = Punctuated::<Expr, Token![,]>::parse_terminated(&content)?;
                    chain = exprs.into_iter().collect();
                }
                "guard" => guard = Some(input.parse::<Ident>()?),
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("unknown key `{other}`, expected `base`, `chain`, or `guard`"),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(ReadMemoryStructAttr { base, chain, guard })
    }
}
