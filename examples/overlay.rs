use rosu_memory_lib::init_loop;
use rosu_memory_lib::reader::overlay::stable::memory::key_overlay_std;
use rosu_memory_lib::Error;

fn main() -> Result<(), Error> {
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");
    loop {
        match key_overlay_std(&process, &mut state) {
            Ok(key_overlay) => println!("Current key overlay: {key_overlay:?}"),
            Err(e) => println!("Error: {e:?}"),
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
