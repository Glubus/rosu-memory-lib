use std::path::PathBuf;

use crate::common::GameMode;
use crate::reader::beatmap::common::{
    BeatmapInfo, BeatmapLocation, BeatmapMetadata, BeatmapStarRating, BeatmapStats, BeatmapStatus,
    BeatmapTechnicalInfo,
};
use crate::reader::beatmap::stable::memory::{audio, filename, folder};
use crate::reader::beatmap::stable::{beatmap_addr, offset::BEATMAP_OFFSET};
use crate::reader::common::stable::memory::path_folder;
use crate::reader::structs::State;
use crate::Error;
use rosu_map::section::hit_objects::HitObjectKind;
use rosu_map::Beatmap as RmBeatmap;
use rosu_mem::process::{Process, ProcessTraits};

pub fn path(p: &Process, state: &mut State) -> Result<PathBuf, Error> {
    let folder = folder(p, state)?;
    let filename = filename(p, state)?;
    let songs_path = path_folder(p, state)?;

    Ok(songs_path.join(folder).join(filename))
}

pub fn audio_path(p: &Process, state: &mut State) -> Result<PathBuf, Error> {
    let folder = folder(p, state)?;
    let audio = audio(p, state)?;
    let songs_path = path_folder(p, state)?;

    Ok(songs_path.join(folder).join(audio))
}

// generate getters that use the default logic
macro_rules! generate_beatmap_field_getter {
    (
        $( $fn_name:ident : $ret_ty:ty = $field:ident ; )*
    ) => {
        $(
            pub fn $fn_name(p: &Process, state: &mut State) -> Result<$ret_ty, Error> {
                let path = path(p, state)?;
                let b = RmBeatmap::from_path(path)?;
                Ok(b.$field)
            }
        )*
    };
}
// generate getters that use custom logic
macro_rules! generate_beatmap_custom_getter_safe {
    (
        $( $fn_name:ident : $ret_ty:ty = |$b:ident| $body:block )*
    ) => {
        $(
            pub fn $fn_name(p: &Process, state: &mut State) -> Result<$ret_ty, Error> {
                let path = path(p, state)?;
                let $b = RmBeatmap::from_path(path)?;
                $body
            }
        )*
    };
}

generate_beatmap_field_getter! {
    beatmap_id: i32 = beatmap_id;
    beatmap_set_id: i32 = beatmap_set_id;
    author: String = artist;
    creator: String = creator;
    title_romanized: String = title;
    title: String = title_unicode;
    difficulty: String = version;
    tags: String = tags;
    od: f32 = overall_difficulty;
    ar: f32 = approach_rate;
    cs: f32 = circle_size;
    hp: f32 = hp_drain_rate;
}

generate_beatmap_custom_getter_safe! {
    slider_count: i32 = |b| {
        Ok(b.hit_objects.iter()
            .filter(|h| matches!(h.kind, HitObjectKind::Slider(_)))
            .count() as i32)
    }
    object_count: u32 = |b| {
        Ok(b.hit_objects.len() as u32)
    }
    length: i32 = |b| {
        let last = b.hit_objects.last().ok_or_else(|| Error::Other("Empty hitobject list".into()))?;

        let duration = match &last.kind {
            HitObjectKind::Hold(hold_data) => last.start_time + hold_data.duration,
            _ => last.start_time,
        };

        Ok(duration as i32)
    }
    drain_time: i32 = |b| {
        let first = b.hit_objects.first().ok_or_else(|| Error::Other("Empty hitobject list".into()))?;
        let last = b.hit_objects.last().ok_or_else(|| Error::Other("Empty hitobject list".into()))?;

        let duration = match &last.kind {
            HitObjectKind::Hold(hold_data) => last.start_time + hold_data.duration,
            _ => last.start_time,
        };

        Ok((duration - first.start_time) as i32)
    }
    mode: GameMode = |b| {
        Ok(GameMode::from(b.mode as u32))
    }

}

// cant do this in file mode
pub fn status(p: &Process, state: &mut State) -> Result<BeatmapStatus, Error> {
    // cant do this in file mode
    crate::reader::beatmap::stable::memory::status(p, state)
}

pub fn star_rating(p: &Process, state: &mut State) -> Result<BeatmapStarRating, Error> {
    let folder = folder(p, state)?;
    let filename = filename(p, state)?;
    let songs_path = path_folder(p, state)?;
    let path = songs_path.join(folder).join(filename);
    let b = rosu_pp::Beatmap::from_path(path)?;
    let diff_attrs = rosu_pp::Difficulty::new().calculate(&b);
    let diff_dt = rosu_pp::Difficulty::new().mods(64).calculate(&b);
    let diff_ht = rosu_pp::Difficulty::new().mods(256).calculate(&b);
    let no_mod = diff_attrs.stars();
    let dt = diff_dt.stars();
    let ht = diff_ht.stars();
    Ok(BeatmapStarRating { no_mod, dt, ht })
}

pub fn md5(p: &Process, state: &mut State) -> Result<String, Error> {
    // TODO: implement this for now will get from memory
    crate::reader::beatmap::stable::memory::md5(p, state)
}
pub fn stats(p: &Process, state: &mut State) -> Result<BeatmapStats, Error> {
    let beatmap_addr = path(p, state)?;
    let b = RmBeatmap::from_path(beatmap_addr)?;
    Ok(BeatmapStats {
        ar: b.approach_rate,
        od: b.overall_difficulty,
        cs: b.circle_size,
        hp: b.hp_drain_rate,
        length: length(p, state)?,
        star_rating: star_rating(p, state)?,
        object_count: b.hit_objects.len() as i32,
        slider_count: b
            .hit_objects
            .iter()
            .filter(|h| matches!(h.kind, HitObjectKind::Slider(_)))
            .count() as i32,
    })
}

pub fn info(p: &Process, state: &mut State) -> Result<BeatmapInfo, Error> {
    let beatmap_file = path(p, state)?;
    let beatmap_addr = beatmap_addr(p, state)?;
    let b = RmBeatmap::from_path(beatmap_file)?;
    // done like that to be more efficient reading the string one by one would need to reload addr everytime which cost more
    Ok(BeatmapInfo {
        technical: BeatmapTechnicalInfo {
            md5: crate::reader::beatmap::stable::file::md5(p, state)?,
            id: b.beatmap_id,
            set_id: b.beatmap_set_id,
            mode: GameMode::Osu,
            ranked_status: status(p, state)?,
        },
        metadata: BeatmapMetadata {
            author: b.artist,
            creator: b.creator,
            title_romanized: b.title,
            title_original: b.title_unicode,
            difficulty: b.version,
            tags: b.tags,
        },
        stats: BeatmapStats {
            ar: b.approach_rate,
            od: b.overall_difficulty,
            cs: b.circle_size,
            hp: b.hp_drain_rate,
            length: length(p, state)?,
            star_rating: star_rating(p, state)?,
            object_count: b.hit_objects.len() as i32,
            slider_count: b
                .hit_objects
                .iter()
                .filter(|h| matches!(h.kind, HitObjectKind::Slider(_)))
                .count() as i32,
        },
        location: BeatmapLocation {
            folder: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.folder)?,
            filename: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.filename)?,
            audio: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.audio)?,
            cover: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.cover)?,
        },
    })
}
