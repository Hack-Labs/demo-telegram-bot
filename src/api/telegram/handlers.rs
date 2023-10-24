use std::error::Error;

use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::types::Me;

use crate::plotter;
use crate::api::telegram::menu;



#[derive(BotCommands)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
pub enum Command {

    #[command(description = "Главное меню")]
    Start,

    #[command(description = "Справка")]
    Help

}




pub async fn message_handler(bot: Bot,
			     msg: Message,
			     me: Me)

			     -> Result<(), Box<dyn Error + Send + Sync>> {

    if let Some(text) = msg.text() {
	match BotCommands::parse(text, me.username()) {

	    Ok(Command::Start) => {
                menu::menu("main_menu", bot, msg.chat.id, None).await?;
            }

            Ok(Command::Help) => {
                menu::help(
		    bot,
		    msg.chat.id,
		    Command::descriptions().to_string()
		)
		    .await?;
            }

            Err(_) => {
                menu::unknown_command(bot, msg.chat.id).await?;
            }
        }
    }

    log::info!("Get command: {:?}", msg.text().unwrap());
    Ok(())
}




pub async fn callback_handler(bot: Bot,
			      query: CallbackQuery)

			      -> Result<(), Box<dyn Error + Send + Sync>> {

    if let (Some(menu_item),
	    Some(Message {id, chat, ..})) = (query.data, query.message) {

	bot.answer_callback_query(query.id).await?;

	let mut err: bool = false;

	match menu_item.as_str() {
	    "Главное меню" | "Назад" => {
		menu::menu("main_menu", bot, chat.id, Some(id)).await?;
	    },

	    "Пара: BTC/USDT (SMA + MAE)" | "Обновить данные" => {
		plotter::make_demo_plot().await?;
		menu::menu("plot_menu", bot, chat.id, Some(id)).await?;
	    },

	    _ => err = true
	}

	if err == false {
	    log::info!("Get command: {}", menu_item);
	} else {
	    log::warn!("Get command: {}", menu_item);
	    log::warn!("Check the button name!");
	}
    }

    Ok(())
}
