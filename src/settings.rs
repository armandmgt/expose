use config::Config;
use url::Url;
use serde::{Deserialize};
use std::path::PathBuf;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct Http {
    pub url: Url,
    pub secure: bool,
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Files {
    pub static_dir: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub http: Http,
    pub files: Files,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        dotenv::dotenv().ok();
        let env = env::var("ENV_TYPE").unwrap_or_else(|_| "development".into());
        let s = Config::builder().
            add_source(config::File::with_name("conf/default")).
            add_source(config::File::with_name(&format!("conf/{}", env))).
            add_source(config::Environment::with_prefix("APP")).
            build().unwrap();
        s.try_deserialize::<Self>()
    }
}
