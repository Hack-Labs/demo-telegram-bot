use std::error::Error;

use teloxide::types::{
    MessageId,
    InputFile,
    InputMedia,
    InputMediaPhoto,
    InlineKeyboardButton,
    InlineKeyboardMarkup,
    ParseMode::MarkdownV2
};

use teloxide::prelude::*;




pub async fn menu(name: &str,
		  bot: Bot,
		  chat_id: ChatId,
		  msg_id: Option<MessageId>)

		  -> Result<(), Box<dyn Error + Send + Sync>> {

    let keyboard: InlineKeyboardMarkup = make_button_menu(name);
    let message: &str = "DEMO \\| Tech\\-Indicators";

    let img_path: &str = match name {
	"main_menu" => "img/telegram/main_menu.png",
	"plot_menu" => "img/plots/plot.png",
	_ => panic!()
    };

    if let Some(msg_id) = msg_id {
	let media = InputMedia::Photo(
	    InputMediaPhoto {
		media: InputFile::file(img_path),
		caption: Some(message.to_string()),
		parse_mode: Some(MarkdownV2),
		caption_entities: None,
		has_spoiler: false }
	);

	bot.edit_message_media(chat_id, msg_id, media)
	    .reply_markup(keyboard)
	    .await?;

    } else {
	bot.send_photo(chat_id, InputFile::file(img_path))
	    .parse_mode(MarkdownV2)
	    .caption(message)
	    .reply_markup(keyboard)
	    .await?;
    }

    Ok(())
}




pub fn make_button_menu(name: &str) -> InlineKeyboardMarkup {
    let mut button_menu: Vec<Vec<InlineKeyboardButton>> = vec![];

    let button_names = match name {
	"main_menu" => vec!["Пара: BTC/USDT (SMA + MAE)"],
	"plot_menu" => vec!["Обновить данные", "Назад"],

	_ => panic!("Check the name")
    };

    for items in button_names.chunks(1) {
	button_menu.push(
	    items
		.iter()
		.map(
		    | &item | {
			InlineKeyboardButton::callback(
			    item.to_owned(),
			    item.to_owned()
			)
		    }
		)
		.collect()
	)
    }

    InlineKeyboardMarkup::new(button_menu)
}




pub async fn help(bot: Bot,
		  chat_id: ChatId,
		  body: String)

		  -> Result<(), Box<dyn Error + Send + Sync>> {

    bot.send_message(chat_id, body).await?;
    Ok(())
}




pub async fn unknown_command(bot: Bot,
			     chat_id: ChatId)

			     -> Result<(),Box<dyn Error + Send + Sync>> {

    let msg: &str = concat!(
	"Сообщение не распознано в демо-версии продукта. ",
	"Обратитесь к поставщику"
    );

    bot.send_message(chat_id, msg).await?;
    Ok(())
}
