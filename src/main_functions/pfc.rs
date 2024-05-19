use teloxide::{requests::Requester, types::Message, Bot};

use crate::{HandlerResult, MyDialogue};

pub async fn pfc(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "БЖУ").await?;
    Ok(())
}
