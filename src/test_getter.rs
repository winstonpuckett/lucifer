use std::{fs::{self, DirEntry}, io};

pub fn find(folder: &str) -> io::Result<()> {
    let mut files = fs::read_dir(folder)
        .unwrap()
        .filter(is_lucifer_file);

    let _settings_file: DirEntry = files.find(is_settings_file).unwrap().unwrap();
    let _test_files = files.filter(|f| !is_settings_file(f));

    Ok(())
}

fn is_lucifer_file(entry: &Result<DirEntry, std::io::Error>) -> bool {
    path_ends_with(entry, ".lucifer.yaml")
}

fn is_settings_file(entry: &Result<DirEntry, std::io::Error>) -> bool {
    path_ends_with(entry, "settings.lucifer.yaml")
}

fn path_ends_with(entry: &Result<DirEntry, std::io::Error>, comparison: &str) -> bool {
    entry 
        .as_ref()
        .unwrap()
        .path()
        .display()
        .to_string()
        .ends_with(comparison)
}