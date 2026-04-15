use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const MAX_RECENT_FILES: usize = 20;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub main_folder: Option<PathBuf>,
    pub recent_files: Vec<PathBuf>,
    pub window_width: f32,
    pub window_height: f32,
    pub sidebar_width: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            main_folder: None,
            recent_files: Vec::new(),
            window_width: 1000.0,
            window_height: 700.0,
            sidebar_width: 220.0,
        }
    }
}

impl AppConfig {
    pub fn add_recent_file(&mut self, path: PathBuf) {
        self.recent_files.retain(|p| p != &path);
        self.recent_files.insert(0, path);
        self.recent_files.truncate(MAX_RECENT_FILES);
    }

    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("mdnotes").join("config.json"))
    }

    pub fn load() -> Self {
        Self::config_path()
            .and_then(|path| std::fs::read_to_string(path).ok())
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(path, serde_json::to_string_pretty(self).unwrap_or_default());
        }
    }
}
