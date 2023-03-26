use std::borrow::Cow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message<'a> {
    HttpRequest { content: Cow<'a, [u8]> }
}

impl<'a> Message<'a> {
    pub fn http_request(content: &'a [u8]) -> Message<'a> {
        Self::HttpRequest { content: Cow::from(content) }
    }
}
