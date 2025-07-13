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
    accuracy: 0x4,
    rankedscore: 0xC,
    username: 0x30,
    id: 0x70,
    level: 0x74,
    playcount: 0x7C, // start read
    playmode: 0x80,
    rank: 0x84,
    pp: 0x88,
    bancho_status: 0x8c, // end read
    country_code: 0x9c,

};
