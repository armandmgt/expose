mod awc_client;
mod connection;
mod dto;
mod error;

use crate::{awc_client::client, connection::Connection};
use anyhow::Result;
use clap::Parser;
use log::debug;

#[derive(Parser, Debug)]
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

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::parse();
    let awc_client = client();
    env_logger::Builder::new()
        .filter_level(options.verbose.log_level_filter())
        .init();

    let connection = Connection::create(&awc_client, &options).await?;
    debug!("Connection successfully created");
    // connection.subscribe(&options).await?;
    connection.delete(&awc_client, &options).await
}
