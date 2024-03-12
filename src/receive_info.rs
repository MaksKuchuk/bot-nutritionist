use std::str::FromStr;

use teloxide::{prelude::*, types::KeyboardRemove, utils::command::BotCommands};

use crate::{utils::create_keyboard, HandlerResult, MyDialogue};

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveGender,
    ReceiveAge {
        gender: Gender,
    },
    ReceiveHeight {
        gender: Gender,
        age: u16,
    },
    ReceiveWeight {
        gender: Gender,
        age: u16,
        height: u16,
    },
    ReceivePhysicalActivityLevel {
        gender: Gender,
        age: u16,
        height: u16,
        weight: u16,
    },
    ReceiveGoal {
        gender: Gender,
        age: u16,
        height: u16,
        weight: u16,
        physical_activity_level: PhysicalActivityLevel,
    },
    Final {
        gender: Gender,
        age: u16,
        height: u16,
        weight: u16,
        physical_activity_level: PhysicalActivityLevel,
        goal: Goal,
    },
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Поддерживаемые команды:")]
pub enum Command {
    #[command(description = "вывести этот текст.")]
    Help,
    #[command(description = "начать чат с ботом.")]
    Start,
    #[command(description = "получить информацио о пользователе.")]
    Portfolio,
    #[command(description = "debug command.")]
    Test,
}

pub async fn test_func(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let keyboard = create_keyboard(
        2,
        vec![
            &Gender::Male.to_string(),
            &Gender::Female.to_string(),
            &Gender::RyanGosling.to_string(),
        ],
    );

    bot.send_message(msg.chat.id, "lalala.")
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

#[derive(Clone, Default)]
pub enum Gender {
    #[default]
    Male,
    Female,
    RyanGosling,
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        let string_literal = match self {
            Gender::Male => "Мужской",
            Gender::Female => "Женский",
            Gender::RyanGosling => "Раян Гослинг",
        };
        string_literal.to_owned()
    }
}

impl FromStr for Gender {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Мужской" => Ok(Gender::Male),
            "Женский" => Ok(Gender::Female),
            "Раян Гослинг" => Ok(Gender::RyanGosling),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Default)]
pub enum PhysicalActivityLevel {
    #[default]
    Low,
    Moderate,
    High,
}

impl ToString for PhysicalActivityLevel {
    fn to_string(&self) -> String {
        let string_literal = match self {
            PhysicalActivityLevel::Low => "Низкий",
            PhysicalActivityLevel::Moderate => "Средний",
            PhysicalActivityLevel::High => "Высокий",
        };
        string_literal.to_owned()
    }
}

impl FromStr for PhysicalActivityLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Низкий" => Ok(PhysicalActivityLevel::Low),
            "Средний" => Ok(PhysicalActivityLevel::Moderate),
            "Высокий" => Ok(PhysicalActivityLevel::High),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Default)]
pub enum Goal {
    #[default]
    WeightLoss,
    WeightMaintenance,
    WeightGain,
}

impl ToString for Goal {
    fn to_string(&self) -> String {
        let string_literal = match self {
            Goal::WeightLoss => "Похудение",
            Goal::WeightMaintenance => "Поддержание веса",
            Goal::WeightGain => "Массанабор",
        };
        string_literal.to_owned()
    }
}

impl FromStr for Goal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Похудение" => Ok(Goal::WeightLoss),
            "Поддержание веса" => Ok(Goal::WeightMaintenance),
            "Массанабор" => Ok(Goal::WeightGain),
            _ => Err(()),
        }
    }
}

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
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
                Ok(g) => {
                    bot.send_message(msg.chat.id, "Успех")
                        .reply_markup(KeyboardRemove::new())
                        .await?;
                    dialogue
                        .update(State::Final {
                            gender,
                            age,
                            height,
                            weight,
                            physical_activity_level,
                            goal: g,
                        })
                        .await?;
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
    bot.send_message(msg.chat.id, "Портфолио").await?;
    Ok(())
}
