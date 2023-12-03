use std::env;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub chunks: usize,
    pub path: String,
}

impl Config {
    pub fn parse() -> Self {
        Config {
            ip: Self::get_ip(),
            port: Self::get_port(),
            chunks: Self::get_chunk_size(),
            path: Self::get_path(),
        }
    }

    fn get_ip() -> String {
        let args: Vec<String> = env::args().collect();
        let mut ip = String::from("0.0.0.0");

        for (index, arg) in args.iter().enumerate() {
            if arg == "--ip" || arg == "-i" {
                ip = args[index + 1].clone();
            }
        }

        ip.to_string()
    }

    fn get_port() -> String {
        let args: Vec<String> = env::args().collect();
        let mut port = String::from("3000");

        for (index, arg) in args.iter().enumerate() {
            if arg == "--port" || arg == "-p" {
                port = args[index + 1].clone();
            }
        }

        port.to_string()
    }

    fn get_chunk_size() -> usize {
        let args: Vec<String> = env::args().collect();
        let mut chunks: usize = 4096;

        for (index, arg) in args.iter().enumerate() {
            if arg == "--chunks" || arg == "--chunk" || arg == "-c" {
                chunks = args[index + 1].clone().parse::<usize>().unwrap();
            }
        }

        chunks
    }

    fn get_path() -> String {
        let args: Vec<String> = env::args().collect();
        let mut path = String::from(".");

        for (index, arg) in args.iter().enumerate() {
            if arg == "--path" || arg == "-P" {
                path = args[index + 1].clone();
            }
        }

        fs::canonicalize(path).unwrap().display().to_string()
    }
}
