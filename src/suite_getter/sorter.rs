use std::fs::DirEntry;

pub fn is_lucifer_file(entry: &DirEntry) -> bool {
    path_ends_with(entry, ".yaml")
}

pub fn path_ends_with(entry: &DirEntry, comparison: &str) -> bool {
    entry 
        .path()
        .display()
        .to_string()
        .ends_with(comparison)
}