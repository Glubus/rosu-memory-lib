use super::types::type_to_read_fn;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Result, Type};

/// Generate the individual `p.read_TYPE(addr + offset)?` expression,
/// applying a `From` conversion if `via` is set.
pub fn individual_read_expr(
    field_ty: &Type,
    via: &Option<Type>,
    addr_ts: &TokenStream,
    offset_ts: &TokenStream,
) -> Result<TokenStream> {
    let effective = via.as_ref().unwrap_or(field_ty);

    if let Some(read_fn) = type_to_read_fn(effective) {
        let ts = if via.is_some() {
            quote! { #field_ty::from(#addr_ts.#read_fn(#offset_ts)?) }
        } else {
            quote! { #addr_ts.#read_fn(#offset_ts)? }
        };
        Ok(ts)
    } else {
        Err(syn::Error::new_spanned(
            field_ty,
            "cannot infer read function — use `via = T` for From<T> conversions, \
             or `#[nested(offset)]` for ReadMemory types",
        ))
    }
}
