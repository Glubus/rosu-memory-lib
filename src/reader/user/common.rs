#[derive(Debug, Clone)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub pp: i32,
    pub rankedscore: i64,
    pub level: f32,
    pub playcount: i32,
    pub rank: i32,
    pub playmode: i32,
    pub accuracy: f64,
    pub country_code: i32,
    pub bancho_status: i32,
}
