use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);
pub const ERROR_EVT: &str = "error";

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Err,
    Private,
    Public,
}
