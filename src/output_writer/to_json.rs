use json::JsonValue;

use crate::test_runner::{SuiteResult, TestResult, Failure};

pub fn suite_to_json(suite: &SuiteResult) -> String {
    let mut json = json::JsonValue::new_object();
    json["testResults"] = json::JsonValue::Array(
        suite
            .test_results
            .clone()
            .into_iter()
            .map(test_result_to_json)
            .collect::<Vec<JsonValue>>(),
    );
    json::stringify(json)
}

fn test_result_to_json(result: TestResult) -> JsonValue {
    json::object! {
        succeeded: result.succeeded,
        milliseconds: result.performance as u64,
        failures: result.failures
            .clone()
            .into_iter()
            .map(failure_to_json)
            .collect::<Vec<JsonValue>>()
    }
}

fn failure_to_json(failure: Failure) -> JsonValue {
    json::object! {
        type: failure.failure_type.to_string(),
        expectation: String::from(&failure.expectation),
        actual: String::from(&failure.actual)
    }
}