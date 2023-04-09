use std::convert::Infallible;
use std::future::{Ready, ready};
use std::ops::Deref;
use actix::Addr;
use actix_web::{FromRequest, HttpRequest, HttpResponse, web};
use actix_web::dev::Payload;
use futures_util::StreamExt;
use sqlx::PgPool;
use crate::models::connection::Connection;
use crate::{util, websockets};
use crate::errors::{AppResponse};
use crate::errors::AppError;
use crate::settings::Settings;
use crate::util::wildcard_host_guard::get_host_uri;
use crate::websockets::server::ConnectionsWsServer;

struct RequestData {}
impl FromRequest for RequestData {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    // TODO: should I really re-implem http request deserialization ???
    fn from_request(_req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(RequestData{}))
    }
}

pub async fn process(
    req: HttpRequest,
    mut payload: web::Payload,
    conn_server: web::Data<Addr<ConnectionsWsServer>>,
    db: web::Data<PgPool>,
    settings: web::Data<Settings>
) -> AppResponse {
    let host = get_host_uri(req.head()).unwrap().to_string();
    let host_without_port = host.splitn(2, ':').next().unwrap();
    let subdomain = match host_without_port.strip_suffix(&settings.http.vhost_suffix.deref()) {
        Some(value) => value,
        None => return Ok(HttpResponse::NotFound().finish()),
    };
    let connection = Connection::get_by_subdomain(&db, subdomain).await.map_err(|_| { AppError::NotFound })?;

    let (tx, _rx) = conn_server.send(websockets::server::HttpRequest {
        connection_id: connection.id.to_string(),
        http_head: req.head().clone(),
    }).await?;
    let body_sender = actix_web::rt::spawn(async move {
        while let Some(chunk) = payload.next().await {
            match chunk {
                Ok(chunk) => tx.send(chunk).unwrap(),
                Err(_) => return
            }
        }
    });
    body_sender.await?;

    Ok(HttpResponse::Ok().finish())
}

pub fn urls(settings: &Settings, cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("")
        .guard(util::wildcard_host_guard::WildcardHost(settings.http.vhost_suffix.clone()))
        .default_service(web::to(process)));
}
