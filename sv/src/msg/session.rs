use actix::prelude::Message as ActixMessage;
use serde::{Deserialize, Serialize};

#[derive(ActixMessage, Clone, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct SessionMessage {
    pub event: String,
    pub from: String,
    pub to: String,
    pub data: String,
}

impl SessionMessage {
    pub fn new(event: &str, from: &str, to: &str, data: &str) -> Self {
        Self {
            data: data.to_string(),
            to: to.to_string(),
            from: from.to_string(),
            event: event.to_string(),
        }
    }
}
