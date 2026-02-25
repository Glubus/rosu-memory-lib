use super::field_attr::FieldAttr;
use syn::{parse::ParseStream, Expr, Ident, Result, Token, Type};

/// Parse the contents of `#[offset(...)]`
pub fn parse_offset_attr(input: ParseStream) -> Result<FieldAttr> {
    let expr: Expr = input.parse()?;
    let mut via: Option<Type> = None;

    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
        let key: Ident = input.parse()?;
        if key != "via" {
            return Err(syn::Error::new(key.span(), "expected `via` after offset"));
        }
        input.parse::<Token![=]>()?;
        via = Some(input.parse::<Type>()?);
    }
    Ok(FieldAttr::Offset { expr, via })
}

/// Parse the contents of `#[nested(...)]`
pub fn parse_nested_attr(input: ParseStream) -> Result<FieldAttr> {
    let expr: Expr = input.parse()?;
    Ok(FieldAttr::Nested(expr))
}

/// Parse the contents of `#[computed(...)]`
pub fn parse_computed_attr(input: ParseStream) -> Result<FieldAttr> {
    let expr: Expr = input.parse()?;
    Ok(FieldAttr::Computed(expr))
}
