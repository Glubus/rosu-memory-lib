use rosu_memory_lib::init_loop;
use rosu_memory_lib::reader::common::stable::memory::is_paused;
use rosu_memory_lib::Error;

fn main() -> Result<(), Error> {
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");
    loop {
        match is_paused(&process, &mut state) {
            Ok(paused) => println!("Current paused: {paused:?}"),
            Err(e) => println!("Error: {e:?}"),
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
