// this function serves the `frontend` directory
use std::net::TcpStream;
use crate::operations::file;
use crate::response::Response;
use std::path::Path;
use std::io::Write;
use crate::log::Logger;

const CSS: &str = include_str! ("../../public/css/style.css");
const JS: &str = include_str! ("../../public/js/index.js");

pub fn serve_static (mut stream: &TcpStream, request: String) {
	
	let path = request.split_whitespace().nth(1).unwrap();

	if path == "/qserve-static-files/css/style.css" {
		stream.write_all(Response::ok(&CSS, "text/css").as_bytes());
	} else if path == "/qserve-static-files/js/index.js" {
		stream.write_all(Response::ok(&JS, "text/javascript").as_bytes());
	} else {
		Logger::warn("Requested static file not found")
	}
	
}
