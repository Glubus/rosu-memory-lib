pub mod common;
pub mod stable;

use std::path::PathBuf;

use crate::impl_osu_accessor;
use crate::reader::beatmap::common::BeatmapInfo;
use crate::reader::beatmap::common::BeatmapStarRating;
use crate::reader::beatmap::common::BeatmapStats;
use crate::reader::beatmap::common::BeatmapStatus;
use crate::reader::common::GameMode;
use crate::reader::common::OsuClientKind;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::Process;

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
        fn id() -> i32 => stable::memory::id,
        fn set_id() -> i32 => stable::memory::set_id,
        fn tags() -> String => stable::memory::tags,
        fn length() -> i32 => stable::memory::length,
        fn drain_time() -> i32 => stable::memory::drain_time,
        fn author() -> String => stable::memory::author,
        fn creator() -> String => stable::memory::creator,
        fn md5() -> String => stable::memory::md5,
        fn title_romanized() -> String => stable::memory::title_romanized,
        fn title() -> String => stable::memory::title,
        fn difficulty() -> String => stable::memory::difficulty,
        fn od() -> f32 => stable::memory::od,
        fn ar() -> f32 => stable::memory::ar,
        fn cs() -> f32 => stable::memory::cs,
        fn hp() -> f32 => stable::memory::hp,
        fn object_count() -> u32 => stable::memory::object_count,
        fn slider_count() -> i32 => stable::memory::slider_count,
        fn folder() -> String => stable::memory::folder,
        fn filename() -> String => stable::memory::filename,
        fn audio() -> String => stable::memory::audio,
        fn cover() -> String => stable::memory::cover,
        fn mode() -> GameMode => stable::memory::mode,
        fn status() -> BeatmapStatus => stable::memory::status,
        fn info() -> BeatmapInfo => stable::memory::info,
        fn stats() -> BeatmapStats => stable::memory::stats,
        fn path() -> PathBuf => stable::file::path,
        fn audio_path() -> PathBuf => stable::file::audio_path,
        fn star_rating() -> BeatmapStarRating => stable::file::star_rating,
    }
}
