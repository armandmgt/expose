use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub content: String,
}