use super::field_attr::FieldAttr;
use syn::{parse::ParseStream, Expr, Ident, Result, Token, Type};

/// Parse the contents of `#[ptr_chain(...)]`
/// Syntax: `base_expr, offset1, ..., final_offset [, via = Type]`
pub fn parse_ptr_chain_attr(input: ParseStream) -> Result<FieldAttr> {
    let base: Expr = input.parse()?;
    let mut offsets: Vec<Expr> = vec![];
    let mut via: Option<Type> = None;

    while input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
        if input.is_empty() {
            break;
        }

        if input.peek(Ident) {
            let fork = input.fork();
            let maybe_via: Ident = fork.parse()?;
            if maybe_via == "via" && fork.peek(Token![=]) {
                input.parse::<Ident>()?; // "via"
                input.parse::<Token![=]>()?;
                via = Some(input.parse::<Type>()?);
                break;
            }
        }
        offsets.push(input.parse::<Expr>()?);
    }

    if offsets.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "ptr_chain requires at least one offset",
        ));
    }
    Ok(FieldAttr::PtrChain { base, offsets, via })
}
