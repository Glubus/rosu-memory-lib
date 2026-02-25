use crate::reader::structs::Hit;

#[derive(Debug, Clone)]
pub struct GameplayInfo {
    pub score: i32,
    pub mods: u32,
    pub combo: i16,
    pub max_combo: i16,
    pub hp: f64,
    pub username: String,
    pub ig_time: i32,
    pub retries: i32,
    pub hits: Hit,
}
