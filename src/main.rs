use std::io::Write;

use args::Args;
use logger::{log, log_newline};

mod suite;
mod executor;
mod logger;
mod args;

fn main() {
    let args = args::get();

    let exit_code: i32 = match args.run_mode {
        args::RunMode::None => run(&args),
        args::RunMode::Help => help(&args),
        args::RunMode::Version => version(&args),
    };

    std::process::exit(exit_code)
}

fn run(args: &Args) -> i32 {
    logger::log_newline();
    logger::log("🐍  LUCIFER  🐍");
    logger::log(&format!("Executing tests in '{0}'", args.input_directory));

    let suite = suite::get(args).unwrap();
    let results = executor::execute(suite, args);

    let mut data = json::JsonValue::new_object();
    data["testResults"] = json::JsonValue::new_array();//results;
    
    let mut success = true;
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
                    executor::FailureType::FileDoesNotExist => "fileMissing",
                    executor::FailureType::FileContents => "fileContents",
                },
                expectation: f.expectation,
                actual: f.actual
            }).unwrap();
        }

        data["testResults"].push(json::object!{
            succeeded: r.succeeded,
            // TODO: is u64 enough?
            milliseconds: r.performance as u64,
            failures: failures
        }).unwrap();
    }

    let mut file = std::fs::File::create(format!("{}/{}", args.output_directory,"results.json")).expect("create failed");
    file.write_all(json::stringify(data).as_bytes()).expect("write failed");

    if success {
        0
    } else {
        1
    }
}

fn help(_command: &Args) -> i32 {
    log("LUCIFER");
    log("Illuminating CLI testing.");
    log("Winston Puckett");
    log_newline();
    log("Helpful Links:");
    log("• Documentation: https://github.com/winstonpuckett/lucifer");
    log("• View source code: https://github.com/winstonpuckett/lucifer");
    log("• Support the project: https://github.com/winstonpuckett/lucifer");
    log("• Need help?: https://github.com/winstonpuckett/lucifer/issues");
    
    log_newline();
    log(&format!("version: {0}", env!("CARGO_PKG_VERSION")));
    log_newline();
    
    log("USAGE:");
    log("    lucifer [FLAGS] [OPTIONS]");
    log_newline();
    
    log("FLAGS:");
    log("    -h, --help                                  Print the help output.");
    log("    -v, --version                               Print the currently running version.");
    log("    -s, --silent                                Suppress all console output.");
    log("    -n, --no-file                               Suppress all file output.");
    log("    -e, --emoji                                Use emoji output in the console.");
    log_newline();
    log("OPTIONS:");
    log("    -i, --input-directory <folder_path>         The path to the test files. Default: .");
    log("    -o, --output-directory <folder_path>        Where to store resulting files. Default: .");
    log("    -c, --command <file_path>                   The default command to run.");
    0
}

fn version(_command: &Args) -> i32 {
    log(&format!("v{0}", env!("CARGO_PKG_VERSION")));
    0
}