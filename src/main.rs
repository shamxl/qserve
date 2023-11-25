mod handlers;
mod operations;
mod arguments;
mod log;
mod response;

use log::Logger;
use std::{
	net::TcpListener,
	io::{ BufRead, BufReader }
};
use handlers::{
	files::files,
	home::home
};


fn main() {
    let ip = arguments::get_ip();
    let port = arguments::get_port();
    let address = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(&address).unwrap();
    Logger::info(format!("listening on: {}", &address));

    for stream in listener.incoming() {
        std::thread::spawn(|| {
            let mut stream = stream.unwrap();
            let buffer = BufReader::new(&mut stream);
            let request = match buffer.lines().next() {
                Some(Ok(request)) => request,
                Some(Err(_)) => {
                    Logger::error("Invalid request");
                    return;
                }
                None => {
                    Logger::warn("Empty request");
                    return;
                }
            };

            let path = request.split_whitespace().nth(1).unwrap();
            Logger::get(path);

            if path == "/" {
                home(&stream, request.clone());
            } else {
                files(&stream, request.clone());
            }
        });
    }
}
