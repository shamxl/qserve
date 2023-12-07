use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub chunks: usize,
    pub path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip: "0.0.0.0".to_string(),
            port: "3000".to_string(),
            chunks: 4096,
            path: ".".to_string(),
        }
    }
}

impl Config {
    fn parse_args(args: &[String], long_arg: &str, short_arg: &str, default: String) -> String {
        let mut value = String::from(default);

        for (index, arg) in args.iter().enumerate() {
            match arg.as_str() {
                arg if arg == short_arg || arg == long_arg => {
                    value = args.get(index + 1).cloned().unwrap_or_default();
                }

                _ => {}
            }
        }

        value
    }

    pub fn parse() -> Self {
        let config = Config::default();
        let args: Vec<String> = env::args().collect();
        let path = Self::parse_args(&args, "--path", "-P", config.path);
        let config_path = fs::canonicalize(&path);

        Config {
            ip: Self::parse_args(&args, "--ip", "-i", config.ip),
            port: Self::parse_args(&args, "--port", "-p", config.port),
            chunks: Self::parse_args(&args, "--chunks", "-c", config.chunks.to_string()).parse().unwrap(),
            path: config_path.unwrap().display().to_string(),
        }
    }
}

