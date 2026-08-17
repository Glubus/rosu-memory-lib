use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Ident, Result, Token,
};

/// Everything parsed from `#[read_memory(...)]` on the struct itself.
#[derive(Default)]
pub struct ReadMemoryStructAttr {
    /// Each element becomes `addr = p.read_i32(addr + element)?`
    pub chain: Vec<Expr>,
    /// Optional `GameState` variant ident used to generate a guard check.
    pub guard: Option<Ident>,
    /// Base address initialization expression for generating .read() method.
    /// If provided, generates a .read(p, state) method that computes the base.
    pub init_base: Option<Expr>,
}

impl Parse for ReadMemoryStructAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut chain: Vec<Expr> = vec![];
        let mut guard: Option<Ident> = None;
        let mut init_base: Option<Expr> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "chain" => {
                    let content;
                    bracketed!(content in input);
                    let exprs = Punctuated::<Expr, Token![,]>::parse_terminated(&content)?;
                    chain = exprs.into_iter().collect();
                }
                "guard" => guard = Some(input.parse::<Ident>()?),
                "init_base" => init_base = Some(input.parse::<Expr>()?),
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("unknown key `{other}`, expected `chain`, `guard`, or `init_base`"),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(ReadMemoryStructAttr {
            chain,
            guard,
            init_base,
        })
    }
}
