//! Module de lecture mémoire overlay touches pour le client stable d'osu!
//!
//! Fournit la lecture des données de l'overlay des touches depuis la mémoire du processus osu!stable.
//! Le parcours dynamique du tableau reste impératif, mais les entrées à offsets fixes
//! sont lues par `ReadMemory` en batches.

use crate::reader::beatmap::stable::BeatmapInfo;
use crate::reader::common::stable::GameStateInfo;
use crate::reader::common::{GameMode, GameState};
use crate::reader::overlay::common::{Key, KeyOverlay};
use crate::reader::structs::State;
use crate::reader::user::stable::memory::playmode;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};
use rosu_memory_macros::ReadMemory;

/// Pointeurs des quatre entrées du tableau de touches. Les pointeurs sont
/// contigus : le derive les récupère dans une seule lecture mémoire.
#[derive(ReadMemory)]
struct KeyPointers {
    #[offset(0x8)]
    key_1: i32,
    #[offset(0xC)]
    key_2: i32,
    #[offset(0x10)]
    mouse_1: i32,
    #[offset(0x14)]
    mouse_2: i32,
}

/// Représentation mémoire d'une touche. `count` et `pressed` partagent le
/// même objet pointé et sont donc lus dans un unique buffer.
#[derive(ReadMemory)]
struct KeyMemory {
    #[offset(0x14)]
    count: i32,
    #[offset(0x1C)]
    pressed: i32,
}

impl From<KeyMemory> for Key {
    fn from(value: KeyMemory) -> Self {
        Self {
            pressed: value.pressed != 0,
            count: value.count,
        }
    }
}

/// Adresse de base du ruleset osu!Standard en cours de jeu
fn ruleset_addr(p: &Process, state: &mut State) -> Result<i32, Error> {
    let game_state = GameStateInfo::read(p, state)?;
    if game_state.state == GameState::Playing
        && playmode(p, state)? == 0
        && BeatmapInfo::read(p, state)?.technical.mode == GameMode::Osu
    {
        let ruleset_ptr = p.read_i32(state.addresses.rulesets - 0xb)?;
        let ruleset_addr = p.read_i32(ruleset_ptr + 0x4)?;
        Ok(ruleset_addr)
    } else {
        Err(Error::NotAvailable("Not Playing".to_string()))
    }
}

pub mod memory {
    use super::*;

    /// Lire l'état de l'overlay des touches en mode osu!Standard
    pub fn info(p: &Process, state: &mut State) -> Result<KeyOverlay, Error> {
        let ruleset_addr = ruleset_addr(p, state)?;
        let key_ptr = p.read_i32(ruleset_addr + 0xb0)?;
        let temp = p.read_i32(key_ptr + 0x10)?;
        let key_array_addr = p.read_i32(temp + 0x4)?;
        let items_size = p.read_i32(key_array_addr + 0x4)?;

        if items_size < 4 {
            return Err(Error::MemoryRead(format!(
                "Key array size is less than 4, got {}",
                items_size
            )));
        }

        let key_pointers = KeyPointers::read_from_memory(p, state, key_array_addr)?;

        Ok(KeyOverlay {
            key_1: KeyMemory::read_from_memory(p, state, key_pointers.key_1)?.into(),
            key_2: KeyMemory::read_from_memory(p, state, key_pointers.key_2)?.into(),
            mouse_1: KeyMemory::read_from_memory(p, state, key_pointers.mouse_1)?.into(),
            mouse_2: KeyMemory::read_from_memory(p, state, key_pointers.mouse_2)?.into(),
        })
    }
}
