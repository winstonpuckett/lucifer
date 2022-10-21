use std::env;

use crate::ExitCode;

use super::{Args, RunMode};

pub fn mutate(result: &mut Args, current_arg: String, args: &mut std::iter::Peekable<env::Args>) -> Result<(), (ExitCode, Option<String>)> {
    if is_help_command(&current_arg) {
        result.run_mode = RunMode::Help;
    } else if is_version_command(&current_arg) {
        result.run_mode = RunMode::Version;
    } else if is_silent_flag(&current_arg) {
        result.console_silent = true;
    } else if is_no_file_flag(&current_arg) {
        result.no_file = true;
    } else if is_input_directory(&current_arg) {
        if args.peek().is_none() {
            return Err((ExitCode::UserError, Some(format!("Expected an input directory after {:?}, but none was provided.", current_arg))));
        }
            
        result.input_directory = args.next().unwrap();
    } else if is_output_directory(&current_arg) {
        if args.peek().is_none() {
            return Err((ExitCode::UserError, Some(format!("Expected an output directory after {:?}, but none was provided.", current_arg))));
        }

        result.output_directory = args.next().unwrap();
    }

    Ok(())
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