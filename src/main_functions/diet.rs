use teloxide::{payloads::SendMessageSetters, requests::Requester, types::Message, Bot};

use crate::{state::State, utils::create_keyboard, HandlerResult, MyDialogue};

use super::to_main_functions;

pub async fn diet(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::Diet).await?;
    bot.send_message(msg.chat.id, "Рацион питания (TODO)")
        .reply_markup(create_keyboard(
            2,
            vec!["Создать", "Редактировать", "Удалить", "Назад"],
        ))
        .await?;
    Ok(())
}

pub async fn diet_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Создать") => diet_create(bot, dialogue, msg).await,
        Some("Редактировать") => diet_edit(bot, dialogue, msg).await,
        Some("Удалить") => diet_remove(bot, dialogue, msg).await,
        Some("Назад") => to_main_functions(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}

pub async fn diet_create(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::DietCreate).await?;
    bot.send_message(msg.chat.id, "Создание рациона питания.")
        .reply_markup(create_keyboard(
            3,
            vec!["Купить подписку", "Конструктор рациона", "Назад"],
        ))
        .await?;
    Ok(())
}

pub async fn diet_create_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Купить подписку") => Ok(()),
        Some("Конструктор рациона") => Ok(()),
        Some("Назад") => diet(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}

pub async fn diet_edit(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::DietEdit).await?;
    bot.send_message(msg.chat.id, "Изменить рацион питания (TODO).")
        .reply_markup(create_keyboard(1, vec!["Назад"]))
        .await?;
    Ok(())
}

pub async fn diet_edit_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Назад") => diet(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}

pub async fn diet_remove(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::DietRemove).await?;
    bot.send_message(msg.chat.id, "Удалить рацион питания (TODO)")
        .reply_markup(create_keyboard(1, vec!["Назад"]))
        .await?;
    Ok(())
}

pub async fn diet_remove_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Назад") => diet(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}
