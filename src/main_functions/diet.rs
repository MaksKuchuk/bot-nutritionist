use teloxide::{payloads::SendMessageSetters, requests::Requester, types::Message, Bot};

use crate::{
    establish_connection,
    model::{
        usecases::{create_diet, get_diets_by_userid},
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
            let conn = &mut establish_connection();
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
        .reply_markup(
            create_keyboard(2, vec!["Конструктор рациона", "Назад"]).one_time_keyboard(true),
        )
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
        .await?;
    Ok(())
}

pub async fn diet_constructor_parser(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(txt) => {
            let conn = &mut establish_connection();

            match get_user_id(&msg) {
                Some(userid) => {
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
    bot.send_message(msg.chat.id, "Изменить рацион питания (TODO).")
        .reply_markup(create_keyboard(1, vec!["Назад"]))
        .await?;
    Ok(())
}

pub async fn diet_edit_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Назад") => diet(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}

pub async fn diet_remove(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::DietRemove).await?;
    bot.send_message(msg.chat.id, "Удалить рацион питания (TODO)")
        .reply_markup(create_keyboard(1, vec!["Назад"]))
        .await?;
    Ok(())
}

pub async fn diet_remove_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Назад") => diet(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}
