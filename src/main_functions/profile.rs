use std::str::FromStr;

use teloxide::{
    payloads::SendMessageSetters,
    requests::Requester,
    types::{KeyboardRemove, Message},
    Bot,
};

use crate::{
    domain::profile_domain::{Gender, Goal, PhysicalActivityLevel},
    get_db_connection,
    model::{
        usecases::{create_update_user, get_user},
        NewUser,
    },
    state::State,
    utils::{create_keyboard, get_user_id},
    HandlerResult, MyDialogue,
};

use super::to_main_functions;

pub async fn profile(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::Profile).await?;

    let conn = &mut get_db_connection();

    let user_profile = match get_user_id(&msg) {
        Some(userid) => {
            match get_user(conn, userid) {
                Some(user) => format!("Профиль: \n\t Пол: {} \n\t Возраст: {} \n\t Рост: {} \n\t Вес: {} \n\t Уровень физической активности: {} \n\t Цель: {}", user.gender, user.age, user.height, user.weight, user.physical_activity_level, user.goal),
                None => "Профиль пуст".to_string()
            }
        },
        None => {
            log::error!("message don't have user id");
            "".to_string()
        }
    };

    bot.send_message(msg.chat.id, user_profile)
        .reply_markup(create_keyboard(2, vec!["Изменить", "Назад"]))
        .await?;
    Ok(())
}

pub async fn profile_parser(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Изменить") => edit_profile(bot, dialogue, msg).await,
        Some("Назад") => to_main_functions(bot, dialogue, msg).await,
        _ => Ok(()),
    }
}

pub async fn edit_profile(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let keyboard = create_keyboard(
        3,
        vec![&Gender::Male.to_string(), &Gender::Female.to_string()],
    );
    bot.send_message(msg.chat.id, "Выберите свой пол")
        .reply_markup(keyboard)
        .await?;
    dialogue.update(State::ReceiveGender).await?;
    Ok(())
}

pub async fn receive_gender(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(gender) => {
            let gen = Gender::from_str(gender);
            match gen {
                Ok(g) => {
                    bot.send_message(msg.chat.id, "Укажите свой возраст")
                        .reply_markup(KeyboardRemove::new())
                        .await?;
                    dialogue.update(State::ReceiveAge { gender: g }).await?;
                }
                Err(_) => {
                    let _ = bot.send_message(msg.chat.id, "Выберите свой пол").await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Ошибка").await?;
        }
    }
    Ok(())
}

pub async fn receive_age(
    bot: Bot,
    dialogue: MyDialogue,
    gender: Gender,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(age) => {
            let age = age.parse::<u16>();
            match age {
                Ok(a) => {
                    bot.send_message(msg.chat.id, "Укажите свой рост в сантиметрах")
                        .await?;
                    dialogue
                        .update(State::ReceiveHeight { gender, age: a })
                        .await?;
                }
                Err(_) => {
                    let _ = bot
                        .send_message(msg.chat.id, "Укажите свой возраст")
                        .await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Ошибка").await?;
        }
    }
    Ok(())
}

pub async fn receive_height(
    bot: Bot,
    dialogue: MyDialogue,
    (gender, age): (Gender, u16),
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(height) => {
            let height = height.parse::<u16>();
            match height {
                Ok(h) => {
                    bot.send_message(msg.chat.id, "Укажите свой вес в килограммах")
                        .await?;
                    dialogue
                        .update(State::ReceiveWeight {
                            gender,
                            age,
                            height: h,
                        })
                        .await?;
                }
                Err(_) => {
                    let _ = bot
                        .send_message(msg.chat.id, "Укажите свой рост в сантиметрах")
                        .await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Ошибка").await?;
        }
    }
    Ok(())
}

pub async fn receive_weight(
    bot: Bot,
    dialogue: MyDialogue,
    (gender, age, height): (Gender, u16, u16),
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(weight) => {
            let weight = weight.parse::<u16>();
            match weight {
                Ok(w) => {
                    let keyboard = create_keyboard(
                        3,
                        vec![
                            &PhysicalActivityLevel::Minimal.to_string(),
                            &PhysicalActivityLevel::Little.to_string(),
                            &PhysicalActivityLevel::Average.to_string(),
                            &PhysicalActivityLevel::AboveAverage.to_string(),
                            &PhysicalActivityLevel::Increased.to_string(),
                            &PhysicalActivityLevel::High.to_string(),
                            &PhysicalActivityLevel::VeryHigh.to_string(),
                        ],
                    );
                    bot.send_message(msg.chat.id, "Выберите уровень физической активности")
                        .reply_markup(keyboard)
                        .await?;
                    dialogue
                        .update(State::ReceivePhysicalActivityLevel {
                            gender,
                            age,
                            height,
                            weight: w,
                        })
                        .await?;
                }
                Err(_) => {
                    let _ = bot
                        .send_message(msg.chat.id, "Укажите свой вес в килограммах")
                        .await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Ошибка").await?;
        }
    }
    Ok(())
}

pub async fn receive_physical_activity_level(
    bot: Bot,
    dialogue: MyDialogue,
    (gender, age, height, weight): (Gender, u16, u16, u16),
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(pal) => {
            let pal = PhysicalActivityLevel::from_str(pal);
            match pal {
                Ok(p) => {
                    let keyboard = create_keyboard(
                        3,
                        vec![
                            &Goal::WeightLoss.to_string(),
                            &Goal::WeightMaintenance.to_string(),
                            &Goal::WeightGain.to_string(),
                        ],
                    );
                    bot.send_message(msg.chat.id, "Выберите цель")
                        .reply_markup(keyboard)
                        .await?;
                    dialogue
                        .update(State::ReceiveGoal {
                            gender,
                            age,
                            height,
                            weight,
                            physical_activity_level: p,
                        })
                        .await?;
                }
                Err(_) => {
                    let _ = bot
                        .send_message(msg.chat.id, "Выберите уровень физической активности")
                        .await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Ошибка").await?;
        }
    }
    Ok(())
}

pub async fn receive_goal(
    bot: Bot,
    dialogue: MyDialogue,
    (gender, age, height, weight, physical_activity_level): (
        Gender,
        u16,
        u16,
        u16,
        PhysicalActivityLevel,
    ),
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(goal) => {
            let goal = Goal::from_str(goal);
            match goal {
                Ok(goal) => {
                    let conn = &mut get_db_connection();

                    match get_user_id(&msg) {
                        Some(userid) => {
                            let usr = NewUser {
                                id: userid,
                                gender: gender.to_string(),
                                age: age as i32,
                                height: height as i32,
                                weight: weight as i32,
                                physical_activity_level: physical_activity_level.to_string(),
                                goal: goal.to_string(),
                            };

                            bot.send_message(
                                msg.chat.id,
                                match create_update_user(conn, usr) {
                                    Ok(_) => "Успех",
                                    Err(_) => "Профиль не сохранен. Попробуйте еще раз",
                                },
                            )
                            .await?;
                        }
                        None => (),
                    }
                    to_main_functions(bot, dialogue, msg).await?;
                }
                Err(_) => {
                    let _ = bot.send_message(msg.chat.id, "Выберите цель").await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Ошибка").await?;
        }
    }
    Ok(())
}
