/*
    Easy improvements which should be made:
        - Remove all unwraps
        - Abstract getting a String
        - Pass sub objects vs root objects in to_test
            - serialization: to_serialization(y),
            - args: to_args(y),
            - expectations: to_expectations(y)
*/

use std::{fs, io};
extern crate yaml_rust;
use std::str;

use self::{transformer::{Expectations, Serialization, Feature}, sorter::{is_settings_file, is_lucifer_file}};
mod transformer;
mod sorter;

pub fn construct(folder: &str) -> io::Result<Suite> {
    // TODO: Catch errors from unwrapping folders.
    let files = fs::read_dir(folder).unwrap();

    let mut features: Vec<Feature> = vec![];
    // TODO: store default settings somewhere else.
    let mut settings = transformer::Settings {
        command: String::from("echo"),
        version: 0,
        verbose: false,
    };
    
    for file_result in files {
        if file_result.is_err() {
            // TODO: Figure out why a file would be in error.
            continue;
        }
        
        let file = file_result.unwrap();

        if !is_lucifer_file(&file) {
            continue;
        }

        if is_settings_file(&file) {
            settings = transformer::to_settings(file);
            continue;
        }

        features.push(transformer::to_feature(&file));
    }

    let suite = Suite { 
        settings, 
        features
    };

    Ok(suite)
}

pub struct Suite {
    pub settings: transformer::Settings,
    pub features: Vec<transformer::Feature>
}

pub struct Test {
    pub args: Vec<String>,
    pub description: String,
    pub expectations: Expectations,
    pub name: String,
    pub serialization: Serialization
}