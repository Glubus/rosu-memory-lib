pub mod stable;

use crate::impl_osu_accessor;
use crate::reader::common::OsuClientKind;
use crate::reader::resultscreen::stable::ResultScreenInfo;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::Process;

/// Lecteur des données de l'écran de résultats
pub struct ResultScreenReader<'a> {
    pub process: &'a Process,
    pub state: &'a mut State,
    pub osu_type: OsuClientKind,
}

impl<'a> ResultScreenReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuClientKind) -> Self {
        Self {
            process: p,
            state,
            osu_type,
        }
    }

    impl_osu_accessor! {
        fn info() -> ResultScreenInfo => stable::memory::info,
    }
}
