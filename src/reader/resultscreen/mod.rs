pub mod common;
pub mod stable;
use crate::reader::common::GameMode;
use crate::reader::common::OsuType;
use crate::reader::resultscreen::common::ResultScreenInfo;
use crate::reader::structs::Hit;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::Process;

pub struct ResultScreenReader<'a> {
    pub process: &'a Process,
    pub state: &'a mut State,
    pub osu_type: OsuType,
}

impl<'a> ResultScreenReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuType) -> Self {
        Self {
            process: p,
            state,
            osu_type,
        }
    }

    pub fn get_result_screen_info(&mut self) -> Result<ResultScreenInfo, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_screen_info(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_result_username(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_username(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_result_score(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_score(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_result_mode(&mut self) -> Result<GameMode, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_mode(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_result_hit_300(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hit_300(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_result_hit_100(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hit_100(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_result_hit_50(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hit_50(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_result_hit_miss(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hit_miss(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_result_hit_geki(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hit_geki(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_result_hit_katu(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hit_katu(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_result_hits(&mut self) -> Result<Hit, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hits(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_result_accuracy(&mut self) -> Result<f64, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_accuracy(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_result_max_combo(&mut self) -> Result<i16, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_max_combo(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }
}
