/*
    Easy improvements which should be made:
        - Remove all unwraps
        - Abstract getting a String
        - Pass sub objects vs root objects in to_test
            - serialization: to_serialization(y),
            - args: to_args(y),
            - expectations: to_expectations(y)
        - Move this into a sub folder and break out structs
*/

use std::{fs::{self, DirEntry}, io};
extern crate yaml_rust;
use std::str;
mod transformer;
mod sorter;

pub fn construct(folder: &str) -> io::Result<Suite> {
    let mut files = fs::read_dir(folder)
        .unwrap()
        .filter(sorter::is_lucifer_file);

    let settings_file: DirEntry = files.find(sorter::is_settings_file).unwrap().unwrap();
    let settings = transformer::to_settings(settings_file);
    
    let features = files 
        .filter(|f| !sorter::is_settings_file(f))
        .map(|f| transformer::to_feature(f.unwrap()))
        .collect();

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