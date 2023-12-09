use std::{env, fs, path::Path, path::PathBuf, process::exit};

#[derive(Debug)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub chunks: usize,
    pub path: String,
    pub preview: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip: "0.0.0.0".to_string(),
            port: "3000".to_string(),
            chunks: 4096,
            path: ".".to_string(),
            preview: "None".to_string(),
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
        let path = Self::parse_args(&args, "--path", "-P", ".".to_string());
        let mut config_path = fs::canonicalize(path);
        let version = env!("CARGO_PKG_VERSION");
        if Self::parse_args(&args, "--version", "-v", "None".to_string()) != "None" {
            println!("qserve {}", version);
            exit(0);
        }
        if Self::parse_args(&args, "--help", "-h", "None".to_string()) != "None" {
            let message = format!(
                "Qserve - {0}\n
            --ip      or -i – to set the ip address	[default: 0.0.0.0]
            --port    or -p – to set the port		[default: 3000]
            --path    or -P – to set the path to serve	[default: . (current dir)]
            --chunks  or -c – to set chunk size 	[default: 4096]
            --version or -v – print version and exit
            --help    or -h – print this message and exit 
            ",
                version
            );

            println!("{}", message);
            exit(0);
        }

        let mut preview_file = String::from("None");
        let mut dir_path = config_path.as_mut().unwrap().display().to_string();
        if fs::metadata(config_path.as_mut().unwrap())
            .unwrap()
            .is_file()
        {
            preview_file = config_path.as_mut().unwrap().display().to_string();
            dir_path = Path::new(&preview_file)
                .parent()
                .unwrap()
                .display()
                .to_string()
        }
        Config {
            ip: Self::parse_args(&args, "--ip", "-i", config.ip),
            port: Self::parse_args(&args, "--port", "-p", config.port),
            chunks: Self::parse_args(&args, "--chunks", "-c", config.chunks.to_string())
                .parse()
                .unwrap(),
            path: dir_path,
            preview: preview_file,
        }
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
