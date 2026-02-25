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

    let base_setup = gen_base_setup(&struct_attr);
    let guard_block = gen_guard_block(&struct_attr.guard);

    let fields_iter = extract_named_fields(&input.data, struct_name)?;
    let (read_stmts, ctor_fields) = process_fields(fields_iter)?;

    let fn_sig = gen_fn_sig();

    // Generate .read() method if init_base is present
    let read_method = if let Some(init_base_expr) = &struct_attr.init_base {
        quote! {
            pub fn read(
                p: &rosu_mem::process::Process,
                state: &mut crate::reader::structs::State,
            ) -> Result<Self, crate::Error> {
                let base = #init_base_expr;
                Self::read_from_memory(p, state, base)
            }
        }
    } else {
        quote! {}
    };

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

            #read_method
        }
    })
}

fn parse_struct_attr(attrs: &[syn::Attribute]) -> Result<ReadMemoryStructAttr> {
    let mut result = ReadMemoryStructAttr::default();

    for attr in attrs {
        if attr.path().is_ident("read_memory") {
            result = attr.parse_args::<ReadMemoryStructAttr>()?;
        } else if attr.path().is_ident("init_base") {
            result.init_base = Some(attr.parse_args::<syn::Expr>()?);
        }
    }

    Ok(result)
}

fn gen_fn_sig() -> TokenStream {
    quote! {
        pub fn read_from_memory(
            p: &rosu_mem::process::Process,
            state: &mut crate::reader::structs::State,
            base: i32,
        ) -> Result<Self, crate::Error>
    }
}
