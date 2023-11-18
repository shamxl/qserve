pub struct Response;

impl Response {
    pub fn ok(content: &str, content_type: &str) -> String {
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {0}\r\nContent-Type: {1}\r\n\r\n{2}",
            content.len(),
            content_type,
            content
        );

        header
    }

    pub fn ok_with_html (content: &str) -> String {
    	let html = include_str! ("../public/index.html");
    	let formatted_html = html.replace("{}", content);
		let header = format! (
			"HTTP/1.1 200 OK\r\nContent-Length: {0}\r\nContent-Type: {1}\r\n\r\n{2}",
			formatted_html.len(),
			"text/html",
			formatted_html
		);

		header
    }

    pub fn not_found(content: &str, content_type: &str) -> String {
        let header = format!(
            "HTTP/1.1 404 Bad Request\r\nContent-Length: {0}\r\nContent-Type: {1}\r\n\r\n{2}",
            content.len(),
            content_type,
            content
        );

        header
    }
}
