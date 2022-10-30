use yaml_rust::Yaml;

pub fn to_string_from_string_or_i64_default_empty(g: &Yaml) -> String {
    let str_option = g.as_str();
    if str_option.is_some() {
        return String::from(str_option.unwrap());
    }
    
    let i64_option = g.as_i64();
    if i64_option.is_some() {
        return format!("{}", i64_option.unwrap());
    }

    String::from("")
}

pub fn to_string_or_default(y: &Yaml, default: &str) -> String {
    String::from(y.as_str().unwrap_or(default))
}

pub fn to_string_or_blank(y: &Yaml) -> String {
    String::from(y.as_str().unwrap_or(""))
}

pub fn to_string_option(y: &Yaml) -> Option<String> {
    y.as_str().and_then(|o| Some(String::from(o)))
}

pub fn to_u64_option(y: &Yaml) -> Option<u64> {
    y.as_i64().and_then(|v| Some(v as u64))
}

pub fn to_i32_or_default(y: &Yaml, default: i32) -> i32 {
    y.as_i64().unwrap_or(default as i64) as i32
}