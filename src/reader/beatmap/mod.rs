pub mod common;
pub mod stable;

use std::path::PathBuf;

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

    pub fn info(&mut self) -> Result<BeatmapInfo, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::info(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }
    pub fn cover(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::cover(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn path(&mut self) -> Result<PathBuf, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::file::path(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn folder(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::folder(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn filename(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::filename(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn audio(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::audio(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn audio_path(&mut self) -> Result<PathBuf, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::file::audio_path(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn md5(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::md5(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn id(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::id(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn set_id(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::set_id(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn mode(&mut self) -> Result<GameMode, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::mode(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn tags(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::tags(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn length(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::length(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn drain_time(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => {
                stable::memory::drain_time(self.process, self.state)
            }
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn status(&mut self) -> Result<BeatmapStatus, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::status(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn author(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::author(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn creator(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::creator(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn title_romanized(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::title_romanized(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn title(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::title(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn difficulty(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::difficulty(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn od(&mut self) -> Result<f32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::od(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn ar(&mut self) -> Result<f32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::ar(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn cs(&mut self) -> Result<f32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::cs(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn hp(&mut self) -> Result<f32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::hp(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn object_count(&mut self) -> Result<u32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => {
                stable::memory::object_count(self.process, self.state)
            }
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn slider_count(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => {
                stable::memory::slider_count(self.process, self.state)
            }
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn star_rating(&mut self) -> Result<BeatmapStarRating, Error> {
        match self.osu_type {
            OsuClientKind::Stable => {
                stable::file::star_rating(self.process, self.state)
            }
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn stats(&mut self) -> Result<BeatmapStats, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::stats(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }
}
