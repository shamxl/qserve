pub fn decode (url: String) -> String {

	let mut decoded = String::new();
	let mut url_chars = url.chars().peekable();
	
	while let Some(c) = url_chars.next() {
		if c == '%' {
			if let (Some(hex_char_1), Some(hex_char_2)) = (url_chars.next(), url_chars.next()) {
				let hex = format! ("{}{}", hex_char_1, hex_char_2);
				if let Ok(decoded_char) = u8::from_str_radix(&hex, 16).map(char::from) {
					decoded.push(decoded_char);
				} else {
					decoded.push('%');
					decoded.push_str(&hex)
				}
			} else {
				decoded.push('%')
			}
		} else {
			decoded.push(c)
		} 
	} 

	decoded
}
