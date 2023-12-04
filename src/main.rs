mod config;
mod handlers;
mod log;
mod operations;
mod response;

use config::Config;
use handlers::{files::files, home::home};
use log::Logger;
use std::{
    io::{BufRead, BufReader},
    net::TcpListener,
};

fn main() {
	Config::print_version_and_exit();
	Config::print_help_and_exit();
    let config = Config::parse();
    let address = format!("{}:{}", config.ip, config.port);
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
