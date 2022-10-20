use router::route;

mod router;

mod suite_getter;
mod executor;
mod logger;
mod args_getter;

fn main() {
    let args_result = args_getter::get();

    if args_result.is_err() {
        let (code, msg) = args_result.unwrap_err();
        
        eprintln!("{msg}");
        std::process::exit(code as i32)
    }

    let args = args_result.unwrap();

    let exit_code = route(args).unwrap();

    std::process::exit(exit_code)
}

#[derive(Debug)]
pub enum ExitCode {
    Ok = 0,
    FailingTest = 1,
    UserError = 2,
}