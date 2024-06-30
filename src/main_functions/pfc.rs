use teloxide::{requests::Requester, types::Message, Bot};

use crate::{
    get_db_connection,
    model::usecases::{get_diets_by_userid, get_user},
    utils::{get_kcpfc, get_string_diets_pfc, get_user_id},
    HandlerResult, MyDialogue,
};

use super::to_main_functions;

pub async fn pfc(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match get_user_id(&msg) {
        Some(uid) => {
            let conn = &mut get_db_connection();
            let str = match get_user(conn, uid.clone()) {
                Some(usr) => {
                    let kcpfc = get_kcpfc(&usr);
                    let mut s = format!(
                        "Ориентир КБЖУ: \n\t Ккал: {} \n\t Белки: {}г \n\t Жиры: {}г \n\t Углеводы: {}г\n\n",
                        kcpfc.kcal, kcpfc.proteins, kcpfc.fats, kcpfc.carbohydrates
                    );

                    let mut sst = String::new();
                    s += match get_diets_by_userid(conn, uid) {
                        Some(diets) => {
                            sst = get_string_diets_pfc(diets);
                            &sst
                        }
                        None => "",
                    };
                    s
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
