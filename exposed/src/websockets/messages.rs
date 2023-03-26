use actix::prelude::*;
use actix_web::web;

#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct SubscribeToConnection {
    pub connection_id: String,
    pub client_addr: Recipient<HttpRequestMessage>,
}

#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct HttpRequestMessage {
    pub connection_id: String,
    pub request: web::Bytes,
}
