use derive_more::Constructor;
use serde::{Serialize};
pub use sqlx::types::Uuid;
use shared::dto::connection::ConnectionView;

#[derive(Serialize, Constructor)]
pub struct IndexView<'a> {
    pub connections: &'a Vec<ConnectionView>,
}
