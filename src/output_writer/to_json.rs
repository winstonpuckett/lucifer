use json::JsonValue;

use crate::test_runner::{Failure, SuiteResult, TestResult};

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
        feature: result.feature,
        test: result.test,
        succeeded: result.succeeded,
        milliseconds: result.run.performance as u64,
        standardOut: result.run.output,
        standardError: result.run.error,
        exitCode: result.run.exit_code,
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
