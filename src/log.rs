pub struct Logger;

impl Logger {
    pub fn info<T: AsRef<str>>(msg: T) {
        let color_code = (0, 255, 0);
        let log_message = color(color_code, " INFO ", msg.as_ref());

        println!("{}", log_message);
    }

    pub fn warn<T: AsRef<str>>(msg: T) {
        let color_code = (255, 244, 24);
        let log_message = color(color_code, " WARN ", msg.as_ref());

        println!("{}", log_message);
    }

    pub fn error<T: AsRef<str>>(msg: T) {
        let color_code = (255, 0, 0);
        let log_message = color(color_code, " ERROR", msg.as_ref());

        eprintln!("{}", log_message);
    }
}

fn color(rgb: (i32, i32, i32), level: &str, msg: &str) -> String {
    let (r, g, b) = rgb;
    let prefix = format!("\x1B[38;2;{0};{1};{2}m", r, g, b);

    let postfix = "\x1B[0m";

    let colored = format!("{2}[ {0} ]{3} - {1}", level, msg, prefix, postfix);

    colored
}
