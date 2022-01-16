#[macro_use]
extern crate log;
use actix::Actor;
use actix_cors::Cors;
use actix_rt;
use actix_web::{http, middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use std::env;
// internal pkg

mod msg;
mod routes;
mod websocket;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    let server = websocket::Server::new().start();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&env::var("CLIENT_HOST").unwrap())
            .allow_any_method()
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .data(server.clone())
            .configure(routes::routes)
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
