#[derive(Debug, Clone, Copy)]
pub struct Key {
    pub pressed: bool,
    pub count: i32,
}
#[derive(Debug, Clone, Copy)]
pub struct KeyOverlay {
    pub key_1: Key,
    pub key_2: Key,
    pub mouse_1: Key,
    pub mouse_2: Key,
}
