pub mod common;
pub mod stable;
use crate::reader::common::OsuType;
use crate::reader::structs::State;
use crate::reader::user::common::UserInfo;
use crate::Error;
use rosu_mem::process::Process;

pub struct UserReader<'a> {
    pub process: &'a Process,
    pub state: &'a mut State,
    pub osu_type: OsuType,
}

impl<'a> UserReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuType) -> Self {
        Self {
            process: p,
            state,
            osu_type,
        }
    }

    pub fn get_user_info(&mut self) -> Result<UserInfo, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_info(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_user_id(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_id(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_user_bancho_status(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_bancho_status(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_user_country_code(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_country_code(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_user_username(&mut self) -> Result<String, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_username(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_user_pp(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_pp(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_user_rankedscore(&mut self) -> Result<i64, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_rankedscore(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_user_level(&mut self) -> Result<f32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_level(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_user_playcount(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_playcount(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_user_rank(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_rank(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_user_playmode(&mut self) -> Result<i32, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_playmode(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }

    pub fn get_user_accuracy(&mut self) -> Result<f64, Error> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_accuracy(self.process, self.state),
            _ => Err(Error::Unsupported(
                "Unsupported osu type for now".to_string(),
            )),
        }
    }
}
