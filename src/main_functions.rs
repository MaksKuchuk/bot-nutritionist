use teloxide::{payloads::SendMessageSetters, requests::Requester, types::Message, Bot};

use crate::{utils::create_keyboard, HandlerResult, MyDialogue};

pub mod diet;
pub mod notifications;
pub mod pfc;
pub mod pfcfood;
pub mod portfolio;

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let keyboard = create_keyboard(
        3,
        vec![
            "Профиль",
            "Рацион питания",
            "Норма БЖУ",
            "Продукты питания",
            "Уведомления",
        ],
    );
    bot.send_message(msg.chat.id, "Бот-диетолог поможет вам определить суточную норму БЖУ основываясь на ваших физических данных и целях, которые вы преследуете. Также он способен помочь составить рацион питания на неделю и уведомлять вас каждый раз как наступает время приема пищи.")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn start_state(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dbg!("laaaaaaaaaaaaaaaaaaaa");
    match msg.text() {
        Some("Профиль") => portfolio::portfolio(bot, dialogue, msg).await,
        Some("Рацион питания") => diet::diet(bot, dialogue, msg).await,
        Some("Норма БЖУ") => pfc::pfc(bot, dialogue, msg).await,
        Some("Продукты питания") => pfcfood::pfcfood(bot, dialogue, msg).await,
        Some("Уведомления") => notifications::notifications(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}
