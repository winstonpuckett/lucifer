use crate::{args_getter::{RunMode, Args}, CommandResult};

mod test;
mod help;
mod version;

pub fn route(args: Args) -> CommandResult<()> {
    match args.run_mode {
        RunMode::None => self::test::execute(args),
        RunMode::Help => self::help::execute(),
        RunMode::Version => self::version::execute(),
    }
}
