use crate::utils::base_url;
use crate::Options;
use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Conf {
    pub sshd_port: String,
    pub sshd_fingerprint: String,
}

#[allow(clippy::future_not_send)]
pub async fn get(awc_client: &awc::Client, options: &Options) -> Result<Conf> {
    let url = format!("{}/conf", base_url("http", options));

    let conf = awc_client
        .get(url)
        .send()
        .await
        .map_err(|_| anyhow!("Failed requesting conf"))?
        .json()
        .await
        .map_err(|_| anyhow!("Failed parsing conf response"))?;
    Ok(conf)
}
