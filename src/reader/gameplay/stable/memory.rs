use crate::reader::common::stable::memory::check_game_state;
use crate::reader::common::GameState;
use crate::reader::gameplay::common::GameplayInfo;
use crate::reader::gameplay::stable::offset::GAMEPLAY_OFFSET;
use crate::reader::structs::Hit;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};

pub fn get_base(p: &Process, state: &mut State) -> Result<i32, Error> {
    if check_game_state(p, state, GameState::Playing)? {
        let rulesets = match p.read_i32(state.addresses.rulesets - GAMEPLAY_OFFSET.ptr) {
            Ok(val) => val,
            Err(_) => return Err(Error::NotAvailable("Still loading".to_string())),
        };

        let ruleset_addr = match p.read_i32(rulesets + GAMEPLAY_OFFSET.addr) {
            Ok(val) => val,
            Err(_) => return Err(Error::NotAvailable("Still loading".to_string())),
        };

        let gameplay_base = match p.read_i32(ruleset_addr + GAMEPLAY_OFFSET.base) {
            Ok(val) => val,
            Err(_) => return Err(Error::NotAvailable("Still loading".to_string())),
        };

        Ok(gameplay_base)
    } else {
        Err(Error::NotAvailable("Not in Playing".to_string()))
    }
}

pub fn get_base2(p: &Process, state: &mut State) -> Result<i32, Error> {
    let gameplay_base = get_base(p, state)?;
    match p.read_i32(gameplay_base + GAMEPLAY_OFFSET.base2) {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::NotAvailable("Still loading".to_string())),
    }
}

pub fn get_score_gameplay(p: &Process, state: &mut State) -> Result<i32, Error> {
    let base2 = get_base2(p, state)?;
    let score = p.read_i32(base2 + GAMEPLAY_OFFSET.score)?;
    Ok(score)
}

pub fn get_mods(p: &Process, state: &mut State) -> Result<u32, Error> {
    let base2 = get_base2(p, state)?;
    let mods_xor_base = p.read_i32(base2 + GAMEPLAY_OFFSET.mods)?;
    let mods_xor1 = p.read_u64(mods_xor_base + GAMEPLAY_OFFSET.mods_xor)?;
    let mods_xor2 = p.read_u64(mods_xor_base + GAMEPLAY_OFFSET.mods_xor2)?;
    Ok((mods_xor1 ^ mods_xor2) as u32)
}

pub fn get_combo(p: &Process, state: &mut State) -> Result<i16, Error> {
    let base2 = get_base2(p, state)?;
    let combo = p.read_i16(base2 + GAMEPLAY_OFFSET.combo)?;
    Ok(combo)
}

pub fn get_max_combo(p: &Process, state: &mut State) -> Result<i16, Error> {
    let base2 = get_base2(p, state)?;
    let max_combo = p.read_i16(base2 + GAMEPLAY_OFFSET.max_combo)?;
    Ok(max_combo)
}

pub fn get_current_hp(p: &Process, state: &mut State) -> Result<f64, Error> {
    let base = get_base(p, state)?;
    let hp_base = p.read_i32(base + GAMEPLAY_OFFSET.hp_base)?;
    let hp = p.read_f64(hp_base + GAMEPLAY_OFFSET.hp)?;
    Ok(hp)
}

pub fn get_username(p: &Process, state: &mut State) -> Result<String, Error> {
    let base2 = get_base2(p, state)?;
    let username = p.read_string(base2 + GAMEPLAY_OFFSET.username)?;
    Ok(username)
}

pub fn get_ig_time(p: &Process, state: &mut State) -> Result<i32, Error> {
    crate::reader::common::stable::memory::get_ig_time(p, state)
}

pub fn get_retries(p: &Process, state: &mut State) -> Result<i32, Error> {
    let igt_addr = p.read_i32(state.addresses.base - GAMEPLAY_OFFSET.ruleset)?;
    let retries = p.read_i32(igt_addr + GAMEPLAY_OFFSET.retries)?;
    Ok(retries)
}

pub fn get_hits_300(p: &Process, state: &mut State) -> Result<i16, Error> {
    let base2 = get_base2(p, state)?;
    let hits_300 = p.read_i16(base2 + GAMEPLAY_OFFSET.hits._300)?;
    Ok(hits_300)
}

pub fn get_hits_100(p: &Process, state: &mut State) -> Result<i16, Error> {
    let base2 = get_base2(p, state)?;
    let hits_100 = p.read_i16(base2 + GAMEPLAY_OFFSET.hits._100)?;
    Ok(hits_100)
}

pub fn get_hits_50(p: &Process, state: &mut State) -> Result<i16, Error> {
    let base2 = get_base2(p, state)?;
    let hits_50 = p.read_i16(base2 + GAMEPLAY_OFFSET.hits._50)?;
    Ok(hits_50)
}

pub fn get_hits_miss(p: &Process, state: &mut State) -> Result<i16, Error> {
    let base2 = get_base2(p, state)?;
    let hits_miss = p.read_i16(base2 + GAMEPLAY_OFFSET.hits._miss)?;
    Ok(hits_miss)
}

pub fn get_hits_geki(p: &Process, state: &mut State) -> Result<i16, Error> {
    let base2 = get_base2(p, state)?;
    let hits_geki = p.read_i16(base2 + GAMEPLAY_OFFSET.hits._geki)?;
    Ok(hits_geki)
}

pub fn get_hits_katu(p: &Process, state: &mut State) -> Result<i16, Error> {
    let base2 = get_base2(p, state)?;
    let hits_katu = p.read_i16(base2 + GAMEPLAY_OFFSET.hits._katu)?;
    Ok(hits_katu)
}

pub fn get_hits(p: &Process, state: &mut State) -> Result<Hit, Error> {
    let base2 = get_base2(p, state)?;
    Ok(Hit {
        _300: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._300)?,
        _100: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._100)?,
        _50: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._50)?,
        _miss: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._miss)?,
        _geki: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._geki)?,
        _katu: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._katu)?,
    })
}

pub fn get_gameplay_info(p: &Process, state: &mut State) -> Result<GameplayInfo, Error> {
    let base = get_base(p, state)?;
    let base2 = get_base2(p, state)?;

    let hp = p.read_f64(p.read_i32(base + GAMEPLAY_OFFSET.hp_base)? + GAMEPLAY_OFFSET.hp)?;
    let mods = get_mods(p, state)?;

    Ok(GameplayInfo {
        score: p.read_i32(base2 + GAMEPLAY_OFFSET.score)?,
        mods,
        combo: p.read_i16(base2 + GAMEPLAY_OFFSET.combo)?,
        max_combo: p.read_i16(base2 + GAMEPLAY_OFFSET.max_combo)?,
        hp,
        username: p.read_string(base2 + GAMEPLAY_OFFSET.username)?,
        ig_time: get_ig_time(p, state)?, // different base
        retries: get_retries(p, state)?, // different base
        hits: Hit {
            _300: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._300)?,
            _100: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._100)?,
            _50: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._50)?,
            _miss: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._miss)?,
            _geki: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._geki)?,
            _katu: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._katu)?,
        },
    })
}
