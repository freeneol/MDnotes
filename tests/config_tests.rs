use mdnotes::io::config::AppConfig;
use std::path::PathBuf;

#[test]
fn test_config_default() {
    let config = AppConfig::default();
    assert!(config.main_folder.is_none());
    assert!(config.recent_files.is_empty());
    assert_eq!(config.window_width, 1000.0);
    assert_eq!(config.window_height, 700.0);
    assert_eq!(config.sidebar_width, 220.0);
}

#[test]
fn test_config_add_recent_file() {
    let mut config = AppConfig::default();
    config.add_recent_file(PathBuf::from("/tmp/test.md"));
    assert_eq!(config.recent_files.len(), 1);
    assert_eq!(config.recent_files[0], PathBuf::from("/tmp/test.md"));
}

#[test]
fn test_config_recent_file_dedup() {
    let mut config = AppConfig::default();
    config.add_recent_file(PathBuf::from("/tmp/a.md"));
    config.add_recent_file(PathBuf::from("/tmp/b.md"));
    config.add_recent_file(PathBuf::from("/tmp/a.md"));
    assert_eq!(config.recent_files.len(), 2);
    assert_eq!(config.recent_files[0], PathBuf::from("/tmp/a.md"));
}

#[test]
fn test_config_recent_file_max_20() {
    let mut config = AppConfig::default();
    for i in 0..25 {
        config.add_recent_file(PathBuf::from(format!("/tmp/{}.md", i)));
    }
    assert_eq!(config.recent_files.len(), 20);
}

#[test]
fn test_config_serialization_roundtrip() {
    let mut config = AppConfig::default();
    config.main_folder = Some(PathBuf::from("/home/user/notes"));
    config.add_recent_file(PathBuf::from("/tmp/test.md"));
    let json = serde_json::to_string(&config).unwrap();
    let loaded: AppConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config.main_folder, loaded.main_folder);
    assert_eq!(config.recent_files, loaded.recent_files);
}
