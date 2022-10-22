use crate::{
    args_getter::Args, output_writer::write_test_results, suite_getter::get_suite,
    test_runner::run_suite, CommandResult, ExitCode,
};

pub fn execute(args: Args) -> CommandResult<()> {
    let suite = get_suite(args)?;
    let result = run_suite(&suite);

    write_test_results(&result, &suite)?;

    if result.success {
        Ok(())
    } else {
        Err((ExitCode::FailingTest, None))
    }
}
