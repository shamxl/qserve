// this function serves the `frontend` directory
use std::net::TcpStream;
use crate::response::Response;
use std::io::Write;

const CSS: &str = include_str! ("../../public/css/style.css");
const JS: &str = include_str! ("../../public/js/index.js");

pub fn serve_static (mut stream: &TcpStream, request: String) {
	
	let path = request.split_whitespace().nth(1).unwrap();
	
	if path == "/qserve-static-files/css/style.css" {
		let _ = stream.write_all(Response::ok(&CSS, "text/css").as_bytes());
	
	} else if path == "/qserve-static-files/js/index.js" {
		let _ = stream.write_all(Response::ok(&JS, "text/javascript").as_bytes());
	} else {
		let _ = stream.write_all (Response::not_found("File Not Found", "text/plain").as_bytes());
	}
	
}
