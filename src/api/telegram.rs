pub mod handlers;
pub mod menu;

use std::error::Error;
use teloxide::prelude::*;

use crate::config;




pub async fn run_robot() -> Result<(), Box<dyn Error>> {
    log::info!("Run the Telegram module");

    let bot = Bot::new(
	config::get_config!(telegram_bot: token)
    );

    let handler = dptree::entry()
        .branch(
	    Update::filter_message()
		.endpoint(handlers::message_handler)
	)

        .branch(
	    Update::filter_callback_query()
		.endpoint(handlers::callback_handler)
	);

    Dispatcher::builder(bot, handler)
	.enable_ctrlc_handler()
	.build()
	.dispatch()
	.await;

    Ok(())
}
