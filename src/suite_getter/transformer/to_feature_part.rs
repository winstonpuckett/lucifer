use yaml_rust::Yaml;

use crate::suite_getter::{Test, Expectations};

use super::to_primitive::{to_string_from_string_or_i64_default_empty, to_string_or_default, to_string_or_blank, to_string_option, to_u64_option, to_i32_or_default};

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
            file: None,
            contents: None 
        }
    }

    Expectations { 
        performance: to_u64_option(&y["expectations"]["performance"]),
        exit_code: to_i32_or_default(&y["expectations"]["exitCode"], 0), 
        output: to_string_option(&y["expectations"]["output"]),
        error: to_string_option(&y["expectations"]["error"]),
        no_file: to_string_option(&y["expectations"]["noFile"]),
        file: to_string_option(&y["expectations"]["file"]),
        contents: to_string_option(&y["expectations"]["contents"]),
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