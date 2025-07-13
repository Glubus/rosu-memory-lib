use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};

macro_rules! generate_reader_fn {
    (
        $name:ident, $ret_ty:ty, $read_fn:ident
    ) => {
        pub(crate) fn $name(
            p: &Process,
            state: &mut State,
            offset: i32,
            get_base_addr: fn(&Process, &mut State) -> Result<i32, Error>,
        ) -> Result<$ret_ty, Error> {
            let base_addr = get_base_addr(p, state)?;
            Ok(p.$read_fn(base_addr + offset)?)
        }
    };
}
generate_reader_fn!(read_string, String, read_string);
generate_reader_fn!(read_i16, i16, read_i16);
generate_reader_fn!(read_i32, i32, read_i32);
generate_reader_fn!(read_u32, u32, read_u32);
generate_reader_fn!(read_u64, u64, read_u64);
generate_reader_fn!(read_f32, f32, read_f32);
generate_reader_fn!(read_f64, f64, read_f64);
#[macro_export]
macro_rules! generate_offset_getter {
    (
        $( $fn_name:ident : $ret_ty:ty = $read_fn:ident ( $offset:expr , $get_base:ident ); )*
    ) => {
        $(
            pub fn $fn_name(p: &Process, state: &mut State) -> Result<$ret_ty, Error> {
                Ok(<$ret_ty>::from($read_fn(p, state, $offset, $get_base)?))
            }
        )*
    };
}