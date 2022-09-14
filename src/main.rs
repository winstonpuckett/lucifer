use args::{args, Args};
use logger::log;

mod suite;
mod executor;
mod logger;
mod args;

fn main() {
    let args = args();

    let exit_code: i32 = match args.run_mode {
        args::RunMode::None => run(&args),
        args::RunMode::Help => help(&args),
        args::RunMode::Version => version(&args),
    };

    std::process::exit(exit_code)
}

fn run(args: &Args) -> i32 {
    logger::log_newline();
    logger::log("🐍  LUCIFER  🐍");
    logger::log(&format!("Executing tests in '{0}'", args.input_directory));

    let suite = suite::get(args).unwrap();
    let results = executor::execute(suite, args);

    // if in error, 1. otherwise 0.
    if results.into_iter().any(|r| !r.succeeded) {
        1
    } else {
        0
    }
}

fn help(_command: &Args) -> i32 {
    log("help ran");
    0
}

fn version(_command: &Args) -> i32 {
    log("version ran");
    0
}