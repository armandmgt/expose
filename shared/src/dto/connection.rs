use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateConnection {
    pub subdomain: String,
    pub proxied_port: String,
}

#[derive(Deserialize, Serialize, Constructor)]
pub struct ConnectionView {
    pub id: String,
    pub subdomain: String,
    pub proxied_port: String,
    pub upstream_port: Option<String>,
}

#[derive(Deserialize, Serialize, Constructor)]
pub struct ShowView {
    pub connection: ConnectionView,
}
