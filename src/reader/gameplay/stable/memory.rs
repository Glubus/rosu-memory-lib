use crate::reader::common::stable::memory::{check_game_state};
use crate::reader::common::GameState;
use crate::reader::gameplay::common::GameplayInfo;
use crate::reader::gameplay::stable::offset::GAMEPLAY_OFFSET;
use crate::reader::structs::Hit;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};
use crate::{generate_offset_getter, reader::helpers::{read_f64, read_i16, read_i32, read_string, read_u64}};

pub fn rulesets_addr(p: &Process, state: &mut State) -> Result<i32, Error> {
    if check_game_state(p, state, GameState::Playing)? {
        Ok(p.read_i32(state.addresses.rulesets - GAMEPLAY_OFFSET.ptr)?)
    } else {
        Err(Error::NotAvailable("Not in Playing".to_string()))
    }
}

pub fn mods(p: &Process, state: &mut State) -> Result<u32, Error> {
    let mods_xor1 = mods_xor1(p, state)?;
    let mods_xor2 = mods_xor2(p, state)?;
    Ok((mods_xor1 ^ mods_xor2) as u32)
}

generate_offset_getter! {
    ruleset_addr: i32 = read_i32(GAMEPLAY_OFFSET.addr, rulesets_addr);
    gameplay_base: i32 = read_i32(GAMEPLAY_OFFSET.base, ruleset_addr);
    score_base: i32 = read_i32(GAMEPLAY_OFFSET.score_base, gameplay_base);
    hp_base: i32 = read_i32(GAMEPLAY_OFFSET.hp_base, gameplay_base);
    score: i32 = read_i32(GAMEPLAY_OFFSET.score, score_base);
    mods_xor_base: i32 = read_i32(GAMEPLAY_OFFSET.mods, score_base);
    mods_xor1: u64 = read_u64(GAMEPLAY_OFFSET.mods_xor, mods_xor_base);
    mods_xor2: u64 = read_u64(GAMEPLAY_OFFSET.mods_xor2, mods_xor_base);
    combo: i16 = read_i16(GAMEPLAY_OFFSET.combo, score_base);
    max_combo: i16 = read_i16(GAMEPLAY_OFFSET.max_combo, score_base);
    hp: f64 = read_f64(GAMEPLAY_OFFSET.hp, hp_base);
    username: String = read_string(GAMEPLAY_OFFSET.username, score_base);
    hits_300: i16 = read_i16(GAMEPLAY_OFFSET.hits._300, score_base);
    hits_100: i16 = read_i16(GAMEPLAY_OFFSET.hits._100, score_base);
    hits_50: i16 = read_i16(GAMEPLAY_OFFSET.hits._50, score_base);
    hits_miss: i16 = read_i16(GAMEPLAY_OFFSET.hits._miss, score_base);
    hits_geki: i16 = read_i16(GAMEPLAY_OFFSET.hits._geki, score_base);
    hits_katu: i16 = read_i16(GAMEPLAY_OFFSET.hits._katu, score_base);
}




/// this is a wrapper to not confuse people it could be deleted in the future 
/// use -> crate::reader::common::stable::memory::game_time
pub fn game_time(p: &Process, state: &mut State) -> Result<i32, Error> {
    crate::reader::common::stable::memory::game_time(p, state)
}

pub fn retries(p: &Process, state: &mut State) -> Result<i32, Error> {
    let igt_addr = p.read_i32(state.addresses.base - GAMEPLAY_OFFSET.ruleset)?;
    let retries = p.read_i32(igt_addr + GAMEPLAY_OFFSET.retries)?;
    Ok(retries)
}

pub fn hits(p: &Process, state: &mut State) -> Result<Hit, Error> {
    let score_base = score_base(p, state)?;
    // TODO: check issue for reading the full block and 
    // separating bits 
    Ok(Hit {
        _300: p.read_i16(score_base + GAMEPLAY_OFFSET.hits._300)?,
        _100: p.read_i16(score_base + GAMEPLAY_OFFSET.hits._100)?,
        _50: p.read_i16(score_base + GAMEPLAY_OFFSET.hits._50)?,
        _miss: p.read_i16(score_base + GAMEPLAY_OFFSET.hits._miss)?,
        _geki: p.read_i16(score_base + GAMEPLAY_OFFSET.hits._geki)?,
        _katu: p.read_i16(score_base + GAMEPLAY_OFFSET.hits._katu)?,
    })
}

pub fn info(p: &Process, state: &mut State) -> Result<GameplayInfo, Error> {
    let score_base = score_base(p, state)?;

    let hp = hp(p, state)?;
    let mods = mods(p, state)?;

    Ok(GameplayInfo {
        score: p.read_i32(score_base + GAMEPLAY_OFFSET.score)?,
        mods,
        combo: p.read_i16(score_base + GAMEPLAY_OFFSET.combo)?,
        max_combo: p.read_i16(score_base + GAMEPLAY_OFFSET.max_combo)?,
        hp,
        username: p.read_string(score_base + GAMEPLAY_OFFSET.username)?,
        ig_time: game_time(p, state)?, // different base
        retries: retries(p, state)?, // different base
        hits: Hit {
            _300: p.read_i16(score_base + GAMEPLAY_OFFSET.hits._300)?,
            _100: p.read_i16(score_base + GAMEPLAY_OFFSET.hits._100)?,
            _50: p.read_i16(score_base + GAMEPLAY_OFFSET.hits._50)?,
            _miss: p.read_i16(score_base + GAMEPLAY_OFFSET.hits._miss)?,
            _geki: p.read_i16(score_base + GAMEPLAY_OFFSET.hits._geki)?,
            _katu: p.read_i16(score_base + GAMEPLAY_OFFSET.hits._katu)?,
        },
    })
}
