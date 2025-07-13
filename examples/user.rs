use rosu_memory_lib::init_loop;
use rosu_memory_lib::reader::common::OsuClientKind;
use rosu_memory_lib::reader::user::UserReader;
use rosu_memory_lib::Error;

fn main() -> Result<(), Error> {
    let (mut state, process) = init_loop(500)?;
    let mut user_reader = UserReader::new(&process, &mut state, OsuClientKind::Stable);
    match user_reader.info() {
        Ok(user_info) => println!("Current user info: {user_info:?}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match user_reader.username() {
        Ok(username) => println!("Current username: {username}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match user_reader.pp() {
        Ok(pp) => println!("Current pp: {pp}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match user_reader.accuracy() {
        Ok(accuracy) => println!("Current accuracy: {accuracy}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match user_reader.playcount() {
        Ok(playcount) => println!("Current playcount: {playcount}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match user_reader.rank() {
        Ok(rank) => println!("Current rank: {rank}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match user_reader.playmode() {
        Ok(playmode) => println!("Current playmode: {playmode}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match user_reader.country_code() {
        Ok(country_code) => println!("Current country code: {country_code}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match user_reader.bancho_status() {
        Ok(bancho_status) => println!("Current bancho status: {bancho_status}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match user_reader.id() {
        Ok(id) => println!("Current id: {id}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match user_reader.rankedscore() {
        Ok(rankedscore) => println!("Current rankedscore: {rankedscore}"),
        Err(e) => println!("Error: {e:?}"),
    }
    match user_reader.level() {
        Ok(level) => println!("Current level: {level}"),
        Err(e) => println!("Error: {e:?}"),
    }

    Ok(())
}
