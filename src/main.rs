use actix_web::{
    get,
    http::StatusCode,
    web::{self, Json},
    App, Responder,
};
use serde::Serialize;

#[get("/home")]
async fn home() -> impl Responder {
    let response = "Welcome to the home page";
    response
}

#[derive(Serialize)]
struct User {
    firtname: String,
    lastname: String,
}

impl User {
    fn new(firstname: String, lastname: String) -> Self {
        User {
            firtname: firstname,
            lastname: lastname,
        }
    }
}

#[get("/hello/{firstname}/{lastname}")]
async fn hello(params: web::Path<(String, String)>) -> impl Responder {
    let response = User::new(params.0.clone(), params.1.clone());
    (Json(response), StatusCode::OK)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = actix_web::HttpServer::new(|| App::new().service(home).service(hello))
        .bind(("127.0.0.1", 8080))?
        .run();

    println!("Server running at http://127.0.0.1:8080");
    server.await
}
