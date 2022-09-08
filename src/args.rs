use std::env;

pub fn get_command() -> RunCommand {
    // TODO: Don't grab the args twice.
    let args_max_iter_value = env::args().count();
    let args: Vec<String> = env::args().collect();

    // TODO: Store defaults somewhere sensible.
    let mut result = RunCommand {
        command: CommandType::None,
        execution_directory: String::from("."),
        input_directory: String::from("."),
        output_directory: String::from("."),
        console_silent: false,
        no_file: true,
    };

    // First argument is command path. Skip that argument.
    let mut skip = true;

    for i in 0..args_max_iter_value {

        if skip { 
            skip = false;
            continue;
        };
        
        let arg_option = &args[i];
        let arg = arg_option;

        if is_help_command(&arg) {
            result.command = CommandType::Help;
        } else if is_version_command(&arg) {
            result.command = CommandType::Version;
        } else if is_input_directory(&arg) {
            result.input_directory = (&args[i + 1]).to_owned();
            skip = true;
        } else if is_output_directory(&arg) {
            result.output_directory = (&args[i + 1]).to_owned();
            skip = true;
        } else if is_execution_directory(&arg) {
            result.execution_directory = (&args[i + 1]).to_owned();
            skip = true;
        }
    };

    result
}


fn is_help_command(arg: &String) -> bool {
    arg.eq_ignore_ascii_case("help")
}
fn is_version_command(arg: &String) -> bool {
    arg.eq_ignore_ascii_case("version")
}

fn is_input_directory(arg: &String) -> bool {
    arg.eq_ignore_ascii_case("-i")
    || arg.eq_ignore_ascii_case("--input-directory")
}
fn is_output_directory(arg: &String) -> bool {
    arg.eq_ignore_ascii_case("-o")
    || arg.eq_ignore_ascii_case("--output-directory")
}
fn is_execution_directory(arg: &String) -> bool {
    arg.eq_ignore_ascii_case("-o")
    || arg.eq_ignore_ascii_case("--output-directory")
}

pub struct RunCommand {
    pub command: CommandType,

    pub output_directory: String,
    pub execution_directory: String,
    pub input_directory: String,
    
    pub console_silent: bool,
    pub no_file: bool,
}

pub enum CommandType {
    None,
    Help,
    Version
}
