use teloxide::{requests::Requester, types::Message, Bot};

use crate::{HandlerResult, MyDialogue};

pub async fn pfcfood(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "БЖУ продукты").await?;
    Ok(())
}
