use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use toml;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Config {
    server: ServerConfig,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct ServerConfig {
    host: String,
    port: u16,
    threads: u16,
}

pub fn load_config() -> Config {
    let mut file = File::open(Path::new("confing/config.toml")).unwrap();;
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let config: Config = toml::decode_str(&content)
                        .expect("Failed loading config: required config field not set");
    config
}