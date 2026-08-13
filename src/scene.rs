use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::Res;

#[derive(Debug, Clone, Deserialize)]
pub struct Scene {
    pub name: String,
    pub automaton: String,
    pub width: usize,
    pub height: usize,
    pub fps: Option<u64>,
    #[serde(default)]
    pub initial: Vec<String>,
}

impl Scene {
    pub fn from_path(path: impl AsRef<Path>) -> Res<Self> {
        let raw = fs::read_to_string(path)?;
        Ok(toml::from_str(&raw)?)
    }
}
