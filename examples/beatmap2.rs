use rosu_memory_lib::init_loop;
use rosu_memory_lib::reader::beatmap::BeatmapReader;
use rosu_memory_lib::reader::common::OsuClientKind;
use rosu_memory_lib::Error;

fn main() -> Result<(), Error> {
    let (mut state, process) = init_loop(500)?;
    let mut beatmap_reader = BeatmapReader::new(&process, &mut state, OsuClientKind::Stable)?;
    loop {
        match beatmap_reader.info() {
            Ok(beatmap_info) => println!("Current beatmap info: {beatmap_info:?}"),
            Err(e) => println!("Error: {e:?}"),
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
