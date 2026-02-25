use crate::common::GameMode;
use rosu_memory_macros::ReadMemory;

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
    pub fn get_file_path(&self) -> String {
        format!("{}/{}", self.folder, self.filename)
    }
    pub fn get_audio_path(&self) -> String {
        format!("{}/{}", self.folder, self.audio)
    }
    pub fn get_cover_path(&self) -> String {
        format!("{}/{}", self.folder, self.cover)
    }
}

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
