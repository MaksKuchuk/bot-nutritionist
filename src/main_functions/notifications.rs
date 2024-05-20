use teloxide::{payloads::SendMessageSetters, requests::Requester, types::Message, Bot};

use crate::{state::State, utils::create_keyboard, HandlerResult, MyDialogue};

use super::to_main_functions;

pub async fn notifications(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::Notifications).await?;
    bot.send_message(msg.chat.id, "Уведомления (TODO)")
        .reply_markup(create_keyboard(
            2,
            vec!["Выбрать рацион", "Включить", "Выключить", "Назад"],
        ))
        .await?;
    Ok(())
}

pub async fn notifications_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Выбрать рацион") => Ok(()),
        Some("Включить") => Ok(()),
        Some("Выключить") => Ok(()),
        Some("Назад") => to_main_functions(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}
