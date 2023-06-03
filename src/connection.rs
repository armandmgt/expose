use crate::dto;
use crate::Options;
use anyhow::{anyhow, Result};

pub struct Connection {
    id: String,
    _subdomain: String,
    _proxied_port: String,
    _upstream_port: Option<String>,
}

fn base_url(proto: &str, options: &Options) -> String {
    let scheme = if options.no_ssl {
        proto.to_string()
    } else {
        format!("{proto}s")
    };
    format!("{scheme}://{}", options.host)
}

impl Connection {
    #[allow(clippy::future_not_send)]
    pub async fn create(awc_client: &awc::Client, options: &Options) -> Result<Self> {
        let url = format!("{}/connections", base_url("http", options));
        let connection_view: dto::connection::ShowView = awc_client
            .post(url)
            .send_json(&serde_json::json!(dto::connection::Create {
                subdomain: options.subdomain.clone(),
                proxied_port: options.port.clone(),
            }))
            .await
            .map_err(|_| anyhow!("Failed creating the connection"))?
            .json()
            .await
            .map_err(|_| anyhow!("Failed parsing creation response"))?;
        Ok(connection_view.into())
    }

    #[allow(clippy::future_not_send)]
    pub async fn delete(&self, awc_client: &awc::Client, options: &Options) -> Result<()> {
        let url = format!("{}/connections/{}", base_url("http", options), self.id);
        awc_client
            .delete(url)
            .send()
            .await
            .map_err(|_| anyhow!("Failed deleting connection"))?;
        Ok(())
    }
}

impl From<dto::connection::ShowView> for Connection {
    fn from(value: dto::connection::ShowView) -> Self {
        Self {
            id: value.connection.id,
            _subdomain: value.connection.subdomain,
            _proxied_port: value.connection.proxied_port,
            _upstream_port: value.connection.upstream_port,
        }
    }
}
