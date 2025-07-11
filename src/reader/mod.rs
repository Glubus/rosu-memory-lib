pub mod beatmap;
pub mod common;
pub mod gameplay;
pub mod resultscreen;
pub mod user;

use crate::reader::common::stable::memory::get_game_state;
use crate::reader::common::GameState;
use crate::reader::structs::State;
use crate::reader::structs::StaticAddresses;
use crate::Error;
use rosu_mem::process::{Process, ProcessTraits};
use std::time::Duration;
pub mod structs;

static EXCLUDE_WORDS: [&str; 2] = ["umu-run", "waitforexitandrun"];

#[allow(dead_code)]
// Use this function to make callback and get anything you need such as map info or user info or even submit shit
pub fn waiting_for_gamestate<F>(
    p: &Process,
    state: &mut State,
    g_state: GameState,
    callback: Option<F>,
) -> Result<(), Error>
where
    F: Fn(&Process, &mut State) -> Result<(), Error>,
{
    loop {
        if get_game_state(p, state)? == g_state {
            return Ok(());
        }
        if let Some(f) = &callback {
            f(p, state)?;
        }
    }
}

#[allow(dead_code)]
pub fn init_loop(sleep_duration: u64) -> Result<(State, Process), Error> {
    let mut state = State {
        addresses: StaticAddresses::default(),
    };

    loop {
        match Process::initialize("osu!.exe", &EXCLUDE_WORDS) {
            Ok(p) => {
                println!("Found process, pid - {}", p.pid);

                println!("Reading static signatures...");
                match StaticAddresses::new(&p) {
                    Ok(v) => {
                        state.addresses = v;
                        println!("Static addresses read successfully");
                        return Ok((state, p));
                    }
                    Err(e) => {
                        match e {
                            Error::MemoryRead(msg) => {
                                if msg.contains("Process not found") {
                                    println!("Process not found, sleeping for {sleep_duration}ms");
                                    std::thread::sleep(Duration::from_millis(sleep_duration));
                                    continue;
                                }
                                #[cfg(target_os = "windows")]
                                if msg.contains("OS error") {
                                    println!("OS error, sleeping for {sleep_duration}ms");
                                    std::thread::sleep(Duration::from_millis(sleep_duration));
                                    continue;
                                }
                                println!("Unknown error, sleeping for {sleep_duration}ms");
                                std::thread::sleep(Duration::from_millis(sleep_duration));
                            }
                            _ => {
                                println!("Unknown error, sleeping for {sleep_duration}ms");
                                std::thread::sleep(Duration::from_millis(sleep_duration));
                            }
                        }
                        println!("Unknown error, sleeping for {sleep_duration}ms");
                        std::thread::sleep(Duration::from_millis(sleep_duration));
                    }
                }
            }
            Err(_) => {
                println!("Unknown process error, sleeping for {sleep_duration}ms");
                std::thread::sleep(Duration::from_millis(sleep_duration));
            }
        }
    }
}

// Exemple of playing loop
// pub(crate) fn playing(p: &Process, state: &mut State) -> bool {
//     let mode_list = get_mods(p, state);
//     while (GameState::from(get_status(p, state)) == GameState::Playing) {
//         cur_time = reader_gameplay::get_ig_time(p, state);
//         if (cur_time - last_time < 20 && cur_time > 0 && last_time > 0 && last_paused != cur_time) {
//             last_paused = cur_time;
//         }
//         last_time = cur_time;
//         let status = get_status(p, state);
//         let md5 = get_beatmap_md5(p, state);
//         if (last_retries < get_retries(p, state)) {
//             return false;
//         }
//     }
//     true
// }
