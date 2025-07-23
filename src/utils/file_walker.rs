use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn walk_source_dir(path: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "rs" || ext == "js" || ext == "py" || ext == "java" {
                    files.push(path.to_path_buf());
                }
            }
        }
    }
    Ok(files)
}
