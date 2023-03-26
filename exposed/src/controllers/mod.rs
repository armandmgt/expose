mod connections;
mod proxy;

use actix_web::{HttpResponse, get, web};
use actix_files as fs;
use crate::settings::Settings;
use crate::views::*;
use crate::errors::*;

#[get("/")]
pub async fn index() -> AppResponse {
    let template = Index::new("Home");
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub fn urls(settings: &Settings, cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .configure(|cfg| connections::urls(settings, cfg))
        .configure(|cfg| proxy::urls(settings, cfg))
        .service(fs::Files::new("/static", &settings.files.static_dir));
}
