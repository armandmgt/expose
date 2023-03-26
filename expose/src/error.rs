use std::fmt::{Debug};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("awc error {0}")]
    SendRequestError(#[from] awc::error::SendRequestError),
    #[error("awc error {0}")]
    JsonPayloadError(#[from] awc::error::JsonPayloadError),
    #[error("awc error {0}")]
    WsClientError(#[from] awc::error::WsClientError),
    #[error("awc error {0}")]
    WsProtocolError(#[from] awc::error::WsProtocolError),
    #[error("serde_json error {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("io error {0}")]
    IoError(#[from] std::io::Error),
}
