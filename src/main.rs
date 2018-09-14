extern crate actix_web;
use actix_web::{server, App, HttpRequest};

use std::env;

#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::*;

fn index(_req: HttpRequest) -> &'static str {
    "Hello :-)"
}

fn main() {
    SimpleLogger::init(LevelFilter::Info, Config::default()).expect("Set up logger");

    info!("Looking up binding from env var 'OPOSSYNC_BIND'");

    let binding = match env::var("OPOSSYNC_BIND") {
        Ok(value) => {
            info!("Found binding '{}'", value);
            value
        },
        Err(err_kind) => match err_kind {
            env::VarError::NotPresent => {
                info!("Falling back to default binding");
                "127.0.0.1:8088".to_string()
            },
            env::VarError::NotUnicode(value) => panic!("Invalid bind value: {:?}", value),
        }
    };

    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(binding)
        .unwrap()
        .run();
}