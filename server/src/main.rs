#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;
// use http::Request;
// use http::Method;

mod server;
mod http;
mod website_handler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */
