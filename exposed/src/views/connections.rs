use derive_more::Constructor;
use serde::{Serialize};
pub use sqlx::types::Uuid;

#[derive(Serialize, Constructor)]
pub struct ConnectionView {
    pub id: Uuid,
    pub subdomain: String,
    pub proxied_port: String,
    pub upstream_port: Option<String>,
}

#[derive(Serialize, Constructor)]
pub struct Index<'a> {
    pub connections: &'a Vec<ConnectionView>,
}

#[derive(Serialize, Constructor)]
pub struct Show<'a> {
    pub connection: &'a ConnectionView,
}
