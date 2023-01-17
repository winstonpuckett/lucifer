use std::vec;

use yaml_rust::Yaml;

use crate::suite_getter::{Expectations, FileExpectation, Test};

use super::to_primitive::{
    to_i32_or_default, to_string_from_string_or_i64_default_empty, to_string_option,
    to_string_or_blank, to_string_or_default, to_u64_option,
};

pub fn to_test(y: &Yaml) -> Test {
    Test {
        name: to_string_or_default(&(y["name"]), "Unnamed test"),
        description: to_string_or_blank(&y["description"]),
        args: to_args(y),
        expectations: to_expectations(y),
    }
}

fn to_expectations(y: &Yaml) -> Expectations {
    if y["expectations"].is_badvalue() {
        return Expectations {
            performance: None,
            exit_code: 0,
            output: None,
            error: None,
            no_file: None,
            file: vec![],
        };
    }

    Expectations {
        performance: to_u64_option(&y["expectations"]["performance"]),
        exit_code: to_i32_or_default(&y["expectations"]["exitCode"], 0),
        output: to_string_option(&y["expectations"]["output"]),
        error: to_string_option(&y["expectations"]["error"]),
        no_file: to_string_option(&y["expectations"]["noFile"]),
        file: to_file_list(&y["expectations"]["file"]),
    }
}

fn to_file_list(y: &Yaml) -> Vec<FileExpectation> {
    if y.is_badvalue() {
        return vec![];
    }

    let file_list = y.as_vec();
    if file_list.is_none() {
        let path = &y["path"];

        if path.is_badvalue() {
            return vec![];
        } else {
            let path_string = path.as_str().unwrap();
            let parts = extract_file_parts(&y);

            return vec![FileExpectation {
                path: String::from(path_string),
                parts: parts,
            }];
        }
    }

    return file_list
        .unwrap()
        .into_iter()
        .map(|yml: &Yaml| {
            let path = String::from(yml["path"].as_str().unwrap());
            let parts = extract_file_parts(yml);

            return FileExpectation { path, parts };
        })
        .collect();
}

fn extract_file_parts(yml: &Yaml) -> Vec<String> {
    let parts = &yml["parts"];

    if parts.is_badvalue() {
        vec![]
    } else if parts.as_str().is_some() {
        vec![String::from(parts.as_str().unwrap())]
    } else if parts.as_vec().is_some() {
        parts
            .as_vec()
            .unwrap()
            .into_iter()
            .map(|y: &Yaml| -> String { String::from(y.as_str().unwrap()) })
            .collect()
    } else {
        todo!("parts has bad value. We'll need to return a better error.");
    }
}

pub fn to_args(y: &Yaml) -> Vec<String> {
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
