pub mod file;
pub mod memory;
pub mod offset;

use crate::reader::beatmap::stable::offset::*;
use crate::reader::common::stable::memory::check_game_state;
use crate::reader::common::GameState;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};

pub(crate) fn beatmap_addr(p: &Process, state: &mut State) -> Result<i32, Error> {
    match check_game_state(p, state, GameState::SongSelect)?
        || check_game_state(p, state, GameState::Editor)?
        || check_game_state(p, state, GameState::Playing)?
        || check_game_state(p, state, GameState::ResultScreen)?
    {
        true => Ok(p.read_i32(p.read_i32(state.addresses.base - BEATMAP_OFFSET.ptr)?)?),
        false => Err(Error::NotAvailable("Not in song select".to_string())),
    }
}
