mod constructor;
mod executor;

fn main() {
    let suite = constructor::construct("./test_self").unwrap();
    executor::execute(suite);
}