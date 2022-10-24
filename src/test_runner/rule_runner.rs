use std::fs;

use crate::suite_getter::Expectations;
use super::{TestRun, Failure, FailureType};

pub fn assert_all(expectations: &Expectations, result: &TestRun) -> Vec<Failure> {
    let mut all = vec![];

    all.append(&mut assert_performance(expectations, result));
    all.append(&mut assert_exit_code(expectations, result));
    all.append(&mut assert_output(expectations, result));
    all.append(&mut assert_error(expectations, result));
    all.append(&mut assert_no_file(expectations, result));
    all.append(&mut assert_file(expectations, result));
    all.append(&mut assert_file_content(expectations, result));

    all
}

fn assert_performance(expectations: &Expectations, result: &TestRun) -> Vec<Failure> {
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

fn assert_exit_code(expectations: &Expectations, result: &TestRun) -> Vec<Failure> {
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

fn assert_output(expectations: &Expectations, result: &TestRun) -> Vec<Failure> {
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
        actual,
    }]
}

fn assert_error(expectations: &Expectations, result: &TestRun) -> Vec<Failure> {
    if expectations.error.is_none() {
        return vec![];
    }

    let expectation = String::from(expectations.error.as_ref().unwrap());
    let actual = result.error.to_owned();
    if expectation == actual {
        return vec![];
    }

    vec![Failure {
        failure_type: FailureType::Error,
        expectation,
        actual,
    }]
}

fn assert_file(expectations: &Expectations, _: &TestRun) -> Vec<Failure> {
    if expectations.file.is_none() {
        return vec![];
    }

    let file_which_should_exist = expectations.file.to_owned().unwrap();
    let metadata = fs::metadata(&file_which_should_exist);

    if metadata.is_ok() {
        return vec![];
    }

    vec![Failure {
        failure_type: FailureType::FileDoesNotExist,
        expectation: expectations.file.to_owned().unwrap(),
        actual: String::from("File does not exist or cannot be accessed."),
    }]
}

fn assert_file_content(expectations: &Expectations, _: &TestRun) -> Vec<Failure> {
    // If we cover expectations in assert_file or don't expect content, we're good.
    if expectations.file.is_none() || expectations.contents.is_none() {
        return vec![];
    }

    let file_expectation = expectations.to_owned().file.unwrap();
    let content_result = fs::read_to_string(&file_expectation);

    if content_result.is_err() {
        return vec![];
    }

    let expectation = expectations.contents.to_owned().unwrap();
    let actual = content_result.unwrap();

    // If contents equals
    if expectation == actual {
        return vec![];
    }

    vec![Failure {
        failure_type: FailureType::FileContents,
        expectation,
        actual,
    }]
}

fn assert_no_file(expectations: &Expectations, _: &TestRun) -> Vec<Failure> {
    if expectations.no_file.is_none() {
        return vec![];
    }

    let file_which_shouldnt_exist = expectations.no_file.to_owned().unwrap();

    let metadata = fs::metadata(&file_which_shouldnt_exist);

    if metadata.is_err() || !metadata.unwrap().is_file() {
        return vec![];
    }

    vec![Failure {
        failure_type: FailureType::FileExists,
        expectation: String::from(""),
        actual: file_which_shouldnt_exist,
    }]
}
