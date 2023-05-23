use crate::views::Error as ViewError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use askama::Template;
use std::fmt::Debug;
use thiserror::Error;
use tokio::task::JoinError;
use tracing::error;

fn internal_server_error(e: &dyn std::error::Error) -> HttpResponse {
    error!("Internal server error: {:#?}", e);
    let template = ViewError::new("Internal Server Error", 500, "Internal Server Error.");
    let body = template.render().unwrap();
    HttpResponse::InternalServerError()
        .content_type("text/html")
        .body(body)
}

fn unprocessable_entity(msg: &str) -> HttpResponse {
    error!("Unprocessable_entity: {:#?}", msg);
    let template = ViewError::new("Unprocessable Entity", 422, msg);
    let body = template.render().unwrap();
    HttpResponse::UnprocessableEntity()
        .content_type("text/html")
        .body(body)
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("actix web error {0}")]
    ActixError(#[from] actix_web::Error),
    #[error("askama error {0}")]
    TemplateError(#[from] askama::Error),
    #[error("database error {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("awc error {0}")]
    AwcError(#[from] awc::error::SendRequestError),
    #[error("not found")]
    NotFound,
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
                let template =
                    ViewError::new("Error 404", 404, "The page you visited was not found.");
                let body = template.render().unwrap();
                HttpResponse::NotFound()
                    .content_type("text/html")
                    .body(body)
            }
            Self::DatabaseError(reason) => {
                unprocessable_entity(reason.as_database_error().unwrap().message())
            }
            e => internal_server_error(e),
        }
    }
}

pub type AppResponse<T = HttpResponse> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum StaticError {
    #[error("io error {0}")]
    IoError(#[from] std::io::Error),
    #[error("task join error {0}")]
    TokioJoinError(#[from] JoinError),
    #[error("thrussh error {0}")]
    ThrusshError(#[from] thrussh::Error),
    #[error("thrussh_keys error {0}")]
    ThrusshKeysError(#[from] thrussh_keys::Error),
}
