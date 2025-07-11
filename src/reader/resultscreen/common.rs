use crate::reader::common::GameMode;
use crate::reader::structs::Hit;

#[derive(Debug, Default)]
pub struct ResultScreenInfo {
    pub username: String,
    pub mode: GameMode,
    pub max_combo: i16,
    pub score: i32,
    pub hit: Hit,
    pub accuracy: f64,
}
