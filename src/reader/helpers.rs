use crate::reader::common::GameMode;
use crate::reader::structs::Hit;
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
generate_reader_fn!(read_i64, i64, read_i64);
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
// macro to gen wrappers for reader
#[macro_export]
macro_rules! impl_osu_accessor {
    ($(fn $name:ident() -> $ret:ty => $call:path),* $(,)?) => {
        $(
            pub fn $name(&mut self) -> Result<$ret, Error> {
                match self.osu_type {
                    OsuClientKind::Stable => $call(self.process, self.state),
                    _ => Err(Error::Unsupported(
                        "Unsupported osu type for now".to_string(),
                    )),
                }
            }
        )*
    };
}

// TODO : idk where to put this
#[inline]
pub fn calculate_accuracy(gamemode: &GameMode, hit: &Hit) -> Result<f64, Error> {
    let acc = match gamemode {
        GameMode::Osu => {
            let total = (hit._300 + hit._100 + hit._50 + hit._miss) as f64;
            if total == 0.0 {
                return Ok(0.0);
            }
            let score = hit._300 as f64 * 6.0 + hit._100 as f64 * 2.0 + hit._50 as f64;
            (score / (total * 6.0)) * 100.0
        }
        GameMode::Taiko => {
            let total = (hit._300 + hit._100 + hit._50 + hit._miss) as f64;
            if total == 0.0 {
                return Ok(0.0);
            }
            let score = hit._300 as f64 * 2.0 + hit._100 as f64;
            (score / (total * 2.0)) * 100.0
        }
        GameMode::Catch => {
            let caught = (hit._300 + hit._100 + hit._50) as f64;
            let total = (hit._300 + hit._100 + hit._50 + hit._katu + hit._miss) as f64;
            if total == 0.0 {
                return Ok(0.0);
            }
            (caught / total) * 100.0
        }
        GameMode::Mania => {
            let total = (hit._geki + hit._300 + hit._katu + hit._100 + hit._50 + hit._miss) as f64;
            if total == 0.0 {
                return Ok(0.0);
            }
            let score = (hit._geki + hit._300) as f64 * 6.0
                + hit._katu as f64 * 4.0
                + hit._100 as f64 * 2.0
                + hit._50 as f64;
            (score / (total * 6.0)) * 100.0
        }
        _ => return Ok(0.0),
    };

    Ok(acc)
}
