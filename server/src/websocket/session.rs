use actix::{
    fut,
    prelude::{Actor, Addr, Context, Handler, StreamHandler},
    ActorContext, ActorFuture, AsyncContext, ContextFutureSpawner, WrapFuture,
};
use actix_web_actors::ws;
use log::{info, warn};
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::server::{Disconnect, Message, Server};
pub struct WebSocketSession {
    id: String,
    hb: Instant,
    server_addr: Addr<Server>,
}

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

impl WebSocketSession {
    pub fn new(server_addr: Addr<Server>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            hb: Instant::now(),
            server_addr,
        }
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;
}

// impl Handler<Message> for WebSocketSession {
//     type Result = ();

//     fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
//         ctx.text(msg.0);
//     }
// }

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                self.server_addr.do_send(Disconnect {
                    id: self.id.clone(),
                });
                ctx.close(reason);
                ctx.stop();
            }
            Err(err) => {
                warn!("Error handling msg: {:?}", err);
                ctx.stop()
            }
            _ => ctx.stop(),
        }
    }
}
