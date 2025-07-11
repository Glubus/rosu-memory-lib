pub struct ResultScreenOffset {
    pub ptr: i32,
    pub addr: i32,
    pub score_base: i32,
    pub username: i32,
    pub score: i32,
    pub max_combo: i32,
    pub mode: i32,
    pub hits: ResultScreenHitsOffset,
}

pub(crate) const RESULT_SCREEN_OFFSET: ResultScreenOffset = ResultScreenOffset {
    ptr: 0xb,
    addr: 0x4,
    score_base: 0x38,
    username: 0x28,
    score: 0x78,
    max_combo: 0x68,
    mode: 0x64,
    hits: RESULT_SCREEN_HITS_OFFSET,
};

pub struct ResultScreenHitsOffset {
    pub _300: i32,
    pub _100: i32,
    pub _50: i32,
    pub _miss: i32,
    pub _geki: i32,
    pub _katu: i32,
}

pub(crate) const RESULT_SCREEN_HITS_OFFSET: ResultScreenHitsOffset = ResultScreenHitsOffset {
    _300: 0x8A,
    _100: 0x88,
    _50: 0x8C,
    _miss: 0x92,
    _geki: 0x8E,
    _katu: 0x90,
};
