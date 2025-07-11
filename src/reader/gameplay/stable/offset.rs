pub struct GameplayOffset {
    pub ptr: i32,
    pub addr: i32,
    pub base: i32,
    pub base2: i32,
    pub ruleset: i32,
    pub hp_base: i32,
    pub score: i32,
    pub mods: i32,
    pub mods_xor: i32,
    pub mods_xor2: i32,
    pub combo: i32,
    pub max_combo: i32,
    pub hp: i32,
    pub username: i32,
    pub retries: i32,
    pub hits: GameplayHitsOffset,
}

pub const GAMEPLAY_OFFSET: GameplayOffset = GameplayOffset {
    ptr: 0xb,
    addr: 0x4,
    base: 0x68,
    base2: 0x38,
    ruleset: 0x33,
    hp_base: 0x40,
    score: 0x78,
    mods: 0x1C,
    mods_xor: 0xc,
    mods_xor2: 0x8,
    combo: 0x94,
    max_combo: 0x68,
    hp: 0x1C,
    username: 0x28,
    retries: 0x8,
    hits: GAMEPLAY_HITS_OFFSET,
};

pub struct GameplayHitsOffset {
    pub _300: i32,
    pub _100: i32,
    pub _50: i32,
    pub _miss: i32,
    pub _geki: i32,
    pub _katu: i32,
}

pub const GAMEPLAY_HITS_OFFSET: GameplayHitsOffset = GameplayHitsOffset {
    _300: 0x8a,
    _100: 0x88,
    _50: 0x8c,
    _miss: 0x92,
    _geki: 0x8e,
    _katu: 0x90,
};
