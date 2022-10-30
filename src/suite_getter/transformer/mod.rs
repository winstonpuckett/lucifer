use yaml_rust::Yaml;
use std::fs::DirEntry;

use self::{to_yaml::{file_to_yaml, string_to_yaml}, to_feature_part::to_test};

use super::Test;
extern crate yaml_rust;

mod to_yaml;
mod to_feature_part;
mod to_primitive;

pub fn to_feature_from_dir_entry(entry: &DirEntry) -> Feature {
    let file_option = file_to_yaml(entry);
    to_feature_from_map(file_option, entry.file_name().into_string().unwrap())
}

pub fn to_feature_from_str(file_name: &str, entry: &str) -> Feature {
    let file_option = string_to_yaml(entry);
    to_feature_from_map(file_option, String::from(file_name))
}

fn to_feature_from_map(file_option: Option<Yaml>, file_name: String) -> Feature {
    if file_option.is_some() {
        let file = file_option.unwrap();
        Feature {
            name: String::from(file_name),
            has_command: file["command"].as_str().is_some(),
            command: String::from(file["command"].as_str().unwrap()),
            tests: file["tests"]
                .as_vec()
                // TODO: Handle case where tests is not a vec
                .unwrap()
                .into_iter()
                .map(to_test)
                .collect()
        }
    } else {
        Feature {
            name: String::from(file_name),
            has_command: false,
            command: String::from(""),
            tests: vec![]
        }
    }
}

// Suite Structs

pub struct Feature {
    pub name: String,
    pub has_command: bool, // TODO: Remove this. It's a workaround because I don't know Rust very well.
    pub command: String,
    pub tests: Vec<Test>
}
