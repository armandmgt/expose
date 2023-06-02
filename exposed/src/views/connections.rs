use derive_more::Constructor;
use serde::Serialize;
use shared::dto::connection;
pub use sqlx::types::Uuid;

#[derive(Serialize, Constructor)]
pub struct IndexView<'a> {
    pub connections: &'a Vec<connection::View>,
}
