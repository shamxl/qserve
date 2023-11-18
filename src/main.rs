mod operations;
mod handlers;
use crate::handlers::home::home;
use crate::handlers::static_file::serve_static;
mod log;
mod response;
use log::Logger;
use std::net::TcpListener;

use std::io::{BufRead, BufReader};

fn main() {
    // todo: implement a command line interface using `clap`
    // and access ip address and port from it
    let ip = "0.0.0.0";
    let port = "3000";
    //

    let address = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(&address).unwrap();
    Logger::info(format!("listening on port: {}", &port));

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
			if path.starts_with("/qserve-static-files") {
            	serve_static(&stream, request.clone());
            } else {
            	home(&stream, request.clone());
            }
        });
    }
}
