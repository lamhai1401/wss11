use crate::routes::routes;
use crate::websocket::Server;
use actix::Actor;
use actix_web::{body::Body, dev::ServiceResponse, error::Error, test, App};

pub async fn get_test_server() -> test::TestServer {
    test::start(|| App::new().data(Server::new().start()).configure(routes))
}
