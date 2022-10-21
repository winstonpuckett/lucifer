use std::env;

use crate::CommandResult;

mod result_mutator;

pub fn get() -> CommandResult<Args> {
    let mut args = env::args().into_iter().peekable();

    if args.peek().is_none() {
        Ok(get_default_args())
    } else {
        extract_args(args)
    }
}

fn extract_args(mut args: std::iter::Peekable<env::Args>) -> CommandResult<Args> {
    let mut result = get_default_args();

    // First argument is command path. Skip that argument.
    args.next();
    
    while args.peek().is_some() {
        // This unwrap is safe because we've called peek.
        let arg = args.next().unwrap();

        let result = self::result_mutator::mutate(&mut result, arg, &mut args);

        if result.is_err() {
            return Err(result.unwrap_err());
        }
    };
    
    Ok(result)
}

fn get_default_args() -> Args {
    Args {
        run_mode: RunMode::None,
        input_directory: String::from("."),
        output_directory: String::from("."),
        console_silent: false,
        no_file: false,
    }
}

#[derive(Debug)]
pub struct Args {
    pub run_mode: RunMode,

    pub output_directory: String,
    pub input_directory: String,
    
    pub console_silent: bool,
    pub no_file: bool,
}

#[derive(Debug)]
pub enum RunMode {
    None,
    Help,
    Version
}
