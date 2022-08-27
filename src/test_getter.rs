use std::{fs::{self, DirEntry}, io};
use std::str;
extern crate yaml_rust;
use yaml_rust::{YamlLoader};

pub fn find(folder: &str) -> io::Result<Settings> {
    let mut files = fs::read_dir(folder)
        .unwrap()
        .filter(is_lucifer_file);

    let settings_file: DirEntry = files.find(is_settings_file).unwrap().unwrap();
    let settings = get_settings_file(settings_file);
    
    let _test_files = files.filter(|f| !is_settings_file(f));

    Ok(settings)
}

fn get_settings_file(settings_file: DirEntry) -> Settings {
    let content = fs::read_to_string(settings_file.path()).unwrap();
    let settings_yaml = YamlLoader::load_from_str(content.as_str()).unwrap();
    let settings_map = &settings_yaml[0];

    Settings { 
        version: settings_map["version"].as_i64().unwrap() as u8,
        command: String::from(settings_map["command"].as_str().unwrap()), 
        execution_directory: String::from(settings_map["executionDirectory"].as_str().unwrap())
    }
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

// Settings structs

pub struct Settings {
    pub version: u8,
    pub command: String,
    pub execution_directory: String,
}

// Test structs

pub struct Test {
    pub name: String,
    pub description: String,
    pub expectations: Expectations,
    pub serialization: Serialization,
    pub args: [String]
}

pub enum Serialization {
    Auto,
    Parallel,
    Serial,
}

pub struct Expectations {
    pub performance: Option<u64>,
    pub exit_code: Option<i32>,
    pub output: Option<String>,
    pub file: Option<String>,
    pub contents: Option<String>
}