#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod http;
mod server;
mod website_handler;


fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR")); // cargo environment variable
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path); // system environment variable
    println!("Public Path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}

