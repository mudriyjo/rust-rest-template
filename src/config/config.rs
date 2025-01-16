use figment::{
    providers::{Env},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_address: String,
    pub database_url: String,
}

pub fn get_config() -> Config {
    Figment::new()
        .merge(Env::prefixed("SERVICE_"))
        .extract()
        .expect("Can't configure server using env...")
}