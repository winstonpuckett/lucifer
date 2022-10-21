use args_getter::get_args;
use exiter::exit;
use router::route;

mod router;

mod args_getter;
mod test_runner;
mod exiter;
mod logger;
mod suite_getter;

fn main() {
    let result = get_args().and_then(route);

    exit(result);
}

type CommandResult<TOut> = Result<TOut, (ExitCode, Option<String>)>;

#[derive(Debug)]
pub enum ExitCode {
    Ok = 0,
    FailingTest = 1,
    UserError = 2,
}
