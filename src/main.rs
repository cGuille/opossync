extern crate actix_web;
use actix_web::{server, App, HttpRequest};

use std::env;

fn index(_req: HttpRequest) -> &'static str {
    "Hello :-)"
}

fn main() {
    let binding = match env::var("OPOSSYNC_BIND") {
        Ok(value) => value,
        Err(err_kind) => match err_kind {
            env::VarError::NotPresent => "127.0.0.1:8088".to_string(),
            env::VarError::NotUnicode(value) => panic!("Invalid bind value: {:?}", value),
        }
    };

    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(binding)
        .unwrap()
        .run();
}