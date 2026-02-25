use syn::{Expr, ExprLit, Lit};

/// Try to extract a plain integer value from an expression.
/// Only succeeds for bare integer literals (e.g., `0x28`, `100`).
pub fn expr_as_u64(expr: &Expr) -> Option<u64> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Int(li), ..
    }) = expr
    {
        li.base10_parse::<u64>().ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_expr_as_u64_valid() {
        let expr: Expr = parse_quote!(0x28);
        assert_eq!(expr_as_u64(&expr), Some(0x28));

        let expr: Expr = parse_quote!(100);
        assert_eq!(expr_as_u64(&expr), Some(100));
    }

    #[test]
    fn test_expr_as_u64_invalid() {
        let expr: Expr = parse_quote!("100"); // string
        assert_eq!(expr_as_u64(&expr), None);

        let expr: Expr = parse_quote!(1 + 1); // math
        assert_eq!(expr_as_u64(&expr), None);
    }
}
