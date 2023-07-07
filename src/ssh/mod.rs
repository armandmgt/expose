use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use derive_more::Constructor;
use russh::{
    client::{self, Msg, Session},
    Channel,
};
use russh_keys::key;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_util::sync::CancellationToken;

use crate::{connection::Connection, server_conf, Options};

#[derive(Constructor)]
pub struct Client {
    options: Options,
    server_conf: server_conf::Conf,
}

pub async fn start(
    options: Options,
    server_conf: server_conf::Conf,
    _connection: Arc<Connection>,
    cancellation_token: CancellationToken,
) -> Result<()> {
    let config = russh::client::Config::default();

    let host = options
        .host
        .split(':')
        .next()
        .context("Could not get host part")?
        .to_string();
    let addr = (host, server_conf.sshd_port.parse()?);
    let mut session = russh::client::connect(Arc::new(config), addr, Client { options: options.clone(), server_conf }).await?;

    if session
        .authenticate_password("expose", options.secret.clone())
        .await?
    {
        let forward_addr = format!("{}.{}", options.subdomain, options.host);
        tokio::select! {
            res = session.tcpip_forward(forward_addr.clone(), 0) => {
                res?;
            },
            _ = cancellation_token.cancelled() => {}
        };
        session.cancel_tcpip_forward(forward_addr, 0).await?;
    }
    Ok(())
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

    async fn server_channel_open_forwarded_tcpip(
        self,
        channel: Channel<Msg>,
        _connected_address: &str,
        _connected_port: u32,
        _originator_address: &str,
        _originator_port: u32,
        session: Session,
    ) -> Result<(Self, Session), Self::Error> {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", self.options.port)).await?;

        tokio::io::copy_bidirectional(&mut channel.into_stream(), &mut stream).await?;
        Ok((self, session))
    }
}
