use crate::suite_getter;

pub fn log_heading(suite: &suite_getter::Suite, message: &str) {
    log(suite, &format!("🐲 {0}", message));
    log_newline(suite);
}

pub fn log_success(suite: &suite_getter::Suite, message: &str) {
    log(suite, &format!("  🎉 {0}", message));
    log_newline(suite);
}

pub fn log_failure(suite: &suite_getter::Suite, message: &str) {
    log(suite, &format!("  ❌ {0}", message));
    log_newline(suite);
}

pub fn log_details(suite: &suite_getter::Suite, messages: Vec<&str>) {
    for detail in messages {
        log_detail(suite, detail);
    }
    log_newline(suite);
}

pub fn log_detail(suite: &suite_getter::Suite, message: &str) {
    log(suite, &format!("    → {0}", message));
}

pub fn log_newline(suite: &suite_getter::Suite) {
    log(suite, "");
}

pub fn log(suite: &suite_getter::Suite, message: &str) {
    if suite.args.console_silent {
        return;
    }

    println!("{message}")
}
