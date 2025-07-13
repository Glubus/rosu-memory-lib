pub mod common;
pub mod stable;
use crate::impl_osu_accessor;
use crate::reader::common::OsuClientKind;
use crate::reader::structs::State;
use crate::reader::user::common::UserInfo;
use crate::Error;
use rosu_mem::process::Process;
pub struct UserReader<'a> {
    pub process: &'a Process,
    pub state: &'a mut State,
    pub osu_type: OsuClientKind,
}

impl<'a> UserReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuClientKind) -> Self {
        Self {
            process: p,
            state,
            osu_type,
        }
    }
    impl_osu_accessor! {
        fn id() -> i32 => stable::memory::id,
        fn bancho_status() -> i32 => stable::memory::bancho_status,
        fn country_code() -> i32 => stable::memory::country_code,
        fn username() -> String => stable::memory::username,
        fn pp() -> i32 => stable::memory::pp,
        fn rankedscore() -> i64 => stable::memory::rankedscore,
        fn level() -> f32 => stable::memory::level,
        fn playcount() -> i32 => stable::memory::playcount,
        fn rank() -> i32 => stable::memory::rank,
        fn playmode() -> i32 => stable::memory::playmode,
        fn accuracy() -> f64 => stable::memory::accuracy,
        fn info() -> UserInfo => stable::memory::info,
    }
}
