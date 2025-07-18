use crate::generate_offset_getter;
use crate::reader::common::stable::memory::check_game_state;
use crate::reader::common::GameMode;
use crate::reader::common::GameState;
use crate::reader::helpers::{calculate_accuracy, read_i16, read_i32, read_string};
use crate::reader::resultscreen::common::ResultScreenInfo;
use crate::reader::resultscreen::stable::offset::RESULT_SCREEN_OFFSET;
use crate::reader::structs::{Hit, State};
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};

pub fn result_screen_ptr(p: &Process, state: &mut State) -> Result<i32, Error> {
    if check_game_state(p, state, GameState::ResultScreen)? {
        Ok(p.read_i32(state.addresses.rulesets - RESULT_SCREEN_OFFSET.ptr)?)
    } else {
        Err(Error::NotAvailable("Not in ResultScreen".to_string()))
    }
}

pub fn hits(p: &Process, state: &mut State) -> Result<Hit, Error> {
    let score_base = result_screen_base(p, state)?;
    // Read all hits data in one memory operation
    let mut hits_buffer = [0u8; size_of::<i16>() * 6];
    p.read(
        score_base + RESULT_SCREEN_OFFSET.hits._100,
        size_of::<i16>() * 6,
        &mut hits_buffer,
    )?;

    // Safety: unwrap here because buffer is already initialized and filled
    // with zeros, the worst case scenario is hits going to be zeros
    Ok(Hit {
        _100: i16::from_le_bytes(hits_buffer[0..2].try_into().unwrap()),
        _300: i16::from_le_bytes(hits_buffer[2..4].try_into().unwrap()),
        _50: i16::from_le_bytes(hits_buffer[4..6].try_into().unwrap()),
        _geki: i16::from_le_bytes(hits_buffer[6..8].try_into().unwrap()),
        _katu: i16::from_le_bytes(hits_buffer[8..10].try_into().unwrap()),
        _miss: i16::from_le_bytes(hits_buffer[10..12].try_into().unwrap()),
    })
}

pub fn accuracy(p: &Process, state: &mut State) -> Result<f64, Error> {
    calculate_accuracy(&mode(p, state)?, &hits(p, state)?)
}

generate_offset_getter! {
    result_screen_addr: i32 = read_i32(RESULT_SCREEN_OFFSET.addr, result_screen_ptr);
    result_screen_base: i32 = read_i32(RESULT_SCREEN_OFFSET.base, result_screen_addr);
    username: String = read_string(RESULT_SCREEN_OFFSET.username, result_screen_base);
    score: i32 = read_i32(RESULT_SCREEN_OFFSET.score, result_screen_base);
    max_combo: i16 = read_i16(RESULT_SCREEN_OFFSET.max_combo, result_screen_base);
    mode: GameMode = read_i32(RESULT_SCREEN_OFFSET.mode, result_screen_base);
    hits_300: i16 = read_i16(RESULT_SCREEN_OFFSET.hits._300, result_screen_base);
    hits_100: i16 = read_i16(RESULT_SCREEN_OFFSET.hits._100, result_screen_base);
    hits_50: i16 = read_i16(RESULT_SCREEN_OFFSET.hits._50, result_screen_base);
    hits_miss: i16 = read_i16(RESULT_SCREEN_OFFSET.hits._miss, result_screen_base);
    hits_geki: i16 = read_i16(RESULT_SCREEN_OFFSET.hits._geki, result_screen_base);
    hits_katu: i16 = read_i16(RESULT_SCREEN_OFFSET.hits._katu, result_screen_base);
}

pub fn info(p: &Process, state: &mut State) -> Result<ResultScreenInfo, Error> {
    let hits = hits(p, state)?;
    let mode = mode(p, state)?;
    let accuracy = calculate_accuracy(&mode, &hits)?;
    let base = result_screen_base(p, state)?;
    Ok(ResultScreenInfo {
        username: p.read_string(base + RESULT_SCREEN_OFFSET.username)?,
        mode,
        max_combo: p.read_i16(base + RESULT_SCREEN_OFFSET.max_combo)?,
        score: p.read_i32(base + RESULT_SCREEN_OFFSET.score)?,
        hits,
        accuracy,
    })
}
