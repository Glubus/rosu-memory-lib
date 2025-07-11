use crate::reader::structs::State;
use crate::reader::user::common::UserInfo;
use crate::reader::user::stable::offset::USER_PROFILE_OFFSET;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};

pub fn get_user_profile_base(p: &Process, state: &mut State) -> Result<i32, Error> {
    Ok(p.read_i32(p.read_i32(state.addresses.user_profile + USER_PROFILE_OFFSET.ptr)?)?)
}

pub fn get_user_id(p: &Process, state: &mut State) -> Result<i32, Error> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i32(user_profile_base + USER_PROFILE_OFFSET.id)?)
}

pub fn get_user_bancho_status(p: &Process, state: &mut State) -> Result<i32, Error> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i32(user_profile_base + USER_PROFILE_OFFSET.bancho_status)?)
}

pub fn get_user_country_code(p: &Process, state: &mut State) -> Result<i32, Error> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i32(user_profile_base + USER_PROFILE_OFFSET.country_code)?)
}

pub fn get_user_username(p: &Process, state: &mut State) -> Result<String, Error> {
    let user_profile_base = get_user_profile_base(p, state)?;
    let username_ptr = p.read_i32(user_profile_base + USER_PROFILE_OFFSET.username)?;
    Ok(p.read_string(username_ptr)?)
}

pub fn get_user_pp(p: &Process, state: &mut State) -> Result<i32, Error> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i32(user_profile_base + USER_PROFILE_OFFSET.pp)?)
}

pub fn get_user_rankedscore(p: &Process, state: &mut State) -> Result<i64, Error> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i64(user_profile_base + USER_PROFILE_OFFSET.rankedscore)?)
}

pub fn get_user_level(p: &Process, state: &mut State) -> Result<f32, Error> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_f32(user_profile_base + USER_PROFILE_OFFSET.level)?)
}

pub fn get_user_playcount(p: &Process, state: &mut State) -> Result<i32, Error> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i32(user_profile_base + USER_PROFILE_OFFSET.playcount)?)
}

pub fn get_user_rank(p: &Process, state: &mut State) -> Result<i32, Error> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i32(user_profile_base + USER_PROFILE_OFFSET.rank)?)
}

pub fn get_user_playmode(p: &Process, state: &mut State) -> Result<i32, Error> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i32(user_profile_base + USER_PROFILE_OFFSET.playmode)?)
}

pub fn get_user_accuracy(p: &Process, state: &mut State) -> Result<f64, Error> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_f64(user_profile_base + USER_PROFILE_OFFSET.accuracy)?)
}

pub fn get_user_info(p: &Process, state: &mut State) -> Result<UserInfo, Error> {
    let user_profile_base = get_user_profile_base(p, state)?;

    let user_profile = UserInfo {
        id: p.read_i32(user_profile_base + USER_PROFILE_OFFSET.id)?,
        username: p.read_string(p.read_i32(user_profile_base + USER_PROFILE_OFFSET.username)?)?, // TODO: need a fix idk how it show weirdly
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
