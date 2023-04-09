use std::collections::HashMap;
use std::ops::Deref;
use actix::{Actor, Context, Handler, Message, MessageResult, Recipient};
use actix_web::dev::RequestHead;
use actix_web::web;
use tokio::sync::mpsc;
use tracing::debug;
use crate::websockets;

type Client = Recipient<websockets::session::HttpRequestRequireProxy>;

#[derive(Debug)]
pub struct ConnectionsWsServer {
    subscriptions: HashMap<String, Vec<Client>>,
}

impl ConnectionsWsServer {
    pub fn new() -> Self {
        Self { subscriptions: HashMap::new() }
    }
}

impl Actor for ConnectionsWsServer {
    type Context = Context<Self>;
}

#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct SubscribeToConnection {
    pub connection_id: String,
    pub client_addr: Client,
}

impl Handler<SubscribeToConnection> for ConnectionsWsServer {
    type Result = ();

    fn handle(&mut self, msg: SubscribeToConnection, _ctx: &mut Self::Context) {
        debug!("Handling SubscribeToConnection: {:?}", msg);

        if !self.subscriptions.contains_key(msg.connection_id.deref()) {
            self.subscriptions.insert(msg.connection_id.clone(), Vec::new());
        }
        match self.subscriptions.get_mut(msg.connection_id.deref()) {
            Some(clients) => clients.push(msg.client_addr),
            None => {}
        }
    }
}

#[derive(Clone, Message, Debug)]
#[rtype(result = "(mpsc::UnboundedSender<web::Bytes>, mpsc::UnboundedReceiver<web::Bytes>)")]
pub struct HttpRequest {
    pub connection_id: String,
    pub http_head: RequestHead,
}

impl Handler<HttpRequest> for ConnectionsWsServer {
    type Result = MessageResult<HttpRequest>;

    fn handle(&mut self, msg: HttpRequest, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling HttpRequest: {:?}", msg);

        let (tx, rx) = mpsc::unbounded_channel();
        if let Some(subscriptions) = self.subscriptions.get(msg.connection_id.deref()) {
            debug!("Handling HttpRequest: found client connection subscriptions {:?}", subscriptions);
            for client in subscriptions {
                debug!("Handling HttpRequest: sending to client {:?}", client);
                client.do_send(websockets::session::HttpRequestRequireProxy{});
            }
        }
        MessageResult((tx, rx))
    }
}
