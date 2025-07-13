use std::path::PathBuf;

use crate::generate_offset_getter;
use crate::reader::common::stable::offset::COMMON_OFFSET;
use crate::reader::common::GameState;
use crate::reader::helpers::{read_i32, read_u32};
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};

pub fn status_addr(p: &Process, state: &mut State) -> Result<i32, Error> {
    Ok(p.read_i32(state.addresses.status - COMMON_OFFSET.status)?)
}

/// Returns a path to the `Songs` folder
///
/// **Platform-specific**
/// - Windows: Will return full absolute path to the `Songs` folder
/// - Linux: Might return relative path, carefully check by yourself
pub(crate) fn path_folder(p: &Process, state: &mut State) -> Result<PathBuf, Error> {
    let settings_ptr = p.read_i32(state.addresses.settings + COMMON_OFFSET.settings_ptr)?;
    let settings_addr = p.read_i32(settings_ptr + COMMON_OFFSET.settings_addr)?;
    let path = p.read_string(settings_addr + COMMON_OFFSET.path)?;

    // Attempt to construct a absolute path from executable path
    if path == "Songs" {
        if let Some(executable_dir) = &p.executable_dir {
            let path = executable_dir.clone().join("Songs");

            return Ok(path);
        }
    }

    Ok(PathBuf::from(path))
}

pub fn menu_mods_addr(p: &Process, state: &mut State) -> Result<i32, Error> {
    Ok(p.read_i32(state.addresses.menu_mods + COMMON_OFFSET.mods_ptr)?)
}

pub fn playtime_addr(p: &Process, state: &mut State) -> Result<i32, Error> {
    Ok(p.read_i32(state.addresses.playtime + COMMON_OFFSET.ig_time)?)
}

generate_offset_getter! {
    game_state: GameState = read_u32(0, status_addr);
    menu_game_mode: u32 = read_u32(0, menu_mods_addr); // TODO: use GameModsLegacy
    game_time: i32 = read_i32(0, playtime_addr);
}

// this is an helper function to be faster for anyone
pub fn check_game_state(p: &Process, state: &mut State, g_state: GameState) -> Result<bool, Error> {
    Ok(game_state(p, state)? == g_state)
}
