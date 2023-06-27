use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Create {
    pub(crate) subdomain: String,
    pub(crate) proxied_port: String,
}

#[derive(Deserialize, Serialize, Constructor)]
pub struct View {
    pub(crate) id: String,
    pub(crate) subdomain: String,
    pub(crate) proxied_port: String,
    pub(crate) upstream_port: Option<String>,
}

#[derive(Deserialize, Serialize, Constructor)]
pub struct ShowView {
    pub(crate) connection: View,
}
