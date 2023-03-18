use actix_web::{HttpResponse, web, get, post, delete, guard};
use actix_files as fs;
use serde::Deserialize;
use sqlx::PgPool;
use crate::errors::*;
use crate::models::connection::Connection;
use crate::settings::Settings;
use crate::views::connections::*;

#[get("")]
pub async fn index(db: web::Data<PgPool>) -> AppResponse {
    let connections = Connection::get_all(&db).await?;
    let connection_views = connections.into_iter().map(|x|
        ConnectionView::new(x.id, x.subdomain, x.proxied_port, x.upstream_port)
    ).collect();
    let index_view = Index::new(&connection_views);
    let body = serde_json::to_string(&index_view).unwrap();
    Ok(HttpResponse::Ok().content_type("application/json").body(body))
}

#[derive(Deserialize)]
pub struct CreateConnection {
    subdomain: String,
    proxied_port: String,
}

#[post("")]
pub async fn create(db: web::Data<PgPool>, params: web::Json<CreateConnection>) -> AppResponse {
    let connection = Connection::new(params.subdomain.clone(), params.proxied_port.clone());
    connection.insert(&db).await?;
    let connection_view = ConnectionView::new(connection.id.clone(), connection.subdomain.clone(), connection.proxied_port.clone(), connection.upstream_port.clone());
    let create_view = Show::new(&connection_view);
    let body = serde_json::to_string(&create_view).unwrap();
    Ok(HttpResponse::Created().content_type("application/json").body(body))
}

#[delete("/{uuid}")]
pub async fn delete(db: web::Data<PgPool>, path: web::Path<String>) -> AppResponse {
    let uuid = Uuid::parse_str(&path.into_inner()).unwrap();
    let connection = Connection::get(&db, &uuid).await?;
    connection.delete(&db).await?;
    let connection_view = ConnectionView::new(connection.id.clone(), connection.subdomain.clone(), connection.proxied_port.clone(), connection.upstream_port.clone());
    let delete_view = Show::new(&connection_view);
    let body = serde_json::to_string(&delete_view).unwrap();
    Ok(HttpResponse::Ok().content_type("application/json").body(body))
}

pub fn urls(settings: &Settings, cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/connections")
        .guard(guard::Host(settings.http.url.host().unwrap().to_string()))
        .guard(guard::Header("Content-Type", "application/json"))
        .service(index)
        .service(create)
        .service(delete))
        .service(fs::Files::new("/static", &settings.files.static_dir));
}
