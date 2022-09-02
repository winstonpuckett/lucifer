mod constructor;
mod executor;
mod logger;
mod args;

fn main() {
    logger::log("");
    logger::log("🐍  LUCIFER  🐍");
    logger::log(&format!("Executing tests in '{0}'", &args::get_test_folder()));

    let suite = constructor::construct(&args::get_test_folder()).unwrap();
    executor::execute(suite);
}