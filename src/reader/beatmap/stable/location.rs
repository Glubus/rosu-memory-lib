use crate::reader::beatmap::stable::offset::BEATMAP_OFFSET;
use crate::reader::beatmap::stable::read_from_beatmap_ptr_string;
use crate::reader::structs::State;
use crate::Error;
use rosu_mem::process::Process;

pub fn get_folder(p: &Process, state: &mut State) -> Result<String, Error> {
    read_from_beatmap_ptr_string(p, state, BEATMAP_OFFSET.location.folder)
}
pub fn get_filename(p: &Process, state: &mut State) -> Result<String, Error> {
    read_from_beatmap_ptr_string(p, state, BEATMAP_OFFSET.location.filename)
}

pub fn get_audio(p: &Process, state: &mut State) -> Result<String, Error> {
    read_from_beatmap_ptr_string(p, state, BEATMAP_OFFSET.location.audio)
}

pub fn get_cover(p: &Process, state: &mut State) -> Result<String, Error> {
    read_from_beatmap_ptr_string(p, state, BEATMAP_OFFSET.location.cover)
}
