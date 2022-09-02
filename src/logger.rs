use crate::args;

pub fn log_section(messages: Vec<&str>) {
    for m in messages {
        log(m);
    }
    log_newline();
}

pub fn log_newline() {
    log("");
}

pub fn log(message: &str) {
    match args::get_output_type() {
        args::OutputType::Console => println!("{message}"),
        args::OutputType::File => todo!(),
    }
}