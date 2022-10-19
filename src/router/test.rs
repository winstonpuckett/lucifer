use std::{path::Path, fs, io::Write};

use crate::{args_getter::Args, suite_getter, executor};

pub fn execute(args: Args) -> i32 {
    let suite = suite_getter::get(args).unwrap();
    let results = executor::execute(&suite);
    
    let mut success = true;
    let mut data = json::JsonValue::new_object();
    data["testResults"] = json::JsonValue::new_array();
    for r in results {
        success &= r.succeeded;
        let mut failures = json::JsonValue::new_array();
        for f in r.failures {
            failures.push(json::object!{
                type: match f.failure_type{
                    executor::FailureType::Performance => "performance",
                    executor::FailureType::ExitCode => "exitCode",
                    executor::FailureType::Output => "output",
                    executor::FailureType::Error => "error",
                    executor::FailureType::FileExists => "fileExists",
                    executor::FailureType::FileDoesNotExist => "fileMissing",
                    executor::FailureType::FileContents => "fileContents",
                },
                expectation: f.expectation,
                actual: f.actual
            }).unwrap();
        }

        data["testResults"].push(json::object!{
            succeeded: r.succeeded,
            milliseconds: r.performance as u64,
            failures: failures
        }).unwrap();
    }

    if !suite.args.no_file {
        if !Path::new(&suite.args.output_directory).is_dir() {
            fs::create_dir_all(&suite.args.output_directory).unwrap();
        }
        
        let mut file = std::fs::File::create(format!("{}/{}", &suite.args.output_directory, "results.json")).expect("create failed");
        file.write_all(json::stringify(data).as_bytes()).expect("write failed");
    }

    if success {
        0
    } else {
        1
    }
}