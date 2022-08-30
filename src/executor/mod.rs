use std::process::Command;
use std::str;
// use std::env;
// let args: Vec<String> = env::args().collect();
// let output = Command::new("cmd")
//     .args(args)
//     .output()
//     .expect("Could not call tool");
// let code = output.status;
// println!("{code}");
// let stdout = str::from_utf8(&output.stdout).unwrap();
// print!("stdout: {stdout}");
// let stderr = str::from_utf8(&output.stderr).unwrap();
// print!("stderr: {stderr}");

use crate::constructor;

pub fn execute(suite: constructor::Suite) -> TestResult {
    println!("version: {0}, command: {1}, execution_directory: {2}, test_count: {3}"
        , suite.settings.version
        , suite.settings.command
        , suite.settings.execution_directory
        , suite.tests.len());

        let output = Command::new("sh")
            .args(["-c", "echo", "hello"])
            .output()
            .expect("Could not call tool");

        let code = output.status;
        println!("{code}");
        let stdout = str::from_utf8(&output.stdout).unwrap();
        print!("stdout: {stdout}");
        let stderr = str::from_utf8(&output.stderr).unwrap();
        print!("stderr: {stderr}");

    TestResult { }
}

// fn test(cmd: Commandlet) -> TestResult {
//     panic!()
// }

// struct Commandlet {
//     shell: String,
//     args: Vec<String>
// }

pub struct TestResult { 

}