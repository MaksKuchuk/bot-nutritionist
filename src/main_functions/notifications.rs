use teloxide::{payloads::SendMessageSetters, requests::Requester, types::Message, Bot};

use crate::{
    get_db_connection,
    model::{
        usecases::{
            create_update_userdiet, get_choosen_diet, get_diet_by_id, get_diets_by_userid,
            get_diets_by_userid_name, set_user_notification,
        },
        NewChoosenDiet,
    },
    state::State,
    utils::{create_keyboard, get_user_id},
    HandlerResult, MyDialogue,
};

use super::to_main_functions;

pub async fn notifications(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::Notifications).await?;

    match get_user_id(&msg) {
        Some(userid) => {
            let conn = &mut get_db_connection();

            let mut str = String::from(
                    "Вы можете ключить уведомления, которые будут оповещать вас за 10 минут до приема пищи",
                );

            let s;
            str += match get_choosen_diet(conn, userid) {
                Some(diet) => match get_diet_by_id(conn, diet.dietid) {
                    Some(diet) => {
                        s = format!("\nВыбран рацион: \n\t{}", diet.name);
                        &s
                    }
                    None => "\nРациона не существует",
                },
                None => "\nРацион не выбран",
            };

            bot.send_message(msg.chat.id, str)
                .reply_markup(create_keyboard(
                    2,
                    vec!["Выбрать рацион", "Включить", "Выключить", "Назад"],
                ))
                .await?;
            Ok(())
        }
        None => Ok(()),
    }
}

pub async fn notifications_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Выбрать рацион") => notifications_choose_diet(bot, dialogue, msg).await,
        Some("Включить") => notifications_turn(bot, dialogue, msg, 1).await,
        Some("Выключить") => notifications_turn(bot, dialogue, msg, 0).await,
        Some("Назад") => to_main_functions(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}

pub async fn notifications_choose_diet(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    dialogue.update(State::NotificationsChooseDiet).await?;
    bot.send_message(msg.chat.id, "Введите название рациона: ")
        .reply_markup(create_keyboard(1, vec!["Назад"]))
        .await?;
    Ok(())
}

pub async fn notifications_choose_diet_parser(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some("Назад") => notifications(bot, dialogue, msg).await,
        Some(name) => match get_user_id(&msg) {
            Some(userid) => {
                let conn = &mut get_db_connection();

                let diet = get_diets_by_userid_name(conn, userid.clone(), name.to_string());
                bot.send_message(
                    msg.chat.id,
                    match diet {
                        Some(d) => {
                            let userdiet = NewChoosenDiet {
                                userid: userid,
                                dietid: d.dietid,
                                state: 0,
                            };

                            if create_update_userdiet(conn, userdiet) {
                                format!("Рацион выбран")
                            } else {
                                format!("Ошибка выбора рациона")
                            }
                        }
                        None => {
                            format!("Рацион \"{name}\" не найден")
                        }
                    },
                )
                .await?;
                notifications(bot, dialogue, msg).await?;
                Ok(())
            }
            None => Ok(()),
        },
        None => Ok(()),
    }
}

pub async fn notifications_turn(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    st: i32,
) -> HandlerResult {
    let str = match get_user_id(&msg) {
        Some(userid) => {
            let conn = &mut get_db_connection();

            if set_user_notification(conn, userid, st) {
                match st {
                    1 => "Уведомления включены",
                    0 => "Уведомления выключены",
                    _ => "Неверный статус",
                }
            } else {
                "Выберите рацион"
            }
        }
        None => "Вы не пользователь",
    };
    bot.send_message(msg.chat.id, str).await?;
    Ok(())
}
