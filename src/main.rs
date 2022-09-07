use std::process::ExitCode;

use args::{get_command, RunCommand};
use logger::log;

mod constructor;
mod executor;
mod logger;
mod args;



fn main() -> ExitCode {
    let command = get_command();

    let exit_code: u8 = match command.command {
        args::CommandType::None => summarize(command),
        args::CommandType::Run => run(command),
        args::CommandType::Help => help(command),
        args::CommandType::Version => version(command),
    };

    ExitCode::from(exit_code)
}

fn summarize(_command: RunCommand) -> u8 {
    logger::log("summary 🐍  LUCIFER  🐍");
    0
}

fn run(command: RunCommand) -> u8 {
    logger::log("run ran");
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

fn help(_command: RunCommand) -> u8 {
    log("help ran");
    0
}

fn version(_command: RunCommand) -> u8 {
    log("version ran");
    0
}