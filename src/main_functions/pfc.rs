use teloxide::{requests::Requester, types::Message, Bot};

use crate::{
    establish_connection,
    model::usecases::get_user,
    utils::{get_kcpfc, get_user_id},
    HandlerResult, MyDialogue,
};

use super::to_main_functions;

pub async fn pfc(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match get_user_id(&msg) {
        Some(uid) => {
            let conn = &mut establish_connection();
            let str = match get_user(conn, uid) {
                Some(usr) => {
                    let kcpfc = get_kcpfc(&usr);
                    format!(
                        "КБЖУ: \n\t Ккал: {} \n\t Белки: {}г \n\t Жиры: {}г \n\t Углеводы: {}г",
                        kcpfc.kcal, kcpfc.proteins, kcpfc.fats, kcpfc.carbohydrates
                    )
                }
                None => "Для рассчета нормы КБЖУ необходимо заполнить профиль".to_string(),
            };
            bot.send_message(msg.chat.id, str).await?;
        }
        None => (),
    };

    to_main_functions(bot, dialogue, msg).await?;

    Ok(())
}
