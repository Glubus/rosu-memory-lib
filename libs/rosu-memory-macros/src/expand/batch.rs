use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Type;

/// One field that qualifies for inclusion in a batch read.
pub struct BatchCandidate {
    pub name: Ident,
    pub field_ty: Type,
    pub raw_ty: Type,
    pub offset: u64,
    pub size: usize,
    pub needs_from: bool,
}

fn gen_field_read(c: &BatchCandidate, min_off: u64, buf_name: &Ident) -> TokenStream {
    let name = &c.name;
    let field_ty = &c.field_ty;
    let raw_ty = &c.raw_ty;
    let rel_start = (c.offset - min_off) as usize;
    let rel_end = rel_start + c.size;

    if c.needs_from {
        quote! {
            let #name: #field_ty = #field_ty::from(
                #raw_ty::from_le_bytes(#buf_name[#rel_start..#rel_end].try_into().unwrap())
            );
        }
    } else {
        quote! {
            let #name: #field_ty = #field_ty::from_le_bytes(
                #buf_name[#rel_start..#rel_end].try_into().unwrap()
            );
        }
    }
}

/// Generate a single batch-read block for a group of `BatchCandidate`s.
pub fn gen_batch_block(candidates: &[BatchCandidate], batch_idx: usize) -> TokenStream {
    let min_off = candidates.iter().map(|c| c.offset).min().unwrap();
    let max_end = candidates
        .iter()
        .map(|c| c.offset + c.size as u64)
        .max()
        .unwrap();
    let buf_size = (max_end - min_off) as usize;
    let buf_name = format_ident!("__batch_{}", batch_idx);
    let min_off_i32 = min_off as i32;

    let field_reads: Vec<TokenStream> = candidates
        .iter()
        .map(|c| gen_field_read(c, min_off, &buf_name))
        .collect();

    quote! {
        let mut #buf_name = [0u8; #buf_size];
        p.read(__base + #min_off_i32, #buf_size, &mut #buf_name)?;
        #( #field_reads )*
    }
}
