use router::route;

mod router;

mod suite_getter;
mod executor;
mod logger;
mod args_getter;

fn main() {
    let args = args_getter::get();

    let exit_code: i32 = route(args);

    std::process::exit(exit_code)
}
