use crate::config::get_short_path;

const TEMPLATE: &str = include_str!("../../public/index.html");

pub fn generate(content: &str, path: &str) -> String {
    TEMPLATE
        .replace("{content}", content)
        .replace("{path}", &get_short_path(path.to_string()))
}
