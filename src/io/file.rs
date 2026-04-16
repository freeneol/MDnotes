use std::path::{Path, PathBuf};

pub fn read_file(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("无法读取文件: {}", e))
}

pub fn write_file(path: &Path, content: &str) -> Result<(), String> {
    std::fs::write(path, content).map_err(|e| format!("无法写入文件: {}", e))
}

pub fn list_md_files(dir: &Path) -> Vec<FileEntry> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = std::fs::read_dir(dir) {
        let mut items: Vec<_> = read_dir.filter_map(|e| e.ok()).collect();
        items.sort_by_key(|e| e.file_name());
        for entry in items {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() && !name.starts_with('.') {
                let children = list_md_files(&path);
                if !children.is_empty() {
                    entries.push(FileEntry::Directory { name, path, children });
                }
            } else if path.extension().map_or(false, |ext| ext == "md") {
                entries.push(FileEntry::File { name, path });
            }
        }
    }
    entries
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FileEntry {
    File { name: String, path: PathBuf },
    Directory { name: String, path: PathBuf, children: Vec<FileEntry> },
}
