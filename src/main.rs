extern crate term_size;
mod operations;
use operations::file;
mod handlers;
use crate::handlers::home::home;

mod log;
mod response;
use log::Logger;

use response::Response;
use std::net::{SocketAddr, TcpListener};

use std::io::{BufRead, BufReader, Write};

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

            home(stream, request)
        });
    }
}
