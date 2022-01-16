use actix::prelude::{Message as ActixMessage, Recipient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
