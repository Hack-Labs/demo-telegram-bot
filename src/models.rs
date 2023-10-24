use serde::{
    de,
    Serialize,
    Deserialize,
    Deserializer
};

use chrono::prelude::*;




pub trait Timestamp {
    fn to_local_datetime(&self) -> DateTime<Local>;
}


impl Timestamp for i64 {
    fn to_local_datetime(&self) -> DateTime<Local> {
	let naive_datetime = NaiveDateTime::from_timestamp_millis(*self);

	let local_offset: FixedOffset = Local.offset_from_utc_datetime(
	    &naive_datetime.unwrap()
	);

	DateTime::from_naive_utc_and_offset(
	    naive_datetime.unwrap(),
	    local_offset
	)
    }
}




#[derive(Debug)]
pub struct Symbol {
    pub base_asset: String,
    pub quote_asset: String
}


impl Symbol {
    pub fn _from_message() {
	// From telegram-bot
    }

    pub fn to_currency_format(&self) -> String {
	format!("{}%2F{}", self.base_asset, self.quote_asset)
    }

    pub fn to_binance_format(&self) -> String {
	format!("{}{}", self.base_asset, self.quote_asset)
    }

    pub fn to_plot_format(&self) -> String {
	format!("{}/{}", self.base_asset, self.quote_asset)
    }
}




#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CurrencyCandle {
    pub open_time: i64,

    #[serde(deserialize_with = "to_float")]
    pub open: f64,

    #[serde(deserialize_with = "to_float")]
    pub high: f64,

    #[serde(deserialize_with = "to_float")]
    pub low: f64,

    #[serde(deserialize_with = "to_float")]
    pub close: f64,

    #[serde(deserialize_with = "to_float")]
    pub volume: f64,

    pub close_time: i64,

    #[serde(deserialize_with = "to_float")]
    pub quote_asset_volume: f64,

    pub number_of_trades: usize,

    #[serde(deserialize_with = "to_float")]
    pub take_buy_base_asset_volume: f64,

    #[serde(deserialize_with = "to_float")]
    pub take_buy_quote_asset_volume: f64,

    #[serde(deserialize_with = "to_float")]
    pub ignore: f64
}




#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BinanceCandle {
    pub open_time: i64,

    #[serde(deserialize_with = "to_float")]
    pub open: f64,

    #[serde(deserialize_with = "to_float")]
    pub high: f64,

    #[serde(deserialize_with = "to_float")]
    pub low: f64,

    #[serde(deserialize_with = "to_float")]
    pub close: f64,

    #[serde(deserialize_with = "to_float")]
    pub volume: f64,

    pub close_time: i64,

    #[serde(deserialize_with = "to_float")]
    pub quote_asset_volume: f64,

    pub number_of_trades: usize,

    #[serde(deserialize_with = "to_float")]
    pub take_buy_base_asset_volume: f64,

    #[serde(deserialize_with = "to_float")]
    pub take_buy_quote_asset_volume: f64,

    #[serde(deserialize_with = "to_float")]
    pub ignore: f64
}




fn to_float<'a, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'a>
{
    let json_str = String::deserialize(deserializer)?;

    json_str
	.parse::<f64>()
	.map_err(de::Error::custom)
}
