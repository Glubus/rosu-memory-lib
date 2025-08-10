use rosu_memory_lib::init_loop;
use rosu_memory_lib::reader::common::stable::memory::is_watching_replay;
use rosu_memory_lib::Error;

fn main() -> Result<(), Error> {
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");
    loop {
        match is_watching_replay(&process, &mut state) {
            Ok(replay) => println!("Current replay: {replay:?}"),
            Err(e) => println!("Error: {e:?}"),
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
