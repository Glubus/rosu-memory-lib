use rosu_memory_lib::Error;
use rosu_mem::process::Process;
use rosu_memory_lib::init_loop;
use rosu_memory_lib::reader::beatmap::stable::file::get_beatmap_path;
use rosu_memory_lib::reader::common::stable::memory::get_menu_mods;
use rosu_memory_lib::reader::structs::State;
use rosu_mods::GameModsLegacy;
use rosu_pp::Beatmap;
use rosu_pp::{Difficulty, Performance};
use std::time::Duration;

/// This example demonstrates how to calculate PP (Performance Points) in real-time
/// for the currently selected beatmap in osu!stable, taking into account active mods.
///
/// The program will:
/// 1. Initialize a connection to the osu! process
/// 2. Continuously monitor the selected beatmap and mods
/// 3. Calculate and display the PP value for the current beatmap + mods combination
///
/// Optimizations:
/// - Caches the beatmap to avoid reloading the same file multiple times
/// - Only updates PP display when values actually change
/// - Converts mod bits to human-readable format
///
///   Represents the current state of the PP calculator
struct CalculatorState {
    current_pp: f64,
    current_mods: i32,
    current_beatmap: Beatmap,
    current_beatmap_path: String,
}

impl CalculatorState {
    /// Creates a new calculator state with default values
    fn new() -> Self {
        Self {
            current_pp: 0.0,
            current_mods: 0,
            current_beatmap: Beatmap::default(),
            current_beatmap_path: String::new(),
        }
    }

    /// Updates the mods if they have changed and returns whether an update occurred
    fn update_mods(&mut self, new_mods: i32) -> bool {
        if new_mods != self.current_mods {
            self.current_mods = new_mods;

            // Convert mod bits to human-readable format
            let mods_readable = GameModsLegacy::from_bits(self.current_mods as u32).to_string();
            println!("Mods: {mods_readable}");
            true
        } else {
            false
        }
    }

    /// Attempts to load a new beatmap if the path has changed
    fn update_beatmap(&mut self, new_path: String) -> Result<bool, Error> {
        if new_path != self.current_beatmap_path {
            println!("Loading new beatmap: {new_path}");

            // Load and validate the new beatmap
            let beatmap = Beatmap::from_path(&new_path)?;
            if let Err(suspicion) = beatmap.check_suspicion() {
                eprintln!("Warning: Suspicious beatmap detected: {suspicion:?}");
                return Ok(false);
            }

            // Update cached beatmap
            self.current_beatmap = beatmap;
            self.current_beatmap_path = new_path;
            println!("Beatmap loaded successfully!");
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Calculates and updates PP if the value has changed
    fn update_pp(&mut self) {
        let diff_attrs = Difficulty::new()
            .mods(self.current_mods as u32)
            .calculate(&self.current_beatmap);

        let new_pp = Performance::new(diff_attrs).calculate().pp();
        if (new_pp - self.current_pp).abs() > f64::EPSILON {
            self.current_pp = new_pp;
            println!("PP for current beatmap: {:.2}", self.current_pp);
        }
    }
}

/// Processes a single iteration of the monitoring loop
fn process_game_state(
    process: &Process,
    state: &mut State,
    calc_state: &mut CalculatorState,
) -> Result<(), Error> {
    let mut mods_changed = false;
    match get_beatmap_path(process, state) {
        Ok(beatmap_path) => {
            // Update mods if they changed
            if let Ok(new_mods) = get_menu_mods(process, state) {
                mods_changed = calc_state.update_mods(new_mods);
            }

            // Update beatmap if path changed and mods changed else it's useless to recalculate
            if calc_state.update_beatmap(beatmap_path)? && mods_changed {
                calc_state.update_pp();
            }
        }
        Err(e) => {
            eprintln!("Failed to read beatmap path: {e}");
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    // Initialize connection to osu! process, checking every 500ms
    let (mut state, process) = init_loop(500)?;
    println!("Successfully connected to osu! process!");

    // Initialize calculator state
    let mut calc_state = CalculatorState::new();

    // Main monitoring loop
    loop {
        if let Err(e) = process_game_state(&process, &mut state, &mut calc_state) {
            eprintln!("Error during processing: {e}");
        }

        // Wait before next check to avoid excessive CPU usage
        std::thread::sleep(Duration::from_millis(1000));
    }
}
