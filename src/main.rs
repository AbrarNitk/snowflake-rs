//use actix::prelude::*;
use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder, HttpResponse, middleware};
use actix_redis::RedisActor;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    println!("hello");
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=trace,actix_redis=trace");
    env_logger::init();
    dotenv::dotenv().expect("Failed to load .env file");

    HttpServer::new(|| {
        let redis_addr = RedisActor::start("127.0.0.1:6379");
        App::new()
            .wrap(middleware::Logger::default())
            .data(redis_addr)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
