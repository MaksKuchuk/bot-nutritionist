use teloxide::{payloads::SendMessageSetters, requests::Requester, types::Message, Bot};

use crate::{
    establish_connection,
    model::usecases::get_food_by_category,
    state::State,
    utils::{create_keyboard, get_string_foods},
    HandlerResult, MyDialogue,
};

use super::to_main_functions;

pub async fn pfcfood(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::PFCFood).await?;
    bot.send_message(msg.chat.id, "Выберите категорию")
        .reply_markup(create_keyboard(
            2,
            vec![
                "Мясопродукты",
                "Рыбопродукты",
                "Молочные продукты",
                "Хлебобулочные изделия",
                "Крупы, макаронные изделия",
                "Фрукты, ягоды, овощи",
                "Соки",
                "Назад",
            ],
        ))
        .await?;
    Ok(())
}

pub async fn pfcfood_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Мясопродукты") => pfcfood_meat(bot, dialogue, msg).await,
        Some("Рыбопродукты") => pfcfood_fish(bot, dialogue, msg).await,
        Some("Молочные продукты") => pfcfood_milk(bot, dialogue, msg).await,
        Some("Хлебобулочные изделия") => {
            pfcfood_bread(bot, dialogue, msg).await
        }
        Some("Крупы, макаронные изделия") => {
            pfcfood_cereal_pasta(bot, dialogue, msg).await
        }
        Some("Фрукты, ягоды, овощи") => {
            pfcfood_fruit_vegetable_berry(bot, dialogue, msg).await
        }
        Some("Соки") => pfcfood_juice(bot, dialogue, msg).await,
        Some("Назад") => to_main_functions(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}

pub async fn get_pfcfood_string(cat: &str) -> String {
    let conn = &mut establish_connection();

    match get_food_by_category(conn, cat, 5) {
        Some(foods) => {
            let s = get_string_foods(foods);
            if s.is_empty() {
                "Продукты из данной категрории не найдены".to_string()
            } else {
                s
            }
        }
        None => "Ошибка".to_string(),
    }
}

pub async fn pfcfood_meat(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, get_pfcfood_string("Мясопродукты").await)
        .await?;
    Ok(())
}
pub async fn pfcfood_fish(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, get_pfcfood_string("Рыбопродукты").await)
        .await?;
    Ok(())
}
pub async fn pfcfood_milk(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, get_pfcfood_string("Молочные продукты").await)
        .await?;
    Ok(())
}
pub async fn pfcfood_bread(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        get_pfcfood_string("Хлебобулочные изделия").await,
    )
    .await?;
    Ok(())
}
pub async fn pfcfood_cereal_pasta(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        get_pfcfood_string("Крупы, макаронные изделия").await,
    )
    .await?;
    Ok(())
}
pub async fn pfcfood_fruit_vegetable_berry(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        get_pfcfood_string("Фрукты, ягоды, овощи").await,
    )
    .await?;
    Ok(())
}
pub async fn pfcfood_juice(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, get_pfcfood_string("Соки").await)
        .await?;
    Ok(())
}
