use args::{get_command, RunCommand};
use logger::log;

mod constructor;
mod executor;
mod logger;
mod args;



fn main() {
    let command = get_command();

    let exit_code: i32 = match command.command {
        args::CommandType::None => summarize(command),
        args::CommandType::Run => run(command),
        args::CommandType::Help => help(command),
        args::CommandType::Version => version(command),
    };

    std::process::exit(exit_code)
}

fn summarize(_command: RunCommand) -> i32 {
    logger::log("summary 🐍  LUCIFER  🐍");
    0
}

fn run(command: RunCommand) -> i32 {
    logger::log_newline();
    logger::log("🐍  LUCIFER  🐍");
    logger::log(&format!("Executing tests in '{0}'", command.input_directory));

    let suite = constructor::construct(&command.input_directory).unwrap();
    let results = executor::execute(suite);

    // if in error, 1. otherwise 0.
    if results.into_iter().any(|r| !r.succeeded) {
        1
    } else {
        0
    }
}

fn help(_command: RunCommand) -> i32 {
    log("help ran");
    0
}

fn version(_command: RunCommand) -> i32 {
    log("version ran");
    0
}