use actix::{
    fut,
    prelude::{Actor, Addr, Context, Handler, StreamHandler},
    ActorContext, ActorFuture, AsyncContext, ContextFutureSpawner, WrapFuture,
};
use actix_web_actors::ws;
use log::{info, warn};
use std::time::{Duration, Instant};

use super::Server;
use crate::msg::{Connect, Disconnect, Message};

pub struct WebSocketSession {
    id: String,
    hb: Instant,
    server_addr: Addr<Server>,
}

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

impl WebSocketSession {
    pub fn new(server_addr: Addr<Server>, id: String) -> Self {
        Self {
            id,
            hb: Instant::now(),
            server_addr,
        }
    }

    fn send_heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("Websocket Client heartbeat failed, disconnecting!");

                act.server_addr.do_send(Disconnect { id: act.id.clone() });
                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    // run when started
    fn started(&mut self, ctx: &mut Self::Context) {
        self.send_heartbeat(ctx);

        let session_addr = ctx.address();

        self.server_addr
            .send(Connect {
                addr: session_addr.recipient(),
                id: self.id.clone(),
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(_res) => {}
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl Handler<Message> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

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
            Ok(ws::Message::Text(msg)) => {
                warn!("Receive client msg: {:?}", msg);
                ctx.text(msg);
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
