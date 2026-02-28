pub mod common;
pub mod stable;

use crate::impl_osu_accessor;
use crate::reader::common::OsuClientKind;
use crate::reader::overlay::common::KeyOverlay;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::Process;

/// Lecteur des données de l'overlay touches
pub struct OverlayReader<'a> {
    pub process: &'a Process,
    pub state: &'a mut State,
    pub osu_type: OsuClientKind,
}

impl<'a> OverlayReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuClientKind) -> Self {
        Self {
            process: p,
            state,
            osu_type,
        }
    }

    impl_osu_accessor! {
        fn info() -> KeyOverlay => stable::memory::info,
    }
}
