pub mod batch;
pub mod fields;
pub mod stmt;
pub mod struct_setup;

use crate::attr::ReadMemoryStructAttr;
use fields::process_fields;
use proc_macro2::TokenStream;
use quote::quote;
use struct_setup::{extract_named_fields, gen_base_setup, gen_guard_block};
use syn::{DeriveInput, Result};

pub fn derive_read_memory(input: DeriveInput) -> Result<TokenStream> {
    let struct_name = &input.ident;
    let struct_attr = parse_struct_attr(&input.attrs)?;
    let has_base = struct_attr.base.is_some();

    let base_setup = gen_base_setup(&struct_attr);
    let guard_block = gen_guard_block(&struct_attr.guard);

    let fields_iter = extract_named_fields(&input.data, struct_name)?;
    let (read_stmts, ctor_fields) = process_fields(fields_iter)?;

    let fn_sig = gen_fn_sig(has_base);

    Ok(quote! {
        impl #struct_name {
            #fn_sig {
                use rosu_mem::process::ProcessTraits;

                #guard_block
                #base_setup
                #( #read_stmts )*

                Ok(Self {
                    #( #ctor_fields )*
                })
            }
        }
    })
}

fn parse_struct_attr(attrs: &[syn::Attribute]) -> Result<ReadMemoryStructAttr> {
    for attr in attrs {
        if attr.path().is_ident("read_memory") {
            return attr.parse_args::<ReadMemoryStructAttr>();
        }
    }
    Ok(ReadMemoryStructAttr::default())
}

fn gen_fn_sig(has_base: bool) -> TokenStream {
    if has_base {
        quote! {
            pub fn read_from_memory(
                p: &rosu_mem::process::Process,
                state: &mut crate::reader::structs::State,
            ) -> Result<Self, crate::Error>
        }
    } else {
        quote! {
            pub fn read_from_memory(
                p: &rosu_mem::process::Process,
                state: &mut crate::reader::structs::State,
                base: i32,
            ) -> Result<Self, crate::Error>
        }
    }
}
