const TEMPLATE: &str = include_str!("../../public/index.html");

pub fn generate(content: &str, path: &str) -> String {
    TEMPLATE
        .replace("{content}", content)
        .replace("{path}", path)
}
