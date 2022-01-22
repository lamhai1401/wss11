use actix::prelude::{Message as ActixMessage, Recipient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::cst::MessageType;

#[derive(ActixMessage, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct Message(pub String);

impl Into<String> for Message {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Message {
    pub fn err(msg: String) -> Self {
        Self(msg.to_string())
    }

    pub fn new(msg: String) -> Self {
        Self(msg.to_string())
    }

    pub fn to_slice(msg: String) -> Self {
        let arr = msg.clone().into_boxed_str();
        Self(msg.to_string())
    }
}

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
    pub msg_type: MessageType,
    pub data: Value,
    pub from: String,
    pub to: String,
}

impl MessageToClient {
    pub fn new(data: Value, from: &str, msg_type: MessageType) -> Self {
        Self {
            msg_type,
            data,
            from: from.to_string(),
            to: "to".to_string(),
        }
    }
}
