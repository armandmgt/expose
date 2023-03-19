mod connection;
mod error;

use clap::Parser;
use log::error;
use crate::connection::Connection;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Options {
    #[arg(help = "Local port where the request should be proxied.")]
    port: String,

    #[arg(short = 'H', long = "host", help = "Specify the hostname (with optional port) of the server.")]
    host: String,

    #[arg(short = 's', long = "secret", help = "Email and API token to access the server joined by ':'. Example: user@example.com:1fd1e3e1d51628dfb767a6aeb942ff1cecc7e09d")]
    secret: String,

    #[arg(short = 'd', long = "subdomain", help = "Request a specific subdomain. This flag is optional, a randomly generated subdomain will be provided if not set.")]
    subdomain: String,

    #[arg(long = "no-ssl", help = "Connect to server via http instead of https.")]
    no_ssl: bool,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() {
    let options = Options::parse();
    env_logger::Builder::new()
        .filter_level(options.verbose.log_level_filter())
        .init();

    let connection = match Connection::create(&options) {
        Ok(v) => v,
        Err(e) => {
            error!("{:?}", e);
            return
        }
    };
    connection.delete(&options).unwrap();
}
