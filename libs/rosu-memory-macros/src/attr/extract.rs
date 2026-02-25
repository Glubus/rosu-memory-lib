use super::field_attr::FieldAttr;
use super::parse::{parse_computed_attr, parse_nested_attr, parse_offset_attr};
use super::parse_ptr::parse_ptr_chain_attr;
use syn::Result;

/// Extract the first recognised field attribute from a field's attrs
pub fn extract_field_attr(attrs: &[syn::Attribute]) -> Result<Option<FieldAttr>> {
    for attr in attrs {
        let path = attr.path();
        if path.is_ident("offset") {
            let fa = attr.parse_args_with(parse_offset_attr)?;
            return Ok(Some(fa));
        } else if path.is_ident("nested") {
            let fa = attr.parse_args_with(parse_nested_attr)?;
            return Ok(Some(fa));
        } else if path.is_ident("ptr_chain") {
            let fa = attr.parse_args_with(parse_ptr_chain_attr)?;
            return Ok(Some(fa));
        } else if path.is_ident("computed") {
            let fa = attr.parse_args_with(parse_computed_attr)?;
            return Ok(Some(fa));
        } else if path.is_ident("skip") {
            return Ok(Some(FieldAttr::Skip));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_parse_offset_attr() {
        let attr: syn::Attribute = parse_quote!(#[offset(0x10)]);
        let fa = extract_field_attr(&[attr]).unwrap().unwrap();
        match fa {
            FieldAttr::Offset { via, .. } => {
                assert!(via.is_none());
            }
            _ => panic!("Expected Offset"),
        }
    }

    #[test]
    fn test_parse_nested_attr() {
        let attr: syn::Attribute = parse_quote!(#[nested(0x20)]);
        let fa = extract_field_attr(&[attr]).unwrap().unwrap();
        match fa {
            FieldAttr::Nested(_) => {}
            _ => panic!("Expected Nested"),
        }
    }
}
