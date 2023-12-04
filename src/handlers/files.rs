// :D

use {
    crate::{
        config::get_full_path,
        log::Logger,
        operations::{decoder, file},
        response::Response,
    },
    file::{serve_dir, stream},
    std::{fs, io::Write, net::TcpStream},
};

pub fn files(mut tcpstream: &TcpStream, request: String) {
    let req_path = decoder::decode(request.split_whitespace().nth(1).unwrap().to_string());
    let path = get_full_path(req_path);
    let metadata = match fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(e) => {
            let _ = tcpstream.write_all(
                Response::bad_request("error: check terminal for logs", "text/plain").as_bytes(),
            );
            Logger::error(format!("{}", e));
            return;
        }
    };

    if metadata.is_file() {
        stream(tcpstream, metadata.len(), path);
    } else {
        serve_dir(tcpstream, request, path, true)
    }
}
