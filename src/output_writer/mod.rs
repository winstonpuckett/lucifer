use crate::{suite_getter, test_runner::SuiteResult, CommandResult};

use self::{to_json::suite_to_json, writer::write_file};

mod to_json;
mod writer;

pub fn write_test_results(result: &SuiteResult, suite: &suite_getter::Suite) -> CommandResult<()> {
    if suite.args.no_file {
        return Ok(());
    }

    let json_string = suite_to_json(result);
    let file_path = &suite.args.output_directory;

    write_file(file_path, json_string)
}

