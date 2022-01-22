use actix::prelude::{Actor, Context, Handler, Recipient};
use log::{error, info};
use serde_json::{error::Result as SerdeResult, to_string, Value};

use std::collections::HashMap;

use crate::msg::{Connect, Disconnect, Message, MessageToClient, MessageType, SessionMessage};

pub struct Server {
    sessions: HashMap<String, Recipient<Message>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    fn dispatch(&self, data: SerdeResult<String>) {
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

impl Handler<SessionMessage> for Server {
    type Result = ();
    fn handle(&mut self, msg: SessionMessage, _: &mut Context<Self>) {
        info!("Receive SessionMessage: {:?}", msg);
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
        match msg.msg_type {
            MessageType::Private => {
                // get data
                let mut resp = match msg.data {
                    Value::Array(data) => data,
                    _ => vec![Value::String(msg.from.clone())],
                };

                // set sender
                let receiver_id =
                    match std::mem::replace(&mut resp[0], Value::String(msg.from.to_string())) {
                        Value::String(v) => v,
                        _ => "".to_string(),
                    };

                match serde_json::to_string(&resp) {
                    Ok(v) => {
                        // get session
                        let receiver = self.sessions.get(&receiver_id);
                        if let Some(r) = receiver {
                            match r.do_send(Message::new(v)) {
                                Err(err) => println!(
                                    "Sending msg to {:?} err: {:?}",
                                    receiver_id.to_string(),
                                    err
                                ),
                                _ => {}
                            }
                        };
                    }
                    Err(err) => {
                        error!("serde_json {:?} err: {:?}", resp, err);
                    }
                };
            }
            MessageType::Public => self.dispatch(to_string(&msg.data)),
            _ => {}
        }
    }
}
