use crate::CommandResult;

pub fn execute() -> CommandResult<()> {
    println!("🐉 LUCIFER 🐉");
    println!("Illuminating CLI testing.");
    println!("Winston Puckett");
    println!();
    println!("Helpful Links:");
    println!("• Documentation: https://github.com/winstonpuckett/lucifer");
    println!("• View source code: https://github.com/winstonpuckett/lucifer");
    println!("• Need help?: https://github.com/winstonpuckett/lucifer/issues");
    
    println!();
    println!("version: {0}", env!("CARGO_PKG_VERSION"));
    println!();
    
    println!("USAGE:");
    println!("    lucifer [FLAGS] [OPTIONS]");
    println!();
    
    println!("FLAGS:");
    println!("    -h, --help                                  Print the help output.");
    println!("    -v, --version                               Print the currently running version.");
    println!("    -s, --silent                                Suppress all console output.");
    println!("    -n, --no-file                               Suppress all file output.");
    println!();
    println!("OPTIONS:");
    println!("    -i, --input-directory <folder_path>         The path to the test files. Default: .");
    println!("    -o, --output-directory <folder_path>        Where to store resulting files. Default: .");

    Ok(())
}