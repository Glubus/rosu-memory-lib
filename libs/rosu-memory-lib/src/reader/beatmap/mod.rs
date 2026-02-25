pub mod stable;

use crate::impl_osu_accessor;
use crate::reader::beatmap::stable::BeatmapInfo;
use crate::reader::common::OsuClientKind;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::Process;

/// Reader for beatmap information
pub struct BeatmapReader<'a> {
    pub process: &'a Process,
    pub state: &'a mut State,
    pub osu_type: OsuClientKind,
}

impl<'a> BeatmapReader<'a> {
    pub fn new(
        p: &'a Process,
        state: &'a mut State,
        osu_type: OsuClientKind,
    ) -> Result<Self, Error> {
        Ok(Self {
            process: p,
            state,
            osu_type,
        })
    }

    impl_osu_accessor! {
        fn info() -> BeatmapInfo => stable::memory::info,
    }
}
