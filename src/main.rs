mod awc_client;
mod connection;
mod error;
mod server_conf;
mod ssh;
mod utils;

use std::sync::Arc;

use crate::{awc_client::client, connection::Connection};
use anyhow::Result;
use clap::Parser;
use futures_util::future::join_all;
use log::debug;
use tokio::signal;
use tokio_util::sync::CancellationToken;

#[derive(Parser, Clone, Debug)]
#[command(version, about)]
pub struct Options {
    #[arg(help = "Local port where the request should be proxied.")]
    port: String,

    #[arg(
        short = 'H',
        long = "host",
        help = "Specify the hostname (with optional port) of the server."
    )]
    host: String,

    #[arg(
        short = 's',
        long = "secret",
        help = "Email and API token to access the server joined by ':'. Example: user@example.com:1fd1e3e1d51628dfb767a6aeb942ff1cecc7e09d"
    )]
    secret: String,

    #[arg(
        short = 'd',
        long = "subdomain",
        help = "Request a specific subdomain. This flag is optional, a randomly generated subdomain will be provided if not set."
    )]
    subdomain: String,

    #[arg(long = "no-ssl", help = "Connect to server via http instead of https.")]
    no_ssl: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn ssh_client_task(
    options: Options,
    server_conf: server_conf::Conf,
    connection: Arc<Connection>,
    cancellation_token: CancellationToken,
) -> tokio::task::JoinHandle<Result<()>> {
    tokio::task::spawn(async move { ssh::start(options, server_conf, connection, cancellation_token).await })
}

#[actix_web::main]
async fn main() -> Result<()> {
    let options = Options::parse();
    env_logger::Builder::new()
        .filter_level(options.verbose.log_level_filter())
        .init();

    let awc_client = client();

    let server_conf = server_conf::get(&awc_client, &options).await?;

    let connection = Arc::new(Connection::create(&awc_client, &options).await?);
    debug!("Connection successfully created");

    let cancellation_token = CancellationToken::new();
    let tasks = vec![
        ssh_client_task(options.clone(), server_conf, connection.clone(), cancellation_token.clone()),
        tokio::task::spawn(async move {
            signal::ctrl_c()
                .await
                .map(|_| {
                    debug!("received ctrl-c, should now shutdown");
                    cancellation_token.cancel();
                })
                .map_err(Into::into)
        }),
    ];
    let results = join_all(tasks).await;
    let result = results
        .into_iter()
        .map(|join_res| join_res?)
        .fold(Ok(()), Result::and);

    connection.delete(&awc_client, &options).await.and(result)?;

    Ok(())
}
