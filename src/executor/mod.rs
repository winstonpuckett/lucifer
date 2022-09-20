use std::{process::Command, time::Instant};
use std::str;

use crate::{suite, logger};

pub fn execute(suite: &suite::Suite) -> Vec<TestResult> {
    let mut results: Vec<TestResult> = vec![];

    let (shell, first_arg) = get_shell();

    for feature in suite.features.iter() {
        logger::log_newline();
        logger::log_heading(&format!("Feature: {0}", feature.name));
        
        for test in feature.tests.iter() {
            let mut result = TestResult {
                succeeded: true,
                failures: vec![],
                performance: 0
            };

            // Prefer test, feature, suite, args for where the test can come from.
            let tool = if test.command.is_some() {
                test.command.to_owned().unwrap()
            } else if feature.has_command {
                feature.command.to_owned().unwrap()
            } else if suite.settings.has_command {
                suite.settings.command.to_owned().unwrap()
            } else {
                suite.args.command.to_owned().unwrap()
            };

            let mut command = Command::new(&shell);
            let arg = to_arg(tool, &test.args);
            let command_with_args = command.args([&first_arg, &arg]);
            
            let now = Instant::now();
            let output_option = command_with_args.output();
            let time_in_milliseconds = now.elapsed().as_millis();

            result.performance = time_in_milliseconds;

            let output = output_option.unwrap();

            let stdout = str::from_utf8(&output.stdout).unwrap();
            let stderr = str::from_utf8(&output.stderr).unwrap();

            if suite.settings.verbose {
                println!("Standard Out: '{stdout}'");
                println!("Standard Error: '{stderr}'");
            }

            let performance_satisfied = (time_in_milliseconds as u64) <= test.expectations.performance;

            let exit_code_satisfied = output.status.code().unwrap() == test.expectations.exit_code;

            let mut output_expectation = String::from("");
            let output_satisfied = if test.expectations.output.is_none() {
                true
            } else {
                output_expectation = test.to_owned().expectations.to_owned().output.unwrap();
                output_expectation == stdout
            };

            let mut error_expectation = String::from("");
            let error_satisfied = if test.expectations.error.is_none() {
                true
            } else {
                error_expectation = test.to_owned().expectations.to_owned().error.unwrap();
                error_expectation == stderr
            };

            if performance_satisfied
                && exit_code_satisfied 
                && output_satisfied
                && error_satisfied {
                logger::log_success(&format!("'{0}' succeeded in {1}ms", test.name, time_in_milliseconds));
            } else {
                logger::log_failure(&format!("'{0}' failed in {1}ms", test.name, time_in_milliseconds));
                result.succeeded = false;

                logger::log_detail(&format!("Reproduce with: '{0}'", arg));
                logger::log_newline();

                if !performance_satisfied {
                    logger::log_details(vec![
                        &format!("Expected performance: {0}ms", test.expectations.performance),
                        &format!("Actual performance: {0}ms", time_in_milliseconds)
                    ]);

                    result.failures.push(Failure {
                        failure_type: FailureType::Performance,
                        expectation: test.expectations.performance.to_string(),
                        actual: time_in_milliseconds.to_string()
                    })
                }

                if !exit_code_satisfied {
                    logger::log_details(vec![
                        &format!("Expected exit code: {0}", test.expectations.exit_code),
                        &format!("Actual exit code: {0}", output.status.code().unwrap())
                    ]);

                    result.failures.push(Failure {
                        failure_type: FailureType::ExitCode,
                        expectation: test.expectations.exit_code.to_string(),
                        actual: output.status.code().unwrap().to_string()
                    })
                }

                if !output_satisfied {
                    logger::log_details(vec![
                        &format!("Expected output: {0}", output_expectation),
                        &format!("Actual output: {0}", stdout)
                    ]);

                    result.failures.push(Failure {
                        failure_type: FailureType::Output,
                        expectation: output_expectation,
                        actual: String::from(stdout)
                    })
                }

                if !error_satisfied {
                    logger::log_details(vec![
                        &format!("Expected error: {0}", error_expectation),
                        &format!("Actual error: {0}", stderr)
                    ]);

                    result.failures.push(Failure {
                        failure_type: FailureType::Error,
                        expectation: error_expectation,
                        actual: String::from(stderr)
                    })
                }
            }
            
            results.push(result);
        }
    }

    results
}

fn to_arg(command: String, args: &Vec<String>) -> String {
    std::format!("{0} {1}", 
        command, 
        args.join(" "))
}

fn get_shell() -> (String, String) {
    if cfg!(target_os = "windows") {
        (String::from("cmd"), String::from("/C"))
    } else {
        (String::from("sh"), String::from("-c"))
    }
}

pub struct TestResult { 
    pub succeeded: bool,
    pub failures: Vec<Failure>,
    pub performance: u128,
}

pub struct Failure {
    pub failure_type: FailureType,
    pub expectation: String,
    pub actual: String
}

pub enum FailureType {
    Performance,
    ExitCode,
    Output,
    Error,
    FileDoesNotExist,
    FileContents
}
