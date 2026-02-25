//! Common stable client memory reading module
//!
//! Provides game state, timing, and basic game information reading

use crate::reader::common::GameState;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};
use rosu_memory_macros::ReadMemory;
use std::path::PathBuf;

/// Game state information
#[derive(Debug, Clone, ReadMemory)]
#[read_memory(init_base = p.read_i32(state.addresses.status - 0x4)?)]
pub struct GameStateInfo {
    #[offset(0x0, via = u32)]
    pub state: GameState,
}

/// Menu mode information
#[derive(Debug, Clone, ReadMemory)]
#[read_memory(init_base = p.read_i32(state.addresses.menu_mods + 0x9)?)]
pub struct MenuInfo {
    #[offset(0x0)]
    pub mods: u32,
}

/// In-game timing information
#[derive(Debug, Clone, ReadMemory)]
#[read_memory(init_base = p.read_i32(state.addresses.playtime + 0x5)?)]
pub struct TimingInfo {
    #[offset(0x0)]
    pub game_time: i32,
}

/// Pause state information
#[derive(Debug, Clone, ReadMemory)]
#[init_base(p.read_i32(state.addresses.base - 0x33)?)]
pub struct PauseInfo {
    #[offset(0x21, via = i8)]
    pub pause_val: i8,
}

impl PauseInfo {
    pub fn is_paused(&self) -> bool {
        self.pause_val == 1
    }
}

/// Replay watching state
#[derive(Debug, Clone)]
pub struct ReplayInfo {
    pub is_watching: bool,
}

impl ReplayInfo {
    /// Check if currently watching a replay
    pub fn read(p: &Process, state: &mut State) -> Result<Self, Error> {
        let addr = p.read_i32(state.addresses.check_replay_addr + 0x46)?;
        let val = p.read_u8(addr)?;
        Ok(Self {
            is_watching: val == 1,
        })
    }
}

/// Songs folder path information
#[derive(Debug, Clone)]
pub struct PathInfo {
    pub songs_folder: PathBuf,
}

impl PathInfo {
    /// Read Songs folder path from memory
    ///
    /// **Platform-specific**
    /// - Windows: Will return full absolute path to the `Songs` folder
    /// - Linux: Might return relative path, carefully check
    pub fn read(p: &Process, state: &mut State) -> Result<Self, Error> {
        let settings_ptr = p.read_i32(state.addresses.settings + 0x8)?;
        let settings_addr = p.read_i32(settings_ptr + 0xb8)?;
        let path = p.read_string(settings_addr + 0x4)?;

        let songs_folder = if path == "Songs" {
            if let Some(executable_dir) = &p.executable_dir {
                executable_dir.clone().join("Songs")
            } else {
                PathBuf::from(path)
            }
        } else {
            PathBuf::from(path)
        };

        Ok(Self { songs_folder })
    }
}
