use rosu_memory_lib::init_loop;
use rosu_memory_lib::reader::common::OsuClientKind;
use rosu_memory_lib::reader::resultscreen::ResultScreenReader;
use rosu_memory_lib::Error;

fn main() -> Result<(), Error> {
    let (mut state, process) = init_loop(500)?;
    let mut resultscreen_reader =
        ResultScreenReader::new(&process, &mut state, OsuClientKind::Stable);
    match resultscreen_reader.accuracy() {
        Ok(accuracy) => println!("Current accuracy: {accuracy}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match resultscreen_reader.max_combo() {
        Ok(max_combo) => println!("Current max combo: {max_combo}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match resultscreen_reader.score() {
        Ok(score) => println!("Current score: {score}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match resultscreen_reader.username() {
        Ok(username) => println!("Current username: {username}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match resultscreen_reader.mode() {
        Ok(mode) => println!("Current mode: {mode:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match resultscreen_reader.hits_miss() {
        Ok(hits_miss) => println!("Current hits miss: {hits_miss}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match resultscreen_reader.hits_geki() {
        Ok(hits_geki) => println!("Current hits geki: {hits_geki}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match resultscreen_reader.hits_katu() {
        Ok(hits_katu) => println!("Current hits katu: {hits_katu}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match resultscreen_reader.hits_300() {
        Ok(hits_300) => println!("Current hits 300: {hits_300}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match resultscreen_reader.hits_100() {
        Ok(hits_100) => println!("Current hits 100: {hits_100}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match resultscreen_reader.hits_50() {
        Ok(hits_50) => println!("Current hits 50: {hits_50}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match resultscreen_reader.hits() {
        Ok(hits) => println!("Current hits: {hits:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match resultscreen_reader.info() {
        Ok(resultscreen_info) => println!("Current resultscreen info: {resultscreen_info:?}"),
        Err(e) => println!("Error: {e:?}"),
    }

    Ok(())
}
