pub struct UserProfileOffset {
    pub ptr: i32,
    pub id: i32,
    pub username: i32,
    pub pp: i32,
    pub rankedscore: i32,
    pub level: i32,
    pub playcount: i32,
    pub rank: i32,
    pub playmode: i32,
    pub accuracy: i32,
    pub country_code: i32,
    pub bancho_status: i32,
}

pub(crate) const USER_PROFILE_OFFSET: UserProfileOffset = UserProfileOffset {
    ptr: 0x7,
    id: 0x70,
    username: 0x30,
    pp: 0x88,
    rankedscore: 0xC,
    level: 0x74,
    playcount: 0x7C,
    rank: 0x84,
    playmode: 0x80,
    accuracy: 0x4,
    country_code: 0x9c,
    bancho_status: 0x8c,
};
