use args::Args;
use logger::log;

mod suite;
mod executor;
mod logger;
mod args;

fn main() {
    let args = args::get();

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

    if results.into_iter().all(|r| r.succeeded) {
        0
    } else {
        1
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