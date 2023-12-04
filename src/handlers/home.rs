use {
    crate::{
        config::get_full_path,
        log::Logger,
        operations::file::{serve_dir, stream},
        response::Response,
    },
    std::{fs, io::Write, net::TcpStream},
};

pub fn home(mut tcpstream: &TcpStream, request: String) {
    let path = get_full_path(".".to_string());
    match fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                serve_dir(tcpstream, request.clone(), path, false)
            } else {
                stream(tcpstream, metadata.len(), path)
            }
        }

        Err(e) => {
            Logger::error(format!("{}", e));
            let _ = tcpstream.write_all(
                Response::bad_request(
                    "error: failed to check metadata of file, check terminal for info",
                    "text/plain",
                )
                .as_bytes(),
            );
        }
    }
}
