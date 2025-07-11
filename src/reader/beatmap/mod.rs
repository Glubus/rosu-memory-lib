pub mod common;
pub mod stable;

use crate::reader::beatmap::common::BeatmapInfo;
use crate::reader::beatmap::common::BeatmapStarRating;
use crate::reader::beatmap::common::BeatmapStats;
use crate::reader::beatmap::common::BeatmapStatus;
use crate::reader::common::GameMode;
use crate::reader::common::OsuType;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::Process;

pub struct BeatmapReader<'a> {
    pub process: &'a Process,
    pub state: &'a mut State,
    pub osu_type: OsuType,
}

impl<'a> BeatmapReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuType) -> Result<Self, Error> {
        Ok(Self {
            process: p,
            state,
            osu_type,
        })
    }

    pub fn get_beatmap_info(&mut self) -> Result<BeatmapInfo, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_info(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_path(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuType::Stable => stable::file::get_beatmap_path(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_audio_path(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuType::Stable => stable::file::get_audio_path(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_md5(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_md5(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_id(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_id(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_set_id(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_set_id(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_mode(&mut self) -> Result<GameMode, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_mode(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_tags(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_tags(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_length(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_length(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_drain_time(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_drain_time(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_status(&mut self) -> Result<BeatmapStatus, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_status(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_author(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_author(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_creator(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_creator(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_title_romanized(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_title_romanized(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_title_original(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_title_original(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_difficulty(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_difficulty(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_od(&mut self) -> Result<f32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_od(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_ar(&mut self) -> Result<f32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_ar(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_cs(&mut self) -> Result<f32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_cs(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_hp(&mut self) -> Result<f32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_hp(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_object_count(&mut self) -> Result<u32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_object_count(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_slider_count(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_slider_count(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_star_rating(&mut self) -> Result<BeatmapStarRating, Error> {
        match self.osu_type {
            OsuType::Stable => stable::file::get_beatmap_star_rating(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_beatmap_stats(&mut self) -> Result<BeatmapStats, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_stats(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }
}
