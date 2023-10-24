#![allow(unused)]


use crate::models::{
    Symbol,
    CurrencyCandle,
    BinanceCandle
};

use reqwest::{
    Client,
    Response,
    StatusCode
};

use serde_json;
use crate::config;




pub async fn get_candles(symbol: &Symbol,
			 interval: &str,
			 limit: u32) -> Option<Vec<BinanceCandle>> {

    let client = Client::builder()
        .build()
        .unwrap();

    let url = format!(
	"{}/klines?symbol={}&interval={}&limit={}",
	config::get_config!(api_entry_point: binance),
	symbol.to_binance_format(),
	interval,
	limit
    );

    let response: Response = client.get(&url)
        .send()
        .await
	.unwrap();

    let candles: Vec<BinanceCandle> = match response.status() {
	StatusCode::OK => {
	    serde_json::from_value::<Vec<BinanceCandle>>(
		response
		    .json()
		    .await
		    .unwrap()
	    )
		.unwrap()
	}

	_ => {

	    log::error!(
		"{}: {:#?}",
		response.status(),
		response
		    .json::<serde_json::Value>()
		    .await
		    .unwrap()
	    );

	    return None;
	}
    };

    Some(candles)
}




// TODO: implement exchange module
async fn _get_curency_candles(symbol: &Symbol,
			      interval: &str,
			      limit: u32) -> Option<Vec<CurrencyCandle>> {

    let client = Client::builder()
        .build()
        .unwrap();

    let url = format!(
	"{}/candles?symbol={}&interval={}&limit={}",
	config::get_config!(api_entry_point: currency),
	symbol.to_currency_format(),
	interval,
	limit
    );

    let response: Response = client.get(&url)
        .send()
        .await
	.unwrap();

    let candles: Vec<CurrencyCandle> = match response.status() {
	StatusCode::OK => {
	    serde_json::from_value::<Vec<CurrencyCandle>>(
		response
		    .json()
		    .await
		    .unwrap()
	    )
		.unwrap()
	}

	_ => {

	    log::error!(
		"{}: {:#?}",
		response.status(),
		response
		    .json::<serde_json::Value>()
		    .await
		    .unwrap()
	    );

	    return None;
	}
    };

    Some(candles)
}
