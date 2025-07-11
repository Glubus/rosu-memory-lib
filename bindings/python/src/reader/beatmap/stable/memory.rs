use rosu_memory_lib::reader::beatmap::common::BeatmapInfo;
use pyo3::prelude::*;
use rosu_memory_lib::reader::beatmap::common::BeatmapLocation;
use rosu_memory_lib::reader::beatmap::common::BeatmapStats;
use rosu_memory_lib::reader::beatmap::common::BeatmapStarRating;
use rosu_memory_lib::reader::beatmap::common::BeatmapMetadata;
use rosu_memory_lib::reader::beatmap::common::BeatmapTechnicalInfo;
use crate::common::{PyProcess, PyState};

#[pyclass]
#[derive(Debug)]
pub struct PyBeatmapInfo {
    pub metadata: PyBeatmapMetadata,
    pub location: PyBeatmapLocation,
    pub stats: PyBeatmapStats,
    pub technical: PyBeatmapTechnicalInfo,
}

#[pymethods]
impl PyBeatmapInfo {
    #[getter]
    fn metadata(&self) -> PyResult<PyBeatmapMetadata> {
        Ok(PyBeatmapMetadata::from(self.metadata.clone()))
    }

    #[getter]
    fn location(&self) -> PyResult<PyBeatmapLocation> {
        Ok(PyBeatmapLocation::from(self.location.clone()))
    }

    #[getter]
    fn stats(&self) -> PyResult<PyBeatmapStats> {
        Ok(PyBeatmapStats::from(self.stats.clone()))
    }

    #[getter]
    fn technical(&self) -> PyResult<PyBeatmapTechnicalInfo> {
        Ok(PyBeatmapTechnicalInfo::from(self.technical.clone()))
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

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyBeatmapMetadata {
    pub author: String,
    pub creator: String,
    pub title_romanized: String,
    pub title_original: String,
    pub difficulty: String,
    pub tags: String,
}

#[pymethods]
impl PyBeatmapMetadata {
    #[getter]
    fn author(&self) -> PyResult<String> {
        Ok(self.author.clone())
    }

    #[getter]
    fn creator(&self) -> PyResult<String> {
        Ok(self.creator.clone())
    }

    #[getter]
    fn title_romanized(&self) -> PyResult<String> {
        Ok(self.title_romanized.clone())
    }

    #[getter]
    fn title_original(&self) -> PyResult<String> {
        Ok(self.title_original.clone())
    }

    #[getter]
    fn difficulty(&self) -> PyResult<String> {
        Ok(self.difficulty.clone())
    }

    #[getter]
    fn tags(&self) -> PyResult<String> {
        Ok(self.tags.clone())
    }
}

impl From<BeatmapMetadata> for PyBeatmapMetadata {
    fn from(metadata: BeatmapMetadata) -> Self {
        PyBeatmapMetadata {
            author: metadata.author,
            creator: metadata.creator,
            title_romanized: metadata.title_romanized,
            title_original: metadata.title_original,
            difficulty: metadata.difficulty,
            tags: metadata.tags,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyBeatmapLocation {
    pub folder: String,
    pub filename: String,
    pub audio: String,
    pub cover: String,
}

#[pymethods]
impl PyBeatmapLocation {
    #[getter]
    fn folder(&self) -> PyResult<String> {
        Ok(self.folder.clone())
    }

    #[getter]
    fn filename(&self) -> PyResult<String> {
        Ok(self.filename.clone())
    }

    #[getter]
    fn audio(&self) -> PyResult<String> {
        Ok(self.audio.clone())
    }

    #[getter]
    fn cover(&self) -> PyResult<String> {
        Ok(self.cover.clone())
    }
}

impl From<BeatmapLocation> for PyBeatmapLocation {
    fn from(location: BeatmapLocation) -> Self {
        PyBeatmapLocation {
            folder: location.folder,
            filename: location.filename,
            audio: location.audio,
            cover: location.cover,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyBeatmapStarRating {
    pub no_mod: f64,
    pub dt: f64,
    pub ht: f64,
}

#[pymethods]
impl PyBeatmapStarRating {
    #[getter]
    fn no_mod(&self) -> PyResult<f64> {
        Ok(self.no_mod)
    }

    #[getter]
    fn dt(&self) -> PyResult<f64> {
        Ok(self.dt)
    }

    #[getter]
    fn ht(&self) -> PyResult<f64> {
        Ok(self.ht)
    }
}

impl From<BeatmapStarRating> for PyBeatmapStarRating {
    fn from(star_rating: BeatmapStarRating) -> Self {
        PyBeatmapStarRating {
            no_mod: star_rating.no_mod,
            dt: star_rating.dt,
            ht: star_rating.ht,
        }
    }
}

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

#[pymethods]
impl PyBeatmapStats {
    #[getter]
    fn ar(&self) -> PyResult<f32> {
        Ok(self.ar)
    }

    #[getter]
    fn od(&self) -> PyResult<f32> {
        Ok(self.od)
    }

    #[getter]
    fn cs(&self) -> PyResult<f32> {
        Ok(self.cs)
    }

    #[getter]
    fn hp(&self) -> PyResult<f32> {
        Ok(self.hp)
    }

    #[getter]
    fn total_length(&self) -> PyResult<i32> {
        Ok(self.total_length)
    }

    #[getter]
    fn star_rating(&self) -> PyResult<PyBeatmapStarRating> {
        Ok(self.star_rating.clone())
    }

    #[getter]
    fn object_count(&self) -> PyResult<i32> {
        Ok(self.object_count)
    }

    #[getter]
    fn slider_count(&self) -> PyResult<i32> {
        Ok(self.slider_count)
    }
}

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

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyBeatmapTechnicalInfo {
    pub md5: String,
    pub id: i32,
    pub set_id: i32,
    pub mode: String,
    pub ranked_status: String,
}

#[pymethods]
impl PyBeatmapTechnicalInfo {
    #[getter]
    fn md5(&self) -> PyResult<String> {
        Ok(self.md5.clone())
    }

    #[getter]
    fn id(&self) -> PyResult<i32> {
        Ok(self.id)
    }

    #[getter]
    fn set_id(&self) -> PyResult<i32> {
        Ok(self.set_id)
    }

    #[getter]
    fn mode(&self) -> PyResult<String> {
        Ok(self.mode.clone())
    }

    #[getter]
    fn ranked_status(&self) -> PyResult<String> {
        Ok(self.ranked_status.clone())
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

#[pyfunction]
pub fn get_beatmap_info(process: &PyProcess, state: &mut PyState) -> PyResult<PyBeatmapInfo> 
{
    match rosu_memory_lib::reader::beatmap::stable::memory::get_beatmap_info(&process.0, &mut state.0) {
        Ok(beatmap_info) => {
            let bm = PyBeatmapInfo::from(beatmap_info);
            Ok(bm)
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
    }
}