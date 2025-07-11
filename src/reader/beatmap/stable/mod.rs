pub mod file;
pub mod location;
pub mod memory;
pub mod offset;

use crate::reader::beatmap::stable::offset::*;
use crate::reader::common::stable::memory::check_game_state;
use crate::reader::common::GameState;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};

pub(crate) fn get_beatmap_addr(p: &Process, state: &mut State) -> Result<i32, Error> {
    match check_game_state(p, state, GameState::SongSelect)?
        || check_game_state(p, state, GameState::Editor)?
        || check_game_state(p, state, GameState::Playing)?
        || check_game_state(p, state, GameState::ResultScreen)?
    {
        true => Ok(p.read_i32(p.read_i32(state.addresses.base - BEATMAP_OFFSET.ptr)?)?),
        false => Err(Error::NotAvailable("Not in song select".to_string())),
    }
}

pub(crate) fn read_from_beatmap_ptr_string(
    p: &Process,
    state: &mut State,
    offset: i32,
) -> Result<String, Error> {
    let beatmap_addr = get_beatmap_addr(p, state)?;
    Ok(p.read_string(beatmap_addr + offset)?)
}
