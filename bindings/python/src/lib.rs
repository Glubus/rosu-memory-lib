use pyo3::prelude::*;
mod reader;
mod common;

use common::{PyProcess, PyState};
use reader::beatmap::stable::memory::get_beatmap_info;

#[pyfunction]
fn init_loop(interval: u64) -> PyResult<(PyState, PyProcess)> {
    match rosu_memory_lib::reader::init_loop(interval) {
        Ok((state, process)) => Ok((PyState(state), PyProcess(process))),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
}

/// Python bindings for rosu-memory-lib
#[pymodule]
fn rosu_memory_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_loop, m)?)?;
    m.add_function(wrap_pyfunction!(get_beatmap_info, m)?)?;
    Ok(())
} 