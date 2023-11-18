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
