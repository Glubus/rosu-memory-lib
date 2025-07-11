pub mod error;
pub mod reader;

pub use error::{Error, Result};
pub use reader::init_loop;
pub use reader::waiting_for_gamestate;

// Re-export commonly used items
pub use reader::beatmap;
pub use reader::common;
pub use reader::resultscreen;
