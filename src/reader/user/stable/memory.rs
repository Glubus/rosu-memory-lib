use crate::generate_offset_getter;
use crate::reader::helpers::{read_f32, read_f64, read_i32, read_i64, read_string};
use crate::reader::structs::State;
use crate::reader::user::common::UserInfo;
use crate::reader::user::stable::offset::USER_PROFILE_OFFSET;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};

pub fn user_base(p: &Process, state: &mut State) -> Result<i32, Error> {
    Ok(p.read_i32(p.read_i32(state.addresses.user_profile + USER_PROFILE_OFFSET.ptr)?)?)
}
generate_offset_getter! {
    id: i32 = read_i32(USER_PROFILE_OFFSET.id, user_base);
    bancho_status: i32 = read_i32(USER_PROFILE_OFFSET.bancho_status, user_base);
    country_code: i32 = read_i32(USER_PROFILE_OFFSET.country_code, user_base);
    username: String = read_string(USER_PROFILE_OFFSET.username, user_base);
    pp: i32 = read_i32(USER_PROFILE_OFFSET.pp, user_base);
    rankedscore: i64 = read_i64(USER_PROFILE_OFFSET.rankedscore, user_base);
    level: f32 = read_f32(USER_PROFILE_OFFSET.level, user_base);
    playcount: i32 = read_i32(USER_PROFILE_OFFSET.playcount, user_base);
    rank: i32 = read_i32(USER_PROFILE_OFFSET.rank, user_base);
    playmode: i32 = read_i32(USER_PROFILE_OFFSET.playmode, user_base);
    accuracy: f64 = read_f64(USER_PROFILE_OFFSET.accuracy, user_base);
}

pub fn info(p: &Process, state: &mut State) -> Result<UserInfo, Error> {
    let user_profile_base = user_base(p, state)?;

    let user_profile = UserInfo {
        id: p.read_i32(user_profile_base + USER_PROFILE_OFFSET.id)?,
        username: p.read_string(user_profile_base + USER_PROFILE_OFFSET.username)?, 
        pp: p.read_i32(user_profile_base + USER_PROFILE_OFFSET.pp)?,
        rankedscore: p.read_i64(user_profile_base + USER_PROFILE_OFFSET.rankedscore)?,
        level: p.read_f32(user_profile_base + USER_PROFILE_OFFSET.level)?,
        playcount: p.read_i32(user_profile_base + USER_PROFILE_OFFSET.playcount)?,
        rank: p.read_i32(user_profile_base + USER_PROFILE_OFFSET.rank)?,
        playmode: p.read_i32(user_profile_base + USER_PROFILE_OFFSET.playmode)?,
        accuracy: p.read_f64(user_profile_base + USER_PROFILE_OFFSET.accuracy)?,
        country_code: p.read_i32(user_profile_base + USER_PROFILE_OFFSET.country_code)?,
        bancho_status: p.read_i32(user_profile_base + USER_PROFILE_OFFSET.bancho_status)?,
    };
    Ok(user_profile)
}
