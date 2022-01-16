use actix::prelude::Message as ActixMessage;
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(ActixMessage, Clone, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct SessionMessage {
    pub event: String,
    pub from: String,
    pub to: String,
    pub data: String,
}
