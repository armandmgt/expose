use std::ops::Deref;
use futures_util::StreamExt as _;
use actix::Addr;
use actix_web::{HttpResponse, web, Error, HttpRequest};
use sqlx::PgPool;
use crate::models::connection::Connection;
use crate::{util, websockets};
use crate::settings::Settings;
use crate::util::wildcard_host_guard::get_host_uri;
use crate::websockets::server::ConnectionsWsServer;

pub async fn process(req: HttpRequest, mut payload: web::Payload, conn_server: web::Data<Addr<ConnectionsWsServer>>, db: web::Data<PgPool>, settings: web::Data<Settings>) -> Result<HttpResponse, Error> {
    let host = get_host_uri(req.head()).unwrap().to_string();
    let host_without_port = host.splitn(2, ':').next().unwrap();
    let subdomain = match host_without_port.strip_suffix(&settings.http.vhost_suffix.deref()) {
        Some(value) => value,
        None => return Ok(HttpResponse::NotFound().finish()),
    };
    let connection = match Connection::get_by_subdomain(&db, subdomain).await {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let mut bytes = web::BytesMut::new();
    while let Some(item) = payload.next().await {
        bytes.extend_from_slice(&item?);
    }
    conn_server.do_send(websockets::messages::HttpRequestMessage {
        connection_id: connection.id.to_string(),
        request: bytes.into(),
    });

    Ok(HttpResponse::Ok().body(connection.id.to_string()))
}

pub fn urls(settings: &Settings, cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("")
        .guard(util::wildcard_host_guard::WildcardHost(settings.http.vhost_suffix.clone()))
        .default_service(web::to(process)));
}
