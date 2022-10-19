use router::route;

mod router;

mod suite;
mod executor;
mod logger;
mod args;

fn main() {
    let args = args::get();

    let exit_code: i32 = route(args);

    std::process::exit(exit_code)
}
