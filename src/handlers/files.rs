use crate::response::Response;
use std::{io::Write, net::TcpStream};

pub fn files (mut stream: &TcpStream, request: String) {

    let content = ["<div class=\"file type-dir\"><p>src</p></div>", "<div class=\"file\"><p>Cargo.toml</p></div>"];
    let _ = stream.write_all(Response::ok_with_html (&content.join("\n")).as_bytes());
}
