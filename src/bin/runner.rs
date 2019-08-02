extern crate actix_web;
extern crate runner_server;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn main() {
 
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000!")
        .run()
        .unwrap();
}

fn hello(_req: HttpRequest) -> impl Responder {
    "hello word!"
}
