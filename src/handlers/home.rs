use crate::log;
use crate::response;
use log::Logger;
use response::Response;
use std::{io::Write, net::TcpStream};

pub fn home(mut stream: TcpStream, request: String) {
    let path = request.split_whitespace().nth(1).unwrap();

    Logger::info(format!("requested path: {}", path).as_str());

    let html = include_str!("../../public/index.html");
    stream.write_all(Response::ok(&html, "text/html").as_bytes());
}
