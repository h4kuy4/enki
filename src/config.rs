use crate::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub listen_addr: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub user_name: String,
    pub password: String,
}

impl Config {
    pub fn from_env() -> Result<Config> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into().map_err(|e| e.into())
    }
}
