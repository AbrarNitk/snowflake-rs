//use actix::prelude::*;
use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder, HttpResponse, middleware};
use actix_redis::RedisActor;

extern crate id_generator;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

lazy_static::lazy_static!{
    static ref ID: std::sync::Mutex<id_generator::UniqueIdGen> = std::sync::Mutex::new(id_generator::UniqueIdGen::new(1));
}

async fn id() -> impl Responder {
    HttpResponse::Ok().body(serde_json::json!({"id": ID.lock().unwrap().next_id()}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // std::env::set_var("RUST_LOG", "actix_web=trace,actix_redis=trace");
    // env_logger::init();
    // dotenv::dotenv().expect("Failed to load .env file");

    // let node_id: u16 = std::env::var("NODE_ID").unwrap().parse().map_err(|_e| std::io::ErrorKind::NotFound)?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(health))
            .route("/v1", web::get().to(id))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
