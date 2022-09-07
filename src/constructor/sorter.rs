use std::{fs::{DirEntry}};

pub fn is_lucifer_file(entry: &DirEntry) -> bool {
    path_ends_with(entry, ".lucifer.yaml")
}

pub fn is_settings_file(entry: &DirEntry) -> bool {
    path_ends_with(entry, "settings.lucifer.yaml")
}

pub fn path_ends_with(entry: &DirEntry, comparison: &str) -> bool {
    entry 
        .path()
        .display()
        .to_string()
        .ends_with(comparison)
}