use std::{fs::{DirEntry}};

pub fn is_lucifer_file(entry: &Result<DirEntry, std::io::Error>) -> bool {
    path_ends_with(entry, ".lucifer.yaml")
}

pub fn is_settings_file(entry: &Result<DirEntry, std::io::Error>) -> bool {
    path_ends_with(entry, "settings.lucifer.yaml")
}

pub fn path_ends_with(entry: &Result<DirEntry, std::io::Error>, comparison: &str) -> bool {
    entry 
        .as_ref()
        .unwrap()
        .path()
        .display()
        .to_string()
        .ends_with(comparison)
}