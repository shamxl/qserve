use crate::log;
use crate::response;
use log::Logger;
use response::Response;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub fn home(mut stream: TcpStream) {
    let buffer = BufReader::new(&mut stream);
    let request = match buffer.lines().next() {
        Some(Ok(request)) => request,
        Some(Err(_)) => {
            Logger::error("invalid request");
            return;
        }
        None => {
            Logger::warn("empty request");
            return;
        }
    };

    let path = request.split_whitespace().nth(1).unwrap();

    Logger::info(format!("requested path: {}", path).as_str());

	let html = include_str!("../../public/index.html");
    stream.write_all(Response::ok(&html, "text/html").as_bytes());
}
