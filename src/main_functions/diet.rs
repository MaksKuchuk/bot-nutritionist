use teloxide::{payloads::SendMessageSetters, requests::Requester, types::Message, Bot};

use crate::{
    get_db_connection,
    model::{
        usecases::{
            create_diet, delete_diet, get_diets_by_userid, get_diets_by_userid_name,
            get_random_example_diet, get_user, update_diet,
        },
        NewUserDiet,
    },
    state::State,
    utils::{
        create_example_diet_string, create_keyboard, get_kcpfc, get_string_diets, get_user_id,
        is_diet_right,
    },
    HandlerResult, MyDialogue,
};

use super::to_main_functions;

pub async fn diet(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::Diet).await?;

    let str = match get_user_id(&msg) {
        Some(userid) => {
            let conn = &mut get_db_connection();
            let d = get_diets_by_userid(conn, userid);
            match d {
                Some(diets) => get_string_diets(diets),
                None => String::from("Рационы не найдены."),
            }
        }
        None => String::from("Ошибка. Ты не пользователь."),
    };

    bot.send_message(msg.chat.id, str)
        .reply_markup(create_keyboard(
            3,
            vec![
                "Создать",
                "Редактировать",
                "Удалить",
                "Пример рациона",
                "Назад",
            ],
        ))
        .await?;
    Ok(())
}

pub async fn diet_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Создать") => diet_create(bot, dialogue, msg).await,
        Some("Редактировать") => diet_edit(bot, dialogue, msg).await,
        Some("Удалить") => diet_remove(bot, dialogue, msg).await,
        Some("Пример рациона") => diet_example(bot, dialogue, msg).await,
        Some("Назад") => to_main_functions(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}

pub async fn diet_example(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match get_user_id(&msg) {
        Some(uid) => {
            let conn = &mut get_db_connection();
            let str = match get_user(conn, uid.clone()) {
                Some(usr) => {
                    let kcpfc = get_kcpfc(&usr);

                    match get_random_example_diet(conn) {
                        Some(v) => create_example_diet_string(v, kcpfc),
                        None => "Рацион не найден".to_string(),
                    }
                }
                None => "Для рассчета примерного рациона необходимо заполнить профиль".to_string(),
            };
            bot.send_message(msg.chat.id, str).await?;
        }
        None => (),
    };
    Ok(())
}

pub async fn diet_create(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::DietCreate).await?;
    bot.send_message(msg.chat.id, "Создание рациона питания.")
        .reply_markup(create_keyboard(2, vec!["Конструктор рациона", "Назад"]))
        .await?;
    Ok(())
}

pub async fn diet_create_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        // Some("Купить подписку") => Ok(()),
        Some("Конструктор рациона") => diet_constructor(bot, dialogue, msg).await,
        Some("Назад") => diet(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}

pub async fn diet_constructor(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::DietConstructor).await?;
    bot.send_message(msg.chat.id, "Введите название рациона: ")
        .reply_markup(create_keyboard(2, vec!["Назад"]))
        .await?;
    Ok(())
}

pub async fn diet_constructor_parser(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some("Назад") => (),
        Some(txt) => {
            let conn = &mut get_db_connection();

            match get_user_id(&msg) {
                Some(userid) => {
                    if let Some(_) = get_diets_by_userid_name(conn, userid.clone(), txt.to_string())
                    {
                        bot.send_message(msg.chat.id, "Рацион с таким названием уже существует")
                            .await?;
                        return Ok(());
                    }

                    let usrdiet = NewUserDiet {
                        userid,
                        name: txt.to_string(),
                        diet: String::new(),
                    };

                    bot.send_message(
                        msg.chat.id,
                        if create_diet(conn, usrdiet) {
                            "Рацион питания успешно создан. Чтобы заполнить его нажмите на кнопку \"Редактировать\""
                        } else {
                            "Рацион не создан. Попробуйте еще раз"
                        },
                    )
                    .await?;
                }
                None => (),
            }
        }
        _ => (),
    }
    diet(bot, dialogue, msg).await?;
    Ok(())
}

pub async fn diet_edit(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::DietEdit).await?;
    bot.send_message(msg.chat.id, "Введите название рациона: ")
        .reply_markup(create_keyboard(1, vec!["Назад"]))
        .await?;
    Ok(())
}

pub async fn diet_edit_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Назад") => diet(bot, dialogue, msg).await,
        Some(name) => match get_user_id(&msg) {
            Some(userid) => {
                let conn = &mut get_db_connection();

                match get_diets_by_userid_name(conn, userid.clone(), name.to_string()) {
                    Some(_) => {
                        bot.send_message(
                                msg.chat.id,
                                "Введите список приемов пищи, с временем в 24-х часовом формате, и продуктов питания, разделяя приемы пищи пустой строкой, а каждый новый продукт описывайте в формате (название, масса(в граммах), (БЖУ)). Пример:\n\n8:20\nрис, 150, (2.9 25.2 0.4)\nтреска отварная, 50, (17.8 0 0.7)\nбелый хлеб, 50, (11 48 4)\n\n13:00\nгречка на воде, 150, (3.38 19.94 0.62)\nяйцо куриное вареное, 30, (13 1.12 10.61)\n\n19:10\nяблоко красное, 100, (0.4 17 0)\nбанан, 100, (1.2 22 0.2)",
                            )
                            .reply_markup(create_keyboard(1, vec!["Назад"]))
                            .await?;
                        dialogue
                            .update(State::DietEditName {
                                name: name.to_string(),
                                userid: userid,
                            })
                            .await?;
                        Ok(())
                    }
                    None => diet_edit(bot, dialogue, msg).await,
                }
            }
            None => Ok(()),
        },
        None => Ok(()),
    }
}

pub async fn diet_edit_name_parser(
    bot: Bot,
    dialogue: MyDialogue,
    (name, userid): (String, String),
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some("Назад") => diet(bot, dialogue, msg).await,
        Some(d) => match is_diet_right(d) {
            Ok(()) => {
                let conn = &mut get_db_connection();

                let usrdiet = NewUserDiet {
                    userid,
                    name,
                    diet: d.to_string(),
                };

                bot.send_message(
                    msg.chat.id,
                    if update_diet(conn, usrdiet) {
                        "Рацион питания успешно обновлен"
                    } else {
                        "Рацион не обновлен. Попробуйте еще раз"
                    },
                )
                .await?;
                diet(bot, dialogue, msg).await?;
                Ok(())
            }
            Err(s) => {
                bot.send_message(msg.chat.id, s)
                    .reply_markup(create_keyboard(1, vec!["Назад"]))
                    .await?;
                Ok(())
            }
        },
        None => Ok(()),
    }
}

pub async fn diet_remove(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::DietRemove).await?;
    bot.send_message(msg.chat.id, "Введите название рациона: ")
        .reply_markup(create_keyboard(1, vec!["Назад"]))
        .await?;
    Ok(())
}

pub async fn diet_remove_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Назад") => diet(bot, dialogue, msg).await,
        Some(name) => match get_user_id(&msg) {
            Some(userid) => {
                let conn = &mut get_db_connection();

                bot.send_message(
                    msg.chat.id,
                    if delete_diet(conn, userid, name.to_string()) {
                        format!("Рацион \'{name}\' удален")
                    } else {
                        format!("Ошибка удаления рациона \'{name}\'")
                    },
                )
                .await?;
                diet(bot, dialogue, msg).await?;
                Ok(())
            }
            None => Ok(()),
        },
        None => Ok(()),
    }
}
