use rosu_memory_lib::init_loop;
use rosu_memory_lib::reader::gameplay::stable::memory::get_ig_time;
use rosu_memory_lib::Error;

fn main() -> Result<(), Error> {
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");
    loop {
        match get_ig_time(&process, &mut state) {
            Ok(ig_time) => println!("Current ig time: {ig_time:?}"),
            Err(e) => println!("Error: {e:?}"),
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
