use std::{fs, io::Write, path::Path};

use crate::{
    suite_getter,
    test_runner::SuiteResult,
};

use self::to_json::suite_to_json;

mod to_json;

pub fn write_test_results(result: &SuiteResult, suite: &suite_getter::Suite) {
    if suite.args.no_file {
        return;
    }

    let json_string = suite_to_json(result);
    let file_path = &suite.args.output_directory;

    write_file(file_path, json_string);
}

fn write_file(output_path: &String, json_string: String) {
    let path = Path::new(output_path);
    {
        let path_exists = path.try_exists();
        if path_exists.is_err() || path_exists.unwrap() == false {
            if output_path.ends_with(".json") {
                fs::create_dir_all(path.parent().unwrap()).unwrap();
            } else {
                fs::create_dir_all(path).unwrap();
            }

        }
    }

    let path_string = if output_path.ends_with(".json") {
        output_path.to_owned()
    } else {
        String::from(format!("{}/{}", output_path, "results.json"))
    };
    
    
    let mut file = std::fs::File::create(path_string).expect("create failed");

    file.write_all(json_string.as_bytes())
        .expect("write failed");
}
