use {
    crate::{config::Config, log::Logger, operations::html, response::Response},
    std::{
        fs,
        fs::DirEntry,
        io::{Read, Write},
        net::TcpStream,
        path::Path,
    },
};

pub fn read_dir(path: &str) -> std::io::Result<Vec<DirEntry>> {
    let dir = fs::read_dir(path)?;
    let mut contents = Vec::new();
    for e in dir {
        contents.push(e.unwrap())
    }

    Ok(contents)
}

pub fn stream(mut stream: &TcpStream, content_len: u64, path: String) {
    let config = Config::parse();

    match fs::File::open(&path) {
        Ok(mut file_buffer) => {
            let mut buffer = vec![0; config.chunks];
            let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();
            let _ = stream.write_all(Response::send_file(content_len, file_name).as_bytes());

            loop {
                match file_buffer.read(&mut buffer) {
                    Ok(bytes) => {
                        if bytes == 0 {
                            break;
                        }

                        let _ = stream.write_all(&buffer[..bytes]);
                    }
                    Err(e) => Logger::error(format!("file reading failed, {}", e)),
                }
            }
        }

        Err(e) => {
            let _ = stream.write_all(
                Response::bad_request("error: check terminal for logs", "text/plain").as_bytes(),
            );
            Logger::error(format!("{}", e))
        }
    }
}

pub fn serve_dir(mut stream: &TcpStream, request: String, path: String) {
    match read_dir(&path) {
        Ok(dir_contents) => {
            let mut contents = Vec::new();
            let parent_directory = Path::new(request.split_whitespace().nth(1).unwrap())
                .parent()
                .unwrap()
                .display();
            contents.push(format!(
                "<a class=\"back\" href=\"{}\"></a>",
                parent_directory
            ));
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
                Response::ok(&html::generate(&contents.join("\n"), &path), "text/html").as_bytes(),
            );
        }
        Err(e) => {
            let _ = stream.write_all(
                Response::bad_request("error reading dir, check terminal for logs", "text/plain")
                    .as_bytes(),
            );
            Logger::error(format!("{}", e));
        }
    };
}
