use std::net::TcpStream;
use std::io::Write;
use crate::response::Response;
use crate::operations::{file,html};

pub fn home(mut stream: &TcpStream, _request: String) {
	let mut contents = Vec::new();
	
	match file::read_dir(".") {
		Ok(dir) => {
			for i in dir {
				contents.push(format!(
					"<a href=\"{0}\">{1}</a>",
					i.path().display(),
					i.file_name().to_str().unwrap()
				))
			}

			let html_content = html::generate(&contents.join("\n"), "/");
			let _ = stream.write_all(Response::ok(&html_content, "text/html").as_bytes());
		}
		Err(_) => {
			let _ = stream.write_all(Response::bad_request("error reading dir, check terminal for logs", "text/html").as_bytes());
		}
	}
}
