use teloxide::{payloads::SendMessageSetters, requests::Requester, types::Message, Bot};

use crate::{
    get_db_connection,
    model::{
        usecases::{create_diet, delete_diet, get_diets_by_userid, get_diets_by_userid_name},
        NewUserDiet,
    },
    state::State,
    utils::{create_keyboard, get_string_diets, get_user_id},
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
        Some(txt) => Ok(()),
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
