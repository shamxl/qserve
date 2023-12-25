pub struct Logger;
const POSTFIX: &str = "\x1B[0m";

fn color(rgb: (i32, i32, i32)) -> String {
    let (r, g, b) = rgb;

    format!("\x1B[38;2;{0};{1};{2}m", r, g, b)
}

impl Logger {
    /*pub fn info<T: AsRef<str>>(msg: T) {
        let message = format!(
            "{0}INFO{1} - {2}",
            color((0, 255, 0)),
            POSTFIX,
            msg.as_ref()
        );

        println!("{}", message);
    }*/

    pub fn error<T: AsRef<str>>(msg: T) {
        let message = format!(
            "{0}ERROR{1} - {2}",
            color((255, 0, 0)),
            POSTFIX,
            msg.as_ref()
        );

        println!("{}", message);
    }

    pub fn warn<T: AsRef<str>>(msg: T) {
        let message = format!(
            "{0}WARN{1} - {2}",
            color((255, 244, 4)),
            POSTFIX,
            msg.as_ref()
        );
        println!("{}", message);
    }

    pub fn get<T: AsRef<str>>(msg: T) {
        let message = format!(
            "{0}[{1} GET {0}]{1} - {0}{2}{1}",
            color((70, 200, 30)),
            POSTFIX,
            msg.as_ref()
        );
        println!("{}", message);
    }
}


pub fn print_url (ip: &str, port: &str) {
	let version = env!("CARGO_PKG_VERSION");
	let base = color((135, 251, 255));
	let green = color((0, 255, 0));
	let red = color((255, 0, 0));
	let message = format! ("\n\n\n\n
  {base}Qserve{POSTFIX} - {0}

  - {green}local{POSTFIX}   {base}→{POSTFIX} {2}:{1}
  - {green}network{POSTFIX} {base}→{POSTFIX} {3}:{1}

  {red}ctrl+c{POSTFIX} to quit\n\n
	", version, port, "localhost", ip);

	println! ("{}", message)
}
