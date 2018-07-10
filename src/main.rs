extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn index(_req: HttpRequest) -> &'static str {
    "Hello :-)"
}

fn main() {
    let binding = "127.0.0.1:8088";

    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(binding)
        .unwrap()
        .run();
}