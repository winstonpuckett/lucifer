use yaml_rust::{YamlLoader, Yaml};
use std::{fs::{self, DirEntry}};

use super::Test;
extern crate yaml_rust;

pub fn to_settings(settings_file: DirEntry) -> Settings {
    let settings_map = file_to_map(&settings_file);

    Settings { 
        version: settings_map["version"].as_i64().unwrap() as u8,
        command: String::from(settings_map["command"].as_str().unwrap()), 
        execution_directory: String::from(settings_map["executionDirectory"].as_str().unwrap())
    }
}

fn file_to_map(file: &DirEntry) -> yaml_rust::Yaml {
    let content = fs::read_to_string(file.path()).unwrap();
    let settings_yaml = YamlLoader::load_from_str(content.as_str()).unwrap();
    let settings_map = &settings_yaml[0];
    
    settings_map.to_owned()
}

pub fn to_feature(entry: &DirEntry) -> Feature {
    Feature { 
        name: entry.file_name().into_string().unwrap(), 
        tests: file_to_map(entry)["tests"]
            .as_vec()
            .unwrap()
            .into_iter()
            .map(to_test)
            .collect()
    }
}

fn to_test(y: &Yaml) -> Test {
    Test {
        name: String::from(y["name"].as_str().unwrap_or("Unnamed test")),
        description: String::from(y["description"].as_str().unwrap_or("")),
        serialization: to_serialization(y),
        args: to_args(y),
        expectations: to_expectations(y) 
    }
}

fn to_serialization(y: &Yaml) -> Serialization {
    let serialization_key = y["serialization"].as_str().unwrap();
    
    let serialization = if serialization_key == "auto" {
        Serialization::Auto
    } else if serialization_key == "parallel" {
        Serialization::Parallel
    } else if serialization_key == "serial" {
        Serialization::Serial
    } else {
        panic!("Could not parse serialization: {:?}", serialization_key)
    };

    serialization
}

fn to_args(y: &Yaml) -> Vec<String> {
    y["args"]
        .as_vec()
        .unwrap()
        .into_iter()
        .map(|g| String::from(g.as_str().unwrap()))
        .collect()
}

fn to_expectations(y: &Yaml) -> Expectations {
    if y["expectations"].is_badvalue() {
        // TODO: set default expectations somewhere else
        return Expectations { 
            performance: 1000, 
            exit_code: 0,
            output: None,
            file: None,
            contents: None 
        }
    }

    Expectations { 
        performance: y["expectations"]["performance"].as_i64().unwrap_or(1000) as u64,
        exit_code: y["expectations"]["exitCode"].as_i64().unwrap_or(0) as i32,
        output: y["expectations"]["output"].as_str().and_then(|o| Some(String::from(o))),
        file: y["expectations"]["file"].as_str().and_then(|o| Some(String::from(o))),
        contents: y["expectations"]["contents"].as_str().and_then(|o| Some(String::from(o))) 
    }
}


// Suite Structs

pub struct Settings {
    pub version: u8,
    pub command: String,
    pub execution_directory: String,
}

pub struct Feature {
    pub name: String,
    pub tests: Vec<Test>
}

pub enum Serialization {
    Auto,
    Parallel,
    Serial,
}

pub struct Expectations {
    pub performance: u64,
    pub exit_code: i32,
    pub output: Option<String>,
    pub file: Option<String>,
    pub contents: Option<String>
}