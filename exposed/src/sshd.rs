use std::{
    future,
    sync::{Arc, Mutex},
};

use thrussh::{
    server::{self, Auth, Session},
    ChannelId, CryptoVec,
};
use thrussh_keys::key;
use tracing::debug;

use crate::errors::StaticError;

struct ClientHandle {
    id_tuple: (usize, ChannelId),
    handle: thrussh::server::Handle,
}

#[derive(Clone)]
pub struct Server {
    config: Arc<thrussh::server::Config>,
    client_pubkey: Arc<thrussh_keys::key::PublicKey>,
    client: Arc<Mutex<Option<ClientHandle>>>,
    id: usize,
}

impl Server {
    pub fn new() -> Self {
        let client_key = thrussh_keys::key::KeyPair::generate_ed25519().unwrap();
        let client_pubkey = Arc::new(client_key.clone_public_key());
        let mut config = thrussh::server::Config::default();
        config.connection_timeout = Some(std::time::Duration::from_secs(3));
        config.auth_rejection_time = std::time::Duration::from_secs(3);
        config
            .keys
            .push(thrussh_keys::key::KeyPair::generate_ed25519().unwrap());
        let config = Arc::new(config);
        Self {
            config,
            client_pubkey,
            client: Arc::new(Mutex::new(None)),
            id: 0,
        }
    }

    pub async fn start(self) -> Result<(), StaticError> {
        debug!("sshd server key fingerprint: {:?}", self.config.keys);
        thrussh::server::run(self.config.clone(), "0.0.0.0:2222", self)
            .await
            .map_err(Into::into)
    }
}

impl server::Server for Server {
    type Handler = Self;
    fn new(&mut self, _: Option<std::net::SocketAddr>) -> Self {
        let s = self.clone();
        self.id += 1;
        s
    }
}

impl server::Handler for Server {
    type Error = StaticError;
    type FutureAuth = future::Ready<Result<(Self, server::Auth), Self::Error>>;
    type FutureUnit = future::Ready<Result<(Self, Session), Self::Error>>;
    type FutureBool = future::Ready<Result<(Self, Session, bool), Self::Error>>;

    fn finished_auth(self, auth: Auth) -> Self::FutureAuth {
        std::future::ready(Ok((self, auth)))
    }
    fn finished_bool(self, b: bool, s: Session) -> Self::FutureBool {
        std::future::ready(Ok((self, s, b)))
    }
    fn finished(self, s: Session) -> Self::FutureUnit {
        std::future::ready(Ok((self, s)))
    }

    fn auth_publickey(self, _: &str, _: &key::PublicKey) -> Self::FutureAuth {
        self.finished_auth(server::Auth::Accept)
    }

    fn channel_open_session(self, channel: ChannelId, session: Session) -> Self::FutureUnit {
        debug!("channel: {:?}", channel);
        {
            let mut client_handle = self.client.lock().unwrap();
            *client_handle = Some(ClientHandle {
                id_tuple: (self.id, channel),
                handle: session.handle(),
            });
        }
        self.finished(session)
    }

    fn data(self, channel: ChannelId, data: &[u8], mut session: Session) -> Self::FutureUnit {
        debug!("channel: {:?} | data {:?}", channel, data);
        // let fut = async {
        //     let mut client_handle = self.client.lock().unwrap();
        //     let (_, channel) = client_handle.id_tuple;
        //     let ref mut s = client_handle.handle;
        //     s.data(channel, CryptoVec::from_slice(data));
        // };
        session.data(channel, CryptoVec::from_slice(data));
        // fut.then(|_| self.finished(session))
        self.finished(session)
    }
}
