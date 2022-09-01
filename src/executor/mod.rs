use std::process::Command;
use std::str;

use crate::{constructor, logger};

pub fn execute(suite: constructor::Suite) -> TestResult {
    let (shell, first_arg) = get_shell();

    for feature in suite.features {
        format!("Feature: {0}", feature.name);
        logger::log("");

        for test in feature.tests {
            let output = Command::new(&shell)
                .args([&first_arg, &to_arg(&suite.settings.command, &test.args)])
                .output()
                .expect("Could not call tool");
            
            logger::log(&format!("Test: {:?}", test.name));
            logger::log(&format!("Description: {:?}", test.description));
            let code = output.status;
            logger::log(&format!("{code}"));
            let stdout = str::from_utf8(&output.stdout).unwrap();
            logger::log(&format!("stdout: {stdout}"));
            let stderr = str::from_utf8(&output.stderr).unwrap();
            logger::log(&format!("stderr: {stderr}"));
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