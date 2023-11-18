pub struct Logger;
const POSTFIX: &str = "\x1B[0m";


fn color (rgb: (i32, i32, i32)) -> String {
	let (r,g,b) = rgb;

	return format! ("\x1B[38;2;{0};{1};{2}m", r, g, b)
}



impl Logger {

	pub fn info<T: AsRef<str>> (msg: T) {
		let message = format! (
			"{0}INFO{1} - {2}",
			color((0, 255, 0)),
			POSTFIX,
			msg.as_ref()
		);

		println! ("{}", message);
	}

	pub fn error<T: AsRef<str>> (msg: T) {
		let message = format! (
            "{0}ERROR{1} - {2}",
            color((255, 0, 0)),
            POSTFIX,
            msg.as_ref()
        );

		println! ("{}", message);		
	}

	pub fn warn<T: AsRef<str>> (msg: T) {
		let message = format! (
	    	"{0}WARN{1} - {2}",
	    	color((255, 244, 4)),
	        POSTFIX,
	        msg.as_ref()
	    );
		println! ("{}", message);
	}

	pub fn get<T: AsRef<str>> (msg: T) {
		let message = format! (
            "{0}GET{1} - {0}{2}{1}",
            color((70, 200, 30)),
            POSTFIX,
            msg.as_ref()
        );
        println! ("{}", message);
	}
}

