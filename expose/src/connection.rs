use log::info;
use crate::error::Error;
use crate::Options;

pub struct Connection {
    id: String,
    subdomain: String,
    proxied_port: String,
    upstream_port: Option<String>,
}

impl Connection {
    pub fn create(_options: Options) -> Result<Self, Error> {
        info!("{:?}", exposed::controllers::connections::CreateConnection{});
        Err(Error{reason: "something".to_string() })
    }
}
