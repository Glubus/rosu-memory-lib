use crate::reader::beatmap::common::BeatmapLocationOffset;
use crate::reader::beatmap::common::BeatmapMetadataOffset;
use crate::reader::beatmap::common::BeatmapOffset;
use crate::reader::beatmap::common::BeatmapStatsOffset;
use crate::reader::beatmap::common::BeatmapTechnicalOffset;

pub(crate) const BEATMAP_LOCATION_OFFSET: BeatmapLocationOffset = BeatmapLocationOffset {
    folder: 0x78,
    filename: 0x90,
    audio: 0x64,
    cover: 0x68,
};

pub(crate) const BEATMAP_STATS_OFFSET: BeatmapStatsOffset = BeatmapStatsOffset {
    ar: 0x2c,
    cs: 0x30,
    hp: 0x34,
    od: 0x38,
    object_count: 0xf8,
    total_length: 0x134,
    drain_time: 0x0,  // TODO
    star_rating: 0x0, // TODO
    slider_count: 0x146,
};

pub(crate) const BEATMAP_TECHNICAL_OFFSET: BeatmapTechnicalOffset = BeatmapTechnicalOffset {
    md5: 0x6c,
    id: 0xc8,
    set_id: 0xcc,
    mode: 0x11c,
    ranked_status: 0x12c,
};

pub(crate) const BEATMAP_OFFSET: BeatmapOffset = BeatmapOffset {
    ptr: 0xC,
    metadata: BEATMAP_METADATA_OFFSET,
    location: BEATMAP_LOCATION_OFFSET,
    stats: BEATMAP_STATS_OFFSET,
    technical: BEATMAP_TECHNICAL_OFFSET,
};

pub(crate) const BEATMAP_METADATA_OFFSET: BeatmapMetadataOffset = BeatmapMetadataOffset {
    author: 0x18,
    creator: 0x7C,
    title_romanized: 0x24,
    title_original: 0x28,
    difficulty: 0xAC,
    tags: 0x20,
};

// 0x12a Could be draintime ->
