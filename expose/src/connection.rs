use std::ops::Deref;
use reqwest::{blocking::{Response}, Url};
use shared::dto::connection::{ShowView};
use crate::error::Error;
use crate::Options;

pub struct Connection {
    id: String,
    subdomain: String,
    proxied_port: String,
    upstream_port: Option<String>,
}

fn base_url(options: &Options) -> Url {
    let scheme = if options.no_ssl {
        "http"
    } else {
        "https"
    };
    Url::parse(format!("{}://{}", scheme, options.host).deref()).unwrap()
}

fn client() -> reqwest::blocking::Client {
    let mut header_map = reqwest::header::HeaderMap::new();
    header_map.append(reqwest::header::CONTENT_TYPE, "application/json".parse().unwrap());
    reqwest::blocking::Client::builder().default_headers(header_map).build().unwrap()
}

impl Connection {
    pub fn create(options: &Options) -> Result<Self, Error> {
        let mut url = base_url(&options);
        url.set_path("/connections");
        let result = client().post(url).json(
            &shared::dto::connection::CreateConnection {
                subdomain: options.subdomain.clone(),
                proxied_port: options.port.clone(),
            }
        ).send();
        match result {
            Ok(v) => {
                let connection_view: ShowView = v.json()?;
                Ok(connection_view.into())
            }
            Err(e) => Err(e.into())
        }
    }

    pub fn delete(&self, options: &Options) -> Result<Response, Error> {
        let mut url = base_url(&options);
        url.set_path(format!("/connections/{}", self.id).deref());
        client().delete(url).send().map_err(Error::from)
    }
}

impl From<ShowView> for Connection {
    fn from(value: ShowView) -> Self {
        Self {
            id: value.connection.id.clone(),
            subdomain: value.connection.subdomain.clone(),
            proxied_port: value.connection.proxied_port.clone(),
            upstream_port: value.connection.upstream_port.clone(),
        }
    }
}
