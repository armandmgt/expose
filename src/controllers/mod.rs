mod connections;

use actix_web::{HttpResponse, get, web};
use crate::errors::*;
use crate::views::*;

#[get("/")]
pub async fn index() -> AppResponse {
    let template = Index::new("Home");
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub fn urls(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .configure(connections::urls);
}
