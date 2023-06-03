use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("awc error {0}")]
    SendRequest(#[from] awc::error::SendRequestError),
    #[error("awc error {0}")]
    JsonPayload(#[from] awc::error::JsonPayloadError),
    #[error("awc error {0}")]
    WsClient(#[from] awc::error::WsClientError),
    #[error("awc error {0}")]
    WsProtocol(#[from] awc::error::WsProtocolError),
    #[error("serde_json error {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("io error {0}")]
    Io(#[from] std::io::Error),
}
