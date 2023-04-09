use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    HttpRequest {}
}

impl Message {
    pub fn http_request() -> Message {
        Self::HttpRequest {}
    }
}
