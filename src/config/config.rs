use clap::Parser;

#[derive(clap::Parser, Debug)]
pub struct Config {
    #[clap(long,env)]
    pub server_adress: String
}

pub fn get_config() -> Config {
    let config = Config::parse();
    config
}