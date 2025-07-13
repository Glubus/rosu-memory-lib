pub mod common;
pub mod stable;

use crate::impl_osu_accessor;
use crate::reader::common::OsuClientKind;
use crate::reader::gameplay::common::GameplayInfo;
use crate::reader::structs::Hit;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::Process;
pub struct GameplayReader<'a> {
    pub process: &'a Process,
    pub state: &'a mut State,
    pub osu_type: OsuClientKind,
}

impl<'a> GameplayReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuClientKind) -> Self {
        Self {
            process: p,
            state,
            osu_type,
        }
    }
    impl_osu_accessor! {
        fn score() -> i32 => stable::memory::score,
        fn mods() -> u32 => stable::memory::mods,
        fn combo() -> i16 => stable::memory::combo,
        fn max_combo() -> i16 => stable::memory::max_combo,
        fn hp() -> f64 => stable::memory::hp,
        fn username() -> String => stable::memory::username,
        fn game_time() -> i32 => stable::memory::game_time,
        fn retries() -> i32 => stable::memory::retries,
        fn hits() -> Hit => stable::memory::hits,
        fn hits_300() -> i16 => stable::memory::hits_300,
        fn hits_100() -> i16 => stable::memory::hits_100,
        fn hits_50() -> i16 => stable::memory::hits_50,
        fn hits_miss() -> i16 => stable::memory::hits_miss,
        fn hits_geki() -> i16 => stable::memory::hits_geki,
        fn hits_katu() -> i16 => stable::memory::hits_katu,
        fn info() -> GameplayInfo => stable::memory::info,

    }
}
