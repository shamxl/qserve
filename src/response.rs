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

    pub fn bad_request(content: &str, content_type: &str) -> String {
        let header = format!(
            "HTTP/1.1 404 Bad Request\r\nContent-Length: {0}\r\nContent-Type: {1}\r\n\r\n{2}",
            content.len(),
            content_type,
            content
        );

        header
    }

    pub fn send_file(content_length: u64, filename: &str) -> String {
        let header = format! (
    		"HTTP/1.1 200 OK\r\nContent-Disposition: attachment; filename=\"{0}\"\r\nContent-Length: {1}\r\n\r\n",
    		filename,
    		content_length
    	);

        header
    }
}
