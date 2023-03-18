
use actix_web::{HttpResponse, web, post, guard};
use crate::errors::*;
use crate::models::request::Request;
use crate::controllers::types::Payload;

#[post("")]
pub async fn process(params: web::Json<Payload>) -> AppResponse {
    
    let request = Request;
    let result = request.handle(params);
    
    let body = serde_json::to_string(&result).unwrap();

    Ok(HttpResponse::Created().content_type("application/json").body(body))
}


pub fn urls(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/request")
        .guard(guard::Header("Content-Type", "application/json"))
        .service(process));
}
