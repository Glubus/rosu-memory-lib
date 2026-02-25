use quote::format_ident;
use syn::Type;

/// Retrieve the identifier string of a given Type, if it's a simple path.
pub fn type_name(ty: &Type) -> Option<String> {
    if let Type::Path(tp) = ty {
        tp.path.segments.last().map(|s| s.ident.to_string())
    } else {
        None
    }
}

/// Returns the byte size of a primitive type known to the macro.
/// Returns `None` for `String`, nested structs, or unrecognised types.
pub fn primitive_byte_size(ty: &Type) -> Option<usize> {
    match type_name(ty)?.as_str() {
        "i8" | "u8" => Some(1),
        "i16" | "u16" => Some(2),
        "i32" | "u32" | "f32" => Some(4),
        "i64" | "u64" | "f64" => Some(8),
        _ => None,
    }
}

/// Maps a primitive type name to the corresponding `Process::read_*` method identifier.
pub fn type_to_read_fn(ty: &Type) -> Option<proc_macro2::Ident> {
    let method = match type_name(ty)?.as_str() {
        "i8" => "read_i8",
        "u8" => "read_u8",
        "i16" => "read_i16",
        "u16" => "read_u16",
        "i32" => "read_i32",
        "u32" => "read_u32",
        "i64" => "read_i64",
        "u64" => "read_u64",
        "f32" => "read_f32",
        "f64" => "read_f64",
        "String" => "read_string",
        _ => return None,
    };
    Some(format_ident!("{}", method))
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_primitive_byte_size() {
        let ty_i32: Type = parse_quote!(i32);
        assert_eq!(primitive_byte_size(&ty_i32), Some(4));

        let ty_string: Type = parse_quote!(String);
        assert_eq!(primitive_byte_size(&ty_string), None);
    }

    #[test]
    fn test_type_to_read_fn() {
        let ty_u64: Type = parse_quote!(u64);
        let ident = type_to_read_fn(&ty_u64).unwrap();
        assert_eq!(ident.to_string(), "read_u64");

        let ty_string: Type = parse_quote!(String);
        let ident = type_to_read_fn(&ty_string).unwrap();
        assert_eq!(ident.to_string(), "read_string");
    }
}
