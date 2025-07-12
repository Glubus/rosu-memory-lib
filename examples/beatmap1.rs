use rosu_memory_lib::init_loop;
use rosu_memory_lib::reader::beatmap::stable::file::info;
use rosu_memory_lib::Error;

fn main() -> Result<(), Error> {
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");
    loop {
        match info(&process, &mut state) {
            Ok(beatmap_info) => println!("Current beatmap info: {beatmap_info:?}"),
            Err(e) => println!("Error: {e:?}"),
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
