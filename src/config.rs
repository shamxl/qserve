use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

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

    pub fn print_version_and_exit () {
    	let args: Vec<String> = env::args().collect();
    	let mut print = false;
    	
    	for arg in args.iter() {
    		if arg == "--version" || arg == "-v" {
    	    	print = true;
    	    }
    	}

    	if print {
    	
    		let version = env! ("CARGO_PKG_VERSION");
    		println! ("v{}", version);
    		exit(0);
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

pub fn get_short_path(mut req_path: String) -> String {
    let config = Config::parse();

    if !req_path.starts_with('/') {
        let formatted_path = format!("/{}", req_path);
        req_path = formatted_path
    }
    let mut absolute_path = Path::new(&req_path)
        .to_str()
        .unwrap()
        .replacen(&config.path, "", 1);

    if absolute_path == "/." {
        absolute_path = "./".to_string()
    }

    absolute_path
}

pub fn get_full_path(short_path: String) -> String {
    let config = Config::parse();
    let short_path = short_path.replacen('/', "", 1);
    PathBuf::from(config.path)
        .join(short_path)
        .display()
        .to_string()
}
