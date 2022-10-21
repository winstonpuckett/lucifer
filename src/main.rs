use exiter::exit;
use router::route;

mod router;

mod suite_getter;
mod executor;
mod logger;
mod args_getter;
mod exiter;

fn main() {
    let result = args_getter::get()
    .and_then(route);
    
    exit(result);
}

type CommandResult<TOut> = Result<TOut, (ExitCode, Option<String>)>;

#[derive(Debug)]
pub enum ExitCode {
    Ok = 0,
    FailingTest = 1,
    UserError = 2,
}