use crate::reader::common::stable::offset::COMMON_OFFSET;
use crate::reader::common::GameState;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};

pub fn get_game_state(p: &Process, state: &mut State) -> Result<GameState, Error> {
    let status_ptr = p.read_i32(state.addresses.status - COMMON_OFFSET.status)?;
    Ok(GameState::from(p.read_u32(status_ptr)?))
}

pub fn check_game_state(p: &Process, state: &mut State, g_state: GameState) -> Result<bool, Error> {
    Ok(get_game_state(p, state)? == g_state)
}

pub(crate) fn get_path_folder(p: &Process, state: &mut State) -> Result<String, Error> {
    let settings_ptr = p.read_i32(state.addresses.settings + COMMON_OFFSET.settings_ptr)?;
    let settings_addr = p.read_i32(settings_ptr + COMMON_OFFSET.settings_addr)?;
    let path = p.read_string(settings_addr + COMMON_OFFSET.path)?;
    // default (relative path)
    if path == "Songs" {
        return Ok(format!(
            "{}/Songs",
            p.executable_dir.clone().unwrap().display()
        ));
    }
    // custom user path (absolute path)
    Ok(path)
}

pub fn get_menu_mods(p: &Process, state: &mut State) -> Result<i32, Error> {
    let menu_mods_ptr = p.read_i32(state.addresses.menu_mods + COMMON_OFFSET.mods_ptr)?;
    Ok(p.read_i32(menu_mods_ptr)?)
}

pub fn get_ig_time(p: &Process, state: &mut State) -> Result<i32, Error> {
    let playtime_ptr = p.read_i32(state.addresses.playtime + COMMON_OFFSET.ig_time)?;
    Ok(p.read_i32(playtime_ptr)?)
}
