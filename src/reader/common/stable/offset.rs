pub struct CommonOffset {
    pub settings_ptr: i32,
    pub settings_addr: i32,
    pub path: i32,
    pub status: i32,
    pub mods_ptr: i32,
    pub ig_time: i32,
}

pub(crate) const COMMON_OFFSET: CommonOffset = CommonOffset {
    settings_ptr: 0x8,
    settings_addr: 0xb8,
    path: 0x4,
    status: 0x4,
    mods_ptr: 0x9,
    ig_time: 0x5,
};
