use std::{fs, io};
extern crate yaml_rust;
use crate::args::Args;
use self::{transformer::{Expectations, Serialization, Feature}, sorter::is_lucifer_file};
mod transformer;
mod sorter;

pub fn get(args: Args) -> io::Result<Suite> {
    // TODO: Catch errors from unwrapping folders.
    let files = fs::read_dir(&args.input_directory).unwrap();

    let mut features: Vec<Feature> = vec![];

    for file_result in files {
        if file_result.is_err() {
            // TODO: Figure out why a file would be in error.
            continue;
        }
        
        let file = file_result.unwrap();
        
        if !is_lucifer_file(&file) {
            continue;
        }

        features.push(transformer::to_feature(&file));
    }

    let suite = Suite {
        args,
        features
    };

    Ok(suite)
}

pub struct Suite {
    pub args: Args,
    pub features: Vec<transformer::Feature>
}

pub struct Test {
    pub args: Vec<String>,
    pub description: String,
    pub expectations: Expectations,
    pub name: String,
    pub serialization: Serialization
}
