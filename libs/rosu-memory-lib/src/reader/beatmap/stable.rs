//! Beatmap stable client memory reading module
//!
//! This module provides all functionality for reading beatmap data from osu!stable process memory.
//! Instead of individual accessors, use the main struct types which read all data at once.

use crate::reader::common::GameMode;
use crate::reader::common::GameState;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};
use rosu_memory_macros::ReadMemory;

/// Complete beatmap information structure
#[derive(Debug, Clone, ReadMemory)]
pub struct BeatmapInfo {
    #[nested(0)]
    pub metadata: BeatmapMetadata,
    #[nested(0)]
    pub location: BeatmapLocation,
    #[nested(0)]
    pub stats: BeatmapStats,
    #[nested(0)]
    pub technical: BeatmapTechnicalInfo,
}

impl BeatmapInfo {
    /// Read complete beatmap info from memory
    pub fn read(p: &Process, state: &mut State) -> Result<Self, Error> {
        let addr = beatmap_addr(p, state)?;
        Self::read_from_memory(p, state, addr)
    }
}

/// Beatmap metadata (author, creator, title, etc.)
#[derive(Debug, Clone, ReadMemory)]
pub struct BeatmapMetadata {
    #[offset(0x18)]
    pub author: String,
    #[offset(0x7C)]
    pub creator: String,
    #[offset(0x24)]
    pub title_romanized: String,
    #[offset(0x28)]
    pub title_original: String,
    #[offset(0xAC)]
    pub difficulty: String,
    #[offset(0x20)]
    pub tags: String,
}

impl BeatmapMetadata {
    /// Read beatmap metadata from memory
    pub fn read(p: &Process, state: &mut State) -> Result<Self, Error> {
        let addr = beatmap_addr(p, state)?;
        Self::read_from_memory(p, state, addr)
    }
}

/// Beatmap technical information (MD5, IDs, mode, status)
#[derive(Debug, Clone, ReadMemory)]
pub struct BeatmapTechnicalInfo {
    #[offset(0x6C)]
    pub md5: String,
    #[offset(0xC8)]
    pub id: i32,
    #[offset(0xCC)]
    pub set_id: i32,
    #[offset(0x11C, via = i32)]
    pub mode: GameMode,
    #[offset(0x12C, via = i32)]
    pub ranked_status: BeatmapStatus,
}

impl BeatmapTechnicalInfo {
    /// Read beatmap technical info from memory
    pub fn read(p: &Process, state: &mut State) -> Result<Self, Error> {
        let addr = beatmap_addr(p, state)?;
        Self::read_from_memory(p, state, addr)
    }
}

/// Beatmap file location and paths
#[derive(Debug, Clone, ReadMemory)]
pub struct BeatmapLocation {
    #[offset(0x78)]
    pub folder: String,
    #[offset(0x90)]
    pub filename: String,
    #[offset(0x64)]
    pub audio: String,
    #[offset(0x68)]
    pub cover: String,
}

impl BeatmapLocation {
    /// Get the full file path (folder/filename)
    pub fn get_file_path(&self) -> String {
        format!("{}/{}", self.folder, self.filename)
    }

    /// Get the full audio file path (folder/audio)
    pub fn get_audio_path(&self) -> String {
        format!("{}/{}", self.folder, self.audio)
    }

    /// Get the full cover image path (folder/cover)
    pub fn get_cover_path(&self) -> String {
        format!("{}/{}", self.folder, self.cover)
    }

    /// Read beatmap location from memory
    pub fn read(p: &Process, state: &mut State) -> Result<Self, Error> {
        let addr = beatmap_addr(p, state)?;
        Self::read_from_memory(p, state, addr)
    }
}

/// Beatmap difficulty statistics (AR, CS, HP, OD)
#[derive(Debug, Clone, ReadMemory)]
pub struct BeatmapStats {
    #[offset(0x2C)]
    pub ar: f32,
    #[offset(0x30)]
    pub cs: f32,
    #[offset(0x34)]
    pub hp: f32,
    #[offset(0x38)]
    pub od: f32,
    #[offset(0x134)]
    pub length: i32,
    #[skip]
    pub star_rating: BeatmapStarRating,
    #[offset(0xF8)]
    pub object_count: i32,
    #[offset(0x146)]
    pub slider_count: i32,
}

impl BeatmapStats {
    /// Read beatmap stats from memory
    pub fn read(p: &Process, state: &mut State) -> Result<Self, Error> {
        let addr = beatmap_addr(p, state)?;
        Self::read_from_memory(p, state, addr)
    }
}

/// Beatmap star ratings for different mods
#[derive(Debug, Clone)]
pub struct BeatmapStarRating {
    pub no_mod: f64,
    pub dt: f64,
    pub ht: f64,
}

impl Default for BeatmapStarRating {
    fn default() -> Self {
        Self {
            no_mod: 0.0,
            dt: 0.0,
            ht: 0.0,
        }
    }
}

impl BeatmapStarRating {
    /// Read beatmap star ratings from memory
    pub fn read(p: &Process, state: &mut State) -> Result<Self, Error> {
        let addr = beatmap_addr(p, state)?;
        let stats = BeatmapStats::read_from_memory(p, state, addr)?;
        Ok(stats.star_rating)
    }
}

/// Beatmap ranked status enumeration
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[repr(i16)]
pub enum BeatmapStatus {
    #[default]
    Unknown = 0,
    Unsubmitted = 1,
    Unranked = 2,
    Unused = 3,
    Ranked = 4,
    Approved = 5,
    Qualified = 6,
    Loved = 7,
}

impl BeatmapStatus {
    /// Convert status to string representation
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            BeatmapStatus::Unknown => "unknown".to_string(),
            BeatmapStatus::Unsubmitted => "unsubmitted".to_string(),
            BeatmapStatus::Unranked => "unranked".to_string(),
            BeatmapStatus::Unused => "unused".to_string(),
            BeatmapStatus::Ranked => "ranked".to_string(),
            BeatmapStatus::Approved => "approved".to_string(),
            BeatmapStatus::Qualified => "qualified".to_string(),
            BeatmapStatus::Loved => "loved".to_string(),
        }
    }
}

impl From<i16> for BeatmapStatus {
    fn from(value: i16) -> Self {
        match value {
            1 => Self::Unsubmitted,
            2 => Self::Unranked,
            3 => Self::Unused,
            4 => Self::Ranked,
            5 => Self::Approved,
            6 => Self::Qualified,
            7 => Self::Loved,
            _ => Self::Unknown,
        }
    }
}

impl From<i32> for BeatmapStatus {
    fn from(value: i32) -> Self {
        Self::from(value as i16)
    }
}

/// Get the base address of the current beatmap
///
/// Returns an error if the game state is not in one of the valid states
/// (SongSelect, Editor, Playing, ResultScreen)
fn beatmap_addr(p: &Process, state: &mut State) -> Result<i32, Error> {
    let current_state = p.read_u32(state.addresses.status)?;
    let game_state = GameState::from(current_state);

    let valid_states = [
        GameState::SongSelect,
        GameState::Editor,
        GameState::Playing,
        GameState::ResultScreen,
    ];

    if valid_states.contains(&game_state) {
        Ok(p.read_i32(p.read_i32(state.addresses.base - 0xC)?)?)
    } else {
        Err(Error::NotAvailable(
            "Not in valid beatmap state".to_string(),
        ))
    }
}

pub mod memory {
    use super::*;

    /// Read complete beatmap info structure
    pub fn info(p: &Process, state: &mut State) -> Result<BeatmapInfo, Error> {
        BeatmapInfo::read(p, state)
    }

    /// Get just the game mode for the current beatmap
    pub fn mode(p: &Process, state: &mut State) -> Result<GameMode, Error> {
        let info = info(p, state)?;
        Ok(info.technical.mode)
    }
}
