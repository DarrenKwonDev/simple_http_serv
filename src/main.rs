#![allow(dead_code)]

// mod = import, include
mod http;
mod server; // inject module server.rs (find server/mod.rs || server.rs)

use http::Method;
use http::Request;
use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
