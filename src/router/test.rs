use std::{path::Path, fs, io::Write};

use crate::{args_getter::Args, suite_getter, test_runner::{self, SuiteResult}, ExitCode, CommandResult};

pub fn execute(args: Args) -> CommandResult<()> {
    let suite = suite_getter::get(args)?;
    let result = test_runner::run_suite(&suite);
    
    write_test_results(&result, suite);

    if result.success {
        Ok(())
    } else {
        Err((ExitCode::FailingTest, None))
    }
}

fn write_test_results(results: &SuiteResult, suite: suite_getter::Suite) -> bool {
    let mut success = true;
    let mut data = json::JsonValue::new_object();
    data["testResults"] = json::JsonValue::new_array();
    for r in &results.test_results {
        success &= r.succeeded;
        let mut failures = json::JsonValue::new_array();
        for f in &r.failures {
            failures.push(json::object!{
                type: f.failure_type.to_string(),
                expectation: String::from(&f.expectation),
                actual: String::from(&f.actual)
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
    success
}