use std::{process::Command, time::Instant};
use std::str;

use crate::{constructor, logger};

pub fn execute(suite: constructor::Suite) -> Vec<TestResult> {
    let mut results: Vec<TestResult> = vec![];

    let (shell, first_arg) = get_shell();

    for feature in suite.features {
        logger::log_newline();
        logger::log_heading(&format!("Feature: {0}", feature.name));
        
        for test in feature.tests {
            let mut result = TestResult {
                succeeded: true
            };

            let mut command = Command::new(&shell);
            let arg = to_arg(&suite.settings.command, &test.args);
            let command_with_args = command
                .args([&first_arg, &arg]);
            
            let now = Instant::now();

            let output = command_with_args
                .output()
                .unwrap();
            
            let time_in_milliseconds = now.elapsed().as_millis();

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
                output_expectation = test.expectations.output.unwrap();
                output_expectation == stdout
            };

            let mut error_expectation = String::from("");
            let error_satisfied = if test.expectations.error.is_none() {
                true
            } else {
                error_expectation = test.expectations.error.unwrap();
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

                logger::log_failure(&format!("Reproduce with: {0} {1} {2}", shell, first_arg, arg));

                if !performance_satisfied {
                    logger::log_details(vec![
                        &format!("Expected performance: {0}ms", test.expectations.performance),
                        &format!("Actual performance: {0}ms", time_in_milliseconds)
                    ]);
                }

                if !exit_code_satisfied {
                    logger::log_details(vec![
                        &format!("Expected exit code: {0}", test.expectations.exit_code),
                        &format!("Actual exit code: {0}", output.status.code().unwrap())
                    ]);
                }

                if !output_satisfied {
                    logger::log_details(vec![
                        &format!("Expected output: {0}", output_expectation),
                        &format!("Actual output: {0}", stdout)
                    ]);
                }

                if !error_satisfied {
                    logger::log_details(vec![
                        &format!("Expected error: {0}", error_expectation),
                        &format!("Actual error: {0}", stderr)
                    ]);
                }
            }
            
            results.push(result);
        }
    }

    results
}

fn to_arg(command: &String, args: &Vec<String>) -> String {
    std::format!("{0} {1}", 
        command, 
        args.join(" "))
}

pub struct TestResult { 
    pub succeeded: bool
}

fn get_shell() -> (String, String) {
    if cfg!(target_os = "windows") {
        (String::from("cmd"), String::from("/C"))
    } else {
        (String::from("sh"), String::from("-c"))
    }
}