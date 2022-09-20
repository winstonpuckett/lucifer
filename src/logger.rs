use crate::suite;

pub fn log_heading(suite: &suite::Suite, message: &str) {
    log(suite, &format!("🌳 {0} 🌳", message));
    log_newline(suite);
}

pub fn log_success(suite: &suite::Suite, message: &str) {
    log(suite, &format!("  🍏 {0}", message));
    log_newline(suite);
}

pub fn log_failure(suite: &suite::Suite, message: &str) {
    log(suite, &format!("  🍎 {0}", message));
    log_newline(suite);
}

pub fn log_details(suite: &suite::Suite, messages: Vec<&str>) {
    for detail in messages {
        log_detail(suite, detail);
    }
    log_newline(suite);
}

pub fn log_detail(suite: &suite::Suite, message: &str) {
    log(suite, &format!("    🌿 {0}", message));
}

pub fn log_newline(suite: &suite::Suite) {
    log(suite, "");
}

pub fn log(suite: &suite::Suite, message: &str) {
    if suite.args.console_silent {
        return;
    }

    println!("{message}")
}
