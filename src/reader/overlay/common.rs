pub struct Key {
    pub pressed: i32,
    pub count: i32,
}

pub struct KeyOverlay {
    pub key_1: Key,
    pub key_2: Key,
    pub mouse_1: Key,
    pub mouse_2: Key,
}
