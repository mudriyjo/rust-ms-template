use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_address: String,
}

pub fn get_config(profile: &str) -> Config {
    let config_path = format!("{}-config.yaml", profile);
    Figment::new()
        .merge(Yaml::file(config_path))
        .merge(Env::prefixed("SERVICE_"))
        .extract()
        .expect("Can't configure server using env and yaml...")
}
