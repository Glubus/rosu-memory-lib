use crate::attr::ReadMemoryStructAttr;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, Fields, Result};

pub fn gen_base_setup(attr: &ReadMemoryStructAttr) -> TokenStream {
    let chain_steps = attr.chain.iter().map(|step| {
        quote! { __base = p.read_i32(__base + (#step))?; }
    });
    quote! {
        let mut __base: i32 = base;
        #( #chain_steps )*
    }
}

pub fn gen_guard_block(guard: &Option<Ident>) -> TokenStream {
    if let Some(guard_ident) = guard {
        let msg = format!("Not in {} state", guard_ident);
        quote! {
            if !crate::reader::common::stable::memory::check_game_state(
                p,
                state,
                crate::reader::common::GameState::#guard_ident,
            )? {
                return Err(crate::Error::NotAvailable(#msg.to_string()));
            }
        }
    } else {
        quote! {}
    }
}

pub fn extract_named_fields<'a>(
    data: &'a Data,
    name: &Ident,
) -> Result<impl Iterator<Item = &'a syn::Field>> {
    match data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(f) => Ok(f.named.iter()),
            _ => Err(syn::Error::new_spanned(
                name,
                "ReadMemory only supports structs with named fields",
            )),
        },
        _ => Err(syn::Error::new_spanned(
            name,
            "ReadMemory can only be derived for structs",
        )),
    }
}
