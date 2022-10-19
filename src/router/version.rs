pub fn execute() -> i32 {
    println!("v{0}", env!("CARGO_PKG_VERSION"));
    0
}