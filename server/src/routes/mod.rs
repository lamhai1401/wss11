use actix_web::{
    error::Error,
    web,
    web::{block, get, scope, Data, Json, ServiceConfig},
    HttpResponse, Responder, Result,
};

use server::websocket::ws_index;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/ws/").route(web::get().to(ws_index)))
        .service(scope("/api").service(
            scope("/v1").route("", get().to(get_all)), // .route("", web::post().to(questions::create)),
        ));
}

async fn get_all() -> impl Responder {
    format!("hello from get users")
}
