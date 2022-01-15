use actix::prelude::{Actor, Context, Handler, Message as ActixMessage, Recipient};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::{error::Result as SerdeResult, to_string, Value};
use std::collections::HashMap;

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub id: String,
}

#[derive(ActixMessage, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct MessageToClient {
    pub msg_type: String,
    pub data: Value,
}

impl MessageToClient {
    pub fn new(msg_type: &str, data: Value) -> Self {
        Self {
            msg_type: msg_type.to_string(),
            data,
        }
    }
}

pub struct Server {
    sessions: HashMap<String, Recipient<Message>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    fn send_message(&self, data: SerdeResult<String>) {
        match data {
            // Ok(data) => {
            //     info!("Wss send msg {:?}", data); // TODO impl this handle forward msg
            // }
            Ok(data) => {
                info!("Wss send msg to all user {:?}", data); // TODO impl this handle forward msg
                for recipient in self.sessions.values() {
                    match recipient.do_send(Message(data.clone())) {
                        Err(err) => {
                            error!("Error sending client message: {:?}", err);
                        }
                        _ => {}
                    }
                }
            }
            Err(err) => {
                error!("Data did not convert to string {:?}", err);
            }
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<Connect> for Server {
    type Result = ();
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        info!("{:?} id user resgited", msg.id.clone());
        self.sessions.insert(msg.id.clone(), msg.addr);
    }
}

impl Handler<MessageToClient> for Server {
    type Result = ();

    fn handle(&mut self, msg: MessageToClient, _: &mut Context<Self>) -> Self::Result {
        self.send_message(to_string(&msg));
    }
}
