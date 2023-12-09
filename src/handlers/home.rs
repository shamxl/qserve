use {
    crate::{
        config::Config,
        operations::file::{serve_dir, serve_file},
    },
    std::net::TcpStream,
};

pub fn home(tcpstream: &TcpStream, request: String) {
    let config = Config::parse();

    if config.preview != "None" {
        serve_file(tcpstream, config.preview)
    } else {
        serve_dir(tcpstream, request, config.path, false)
    }
}
