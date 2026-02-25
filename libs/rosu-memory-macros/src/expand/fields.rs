use crate::attr::{extract_field_attr, FieldAttr};
use crate::expand::batch::{gen_batch_block, BatchCandidate};
use crate::expand::stmt::gen_field_stmt;
use crate::utils::{expr_as_u64, primitive_byte_size, type_to_read_fn};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Field, Result, Type};

pub fn process_fields<'a>(
    fields: impl Iterator<Item = &'a Field>,
) -> Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let mut read_stmts = vec![];
    let mut ctor_fields = vec![];
    let mut batch_run = vec![];

    for field in fields {
        let field_name = field.ident.as_ref().expect("named field");
        let fa = extract_field_attr(&field.attrs)?;

        if let Some(candidate) = get_batch_candidate(field_name, &field.ty, &fa) {
            batch_run.push(candidate);
            ctor_fields.push(quote! { #field_name, });
            continue;
        }

        flush_batch(&mut batch_run, &mut read_stmts);
        let stmt = gen_field_stmt(field_name, &field.ty, fa)?;
        read_stmts.push(stmt);
        ctor_fields.push(quote! { #field_name, });
    }
    flush_batch(&mut batch_run, &mut read_stmts);
    Ok((read_stmts, ctor_fields))
}

fn get_batch_candidate(
    name: &Ident,
    field_ty: &Type,
    fa: &Option<FieldAttr>,
) -> Option<BatchCandidate> {
    match fa {
        Some(FieldAttr::Offset { expr, via }) => {
            let effective_ty = via.as_ref().unwrap_or(field_ty);
            let off = expr_as_u64(expr)?;
            let sz = primitive_byte_size(effective_ty)?;
            Some(BatchCandidate {
                name: name.clone(),
                field_ty: field_ty.clone(),
                raw_ty: effective_ty.clone(),
                offset: off,
                size: sz,
                needs_from: via.is_some(),
            })
        }
        _ => None,
    }
}

fn flush_batch(batch_run: &mut Vec<BatchCandidate>, read_stmts: &mut Vec<TokenStream>) {
    if batch_run.is_empty() {
        return;
    }

    if batch_run.len() == 1 {
        let c = batch_run.remove(0);
        let name = &c.name;
        let field_ty = &c.field_ty;
        let off = c.offset as i32;
        let read_ts = if c.needs_from {
            let rfn = type_to_read_fn(&c.raw_ty).unwrap();
            quote! { #field_ty::from(p.#rfn(__base + #off)?) }
        } else {
            let rfn = type_to_read_fn(field_ty).unwrap();
            quote! { p.#rfn(__base + #off)? }
        };
        read_stmts.push(quote! { let #name: #field_ty = #read_ts; });
    } else {
        let idx = read_stmts.len();
        read_stmts.push(gen_batch_block(batch_run, idx));
        batch_run.clear();
    }
}
