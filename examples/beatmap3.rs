use rosu_memory_lib::init_loop;
use rosu_memory_lib::reader::beatmap::BeatmapReader;
use rosu_memory_lib::reader::common::OsuClientKind;
use rosu_memory_lib::Error;

fn main() -> Result<(), Error> {
    let (mut state, process) = init_loop(500)?;
    let mut beatmap_reader = BeatmapReader::new(&process, &mut state, OsuClientKind::Stable)?;
    match beatmap_reader.audio_path() {
        Ok(audio_path) => println!("Current beatmap audio path: {audio_path:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.path() {
        Ok(path) => println!("Current beatmap path: {path:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.md5() {
        Ok(md5) => println!("Current beatmap md5: {md5:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.id() {
        Ok(id) => println!("Current beatmap id: {id:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.set_id() {
        Ok(set_id) => println!("Current beatmap set id: {set_id:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.mode() {
        Ok(mode) => println!("Current beatmap mode: {mode:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.tags() {
        Ok(tags) => println!("Current beatmap tags: {tags:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.length() {
        Ok(length) => println!("Current beatmap length: {length:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.drain_time() {
        Ok(drain_time) => println!("Current beatmap drain time: {drain_time:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.author() {
        Ok(author) => println!("Current beatmap author: {author:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.creator() {
        Ok(creator) => println!("Current beatmap creator: {creator:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.title_romanized() {
        Ok(title_romanized) => println!("Current beatmap title romanized: {title_romanized:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.title() {
        Ok(title) => println!("Current beatmap title: {title:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.difficulty() {
        Ok(difficulty) => println!("Current beatmap difficulty: {difficulty:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.od() {
        Ok(od) => println!("Current beatmap od: {od:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.ar() {
        Ok(ar) => println!("Current beatmap ar: {ar:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.cs() {
        Ok(cs) => println!("Current beatmap cs: {cs:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.hp() {
        Ok(hp) => println!("Current beatmap hp: {hp:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.object_count() {
        Ok(object_count) => println!("Current beatmap object count: {object_count:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.slider_count() {
        Ok(slider_count) => println!("Current beatmap slider count: {slider_count:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.folder() {
        Ok(folder) => println!("Current beatmap folder: {folder:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.filename() {
        Ok(filename) => println!("Current beatmap filename: {filename:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.audio() {
        Ok(audio) => println!("Current beatmap audio: {audio:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.cover() {
        Ok(cover) => println!("Current beatmap cover: {cover:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.status() {
        Ok(status) => println!("Current beatmap status: {status:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match beatmap_reader.star_rating() {
        Ok(star_rating) => println!("Current beatmap star rating: {star_rating:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    Ok(())
}
