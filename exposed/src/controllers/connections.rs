use actix::Addr;
use actix_web::{HttpResponse, web, get, post, delete, guard, HttpRequest};
use actix_web_actors::ws;
use sqlx::PgPool;
use shared::dto::connection::{ConnectionView, CreateConnection, ShowView};
use crate::errors::*;
use crate::models::connection::Connection;
use crate::settings::Settings;
use crate::views::connections::*;
use crate::websockets;

#[get("")]
pub async fn index(db: web::Data<PgPool>) -> AppResponse {
    let connections = Connection::get_all(&db).await?;
    let connection_views = connections.into_iter().map(|x|
        ConnectionView::new(x.id.to_string(), x.subdomain, x.proxied_port, x.upstream_port)
    ).collect();
    let index_view = IndexView::new(&connection_views);
    let body = serde_json::to_string(&index_view).unwrap();
    Ok(HttpResponse::Ok().content_type("application/json").body(body))
}

#[post("")]
pub async fn create(db: web::Data<PgPool>, params: web::Json<CreateConnection>) -> AppResponse {
    let connection = Connection::new(params.subdomain.clone(), params.proxied_port.clone());
    connection.insert(&db).await?;
    let connection_view = ConnectionView::new(connection.id.to_string(), connection.subdomain.clone(), connection.proxied_port.clone(), connection.upstream_port.clone());
    let create_view = ShowView::new(connection_view);
    let body = serde_json::to_string(&create_view).unwrap();
    Ok(HttpResponse::Created().content_type("application/json").body(body))
}

#[delete("/{uuid}")]
pub async fn delete(db: web::Data<PgPool>, path: web::Path<String>) -> AppResponse {
    let uuid = Uuid::parse_str(&path.into_inner()).unwrap();
    let connection = Connection::get(&db, &uuid).await?;
    connection.delete(&db).await?;
    let connection_view = ConnectionView::new(connection.id.to_string(), connection.subdomain.clone(), connection.proxied_port.clone(), connection.upstream_port.clone());
    let delete_view = ShowView::new(connection_view);
    let body = serde_json::to_string(&delete_view).unwrap();
    Ok(HttpResponse::Ok().content_type("application/json").body(body))
}

#[get("/{uuid}/subscribe")]
pub async fn subscribe(
    req: HttpRequest,
    path: web::Path<String>,
    stream: web::Payload,
    conn_server: web::Data<Addr<websockets::server::ConnectionsWsServer>>,
) -> AppResponse {
    ws::start(websockets::session::ConnectionSession::new(
        path.into_inner(),
        conn_server.get_ref().clone(),
    ), &req, stream).map_err(AppError::ActixError)
}

pub fn urls(settings: &Settings, cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/connections")
        .guard(guard::Host(settings.http.url.host().unwrap().to_string()))
        .service(web::scope("")
            .guard(guard::Header("Content-Type", "application/json"))
            .service(index)
            .service(create)
            .service(delete))
        .service(subscribe));
}
