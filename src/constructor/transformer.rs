use yaml_rust::{YamlLoader, Yaml};
use std::{fs::{self, DirEntry}};

use super::Test;
extern crate yaml_rust;

pub fn to_settings(settings_file: DirEntry) -> Settings {
    let settings_option = file_to_map(&settings_file);

    if settings_option.is_none() {
        Settings {
            version: 0,
            command: String::from("exit"),
            verbose: false
        }
    } else {
        // TODO: Handle mistyping gracefully.
        let settings = settings_option.unwrap();
        Settings { 
            version: settings["version"].as_i64().unwrap() as u8,
            command: String::from(settings["command"].as_str().unwrap()), 
            verbose: settings["verbose"].as_bool().unwrap_or(false)
        }
    }
}

fn file_to_map(file: &DirEntry) -> Option<yaml_rust::Yaml> {
    let content = fs::read_to_string(file.path()).unwrap();
    let settings_yaml = YamlLoader::load_from_str(content.as_str()).unwrap();

    if settings_yaml.is_empty(){
        return None;
    }

    let settings_map = &settings_yaml[0];
    Some(settings_map.to_owned())
}

pub fn to_feature(entry: &DirEntry) -> Feature {
    let tests_option = file_to_map(entry);
    
    let tests = if tests_option.is_some() {
        tests_option.unwrap()["tests"]
            .as_vec()
            // TODO: Handle case where tests is not a vec
            .unwrap()
            .into_iter()
            .map(to_test)
            .collect()
    } else {
        vec![]
    };

    Feature { 
        name: entry.file_name().into_string().unwrap(), 
        tests
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
    let serialization_key = y["serialization"].as_str().unwrap_or("auto");
    
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
    let args_option = y["args"].as_vec();

    if args_option.is_none() {
        return vec![];
    }

    args_option
        .unwrap()
        .into_iter()
        .map(to_string_from_string_or_i64_default_empty)
        .collect()
}

fn to_string_from_string_or_i64_default_empty(g: &Yaml) -> String {
    let str_option = g.as_str();
    if str_option.is_some() {
        return String::from(str_option.unwrap());
    }
    
    let i64_option = g.as_i64();
    if i64_option.is_some() {
        return format!("{}", i64_option.unwrap());
    }

    String::from("")
}

fn to_expectations(y: &Yaml) -> Expectations {
    if y["expectations"].is_badvalue() {
        // TODO: set default expectations somewhere else
        return Expectations { 
            performance: 1000, 
            exit_code: 0,
            output: None,
            error: None,
            file: None,
            contents: None 
        }
    }

    Expectations { 
        performance: y["expectations"]["performance"].as_i64().unwrap_or(1000) as u64,
        exit_code: y["expectations"]["exitCode"].as_i64().unwrap_or(0) as i32,
        output: y["expectations"]["output"].as_str().and_then(|o| Some(String::from(o))),
        error: y["expectations"]["error"].as_str().and_then(|o| Some(String::from(o))),
        file: y["expectations"]["file"].as_str().and_then(|o| Some(String::from(o))),
        contents: y["expectations"]["contents"].as_str().and_then(|o| Some(String::from(o))) 
    }
}


// Suite Structs

pub struct Settings {
    pub version: u8,
    pub command: String,
    pub verbose: bool,
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
    pub error: Option<String>,
    pub file: Option<String>,
    pub contents: Option<String>
}