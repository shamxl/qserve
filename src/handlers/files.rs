use crate::log::Logger;
use crate::operations::file;
use crate::operations::html;
use crate::response::Response;
use std::fs;
use std::path::Path;
use std::{io::Write, net::TcpStream};

pub fn files(mut stream: &TcpStream, request: String) {
    let path = request
        .split_whitespace()
        .nth(1)
        .unwrap()
        .replacen('/', "./", 1);
    let metadata = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(e) => {
            let _ = stream.write_all(
                Response::bad_request("error: check terminal for logs", "text/plain").as_bytes(),
            );
            Logger::error(format!("{}", e));
            return;
        }
    };

    if metadata.is_file() {
        match file::read_file(&path) {
            Ok(file_content) => {
                let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();
                let _ = stream.write_all(Response::send_file(file_content, file_name).as_bytes());
            }
            Err(e) => {
                let _ = stream.write_all(
                    Response::bad_request(
                        "error reading file, check terminal for logs",
                        "text/plain",
                    )
                    .as_bytes(),
                );
                Logger::error(format!("{}", e));
            }
        };
    } else {
        match file::read_dir(&path) {
            Ok(dir_contents) => {
                let mut contents = Vec::new();
                contents.push("<a class=\"back\" href=\"../\">..</a>".to_string());
                for i in dir_contents {
                    let mut file_type = String::from("file");
                    match fs::metadata(i.path()) {
                        Ok(f_type) => {
                            if f_type.is_dir() {
                                file_type = "dir".to_string()
                            }
                        }
                        Err(_) => Logger::warn("unknown error"),
                    }

                    contents.push(format!(
                        "<a class=\"{0}\" href=\"/{1}\">{2}</a>",
                        file_type,
                        i.path().display(),
                        i.file_name().to_str().unwrap()
                    ));
                }

                let _ = stream.write_all(
                    Response::ok(&html::generate(&contents.join("\n"), &path), "text/html")
                        .as_bytes(),
                );
            }
            Err(e) => {
                let _ = stream.write_all(
                    Response::bad_request(
                        "error reading dir, check terminal for logs",
                        "text/plain",
                    )
                    .as_bytes(),
                );
                Logger::error(format!("{}", e));
            }
        };
    }
}
