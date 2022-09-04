pub fn log_heading(message: &str) {
    log(&format!("🌳 {0} 🌳", message));
    log_newline();
}

pub fn log_success(message: &str) {
    log(&format!("  🍏 {0}", message));
    log_newline();
}

pub fn log_failure(message: &str) {
    log(&format!("  🍎 {0}", message));
    log_newline();
}

pub fn log_details(messages: Vec<&str>) {
    for detail in messages {
        log_detail(detail);
    }
    log_newline();
}

pub fn log_detail(message: &str) {
    log(&format!("    🌿 {0}", message));
}

pub fn log_newline() {
    log("");
}

pub fn log(message: &str) {
    println!("{message}")
}
