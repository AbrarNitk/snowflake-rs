//use actix::prelude::*;
use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder, HttpResponse, middleware};
use actix_redis::RedisActor;

extern crate id_generator;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

lazy_static::lazy_static!{
    static ref ID: std::sync::Mutex<Option<id_generator::UniqueIdGen>> = std::sync::Mutex::new(None);
}

async fn id() -> impl Responder {
    match ID.lock().unwrap().as_mut() {
        Some(ref mut id) => {
            HttpResponse::Ok().body(serde_json::json!({"id": id.next_id(), "success": true}))
        }
        None => {
            HttpResponse::InternalServerError().body(serde_json::json!({"success": false, "message": "Something went wrong"}))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=trace,actix_redis=trace");
    env_logger::init();
    dotenv::dotenv().expect("Failed to load .env file");
    let node_id: u16 = std::env::var("NODE_ID").unwrap().parse().map_err(|_e| std::io::ErrorKind::NotFound)?;

    let mut builder = ID.lock().expect("error while acquiring lock");
    *builder = Some(id_generator::UniqueIdGen::new(node_id));
    std::mem::drop(builder);

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
