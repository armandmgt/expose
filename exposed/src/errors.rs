use std::fmt::{Debug};
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use askama::Template;
use thiserror::Error;
use crate::views::Error as ViewError;
use tracing::error;

fn internal_server_error(e: &dyn std::error::Error) -> HttpResponse {
    error!("Internal server error: {:#?}", e);
    let template = ViewError::new("Internal Server Error", 500, "Internal Server Error.");
    let body = template.render().unwrap();
    HttpResponse::InternalServerError().content_type("text/html").body(body)
}

fn unprocessable_entity(msg: &str) -> HttpResponse {
    error!("Unprocessable_entity: {:#?}", msg);
    let template = ViewError::new("Unprocessable Entity", 422, msg);
    let body = template.render().unwrap();
    HttpResponse::UnprocessableEntity().content_type("text/html").body(body)
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("actix web error {0}")]
    ActixError(#[from] actix_web::Error),
    #[error("askama error {0}")]
    TemplateError(#[from] askama::Error),
    #[error("database error {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("actor error")]
    ActixMailboxError(#[from] actix::MailboxError),
    #[error("thread error {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("not found")]
    NotFound,
    #[error("missing permission: {0}")]
    MissingPermission(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            // Self::ValidationErrors(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    #[tracing::instrument]
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::NotFound => {
                let template = ViewError::new("Error 404", 404, "The page you visited was not found.");
                let body = template.render().unwrap();
                HttpResponse::NotFound().content_type("text/html").body(body)
            }
            Self::MissingPermission(reason) => {
                let msg = format!("You don't have permissions to do this: {}", reason);
                let template = ViewError::new("Error 403", 403, &msg);
                let body = template.render().unwrap();
                HttpResponse::NotFound().content_type("text/html").body(body)
            }
            Self::DatabaseError(reason) => {
                unprocessable_entity(reason.as_database_error().unwrap().message())
            }
            e => internal_server_error(e)
        }
    }
}

pub type AppResponse<T = HttpResponse> = Result<T, AppError>;
