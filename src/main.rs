mod constructor;
mod executor;
mod logger;
mod args;

fn main() {
    logger::log("");
    logger::log("🐍  LUCIFER  🐍");

    let suite = constructor::construct("./test_self").unwrap();
    executor::execute(suite);
}