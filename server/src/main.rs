use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn create_project() -> impl Responder {
    HttpResponse::Ok()
}

async fn get_project() -> impl Responder {
    HttpResponse::Ok()
}

async fn update_project() -> impl Responder {
    HttpResponse::Ok()
}

async fn delete_project() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
