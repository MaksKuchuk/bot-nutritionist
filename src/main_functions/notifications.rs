use teloxide::{requests::Requester, types::Message, Bot};

use crate::{HandlerResult, MyDialogue};

pub async fn notifications(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Уведомления").await?;
    Ok(())
}
