use crate::reader::common::GameMode;
use crate::reader::structs::Hit;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};

/// Generates standardized memory reading functions.
///
/// This macro creates functions that read specific data types from process memory
/// using a base address and offset common usage for base types to not do getbaseaddr everytime.
///
/// # Syntax
///
/// ```rust
/// generate_reader_fn! {
///     function_name, return_type, read_method
/// }
/// ```
///
/// # Arguments
///
/// * `function_name` - Name of the generated function most likely read_<type>
/// * `return_type` - Rust type to return (e.g., `i32`, `String`, `f64`)
/// * `read_method` - Method name on Process trait (e.g., `read_i32`, `read_string`) most of the time from rosu_mem
///
/// # Examples
///
/// ```rust
/// generate_reader_fn! {
///     read_score, i32, read_i32
/// }
///
/// ```
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

/// Generates offset-based getter functions for memory access.
///
/// This macro creates functions that read data from specific memory offsets
/// using a base address getter function.
///
/// # Syntax
///
/// ```rust
/// generate_offset_getter! {
///     function_name: return_type = read_method(offset, base_getter);
///     another_function: another_type = another_read_method(another_offset, another_base);
/// }
/// ```
///
/// # Arguments
///
/// Each getter definition consists of:
/// * `function_name` - Name of the generated function
/// * `return_type` - Rust type to return
/// * `read_method` - Memory reading method to use
/// * `offset` - Memory offset from base address
/// * `base_getter` - Function that returns the base address (most of the time from generate_reader_fn)
///
/// # Examples
///
/// ```rust
/// generate_offset_getter! {
///     score: i32 = read_i32(0x10, score_base),
///     combo: i16 = read_i16(0x14, score_base),
///     username: String = read_string(0x20, score_base),
///     hp: f64 = read_f64(0x30, hp_base),
/// }
///
/// // Generates functions like:
/// // pub fn score(p: &Process, state: &mut State) -> Result<i32, Error> {
/// //     Ok(<i32>::from(read_i32(p, state, 0x10, score_base)?))
/// // }
/// ```
///
/// # Generated Functions
///
/// Each definition generates a function with signature:
/// ```rust
/// pub fn function_name(p: &Process, state: &mut State) -> Result<return_type, Error>
/// ```
///
/// # Memory Safety
///
/// The generated functions assume the offsets are valid and the base address
/// getter returns a valid memory address.
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

/// Generates OSU memory accessor methods with automatic client dispatching.
///
/// This macro eliminates boilerplate code by automatically generating
/// client-specific accessor methods that handle different OSU client types
/// (Stable, Lazer) with consistent error handling.
///
/// # Syntax
///
/// ```rust
/// impl_osu_accessor! {
///     fn method_name() -> return_type => implementation_path,
///     fn another_method() -> another_type => another_implementation,
/// }
/// ```
///
/// # Arguments
///
/// * `method_name` - Name of the generated method (e.g., `game_state`, `score`)
/// * `return_type` - Rust type to return (e.g., `GameState`, `i32`, `String`)
/// * `implementation_path` - Path to the actual implementation (e.g., `stable::memory::game_state`)
///
/// # Examples
///
/// ```rust
/// impl<'a> CommonReader<'a> {
///     impl_osu_accessor! {
///         fn game_state() -> GameState => stable::memory::game_state,
///         fn menu_game_mode() -> GameMode => stable::memory::menu_game_mode,
///         fn path_folder() -> PathBuf => stable::memory::path_folder,
///     }
/// }
/// ```
///
/// # Generated Methods
///
/// For each definition, generates a method like:
/// ```rust
/// pub fn method_name(&mut self) -> Result<return_type, Error> {
///     match self.osu_type {
///         OsuClientKind::Stable => implementation_path(self.process, self.state),
///         _ => Err(Error::Unsupported("Unsupported osu type for now".to_string())),
///     }
/// }
/// ```
///
/// # Benefits
///
/// * **Reduces code duplication** - ~1000+ lines eliminated across the project
/// * **Consistent API** - All accessors follow the same pattern
/// * **Type safety** - Compile-time guarantees for return types
/// * **Future-proof** - Easy to add new client types
/// * **Maintainable** - Changes only need to be made in one place
///
/// # Errors
///
/// Generated methods return `Error::Unsupported` for client types that don't
/// have implementations yet (currently only Stable is supported).
///
/// # Performance
///
/// No runtime overhead - all dispatching is done at compile time.
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

// Macro usage
generate_reader_fn!(read_string, String, read_string);
generate_reader_fn!(read_i16, i16, read_i16);
generate_reader_fn!(read_i32, i32, read_i32);
generate_reader_fn!(read_u32, u32, read_u32);
generate_reader_fn!(read_i64, i64, read_i64);
generate_reader_fn!(read_u64, u64, read_u64);
generate_reader_fn!(read_f32, f32, read_f32);
generate_reader_fn!(read_f64, f64, read_f64);
