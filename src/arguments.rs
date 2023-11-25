use std::env;

pub fn get_ip() -> String {
    let args: Vec<String> = env::args().collect();
    let mut ip = String::from("0.0.0.0");

    for (index, arg) in args.iter().enumerate() {
        if arg == "--ip" || arg == "-i" {
            ip = args[index + 1].clone();
        }
    }

    ip.to_string()
}

pub fn get_port() -> String {
    let args: Vec<String> = env::args().collect();
    let mut port = String::from("3000");

    for (index, arg) in args.iter().enumerate() {
        if arg == "--port" || arg == "-p" {
            port = args[index + 1].clone();
        }
    }

    port.to_string()
}
