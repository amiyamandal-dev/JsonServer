use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    path: String,
    method: String,
    #[serde(rename = "type")]
    request_type: String,
    binary_path: String,
    args: String,
    process_string_from: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    Server: Vec<ServerConfig>,
}

impl Config {
    pub fn new(file_path: &str) -> Config {
        let json_str = match fs::read_to_string(file_path) {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };
        let config: Config = match serde_json::from_str(&json_str) {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };
        config
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let json_str = r#"
        {
            "Server": [
                {
                    "path": "/hello",
                    "method": "GET",
                    "type": "binary",
                    "binary_path": "/home/amiya/mambaforge/bin/python",
                    "args": "hello_world.py --name=abcd",
                    "process_string_from": "<_start_>*<_end_>"
                }
            ]
        }
    "#;

        // Parse the JSON string into Config struct
        let config: Config = serde_json::from_str(json_str).unwrap();

        // Access the parsed values
        for server in config.Server {
            println!("Path: {}", server.path);
            println!("Method: {}", server.method);
            println!("Type: {}", server.request_type);
            println!("Binary Path: {}", server.binary_path);
            println!("Args: {}", server.args);
            println!("Process String From: {}", server.process_string_from);
        }
    }
}
