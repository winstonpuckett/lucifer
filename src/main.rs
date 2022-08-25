use std::process::Command;
use std::str;
use std::env;

// To test:
// cargo run -- /C echo hello, bobby

fn main() {
    let args: Vec<String> = env::args().collect();

    let output = Command::new("cmd")
        .args(args)
        .output()
        .expect("Could not call tool");
    
    let code = output.status;
    println!("{code}");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    print!("stdout: {stdout}");
    
    let stderr = str::from_utf8(&output.stderr).unwrap();
    print!("stderr: {stderr}");
}
