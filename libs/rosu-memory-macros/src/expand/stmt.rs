use crate::attr::FieldAttr;
use crate::utils::individual_read_expr;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Result, Type};

pub fn gen_field_stmt(name: &Ident, ty: &Type, fa: Option<FieldAttr>) -> Result<TokenStream> {
    match fa {
        Some(FieldAttr::Offset { expr, via }) => {
            let rexpr =
                individual_read_expr(ty, &via, &quote! { p }, &quote! { __base + (#expr) })?;
            Ok(quote! { let #name: #ty = #rexpr; })
        }
        Some(FieldAttr::Nested(expr)) => Ok(quote! {
            let #name: #ty = #ty::read_from_memory(p, state, __base + (#expr))?;
        }),
        Some(FieldAttr::PtrChain { base, offsets, via }) => {
            let (derefs, tail) = offsets.split_at(offsets.len() - 1);
            let deref_steps = derefs
                .iter()
                .map(|off| quote! { __pc_addr = p.read_i32(__pc_addr + (#off))?; });
            let foff = &tail[0];
            let rexpr =
                individual_read_expr(ty, &via, &quote! { p }, &quote! { __pc_addr + (#foff) })?;
            Ok(quote! {
                let #name: #ty = {
                    let mut __pc_addr: i32 = #base;
                    #( #deref_steps )*
                    #rexpr
                };
            })
        }
        Some(FieldAttr::Computed(expr)) => Ok(quote! { let #name: #ty = #expr; }),
        Some(FieldAttr::Skip) => Ok(quote! { let #name: #ty = Default::default(); }),
        None => Err(syn::Error::new_spanned(
            name,
            "field needs a ReadMemory attribute",
        )),
    }
}
