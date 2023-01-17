use std::{
    fs::{self, DirEntry},
    io::Error,
    path::Path,
};
extern crate yaml_rust;
use self::{
    sorter::is_lucifer_file,
    transformer::Feature,
};
use crate::{args_getter::Args, CommandResult, ExitCode};
mod sorter;
mod transformer;

pub fn get_suite(args: Args) -> CommandResult<Suite> {
    let path = Path::new(&args.input_directory);
    if path.is_dir() {
        let files = fs::read_dir(&args.input_directory).unwrap();

        let features = files
            .map(to_feature_option)
            .filter(|f| f.is_some())
            .map(|f| f.unwrap())
            .collect();

        let suite = Suite { args, features };

        Ok(suite)
    } else if path.is_file() {
        let file_result = fs::read_to_string(&args.input_directory);
        let file = file_result.unwrap();

        let features = vec![transformer::to_feature_from_str(
            path.file_name().unwrap().to_str().unwrap(),
            &file,
        )];

        let suite = Suite { args, features };

        Ok(suite)
    } else {
        Err((ExitCode::UserError, Some(format!("Could not find path {:?}. It may be that the path does not exist or the current user does not have read permissions.", path))))
    }
}

fn to_feature_option(f: Result<DirEntry, Error>) -> Option<Feature> {
    if f.is_err() {
        return None;
    }

    let file = f.unwrap();

    if !is_lucifer_file(&file) {
        return None;
    }

    Some(transformer::to_feature_from_dir_entry(&file))
}

pub struct Suite {
    pub args: Args,
    pub features: Vec<transformer::Feature>,
}

pub struct Test {
    pub args: Vec<String>,
    pub description: String,
    pub expectations: Expectations,
    pub name: String,
}


#[derive(Clone)]
pub struct Expectations {
    pub performance: Option<u64>,
    pub exit_code: i32,
    pub output: Option<String>,
    pub error: Option<String>,
    pub no_file: Option<String>,
    pub file: Vec<FileExpectation>
}

#[derive(Clone)]
pub struct FileExpectation {
    pub path: String,
    pub parts: Vec<String>
    // TODO: matches?
}