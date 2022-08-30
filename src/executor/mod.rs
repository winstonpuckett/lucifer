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
    // println!("version: {0}, command: {1}, execution_directory: {2}, test_count: {3}"
    //     , suite.settings.version
    //     , suite.settings.command
    //     , suite.settings.execution_directory
    //     , suite.tests.len());

    let (shell, first_arg) = get_shell();

    for feature in suite.features {
        println!("Feature: {0}", feature.name);
        println!("");

        for test in feature.tests {
            let output = Command::new(&shell)
                .args([&first_arg, &to_arg(&suite.settings.command, &test.args)])
                .output()
                .expect("Could not call tool");
            
            println!("Test: {:?}", test.name);
            println!("Description: {:?}", test.description);
            let code = output.status;
            println!("{code}");
            let stdout = str::from_utf8(&output.stdout).unwrap();
            println!("stdout: {stdout}");
            let stderr = str::from_utf8(&output.stderr).unwrap();
            println!("stderr: {stderr}");
        }
    }
    

    

    TestResult { }
}

fn to_arg(command: &String, args: &Vec<String>) -> String {
    std::format!("{0} {1}", 
        command, 
        args.join(" "))
}

// struct Commandlet {
//     shell: String,
//     args: Vec<String>
// }

pub struct TestResult { 

}

fn get_shell() -> (String, String) {
    if cfg!(target_os = "windows") {
        (String::from("cmd"), String::from("/C"))
    } else {
        (String::from("sh"), String::from("-c"))
    }
}