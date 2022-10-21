use std::{fs, io, path::Path};
extern crate yaml_rust;
use crate::args_getter::Args;
use self::{transformer::{Expectations, Serialization, Feature}, sorter::is_lucifer_file};
mod transformer;
mod sorter;

pub fn get(args: Args) -> io::Result<Suite> {
    let path = Path::new(&args.input_directory);
    if path.is_dir() {
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
    } else if path.is_file() {
        let file_result = fs::read_to_string(&args.input_directory);
        let mut features: Vec<Feature> = vec![];

            
        let file = file_result.unwrap();

        features.push(transformer::to_feature_from_str(path.file_name().unwrap().to_str().unwrap(), &file));

        let suite = Suite {
            args,
            features
        };

        Ok(suite)
    } else {
        // The path doesn't exist. Create a failing test for this then uncomment this code
        // Err((ExitCode::UserError, format!("Could not find path \"{:?}\". It may be that the path does not exist or the current user does not have read permissions.", path)))

        // TODO: don't panic.
        panic!()
    }
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
