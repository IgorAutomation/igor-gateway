use std::net::SocketAddr;
use uclicious::Uclicious;

#[derive(Debug, Uclicious)]
#[ucl(include(chunk = ""))]
pub struct Config {
    #[ucl(default = "String::from(\"sqlite::memory:\")")]
    pub database_url: String,
    #[ucl(default = "listen_default()")]
    pub listen: SocketAddr,
}

fn listen_default() -> SocketAddr {
    "127.0.0.1:1337".parse().unwrap()
}
