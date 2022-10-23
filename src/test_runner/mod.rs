use std::{fmt, fs, process, str};
use std::{process::Command, time::Instant};

use crate::suite_getter::Expectations;
use crate::{logger, suite_getter};

pub fn run_suite(suite: &suite_getter::Suite) -> SuiteResult {
    let mut test_results: Vec<TestResult> = vec![];
    let mut success = true;

    let (shell, first_arg) = get_shell();

    logger::log_header(suite);

    for feature in suite.features.iter() {
        logger::log_heading(suite, &format!("Feature: {0}", feature.name));

        for test in feature.tests.iter() {
            let mut result = TestResult {
                succeeded: true,
                failures: vec![],
                performance: 0,
                exit_code: 0,
                output: String::from("")
            };

            let tool = feature.command.to_owned();
            let meaningful_args = to_arg(tool, &test.args);
            let full_args = [&first_arg, &meaningful_args];

            let (time_in_milliseconds, output) = perform_command(&shell, full_args);
            result.performance = time_in_milliseconds;
            result.exit_code = output.status.code().unwrap();
            result.output = String::from(str::from_utf8(&output.stdout).unwrap());

            let failures = assert_all(&test.expectations, &result);

            let stderr = str::from_utf8(&output.stderr).unwrap();
            let mut error_expectation = String::from("");
            let error_satisfied = if test.expectations.error.is_none() {
                true
            } else {
                error_expectation = test.to_owned().expectations.to_owned().error.unwrap();
                error_expectation == stderr
            };

            let mut file_expectation = String::from("");
            let mut file_contents = String::from("");
            let file_satisfied = if test.expectations.file.is_none() {
                true
            } else {
                file_expectation = test.to_owned().expectations.to_owned().file.unwrap();
                let content_option = fs::read_to_string(&file_expectation);

                if content_option.is_ok() {
                    file_contents = content_option.unwrap();
                    true
                } else {
                    false
                }
            };

            let file_contents_satisfied = if file_satisfied && test.expectations.contents.is_some()
            {
                file_contents == test.to_owned().expectations.to_owned().contents.unwrap()
            } else {
                true
            };

            let mut no_file_expectation = String::from("");
            let no_file_satisfied = if test.expectations.no_file.is_none() {
                true
            } else {
                no_file_expectation = test.to_owned().expectations.to_owned().no_file.unwrap();
                let metadata = fs::metadata(&no_file_expectation);

                metadata.is_err() || !metadata.unwrap().is_file()
            };

            if failures.is_empty()
                && error_satisfied
                && no_file_satisfied
                && file_satisfied
                && file_contents_satisfied
            {
                logger::log_success(
                    suite,
                    &format!("'{0}' succeeded in {1}ms", test.name, time_in_milliseconds),
                );
            } else {
                logger::log_failure(
                    suite,
                    &format!("'{0}' failed in {1}ms", test.name, time_in_milliseconds),
                );
                result.succeeded = false;
                success = false;

                logger::log_detail(suite, &format!("Reproduce with: '{0}'", meaningful_args));
                logger::log_newline(suite);

                for f in failures {
                    logger::log_details(
                        suite,
                        get_failure_messages(f),
                    );
                }

                if !error_satisfied {
                    logger::log_details(
                        suite,
                        vec![
                            format!("Expected error: '{0}'", error_expectation),
                            format!("Actual error: '{0}'", stderr),
                        ],
                    );

                    result.failures.push(Failure {
                        failure_type: FailureType::Error,
                        expectation: error_expectation,
                        actual: String::from(stderr),
                    })
                }

                if !no_file_satisfied {
                    logger::log_details(
                        suite,
                        vec![
                            format!(
                                "Expected: This file should not exist '{0}'",
                                no_file_expectation
                            ),
                            format!("Actual: This file exists '{0}'", no_file_expectation),
                        ],
                    );

                    result.failures.push(Failure {
                        failure_type: FailureType::FileExists,
                        expectation: String::from(""),
                        actual: no_file_expectation,
                    })
                }

                if !file_satisfied {
                    logger::log_details(
                        suite,
                        vec![
                            format!("Expected file: '{0}'", file_expectation),
                            format!(
                                "File error message: '{0}'",
                                String::from("File does not exist or cannot be accessed.")
                            ),
                        ],
                    );

                    result.failures.push(Failure {
                        failure_type: FailureType::FileDoesNotExist,
                        expectation: file_expectation,
                        actual: String::from("File does not exist or cannot be accessed."),
                    })
                }

                if !file_contents_satisfied {
                    logger::log_details(
                        suite,
                        vec![
                            format!(
                                "Expected file contents: '{0}'",
                                test.to_owned().expectations.to_owned().contents.unwrap()
                            ),
                            format!("Actual file contents: '{0}'", file_contents),
                        ],
                    );

                    result.failures.push(Failure {
                        failure_type: FailureType::FileContents,
                        expectation: test.to_owned().expectations.to_owned().contents.unwrap(),
                        actual: file_contents,
                    })
                }
            }

            test_results.push(result);
        }
    }

    SuiteResult {
        success,
        test_results,
    }
}

fn assert_all(expectations: &Expectations, result: &TestResult) -> Vec<Failure> {
    let mut all = vec![];
    let mut performance = assert_performance(expectations, result);
    let mut exit_code = assert_exit_code(expectations, result);
    let mut output = assert_output(expectations, result);

    all.append(&mut performance);
    all.append(&mut exit_code);
    all.append(&mut output);

    all
}

fn assert_performance(expectations: &Expectations, result: &TestResult) -> Vec<Failure> {
    let performance_satisfied = result.performance <= expectations.performance.into();

    if !performance_satisfied {
        vec![Failure {
            failure_type: FailureType::Performance,
            expectation: expectations.performance.to_string(),
            actual: result.performance.to_string(),
        }]
    } else {
        vec![]
    }
}

fn assert_exit_code(expectations: &Expectations, result: &TestResult) -> Vec<Failure> {
    let exit_code_satisfied = result.exit_code == expectations.exit_code;

    if !exit_code_satisfied {
        vec![Failure {
            failure_type: FailureType::ExitCode,
            expectation: expectations.exit_code.to_string(),
            actual: result.exit_code.to_string(),
        }]
    } else {
        vec![]
    }
}

fn assert_output(expectations: &Expectations, result: &TestResult) -> Vec<Failure> {
    if expectations.output.is_none() {
        return vec![];
    }

    let expectation = String::from(expectations.output.as_ref().unwrap());
    let actual = result.output.to_owned();
    if expectation == actual {
        return vec![];
    }

    vec![Failure {
        failure_type: FailureType::Output,
        expectation,
        actual 
    }]
}

fn get_failure_messages(failure: Failure) -> Vec<String> {
    match failure.failure_type {
        FailureType::Performance => vec![
            format!("Expected performance: {0}ms", failure.expectation),
            format!("Actual performance: {0}ms", failure.actual),
        ],
        FailureType::ExitCode => vec![
            format!("Expected exit code: {0}", failure.expectation),
            format!("Actual exit code: {0}", failure.actual),
        ],
        FailureType::Output => vec![
            format!("Expected output: '{0}'", failure.expectation),
            format!("Actual output: '{0}'", failure.actual),
        ],
        FailureType::Error => todo!(),
        FailureType::FileExists => todo!(),
        FailureType::FileDoesNotExist => todo!(),
        FailureType::FileContents => todo!(),
    }
}

fn perform_command(shell: &String, args: [&String; 2]) -> (u128, process::Output) {
    let mut command = Command::new(shell);
    command.args(args);

    let now = Instant::now();
    let output_option = command.output();
    let time_in_milliseconds = now.elapsed().as_millis();

    let output = output_option.unwrap();

    (time_in_milliseconds, output)
}

fn to_arg(command: String, args: &Vec<String>) -> String {
    std::format!("{0} {1}", command, args.join(" "))
}

fn get_shell() -> (String, String) {
    if cfg!(target_os = "windows") {
        (String::from("cmd"), String::from("/C"))
    } else {
        (String::from("sh"), String::from("-c"))
    }
}

#[derive(Clone)]
pub struct SuiteResult {
    pub success: bool,
    pub test_results: Vec<TestResult>,
}

#[derive(Clone)]
pub struct TestResult {
    pub succeeded: bool,
    pub failures: Vec<Failure>,
    pub performance: u128,
    pub exit_code: i32,
    pub output: String
}

#[derive(Clone)]
pub struct Failure {
    pub failure_type: FailureType,
    pub expectation: String,
    pub actual: String,
}

#[derive(Clone)]
pub enum FailureType {
    Performance,
    ExitCode,
    Output,
    Error,
    FileExists,
    FileDoesNotExist,
    FileContents,
}

impl fmt::Display for FailureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FailureType::Performance => write!(f, "performance"),
            FailureType::ExitCode => write!(f, "exitCode"),
            FailureType::Output => write!(f, "output"),
            FailureType::Error => write!(f, "error"),
            FailureType::FileExists => write!(f, "fileExists"),
            FailureType::FileDoesNotExist => write!(f, "fileMissing"),
            FailureType::FileContents => write!(f, "fileContents"),
        }
    }
}
