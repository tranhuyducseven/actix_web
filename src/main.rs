use actix_web::{get, web, App, Responder};

#[get("/home")]
async fn home() -> impl Responder {
    let response = "Welcome to the home page";
    response
}

#[get("/hello/{firstname}/{lastname}")]
async fn hello(params: web::Path<(String, String)>) -> impl Responder {
    let response = format!("Hello {} {}", params.0, params.1);
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = actix_web::HttpServer::new(|| App::new().service(home).service(hello))
        .bind(("127.0.0.1", 8080))?
        .run();

    println!("Server running at http://127.0.0.1:8080");
    server.await
}
