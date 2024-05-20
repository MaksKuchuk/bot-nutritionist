use teloxide::{
    types::{KeyboardButton, KeyboardMarkup, Message},
    Bot,
};

use crate::{HandlerResult, MyDialogue};

pub fn create_keyboard(items_in_row: usize, items: Vec<&str>) -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    for i in items.chunks(items_in_row) {
        let row = i
            .iter()
            .map(|&i| KeyboardButton::new(i.to_owned()))
            .collect();

        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard)
        .resize_keyboard(true)
        .one_time_keyboard(true)
        .input_field_placeholder("Выберите один из вариантов:".to_string())
        .persistent()
}

pub async fn test_func(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    // let keyboard = create_keyboard(
    //     2,
    //     vec![
    //         &Gender::Male.to_string(),
    //         &Gender::Female.to_string(),
    //         &Gender::RyanGosling.to_string(),
    //     ],
    // );

    // let c = "command";
    // let url_button = InlineKeyboardButton::callback("text", c);
    // let keyboard = InlineKeyboardMarkup::default().append_row(vec![url_button]);

    // bot.send_message(msg.chat.id, "")
    //     .reply_markup(keyboard)
    //     .await?;

    Ok(())
}
