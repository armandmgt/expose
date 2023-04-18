use actix::{Actor, ActorContext, Addr, AsyncContext, Context, StreamHandler};
use actix::io::SinkWrite;
use actix_codec::Framed;
use awc::{BoxedSocket, ws};
use awc::error::WsProtocolError;
use awc::http::header::CONTENT_TYPE;
use awc::ws::Message::Pong;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::StreamExt;
use log::{debug, info};
use shared::dto;
use shared::dto::connection::{ShowView};
use crate::error::Error;
use crate::Options;

type WsFramedSink = SplitSink<Framed<BoxedSocket, ws::Codec>, ws::Message>;
type WsFramedStream = SplitStream<Framed<BoxedSocket, ws::Codec>>;

struct WsClient {
    sink: SinkWrite<ws::Message, WsFramedSink>,
}

impl WsClient {
    pub fn start(sink: WsFramedSink, stream: WsFramedStream) -> Addr<Self> {
        WsClient::create(|ctx| {
            ctx.add_stream(stream);
            WsClient {
                sink: SinkWrite::new(sink, ctx),
            }
        })
    }
}

impl Actor for WsClient {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        info!("WsClient started");
    }
}

impl actix::io::WriteHandler<WsProtocolError> for WsClient {}

impl StreamHandler<Result<ws::Frame, WsProtocolError>> for WsClient {
    fn handle(&mut self, msg: Result<ws::Frame, WsProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Frame::Text(text) => {
                match serde_json::from_str::<dto::ws::Message>(std::str::from_utf8(text.as_ref()).unwrap()) {
                    Ok(message) => {
                        match message {
                            dto::ws::Message::HttpRequest { .. } => {
                                debug!("Received HttpRequest message: {:?}", message);
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
            ws::Frame::Ping(_) => { self.sink.write(Pong("".into())).unwrap(); }
            _ => {}
        }
    }
}

pub struct Connection {
    id: String,
    subdomain: String,
    proxied_port: String,
    upstream_port: Option<String>,
}

fn base_url(proto: &str, options: &Options) -> String {
    let scheme = if options.no_ssl {
        format!("{}", proto)
    } else {
        format!("{}s", proto)
    };
    format!("{}://{}", scheme, options.host)
}

fn client() -> awc::Client {
    awc::Client::builder().add_default_header((CONTENT_TYPE, "application/json")).finish()
}

impl Connection {
    pub async fn create(options: &Options) -> Result<Self, Error> {
        let url = format!("{}/connections", base_url(&"http", &options));
        let connection_view: ShowView = client().post(url).send_json(
            &serde_json::json!(shared::dto::connection::CreateConnection {
                subdomain: options.subdomain.clone(),
                proxied_port: options.port.clone(),
            })
        ).await?.json().await?;
        Ok(connection_view.into())
    }

    pub async fn subscribe(&self, options: &Options) -> Result<(), Error> {
        debug!("Connecting to ws endpoint {}", format!("{}/connections/{}/subscribe", base_url(&"ws", options), self.id));
        let (_, ws_connection) = awc::Client::builder()
            .max_http_version(awc::http::Version::HTTP_11).finish()
            .ws(format!("{}/connections/{}/subscribe", base_url(&"ws", options), self.id))
            .connect()
            .await?;

        let (sink, stream): (WsFramedSink, WsFramedStream) = ws_connection.split();
        WsClient::start(sink, stream);

        let _ = actix_rt::signal::ctrl_c().await?;

        Ok(())
    }

    pub async fn delete(&self, options: &Options) -> Result<(), Error> {
        let url = format!("{}/connections/{}", base_url(&"http", &options), self.id);
        client().delete(url).send().await.map(|_| ()).map_err(Error::from)
    }
}

impl From<ShowView> for Connection {
    fn from(value: ShowView) -> Self {
        Self {
            id: value.connection.id.clone(),
            subdomain: value.connection.subdomain.clone(),
            proxied_port: value.connection.proxied_port.clone(),
            upstream_port: value.connection.upstream_port.clone(),
        }
    }
}
