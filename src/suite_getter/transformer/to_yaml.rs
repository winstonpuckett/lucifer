use std::fs::{DirEntry, self};

use yaml_rust::YamlLoader;

pub fn file_to_yaml(file: &DirEntry) -> Option<yaml_rust::Yaml> {
    let content = fs::read_to_string(file.path()).unwrap();
    let yaml = YamlLoader::load_from_str(content.as_str()).unwrap();

    if yaml.is_empty(){
        return None;
    }

    Some(yaml[0].to_owned())
}

pub fn string_to_yaml(content: &str) -> Option<yaml_rust::Yaml> {
    let yaml = YamlLoader::load_from_str(content).unwrap();

    if yaml.is_empty(){
        return None;
    }

    Some(yaml[0].to_owned())
}