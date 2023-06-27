use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use derive_more::Constructor;
use log::debug;
use russh::{client, ChannelId};
use russh_keys::key;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::{connection, server_conf, Options};

#[derive(Constructor)]
pub struct Client {
    options: Options,
    server_conf: server_conf::Conf,
    connection: Arc<connection::Connection>,
}

impl Client {
    pub(crate) async fn start(self, cancellation_token: CancellationToken) -> Result<()> {
        let config = russh::client::Config::default();
        let config = Arc::new(config);

        let host = self
            .options
            .host
            .split(':')
            .next()
            .context("Could not get host part")?
            .to_string();
        let addr = (host, self.server_conf.sshd_port.parse()?);
        let mut session = russh::client::connect(config, addr, self).await?;

        if session.authenticate_password("expose", "test").await? {
            let mut channel = session.channel_open_session().await?;
            channel.data(&b"Hello, world!"[..]).await?;
            loop {
                tokio::select! {
                    msg = channel.wait() => {
                        debug!("{msg:?}");
                    },
                    _ = cancellation_token.cancelled() => {
                        return Ok(())
                    }
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl client::Handler for Client {
    type Error = anyhow::Error;

    async fn check_server_key(
        self,
        server_public_key: &key::PublicKey,
    ) -> Result<(Self, bool), Self::Error> {
        let matches = server_public_key.fingerprint() == self.server_conf.sshd_fingerprint;
        Ok((self, matches))
    }

    async fn data(
        self,
        channel: ChannelId,
        data: &[u8],
        session: client::Session,
    ) -> Result<(Self, client::Session), Self::Error> {
        debug!(
            "data on channel {:?}: {:?}",
            channel,
            std::str::from_utf8(data)
        );
        Ok((self, session))
    }
}
