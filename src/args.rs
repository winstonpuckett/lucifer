use std::env;

pub fn get() -> Args {
    let args: Vec<String> = env::args().collect();
    let args_max_iter_value = args.len();

    // TODO: Store defaults somewhere sensible.
    let mut result = Args {
        run_mode: RunMode::None,
        execution_directory: String::from("."),
        input_directory: String::from("."),
        output_directory: String::from("."),
        console_silent: false,
        no_file: false,
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
            result.run_mode = RunMode::Help;
        } else if is_version_command(&arg) {
            result.run_mode = RunMode::Version;
        } else if is_silent_flag(&arg) {
            result.console_silent = true;
        } else if is_no_file_flag(&arg) {
            result.no_file = true;
        } else if is_input_directory(&arg) {
            result.input_directory = (&args[i + 1]).to_owned();
            skip = true;
        } else if is_output_directory(&arg) {
            result.output_directory = (&args[i + 1]).to_owned();
            skip = true;
        }
    };

    result
}


fn is_help_command(arg: &String) -> bool {
    arg.eq_ignore_ascii_case("-h")
    || arg.eq_ignore_ascii_case("--help")
}
fn is_version_command(arg: &String) -> bool {
    arg.eq_ignore_ascii_case("-v")
    || arg.eq_ignore_ascii_case("--version")
}
fn is_silent_flag(arg: &String) -> bool {
    arg.eq_ignore_ascii_case("-s")
    || arg.eq_ignore_ascii_case("--silent")
}
fn is_no_file_flag(arg: &String) -> bool {
    arg.eq_ignore_ascii_case("-n")
    || arg.eq_ignore_ascii_case("--no-file")
}

fn is_input_directory(arg: &String) -> bool {
    arg.eq_ignore_ascii_case("-i")
    || arg.eq_ignore_ascii_case("--input-directory")
}
fn is_output_directory(arg: &String) -> bool {
    arg.eq_ignore_ascii_case("-o")
    || arg.eq_ignore_ascii_case("--output-directory")
}

pub struct Args {
    pub run_mode: RunMode,

    pub output_directory: String,
    pub execution_directory: String,
    pub input_directory: String,
    
    pub console_silent: bool,
    pub no_file: bool,
}

pub enum RunMode {
    None,
    Help,
    Version
}
