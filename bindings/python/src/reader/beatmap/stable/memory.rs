use rosu_memory_lib::reader::beatmap::common::BeatmapInfo;
use pyo3::prelude::*;
use rosu_memory_lib::reader::beatmap::common::BeatmapLocation;
use rosu_memory_lib::reader::beatmap::common::BeatmapStats;
use rosu_memory_lib::reader::beatmap::common::BeatmapStarRating;
use rosu_memory_lib::reader::beatmap::common::BeatmapMetadata;
use rosu_memory_lib::reader::beatmap::common::BeatmapTechnicalInfo;
use crate::common::{PyProcess, PyState};
use crate::{py_struct, py_struct_numeric, py_struct_main, py_from_impl_direct, py_getters};

// Structures avec champs String (nécessitent clone)
py_struct! {
    #[pyclass]
    pub struct PyBeatmapMetadata {
        author: String,
        creator: String,
        title_romanized: String,
        title_original: String,
        difficulty: String,
        tags: String,
    }
}

py_struct! {
    #[pyclass]
    pub struct PyBeatmapLocation {
        folder: String,
        filename: String,
        audio: String,
        cover: String,
    }
}

// Structures avec champs numériques (pas de clone nécessaire)
py_struct_numeric! {
    #[pyclass]
    pub struct PyBeatmapStarRating {
        no_mod: f64,
        dt: f64,
        ht: f64,
    }
}

// Implémenter Copy pour PyBeatmapStarRating
impl Copy for PyBeatmapStarRating {}

// Structure mixte (numérique + complexe) - Structure manuelle, getters automatiques
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyBeatmapStats {
    pub ar: f32,
    pub od: f32,
    pub cs: f32,
    pub hp: f32,
    pub total_length: i32,
    pub star_rating: PyBeatmapStarRating,
    pub object_count: i32,
    pub slider_count: i32,
}

py_getters! {
    impl PyBeatmapStats {
        ar: f32,
        od: f32,
        cs: f32,
        hp: f32,
        total_length: i32,
        star_rating: PyBeatmapStarRating,
        object_count: i32,
        slider_count: i32,
    }
}

py_struct! {
    #[pyclass]
    pub struct PyBeatmapTechnicalInfo {
        md5: String,
        id: i32,
        set_id: i32,
        mode: String,
        ranked_status: String,
    }
}

// Structure principale avec des champs complexes
py_struct_main! {
    #[pyclass]
    pub struct PyBeatmapInfo {
        metadata: PyBeatmapMetadata,
        location: PyBeatmapLocation,
        stats: PyBeatmapStats,
        technical: PyBeatmapTechnicalInfo,
    }
}

// Implémentations From avec les macros
py_from_impl_direct! {
    PyBeatmapMetadata => BeatmapMetadata {
        author,
        creator,
        title_romanized,
        title_original,
        difficulty,
        tags,
    }
}

py_from_impl_direct! {
    PyBeatmapLocation => BeatmapLocation {
        folder,
        filename,
        audio,
        cover,
    }
}

py_from_impl_direct! {
    PyBeatmapStarRating => BeatmapStarRating {
        no_mod,
        dt,
        ht,
    }
}

// Implémentations From manuelles pour les cas complexes
impl From<BeatmapStats> for PyBeatmapStats {
    fn from(stats: BeatmapStats) -> Self {
        PyBeatmapStats {
            ar: stats.ar,
            od: stats.od,
            cs: stats.cs,
            hp: stats.hp,
            total_length: stats.total_length,
            star_rating: PyBeatmapStarRating::from(stats.star_rating),
            object_count: stats.object_count,
            slider_count: stats.slider_count,
        }
    }
}

impl From<BeatmapTechnicalInfo> for PyBeatmapTechnicalInfo {
    fn from(technical: BeatmapTechnicalInfo) -> Self {
        PyBeatmapTechnicalInfo {
            md5: technical.md5,
            id: technical.id,
            set_id: technical.set_id,
            mode: technical.mode.to_string(),
            ranked_status: technical.ranked_status.to_string(),
        }
    }
}

impl From<BeatmapInfo> for PyBeatmapInfo {
    fn from(info: BeatmapInfo) -> Self {
        PyBeatmapInfo {
            metadata: PyBeatmapMetadata::from(info.metadata),
            location: PyBeatmapLocation::from(info.location),
            stats: PyBeatmapStats::from(info.stats),
            technical: PyBeatmapTechnicalInfo::from(info.technical),
        }
    }
}

// Fonction avec conversion automatique
#[pyfunction]
pub fn get_beatmap_info(process: &PyProcess, state: &mut PyState) -> PyResult<PyBeatmapInfo> {
    match rosu_memory_lib::reader::beatmap::stable::memory::get_beatmap_info(&process.0, &mut state.0) {
        Ok(result) => Ok(PyBeatmapInfo::from(result)),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
    }
}