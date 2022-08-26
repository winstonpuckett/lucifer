mod test_getter;

fn main() {
    let settings = test_getter::find("./test_self").unwrap();
    println!("version: {0}, command: {1}, execution_directory: {2}"
        , settings.version
        , settings.command
        , settings.execution_directory);
}


// TODO: Move this to a env module or something.
// To test:
// cargo run -- /C echo hello, bobby
// use std::process::Command;
// use std::str;
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