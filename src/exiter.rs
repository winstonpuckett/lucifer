use crate::{ExitCode, CommandResult};

pub fn exit<TOk: std::fmt::Debug>(result: CommandResult<TOk>) {
    if result.is_ok() {
        std::process::exit(ExitCode::Ok as i32)
    } else {
        let (code, msg_option) = result.unwrap_err();

        if msg_option.is_some() {
            let msg = msg_option.unwrap();
            eprintln!("{msg}");
        }
        
        std::process::exit(code as i32)
    }
}
