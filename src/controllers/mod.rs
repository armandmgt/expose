mod connections;
mod proxy;
pub(crate) mod types;

use actix_web::{HttpResponse, get, web};
use crate::errors::*;
use crate::settings::Settings;
use crate::views::*;

#[get("/")]
pub async fn index() -> AppResponse {
    let template = Index::new("Home");
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub fn urls(settings: &Settings, cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .configure(|cfg| connections::urls(settings, cfg))
        .configure(|cfg| proxy::urls(settings, cfg));
}
