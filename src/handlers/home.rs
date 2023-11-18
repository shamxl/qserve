use crate::log;
use crate::response;
use log::Logger;
use response::Response;
use std::{io::Write, net::TcpStream};

pub fn home(mut stream: &TcpStream, request: String) {

    let html = include_str!("../../public/index.html");
    let _ = stream.write_all(Response::ok(&html, "text/html").as_bytes());
}
