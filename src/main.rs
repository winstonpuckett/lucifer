use std::{io::Write, fs, path::Path};

use args::Args;

mod suite;
mod executor;
mod logger;
mod args;

fn main() {
    let args = args::get();

    let exit_code: i32 = match args.run_mode {
        args::RunMode::None => run(args),
        args::RunMode::Help => help(&args),
        args::RunMode::Version => version(&args),
    };

    std::process::exit(exit_code)
}

fn run(args: Args) -> i32 {
    let suite = suite::get(args).unwrap();
    let results = executor::execute(&suite);
    
    let mut success = true;
    let mut data = json::JsonValue::new_object();
    data["testResults"] = json::JsonValue::new_array();
    for r in results {
        success &= r.succeeded;
        let mut failures = json::JsonValue::new_array();
        for f in r.failures {
            failures.push(json::object!{
                type: match f.failure_type{
                    executor::FailureType::Performance => "performance",
                    executor::FailureType::ExitCode => "exitCode",
                    executor::FailureType::Output => "output",
                    executor::FailureType::Error => "error",
                    executor::FailureType::FileExists => "fileExists",
                    executor::FailureType::FileDoesNotExist => "fileMissing",
                    executor::FailureType::FileContents => "fileContents",
                },
                expectation: f.expectation,
                actual: f.actual
            }).unwrap();
        }

        data["testResults"].push(json::object!{
            succeeded: r.succeeded,
            milliseconds: r.performance as u64,
            failures: failures
        }).unwrap();
    }

    if !suite.args.no_file {
        if !Path::new(&suite.args.output_directory).is_dir() {
            fs::create_dir_all(&suite.args.output_directory).unwrap();
        }
        
        let mut file = std::fs::File::create(format!("{}/{}", &suite.args.output_directory, "results.json")).expect("create failed");
        file.write_all(json::stringify(data).as_bytes()).expect("write failed");
    }

    if success {
        0
    } else {
        1
    }
}

fn help(_command: &Args) -> i32 {
    println!("🐉 LUCIFER 🐉");
    println!("Illuminating CLI testing.");
    println!("Winston Puckett");
    println!();
    println!("Helpful Links:");
    println!("• Documentation: https://github.com/winstonpuckett/lucifer");
    println!("• View source code: https://github.com/winstonpuckett/lucifer");
    println!("• Support the project: https://github.com/winstonpuckett/lucifer");
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
    0
}

fn version(_command: &Args) -> i32 {
    println!("v{0}", env!("CARGO_PKG_VERSION"));
    0
}