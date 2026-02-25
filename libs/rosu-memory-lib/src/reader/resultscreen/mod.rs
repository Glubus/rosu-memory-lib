pub mod common;
pub mod stable;
use crate::impl_osu_accessor;
use crate::reader::common::GameMode;
use crate::reader::common::OsuClientKind;
use crate::reader::resultscreen::common::ResultScreenInfo;
use crate::reader::structs::Hit;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::Process;
pub struct ResultScreenReader<'a> {
    pub process: &'a Process,
    pub state: &'a mut State,
    pub osu_type: OsuClientKind,
}

impl<'a> ResultScreenReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuClientKind) -> Self {
        Self {
            process: p,
            state,
            osu_type,
        }
    }
    impl_osu_accessor! {
        fn username() -> String => stable::memory::username,
        fn score() -> i32 => stable::memory::score,
        fn mode() -> GameMode => stable::memory::mode,
        fn max_combo() -> i16 => stable::memory::max_combo,
        fn hits() -> Hit => stable::memory::hits,
        fn hits_300() -> i16 => stable::memory::hits_300,
        fn hits_100() -> i16 => stable::memory::hits_100,
        fn hits_50() -> i16 => stable::memory::hits_50,
        fn hits_miss() -> i16 => stable::memory::hits_miss,
        fn hits_geki() -> i16 => stable::memory::hits_geki,
        fn hits_katu() -> i16 => stable::memory::hits_katu,
        fn accuracy() -> f64 => stable::memory::accuracy,
        fn info() -> ResultScreenInfo => stable::memory::info,
    }
}
