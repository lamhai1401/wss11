use crate::msg::MessageToClient;

use super::create::get_test_server;
use actix_web_actors::ws;

use actix::Addr;
use actix_web::{
    web::{block, Data, Json},
    Result,
};
use serde_json::to_value;

pub fn get_websocket_frame_data(frame: ws::Frame) -> Option<MessageToClient> {
    match frame {
        ws::Frame::Text(t) => {
            let bytes = t.as_ref();
            let data = String::from_utf8(bytes.to_vec()).unwrap();
            let value: MessageToClient = serde_json::from_str(&data).unwrap();
            return Some(value);
        }
        _ => {}
    }
    None
}

#[actix_rt::test]
async fn test_body() {
    use actix_web::client::Client;
    use futures::Stream;

    let mut srv = get_test_server().await;
    let client = Client::default();
    let ws_conn = client.ws(srv.url("/ws/")).connect().await.unwrap();

    srv.stop().await;
}
