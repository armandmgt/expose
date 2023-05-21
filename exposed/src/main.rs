mod controllers;
mod errors;
mod models;
mod settings;
mod sshd;
mod util;
mod views;
mod websockets;

use std::pin::Pin;

use actix::Actor;
use actix_web::middleware::TrailingSlash::Trim;
use actix_web::{middleware, web, App, HttpServer};
use errors::StaticError;
use futures_util::future::join_all;
use futures_util::TryFutureExt;
use sqlx::postgres::PgPoolOptions;
use tokio::signal;
use tracing::{debug, info};

#[actix_web::main]
async fn main() -> Result<(), StaticError> {
    dotenv::dotenv().ok();
    env_logger::init();

    let settings = settings::Settings::new().expect("error loading settings");

    let db_pool = PgPoolOptions::new()
        .connect(settings.database.url.as_str())
        .await
        .expect("error creating db pool");
    sqlx::migrate!().run(&db_pool).await.unwrap();

    let ws_server = websockets::server::ConnectionsWsServer::new().start();

    let shared_settings = web::Data::new(settings.clone());
    let db_pool = web::Data::new(db_pool);
    let ws_server = web::Data::new(ws_server);
    let sshd_server = sshd::Server::new();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .app_data(shared_settings.clone())
            .app_data(ws_server.clone())
            .wrap(middleware::NormalizePath::new(Trim))
            .wrap(middleware::Logger::new(
                r#"%a %{X-Real-IP}i %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
            ))
            .wrap(middleware::Compress::default())
            .configure(|cfg| controllers::urls(&shared_settings, cfg))
    })
    .disable_signals()
    .bind((
        settings.http.bind_addr.unwrap_or("127.0.0.1".to_string()),
        settings.http.bind_port.unwrap_or(8080),
    ))?;
    info!(
        "Server listening on {:?}",
        server
            .addrs_with_scheme()
            .into_iter()
            .map(|(addr, scheme)| { format!("{scheme}://{addr}") })
            .collect::<Vec<_>>()
            .join(", ")
    );
    let tasks: Vec<Pin<Box<dyn std::future::Future<Output = Result<(), StaticError>>>>> = vec![
        Box::pin(server.run().map_err(Into::into)),
        Box::pin(sshd_server.start()),
        Box::pin(
            signal::ctrl_c()
                .and_then(|_| {
                    debug!("received ctrl-c, should now shutdown");
                    std::future::ready(Ok(()))
                })
                .map_err(Into::into),
        ),
    ];
    let results = join_all(tasks).await;
    results.into_iter().fold(Ok(()), Result::and)
}
