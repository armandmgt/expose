mod settings;
mod models;
mod views;
mod controllers;
mod errors;
mod util;

use actix_web::{App, HttpServer, middleware, web};
use actix_web::middleware::TrailingSlash::Trim;
use sqlx::postgres::PgPoolOptions;

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

    let pool_data = web::Data::new(pool);
    let settings_data = web::Data::new(settings);

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .app_data(settings_data.clone())
            .wrap(middleware::NormalizePath::new(Trim))
            .wrap(middleware::Logger::new(
                r#"%a %{X-Real-IP}i %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
            ))
            .wrap(middleware::Compress::default())
            // .wrap(middlewares::vhost::VHost::new(settings_data.http.vhost_suffix.clone()))
            .configure(|cfg| controllers::urls(&settings_data, cfg))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
