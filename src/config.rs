use std::error::Error;
use serde::Deserialize;
use serde_yaml;




const CONFIG_PATH: &str = "bot-config.yaml";



#[derive(Debug, Deserialize)]
pub struct Config {
    pub telegram_bot: TelegramBot,
    pub api_entry_point: APIEntryPoint
}


#[derive(Debug, Deserialize)]
pub struct TelegramBot {
    pub token: String
}


#[derive(Debug, Deserialize)]
pub struct APIEntryPoint {
    pub binance: String,
    pub currency: String
}




pub fn deserialize_config() -> Result<Config, Box<dyn Error>> {
    let file = std::fs::File::open(CONFIG_PATH)?;
    let config: Config = serde_yaml::from_reader(file)?;

    Ok(config)
}



#[macro_export]
macro_rules! get_config {
    ($key: ident : $($sub_keys: ident).*) => {
	config::deserialize_config().unwrap().$key.$($sub_keys).*
    }
}



pub use get_config;
