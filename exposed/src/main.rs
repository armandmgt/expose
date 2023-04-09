mod settings;
mod models;
mod views;
mod controllers;
mod websockets;
mod errors;
mod util;

use actix::Actor;
use actix_web::{App, HttpServer, middleware, web};
use actix_web::middleware::TrailingSlash::Trim;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let settings = settings::Settings::new().expect("error loading settings");

    let pool = PgPoolOptions::new()
        .connect(settings.database.url.as_str())
        .await
        .expect("error creating db pool");
    sqlx::migrate!().run(&pool).await.unwrap();

    let ws_server = websockets::server::ConnectionsWsServer::new().start();

    let pool_data = web::Data::new(pool);
    let settings_data = web::Data::new(settings.clone());
    let ws_server_data = web::Data::new(ws_server);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .app_data(settings_data.clone())
            .app_data(ws_server_data.clone())
            .wrap(middleware::NormalizePath::new(Trim))
            .wrap(middleware::Logger::new(
                r#"%a %{X-Real-IP}i %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
            ))
            .wrap(middleware::Compress::default())
            .configure(|cfg| controllers::urls(&settings_data, cfg))
    }).bind((
        settings.http.bind_addr.unwrap_or("127.0.0.1".to_string()),
        settings.http.bind_port.unwrap_or(8080)
    ))?;
    info!("Server listening on {:?}", server.addrs_with_scheme().into_iter().map(|(addr, scheme)| {
        format!("{scheme}://{addr}")
    }).collect::<Vec<_>>().join(", "));
    server.run().await
}
