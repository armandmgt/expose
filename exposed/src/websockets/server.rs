use std::collections::{HashMap};
use std::ops::Deref;
use actix::{Actor, Context, Handler, Recipient};
use actix_web::web;
use tracing::{debug};
use crate::websockets::messages::{HttpRequestMessage, SubscribeToConnection};

type Client = Recipient<HttpRequestMessage>;

#[derive(Debug)]
pub struct ConnectionsWsServer {
    subscriptions: HashMap<String, Vec<Client>>,
}

impl ConnectionsWsServer {
    pub fn new() -> Self {
        Self { subscriptions: HashMap::new() }
    }

    /// Send request to subscribers
    fn send_request(&self, id: &String, request: &web::Bytes) {
        if let Some(subscriptions) = self.subscriptions.get(id) {
            debug!("Handling HttpRequestMessage: found client connection subscriptions {:?}", subscriptions);
            for client in subscriptions {
                debug!("Handling HttpRequestMessage: sending to client {:?}", client);
                client.do_send(HttpRequestMessage {
                    connection_id: id.clone(),
                    request: request.clone(),
                });
            }
        }
    }
}

impl Actor for ConnectionsWsServer {
    type Context = Context<Self>;
}

impl Handler<SubscribeToConnection> for ConnectionsWsServer {
    type Result = ();

    fn handle(&mut self, msg: SubscribeToConnection, ctx: &mut Self::Context) {
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

impl Handler<HttpRequestMessage> for ConnectionsWsServer {
    type Result = ();

    fn handle(&mut self, msg: HttpRequestMessage, ctx: &mut Self::Context) -> Self::Result {
        debug!("Handling HttpRequestMessage: {:?}", msg);
        self.send_request(&msg.connection_id, &msg.request)
    }
}
