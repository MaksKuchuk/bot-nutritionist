use teloxide::{
    requests::Requester,
    types::{KeyboardButton, KeyboardMarkup, Message},
    Bot,
};

use crate::{
    establish_connection,
    model::{
        usecases::{create_user, get_user},
        NewUser,
    },
    HandlerResult, MyDialogue,
};

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

pub async fn test_func1(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
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

pub async fn test_func2(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let conn = &mut establish_connection();

    create_user(
        conn,
        NewUser {
            id: 1i32,
            gender: 0i32,
            age: 0i32,
            height: 0i32,
            weight: 0i32,
            physical_activity_level: 0i32,
            goal: 0i32,
        },
    );

    bot.send_message(msg.chat.id, "Все ок").await?;

    Ok(())
}

pub async fn test_func3(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let conn = &mut establish_connection();

    let usr = &get_user(conn).unwrap();

    bot.send_message(
        msg.chat.id,
        format!(
            "{} \n {} \n {} \n {}",
            usr.gender, usr.age, usr.height, usr.goal
        ),
    )
    .await?;

    Ok(())
}
