use std::time::Instant;
use actix::{Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, fut, Handler, Message, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use actix_web_actors::ws::{CloseCode, CloseReason};
use tracing::{debug, error};
use shared::dto;
use crate::websockets::server::{ConnectionsWsServer, SubscribeToConnection};

pub struct ConnectionSession {
    pub alive: Instant,
    pub connection_id: String,
    pub server_addr: Addr<ConnectionsWsServer>,
}

impl ConnectionSession {
    pub fn new(connection_id: String, server_addr: Addr<ConnectionsWsServer>) -> Self {
        Self { alive: Instant::now(), connection_id, server_addr }
    }
}

impl Actor for ConnectionSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let session_addr = ctx.address();
        self.server_addr.
            send(SubscribeToConnection {
                connection_id: self.connection_id.clone(),
                client_addr: session_addr.recipient(),
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(_) => {}
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct HttpRequestRequireProxy {
}

impl Handler<HttpRequestRequireProxy> for ConnectionSession {
    type Result = ();

    fn handle(&mut self, _msg: HttpRequestRequireProxy, ctx: &mut Self::Context) -> Self::Result {
        match serde_json::to_string(&dto::ws::Message::http_request()) {
            Ok(json) => ctx.text(json),
            Err(_) => error!("Failed serializing request content"),
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ConnectionSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(msg) => {
                debug!("Received unexpected message from client: {:?}", msg);
            }
            ws::Message::Binary(_) => ctx.close(Some(CloseReason { code: CloseCode::Unsupported, description: None })),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
