use args::{get_command, RunCommand};
use logger::log;

mod constructor;
mod executor;
mod logger;
mod args;



fn main() {
    let command = get_command();

    match command.command {
        args::CommandType::None => summarize(command),
        args::CommandType::Run => run(command),
        args::CommandType::Help => help(command),
        args::CommandType::Version => version(command),
    }
}

fn summarize(_command: RunCommand) {
    logger::log("summary 🐍  LUCIFER  🐍");
}

fn run(command: RunCommand) {
    logger::log("run ran");
    logger::log("🐍  LUCIFER  🐍");
    logger::log(&format!("Executing tests in '{0}'", command.input_directory));

    let suite = constructor::construct(&command.input_directory).unwrap();
    executor::execute(suite);
}

fn help(_command: RunCommand) {
    log("help ran")
}

fn version(_command: RunCommand) {
    log("version ran")
}