use crate::reader::beatmap::stable::memory::mode;
use crate::reader::common::stable::memory::check_game_state;
use crate::reader::common::GameMode;
use crate::reader::common::GameState;
use crate::reader::overlay::common::{Key, KeyOverlay};
use crate::reader::structs::State;
use crate::reader::user::stable::memory::playmode;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};

pub fn ruleset_addr(p: &Process, state: &mut State) -> Result<i32, Error> {
    if check_game_state(p, state, GameState::Playing)?
        && playmode(p, state)? == 0
        && mode(p, state)? == GameMode::Osu
    {
        let ruleset_ptr = p.read_i32(state.addresses.rulesets - 0xb)?;
        let ruleset_addr = p.read_i32(ruleset_ptr + 0x4)?;
        Ok(ruleset_addr)
    } else {
        Err(Error::NotAvailable("Not Playing".to_string()))
    }
}

pub fn key_ptr(p: &Process, state: &mut State) -> Result<i32, Error> {
    let ruleset_addr = ruleset_addr(p, state)?;
    let key_ptr = p.read_i32(ruleset_addr + 0xb0)?;
    Ok(key_ptr)
}

pub fn key_overlay_std(p: &Process, state: &mut State) -> Result<KeyOverlay, Error> {
    let key_ptr = key_ptr(p, state)?;
    let temp = p.read_i32(key_ptr + 0x10)?;
    let key_array_addr = p.read_i32(temp + 0x4)?;
    let items_size = p.read_i32(key_array_addr + 0x4)?;

    if items_size < 4 {
        return Err(Error::MemoryRead(format!(
            "Key array size is less than 4, got {}",
            items_size
        )));
    }

    let key_1_pressed = p.read_i32(p.read_i32(key_array_addr + 0x8)? + 0x1C)? != 0;
    let key_1_count = p.read_i32(p.read_i32(key_array_addr + 0x8)? + 0x14)?;

    let key_2_pressed = p.read_i32(p.read_i32(key_array_addr + 0xc)? + 0x1C)? != 0;
    let key_2_count = p.read_i32(p.read_i32(key_array_addr + 0xc)? + 0x14)?;

    let mouse_1_pressed = p.read_i32(p.read_i32(key_array_addr + 0x10)? + 0x1C)? != 0;
    let mouse_1_count = p.read_i32(p.read_i32(key_array_addr + 0x10)? + 0x14)?;

    let mouse_2_pressed = p.read_i32(p.read_i32(key_array_addr + 0x14)? + 0x1C)? != 0;
    let mouse_2_count = p.read_i32(p.read_i32(key_array_addr + 0x14)? + 0x14)?;

    Ok(KeyOverlay {
        key_1: Key {
            pressed: key_1_pressed,
            count: key_1_count,
        },
        key_2: Key {
            pressed: key_2_pressed,
            count: key_2_count,
        },
        mouse_1: Key {
            pressed: mouse_1_pressed,
            count: mouse_1_count,
        },
        mouse_2: Key {
            pressed: mouse_2_pressed,
            count: mouse_2_count,
        },
    })
}
