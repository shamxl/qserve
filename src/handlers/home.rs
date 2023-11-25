use crate::log::Logger;
use crate::operations::{file, html};
use crate::response::Response;
use std::fs;
use std::io::Write;
use std::net::TcpStream;

pub fn home(mut stream: &TcpStream, _request: String) {
    let mut contents = Vec::new();

    match file::read_dir(".") {
        Ok(dir) => {
            for i in dir {
                let mut file_type = String::from("file");
                match fs::metadata(i.path()) {
                    Ok(f_type) => {
                        if f_type.is_dir() {
                            file_type = "dir".to_string()
                        }
                    }

                    Err(_) => {
                        Logger::warn("unknown error");
                        return;
                    }
                }
                contents.push(format!(
                    "<a class=\"{0}\" href=\"{1}\">{2}</a>",
                    file_type,
                    i.path().display(),
                    i.file_name().to_str().unwrap()
                ))
            }

            let html_content = html::generate(&contents.join("\n"), "/");
            let _ = stream.write_all(Response::ok(&html_content, "text/html").as_bytes());
        }
        Err(_) => {
            let _ = stream.write_all(
                Response::bad_request("error reading dir, check terminal for logs", "text/html")
                    .as_bytes(),
            );
        }
    }
}
