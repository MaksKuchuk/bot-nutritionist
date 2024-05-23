use teloxide::{payloads::SendMessageSetters, requests::Requester, types::Message, Bot};

use crate::{state::State, utils::create_keyboard, HandlerResult, MyDialogue};

pub mod diet;
pub mod notifications;
pub mod pfc;
pub mod pfcfood;
pub mod profile;

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Бот-диетолог поможет вам определить суточную норму КБЖУ основываясь на ваших физических данных и целях, которые вы преследуете. Также он способен помочь составить рацион питания на неделю и уведомлять вас каждый раз как наступает время приема пищи").await?;
    to_main_functions(bot, dialogue, msg).await?;
    Ok(())
}

pub async fn to_main_functions(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::Start).await?;
    let keyboard = create_keyboard(
        3,
        vec![
            "Профиль",
            "Рацион питания",
            "Норма КБЖУ",
            "Продукты питания",
            "Уведомления",
        ],
    );
    bot.send_message(msg.chat.id, "Выберите функцию")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn main_functions_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Профиль") => profile::profile(bot, dialogue, msg).await,
        Some("Рацион питания") => diet::diet(bot, dialogue, msg).await,
        Some("Норма КБЖУ") => pfc::pfc(bot, dialogue, msg).await,
        Some("Продукты питания") => pfcfood::pfcfood(bot, dialogue, msg).await,
        Some("Уведомления") => notifications::notifications(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}
