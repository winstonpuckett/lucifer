use std::env;

mod result_mutator;

pub fn get() -> Args {
    let mut args = env::args().into_iter().peekable();

    let mut result = get_default_args();

    if args.peek().is_none() {
        return result;
    }

    // First argument is command path. Skip that argument.
    args.next();

    while args.peek().is_some() {
        let arg = args.next().unwrap();

        self::result_mutator::mutate(&mut result, arg, &mut args);
    };

    result
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

pub struct Args {
    pub run_mode: RunMode,

    pub output_directory: String,
    pub input_directory: String,
    
    pub console_silent: bool,
    pub no_file: bool,
}

pub enum RunMode {
    None,
    Help,
    Version
}
