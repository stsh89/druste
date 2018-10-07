extern crate actix_web;

use actix_web::{server, App, HttpRequest};
use std::env;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!\n"
}

fn main() {
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => "3000".to_string(),
    };

    let host = match env::var("HOST") {
        Ok(val) => val,
        Err(_) => "localhost".to_string(),
    };

    let addr = format!("{}:{}", host, port);

    println!("Server is running on {}", addr);

    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(addr)
        .unwrap()
        .run();
}
