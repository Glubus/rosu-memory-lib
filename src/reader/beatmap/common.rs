use crate::common::GameMode;

#[derive(Debug, Clone)]
pub struct BeatmapInfo {
    pub metadata: BeatmapMetadata,
    pub location: BeatmapLocation,
    pub stats: BeatmapStats,
    pub technical: BeatmapTechnicalInfo,
}

#[derive(Debug, Clone)]
pub struct BeatmapMetadata {
    pub author: String,
    pub creator: String,
    pub title_romanized: String,
    pub title_original: String,
    pub difficulty: String,
    pub tags: String,
}
#[derive(Debug, Clone)]
pub struct BeatmapTechnicalInfo {
    pub md5: String,
    pub id: i32,
    pub set_id: i32,
    pub mode: GameMode,
    pub ranked_status: BeatmapStatus,
}
#[derive(Debug, Clone)]
pub struct BeatmapLocation {
    pub folder: String,
    pub filename: String,
    pub audio: String,
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

#[derive(Debug, Clone)]
pub struct BeatmapStats {
    pub ar: f32,
    pub od: f32,
    pub cs: f32,
    pub hp: f32,
    pub length: i32,
    pub star_rating: BeatmapStarRating,
    pub object_count: i32,
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

pub(crate) struct BeatmapOffset {
    pub ptr: i32,
    pub metadata: BeatmapMetadataOffset,
    pub location: BeatmapLocationOffset,
    pub stats: BeatmapStatsOffset,
    pub technical: BeatmapTechnicalOffset,
}

pub struct BeatmapStatsOffset {
    pub ar: i32,
    pub od: i32,
    pub cs: i32,
    pub hp: i32,
    pub object_count: i32,
    pub total_length: i32,
    pub drain_time: i32,
    pub star_rating: i32,
    pub slider_count: i32,
}

pub struct BeatmapLocationOffset {
    pub folder: i32,
    pub filename: i32,
    pub audio: i32,
    pub cover: i32,
}

pub struct BeatmapTechnicalOffset {
    pub md5: i32,
    pub id: i32,
    pub set_id: i32,
    pub mode: i32,
    pub ranked_status: i32,
}

pub struct BeatmapMetadataOffset {
    pub author: i32,
    pub creator: i32,
    pub title_romanized: i32,
    pub title_original: i32,
    pub difficulty: i32,
    pub tags: i32,
}
