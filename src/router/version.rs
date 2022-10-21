use crate::CommandResult;

pub fn execute() -> CommandResult<()> {
    println!("v{0}", env!("CARGO_PKG_VERSION"));

    Ok(())
}