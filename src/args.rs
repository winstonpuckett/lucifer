use std::env;

pub fn get_test_folder() -> String {
    let mut a = env::args();

    a.next();
    a.next().unwrap_or(String::from("."))
}

pub fn get_output_type() -> OutputType {
    for argument in env::args() {
        if argument == "--output-file" {
            return OutputType::File;
        }
    }

    return OutputType::Console;
}

pub enum OutputType {
    Console,
    File
}