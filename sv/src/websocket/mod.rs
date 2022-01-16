use std::time::{Duration, Instant};

use actix::{
    fut,
    prelude::{Actor, Addr, Handler, StreamHandler},
    ActorContext, ActorFuture, AsyncContext, ContextFutureSpawner, WrapFuture,
};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

mod cst;
pub use self::cst::*;

// internal import
mod server;
pub use self::server::*;

mod session;
pub use self::session::*;

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    server_addr: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let id = req.query_string().get(3..).unwrap();
    let res = ws::start(
        WebSocketSession::new(server_addr.get_ref().clone(), id.to_string()),
        &req,
        stream,
    )?;

    Ok(res)
}
