extern crate actix_web;

use actix_web::{server, App, HttpRequest, Responder};

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/{name}", |r| r.f(greet))
    })
        .bind("127.0.0.1:3200")
        .expect("Can not bind to port 3200")
        .run();
}

fn greet(req: &HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");

    format!("Hello {}", name)
}
