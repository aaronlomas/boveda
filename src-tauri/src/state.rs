use std::sync::Mutex;
use std::path::PathBuf;
pub use boveda_core::engine::BovedaEngine;

pub struct AppState {
    pub db_path: PathBuf,
    pub engine: Mutex<Option<BovedaEngine>>,
}

impl AppState {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
            engine: Mutex::new(None),
        }
    }
}
