use crate::{
    args_getter::Args, output_writer::write_test_results, suite_getter::get_suite,
    test_runner::run_suite, CommandResult, ExitCode,
};

pub fn execute(args: Args) -> CommandResult<()> {
    let suite = get_suite(args)?;

    let result = run_suite(&suite);

    write_test_results(&result, &suite)?;

    get_status(result)
}

fn get_status(result: crate::test_runner::SuiteResult) -> Result<(), (ExitCode, Option<String>)> {
    if result.success {
        Ok(())
    } else {
        Err((ExitCode::FailingTest, None))
    }
}
