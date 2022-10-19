use crate::args::{RunMode, Args};

mod test;
mod help;
mod version;

pub fn route(args: Args) -> i32 {
    match args.run_mode {
        RunMode::None => self::test::execute(args),
        RunMode::Help => self::help::execute(),
        RunMode::Version => self::version::execute(),
    }
}
