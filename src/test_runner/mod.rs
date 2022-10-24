use std::{fmt, str};
use std::{process::Command, time::Instant};

mod rule_runner;

use crate::{logger, suite_getter};

use self::rule_runner::assert_all;

pub fn run_suite(suite: &suite_getter::Suite) -> SuiteResult {
    let mut test_results: Vec<TestResult> = vec![];
    let mut success = true;

    let (shell, first_arg) = get_shell();

    logger::log_header(suite);

    for feature in suite.features.iter() {
        logger::log_heading(suite, &format!("Feature: {0}", feature.name));

        for test in feature.tests.iter() {
            let tool = feature.command.to_owned();
            let meaningful_args = to_arg(tool, &test.args);
            let full_args = [&first_arg, &meaningful_args];

            let mut result = TestResult {
                run: perform_command(&shell, full_args),
                failures: vec![],
                succeeded: true
            };

            let failures = assert_all(&test.expectations, &result.run);

            if failures.is_empty() {
                logger::log_success(
                    suite,
                    &format!("'{0}' succeeded in {1}ms", test.name, result.run.performance),
                );
            } else {
                logger::log_failure(
                    suite,
                    &format!("'{0}' failed in {1}ms", test.name, result.run.performance),
                );
                result.succeeded = false;
                success = false;

                logger::log_detail(suite, &format!("Reproduce with: '{0}'", meaningful_args));
                logger::log_newline(suite);

                for f in failures {
                    logger::log_details(suite, get_failure_messages(f));
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
        FailureType::Error => vec![
            format!("Expected error: '{0}'", failure.expectation),
            format!("Actual error: '{0}'", failure.actual),
        ],
        // TODO: This is the only one which breaks the pattern. Rework.
        FailureType::FileExists => vec![
            format!("Expected: This file should not exist '{0}'", failure.actual),
            format!("Actual: This file exists '{0}'", failure.actual),
        ],
        FailureType::FileDoesNotExist => vec![
            format!("Expected file: '{0}'", failure.expectation),
            format!("File error message: '{0}'", failure.actual),
        ],
        FailureType::FileContents => vec![
            format!("Expected file contents: '{0}'", failure.expectation),
            format!("Actual file contents: '{0}'", failure.actual),
        ],
    }
}

fn perform_command(shell: &String, args: [&String; 2]) -> TestRun {
    let mut command = Command::new(shell);
    command.args(args);

    let now = Instant::now();
    let output_option = command.output();
    let time_in_milliseconds = now.elapsed().as_millis();

    let output = output_option.unwrap();

    TestRun {
        performance: time_in_milliseconds,
        exit_code: output.status.code().unwrap(),
        output: String::from(str::from_utf8(&output.stdout).unwrap()),
        error: String::from(str::from_utf8(&output.stderr).unwrap()),
    }
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
    pub run: TestRun
}

#[derive(Clone)]
pub struct TestRun {
    pub performance: u128,
    pub exit_code: i32,
    pub output: String,
    pub error: String,
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
