use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateConnection {
    pub subdomain: String,
    pub proxied_port: String,
}
