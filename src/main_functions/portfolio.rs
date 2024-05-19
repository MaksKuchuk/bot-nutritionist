use std::str::FromStr;

use teloxide::{
    payloads::SendMessageSetters,
    requests::Requester,
    types::{KeyboardRemove, Message},
    Bot,
};

use crate::{
    domain::profile_domain::{Gender, Goal, PhysicalActivityLevel},
    state::State,
    utils::create_keyboard,
    HandlerResult, MyDialogue,
};

pub async fn edit_portfolio(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let keyboard = create_keyboard(
        3,
        vec![
            &Gender::Male.to_string(),
            &Gender::Female.to_string(),
            &Gender::RyanGosling.to_string(),
        ],
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
            bot.send_message(msg.chat.id, "Ошибка.").await?;
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
            bot.send_message(msg.chat.id, "Ошибка.").await?;
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
            bot.send_message(msg.chat.id, "Ошибка.").await?;
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
                            &PhysicalActivityLevel::Low.to_string(),
                            &PhysicalActivityLevel::Moderate.to_string(),
                            &PhysicalActivityLevel::High.to_string(),
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
            bot.send_message(msg.chat.id, "Ошибка.").await?;
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
                            &Goal::WeightMaintenance.to_string(),
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
            bot.send_message(msg.chat.id, "Ошибка.").await?;
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
                Ok(_) => {
                    bot.send_message(msg.chat.id, "Успех")
                        .reply_markup(KeyboardRemove::new())
                        .await?;
                    dialogue.update(State::Start).await?;
                }
                Err(_) => {
                    let _ = bot.send_message(msg.chat.id, "Выберите цель").await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Ошибка.").await?;
        }
    }
    Ok(())
}

pub async fn portfolio(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    edit_portfolio(bot, _dialogue, msg).await?;
    // bot.send_message(msg.chat.id, "Портфолио").await?;
    Ok(())
}
