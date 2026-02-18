use std::{
    io,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct DirectoryOption {
    pub name: String,
    pub path: PathBuf,
}

pub fn list_directories(current_dir: &Path, show_hidden: bool) -> io::Result<Vec<DirectoryOption>> {
    let mut directories = Vec::new();

    for entry in std::fs::read_dir(current_dir)? {
        let entry = match entry {
            Ok(value) => value,
            Err(_) => continue,
        };

        let file_type = match entry.file_type() {
            Ok(value) => value,
            Err(_) => continue,
        };

        if !file_type.is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().into_owned();
        if !show_hidden && name.starts_with('.') {
            continue;
        }

        directories.push(DirectoryOption {
            name,
            path: entry.path(),
        });
    }

    directories.sort_by(|a, b| {
        a.name
            .to_lowercase()
            .cmp(&b.name.to_lowercase())
            .then_with(|| a.name.cmp(&b.name))
    });

    Ok(directories)
}
