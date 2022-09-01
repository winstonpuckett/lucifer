use crate::args;

pub fn log(message: &str) {
    match args::get_output_type() {
        args::OutputType::Console => println!("{message}"),
        args::OutputType::File => todo!(),
    }
}