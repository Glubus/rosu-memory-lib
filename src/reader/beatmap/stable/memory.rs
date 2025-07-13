use crate::common::GameMode;
use crate::generate_offset_getter;
use crate::reader::beatmap::common::{
    BeatmapInfo, BeatmapLocation, BeatmapMetadata, BeatmapStats, BeatmapStatus,
    BeatmapTechnicalInfo,
};
use crate::reader::beatmap::stable::{beatmap_addr, offset::BEATMAP_OFFSET};
use crate::reader::helpers::{read_f32, read_i32, read_string, read_u32};
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};

generate_offset_getter! {
    id: i32 = read_i32(BEATMAP_OFFSET.technical.id, beatmap_addr);
    set_id: i32 = read_i32(BEATMAP_OFFSET.technical.set_id, beatmap_addr);
    tags: String = read_string(BEATMAP_OFFSET.metadata.tags, beatmap_addr);
    length: i32 = read_i32(BEATMAP_OFFSET.stats.total_length, beatmap_addr);
    drain_time: i32 = read_i32(BEATMAP_OFFSET.stats.drain_time, beatmap_addr);
    author: String = read_string(BEATMAP_OFFSET.metadata.author, beatmap_addr);
    creator: String = read_string(BEATMAP_OFFSET.metadata.creator, beatmap_addr);
    md5: String = read_string(BEATMAP_OFFSET.technical.md5, beatmap_addr);
    title_romanized: String = read_string(BEATMAP_OFFSET.metadata.title_romanized, beatmap_addr);
    title: String = read_string(BEATMAP_OFFSET.metadata.title_original, beatmap_addr);
    difficulty: String = read_string(BEATMAP_OFFSET.metadata.difficulty, beatmap_addr);
    od: f32 = read_f32(BEATMAP_OFFSET.stats.od, beatmap_addr);
    ar: f32 = read_f32(BEATMAP_OFFSET.stats.ar, beatmap_addr);
    cs: f32 = read_f32(BEATMAP_OFFSET.stats.cs, beatmap_addr);
    hp: f32 = read_f32(BEATMAP_OFFSET.stats.hp, beatmap_addr);
    object_count: u32 = read_u32(BEATMAP_OFFSET.stats.object_count, beatmap_addr);
    slider_count: i32 = read_i32(BEATMAP_OFFSET.stats.slider_count, beatmap_addr);
    folder: String = read_string(BEATMAP_OFFSET.location.folder, beatmap_addr);
    filename: String = read_string(BEATMAP_OFFSET.location.filename, beatmap_addr);
    audio: String = read_string(BEATMAP_OFFSET.location.audio, beatmap_addr);
    cover: String = read_string(BEATMAP_OFFSET.location.cover, beatmap_addr);
    mode: GameMode = read_i32(BEATMAP_OFFSET.technical.mode, beatmap_addr);
    status: BeatmapStatus = read_i32(BEATMAP_OFFSET.technical.ranked_status, beatmap_addr);
}

pub fn stats(p: &Process, state: &mut State) -> Result<BeatmapStats, Error> {
    let beatmap_addr = beatmap_addr(p, state)?;
    // faster than using read_fn because we dont need to reload addr everytime
    Ok(BeatmapStats {
        ar: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.ar)?,
        od: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.od)?,
        cs: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.cs)?,
        hp: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.hp)?,
        length: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.total_length)?,
        star_rating: crate::reader::beatmap::stable::file::star_rating(p, state)?,
        object_count: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.object_count)?,
        slider_count: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.slider_count)?,
    })
}

pub fn info(p: &Process, state: &mut State) -> Result<BeatmapInfo, Error> {
    let beatmap_addr = beatmap_addr(p, state)?;
    // done like that to be more efficient reading the string one by one would need to reload addr everytime which cost more
    Ok(BeatmapInfo {
        technical: BeatmapTechnicalInfo {
            md5: p.read_string(beatmap_addr + BEATMAP_OFFSET.technical.md5)?,
            id: p.read_i32(beatmap_addr + BEATMAP_OFFSET.technical.id)?,
            set_id: p.read_i32(beatmap_addr + BEATMAP_OFFSET.technical.set_id)?,
            mode: GameMode::Osu,
            ranked_status: BeatmapStatus::from(
                p.read_i32(beatmap_addr + BEATMAP_OFFSET.technical.ranked_status)?,
            ),
        },
        metadata: BeatmapMetadata {
            author: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.author)?,
            creator: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.creator)?,
            title_romanized: p
                .read_string(beatmap_addr + BEATMAP_OFFSET.metadata.title_romanized)?,
            title_original: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.title_original)?,
            difficulty: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.difficulty)?,
            tags: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.tags)?,
        },
        stats: BeatmapStats {
            ar: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.ar)?,
            od: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.od)?,
            cs: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.cs)?,
            hp: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.hp)?,
            length: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.total_length)?,
            star_rating: crate::reader::beatmap::stable::file::star_rating(p, state)?,
            object_count: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.object_count)?,
            slider_count: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.slider_count)?,
        },
        location: BeatmapLocation {
            folder: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.folder)?,
            filename: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.filename)?,
            audio: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.audio)?,
            cover: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.cover)?,
        },
    })
}
