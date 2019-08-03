extern crate actix_web;
extern crate runner_server;
extern crate dotenv;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use diesel::prelude::*;
use runner_server::models::user::*;
use runner_server::{db_connection, models:: {user:: {User}}};

fn main() {
    let conn = db_connection::establish_connection();

    create(&conn);
    query(&conn);
    update(&conn);
    delete(&conn);

    setup_server();
}

fn create(conn: &MysqlConnection) {
    use runner_server::schema::user::*;

    let user = CreateUser {
        id: "1",
        username: "admin",
        nickname: "admin",
        phone: "1",
        email: "1",
        password: "1",
    };

    diesel::insert_into(table)
        .values(&user)
        .execute(conn)
        .unwrap();
}

fn query(conn: &MysqlConnection) {
    use runner_server::schema::user::*;

    let results = table.filter(username.eq("admin"))
        .limit(5)
        .load::<User>(conn)
        .unwrap();

    println!("共查到了：{}条", results.len());    
}

fn update(conn: &MysqlConnection) {
    use runner_server::schema::user::*;

    diesel::update(table.filter(username.eq("admin")))
        .set(username.eq("admin1"))
        .execute(conn)
        .unwrap();
}

fn delete(conn: &MysqlConnection) {
    use runner_server::schema::user::*;

    diesel::delete(table.filter(id.eq("1")))
        .execute(conn)
        .unwrap();
}


fn setup_server() {

    println!("Server runner_api is listening on port 8000!");

    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
            .bind("127.0.0.1:8000")
            .expect("Can not bind to port 8000!")
            .run()
            .unwrap();
}

fn hello(_req: HttpRequest) -> impl Responder {
    "hello word!"
}
