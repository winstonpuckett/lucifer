use crate::{args_getter::{RunMode, Args}, ExitCode};

mod test;
mod help;
mod version;

pub fn route(args: Args) -> Result<i32, (ExitCode, String)> {
    match args.run_mode {
        RunMode::None => Ok(self::test::execute(args)),
        RunMode::Help => Ok(self::help::execute()),
        RunMode::Version => Ok(self::version::execute()),
    }
}
