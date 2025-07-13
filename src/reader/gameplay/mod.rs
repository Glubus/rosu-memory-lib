pub mod common;
pub mod stable;

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

    pub fn score(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::score(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn mods(&mut self) -> Result<u32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::mods(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn combo(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::combo(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn max_combo(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::max_combo(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn hp(&mut self) -> Result<f64, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::hp(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn username(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::username(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn game_time(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => crate::reader::common::stable::memory::game_time(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn retries(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::retries(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn hits_300(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::hits_300(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn hits_100(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::hits_100(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn hits_50(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::hits_50(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn hits_miss(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::hits_miss(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn hits_geki(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::hits_geki(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn hits_katu(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::hits_katu(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn hits(&mut self) -> Result<Hit, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::hits(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn gameplay_info(&mut self) -> Result<GameplayInfo, Error> {
        match self.osu_type {
            OsuClientKind::Stable => stable::memory::info(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }
}
