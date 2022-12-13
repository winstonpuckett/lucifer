use std::env;

use crate::ExitCode;

use super::{Args, RunMode};

pub fn mutate(
    result: &mut Args,
    current_arg: String,
    args: &mut std::iter::Peekable<env::Args>,
) -> Result<(), (ExitCode, Option<String>)> {
    mutate_flags(&current_arg, result);

    return mutate_options(current_arg, result, args);
}

fn mutate_options(current_arg: String, result: &mut Args, args: &mut std::iter::Peekable<env::Args>) -> Result<(), (ExitCode, Option<String>)> {
    let options_parsers = get_options();
    let options = options_parsers
        .iter()
        .filter(|f| {
            f.name_short.eq_ignore_ascii_case(&current_arg)
                || f.name_long.eq_ignore_ascii_case(&current_arg)
        });

    for o in options {
        let r = (o.mutator)(result, &current_arg, args);
        
        if r.is_err() {
            return r;
        }
    }

    Ok(())
}

fn get_options() -> Vec<OptionParser> {
    vec![
        OptionParser {
            name_short: String::from("-i"),
            name_long: String::from("--input-directory"),
            mutator: |result: &mut Args,
                      current_arg: &String,
                      args: &mut std::iter::Peekable<env::Args>| {
                if args.peek().is_none() {
                    return Err((
                        ExitCode::UserError,
                        Some(format!(
                            "Expected an input directory after {:?}, but none was provided.",
                            current_arg
                        )),
                    ));
                }

                result.input_directory = args.next().unwrap();

                Ok(())
            },
        },
        OptionParser {
            name_short: String::from("-o"),
            name_long: String::from("--output-directory"),
            mutator: |result: &mut Args,
                      current_arg: &String,
                      args: &mut std::iter::Peekable<env::Args>| {
                if args.peek().is_none() {
                    return Err((
                        ExitCode::UserError,
                        Some(format!(
                            "Expected an output directory after {:?}, but none was provided.",
                            current_arg
                        )),
                    ));
                }

                result.output_directory = args.next().unwrap();

                Ok(())
            },
        },
    ]
}

fn mutate_flags(current_arg: &String, result: &mut Args) {
    get_flags()
        .iter()
        .filter(|f| {
            f.name_short.eq_ignore_ascii_case(current_arg)
                || f.name_long.eq_ignore_ascii_case(current_arg)
        })
        .for_each(|f| {
            (f.mutator)(result);
        });
}

fn get_flags() -> Vec<FlagParser> {
    vec![
        FlagParser {
            name_short: String::from("-h"),
            name_long: String::from("--help"),
            mutator: |result: &mut Args| {
                result.run_mode = RunMode::Help;
            },
        },
        FlagParser {
            name_short: String::from("-v"),
            name_long: String::from("--version"),
            mutator: |result: &mut Args| {
                result.run_mode = RunMode::Version;
            },
        },
        FlagParser {
            name_short: String::from("-s"),
            name_long: String::from("--silent"),
            mutator: |result: &mut Args| {
                result.console_silent = true;
            },
        },
        FlagParser {
            name_short: String::from("-n"),
            name_long: String::from("--no-file"),
            mutator: |result: &mut Args| {
                result.no_file = true;
            },
        },
    ]
}

struct FlagParser {
    name_short: String,
    name_long: String,
    mutator: fn(result: &mut Args),
}

struct OptionParser {
    name_short: String,
    name_long: String,
    mutator: fn(
        result: &mut Args,
        current_arg: &String,
        args: &mut std::iter::Peekable<env::Args>,
    ) -> Result<(), (ExitCode, Option<String>)>,
}
