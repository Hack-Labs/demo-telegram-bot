use std::str::FromStr;

use chrono::prelude::*;
use plotters::prelude::*;
use::chrono::Duration;

use crate::models::{
    Symbol,
    BinanceCandle,
    Timestamp
};

use crate::indicators::{
    SMA,
    MAE
};

use crate::api;



pub async fn make_demo_plot() -> Result<(), &'static str> {
    log::info!("Make demo plot BTC/USDT + SMA, MAE");

    let symbol = Symbol {
	base_asset: String::from_str("BTC").unwrap(),
	quote_asset: String::from_str("USDT").unwrap()
    };

    let candles = match api::exchanges::get_candles(&symbol, "1d", 150).await {
	Some(candles) => candles,
	_ => return Err("Check the exchange request parameters")
    };

    let price_data: Vec<f64> = candles
	.iter()
	.rev()
	.take(150)
	.map(| kline | kline.close)
	.collect();

    let sma_indicator = SMA::new(&price_data, 40);
    let mae_indicator = MAE::new(&sma_indicator.sequence, 0.1);

    write_demo_plot(&symbol,
		    candles,
		    sma_indicator,
		    mae_indicator,
		    "img/plots/plot.png",
		    (1280, 960));

    Ok(())
}



fn write_demo_plot(symbol: &Symbol,
		   candles: Vec<BinanceCandle>,
		   sma_indicator: SMA,
		   mae_indicator: MAE,
		   plot_path: &str,
		   plot_size: (u32, u32)) {

    let candlestick: Vec<(DateTime<Local>, f64, f64, f64, f64)> = candles
        .iter()
        .rev()
        .take(150)

        .map(| kline | { (kline.open_time.to_local_datetime(),
			  kline.open,
			  kline.high,
			  kline.low,
			  kline.close) })

        .collect();

    let root = BitMapBackend::new(plot_path, plot_size)
        .into_drawing_area();

    root.fill(&WHITE).expect("Error fill");

    if let Err(error) = root.titled(
	symbol.to_plot_format().as_str(),
	("sans-serif", 50))
    {
	log::error!("{:#?}", error)
    }

    let (start_date, end_date) = (
        candlestick[candlestick.len() - 1].0 - Duration::days(1),
        candlestick[0].0 + Duration::days(4)
    );

    let (min_price, max_price) = (
	candlestick
	    .iter()
	    .min_by(| x, y | x.3.total_cmp(&y.3))
	    .unwrap()
	    .3 * 0.9,

	candlestick
	    .iter()
	    .max_by(| x, y | x.2.total_cmp(&y.2))
	    .unwrap()
	    .2 * 1.1
    );

    let local_time =  Local::now();

    let mut chart = ChartBuilder::on(&root)
        .margin(60)
	.set_all_label_area_size(50)

        .caption(
	    format!("Interval: {}, Localtime: {} {}",
		    "1 day",
		    local_time.format("%F"),
		    local_time.format("%H:%M:%S")
	    ),

	    ("sans-serif", 20).into_font()
	)

        .build_cartesian_2d(
	    start_date..end_date,
	    min_price..max_price
	)

        .unwrap();

    chart
        .configure_mesh()
        .light_line_style(WHITE)
        .x_label_formatter(&| datetime | datetime.format("%F").to_string())
        .draw()
        .unwrap();

    chart
        .draw_series(
	    candlestick.iter().map(
		| candle | {
		    CandleStick::new(
			candle.0,
			candle.1,
			candle.2,
			candle.3,
			candle.4,
			RGBColor(98, 209, 61).filled(),
			RGBColor(209, 61, 61).filled(),
			6,
		    )
		}
	    )
	)
        .unwrap();


    // SMA:

    let mut sma_line: Vec<(DateTime<Local>, f64)> = vec![];

    for i in 0..sma_indicator.sequence.len() {
	sma_line.push(
	    (candlestick[i].0, sma_indicator.sequence[i])
	)
    }

    chart
        .draw_series(LineSeries::new(sma_line, BLUE.stroke_width(2)))
        .unwrap()
        .label("SMA 40")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));


    // MAE:

    let mut mae_upper_line: Vec<(DateTime<Local>, f64)> = vec![];
    let mut mae_lower_line: Vec<(DateTime<Local>, f64)> = vec![];

    let mut mae_area: Vec<(DateTime<Local>, f64)> = vec![];

    for i in 0..mae_indicator.upper_sequence.len() {
	mae_upper_line.push(
	    (candlestick[i].0, mae_indicator.upper_sequence[i])
	);

	mae_lower_line.push(
	    (candlestick[i].0, mae_indicator.lower_sequence[i])
	);
    }

    mae_area.extend(mae_upper_line.clone());
    mae_area.extend(mae_lower_line.clone().iter().rev());

    chart
        .draw_series(
	    std::iter::once(
		Polygon::new(
		    mae_area,
		    RED.mix(0.2)
		)
	    )
	)
	.unwrap()
	.label("MAE 10%")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(
	    std::iter::once(
		PathElement::new(
		    mae_upper_line,
		    BLACK
		)
	    )
	)
        .unwrap();

    chart
        .draw_series(
	    std::iter::once(
		PathElement::new(
		    mae_lower_line,
		    BLACK
		)
	    )
	)
        .unwrap();


    // Labels:

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .label_font(("sans-serif", 30.0).into_font())
        .background_style(RGBColor(245, 245, 245).filled())
        .draw()
        .unwrap();

    root.present().expect("Error write plot file");
}
