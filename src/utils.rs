use teloxide::{
    requests::Requester,
    types::{KeyboardButton, KeyboardMarkup, Message, MessageKind, UserId},
    Bot,
};

use crate::{
    domain::{
        diet_domain::KcPFC,
        profile_domain::{Gender, Goal, PhysicalActivityLevel},
    },
    establish_connection,
    model::{
        usecases::{create_update_user, get_user},
        Food, NewUser, User,
    },
    HandlerResult, MyDialogue,
};

pub fn create_keyboard(items_in_row: usize, items: Vec<&str>) -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    for i in items.chunks(items_in_row) {
        let row = i
            .iter()
            .map(|&i| KeyboardButton::new(i.to_owned()))
            .collect();

        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard)
        .resize_keyboard(true)
        .one_time_keyboard(false)
        .input_field_placeholder("Выберите один из вариантов:".to_string())
        .persistent()
}

pub fn get_user_id(msg: &Message) -> Option<String> {
    match &msg.kind {
        MessageKind::Common(msgc) => match &msgc.from {
            Some(usr) => match usr.id {
                UserId(i) => Some(i.to_string()),
            },
            None => None,
        },
        _ => None,
    }
}

pub fn get_kcpfc(usr: &User) -> KcPFC {
    let age = usr.age as f64;
    let height = usr.height as f64;
    let weight = usr.weight as f64;
    let pal = match usr.physical_activity_level.parse::<PhysicalActivityLevel>() {
        Ok(PhysicalActivityLevel::Minimal) => 1.2,
        Ok(PhysicalActivityLevel::Little) => 1.375,
        Ok(PhysicalActivityLevel::Average) => 1.46,
        Ok(PhysicalActivityLevel::AboveAverage) => 1.55,
        Ok(PhysicalActivityLevel::Increased) => 1.64,
        Ok(PhysicalActivityLevel::High) => 1.72,
        Ok(PhysicalActivityLevel::VeryHigh) => 1.9,
        _ => 0.,
    };
    let goal = match usr.goal.parse::<Goal>() {
        Ok(Goal::WeightLoss) => 0.8,
        Ok(Goal::WeightMaintenance) => 1.,
        Ok(Goal::WeightGain) => 1.2,
        _ => 0.,
    };

    let mut kc = 9.99 * weight + 6.25 * height - 4.92 * age;

    match usr.gender.parse::<Gender>() {
        Ok(Gender::Male) => kc += 5.,
        Ok(Gender::Female) => kc -= 161.,
        _ => (),
    };

    kc *= pal * goal;

    KcPFC {
        kcal: kc as u32,
        proteins: (kc * 0.3 / 4.1) as u32,
        fats: (kc * 0.3 / 9.29) as u32,
        carbohydrates: (kc * 0.4 / 4.2) as u32,
    }
}

pub fn get_string_foods(foods: Vec<Food>) -> String {
    let mut str = String::new();

    for f in foods {
        str += &format!(
            "{}: \n\t Ккал: {} \n\t Белки: {}г \n\t Жиры: {}г \n\t Углеводы: {}г \n\n",
            f.name, f.kcal, f.protein, f.fat, f.carbohydrate
        );
    }

    str
}

pub async fn test_func1(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    Ok(())
}

pub async fn test_func2(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let conn = &mut establish_connection();

    let usrid = get_user_id(&msg).unwrap();

    match create_update_user(
        conn,
        NewUser {
            id: usrid,
            gender: String::new(),
            age: 0i32,
            height: 0i32,
            weight: 0i32,
            physical_activity_level: String::new(),
            goal: String::new(),
        },
    ) {
        true => log::info!("user added"),
        false => log::info!("user wasn't add"),
    }

    bot.send_message(msg.chat.id, "Все ок").await?;

    Ok(())
}

pub async fn test_func3(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let conn = &mut establish_connection();

    let usrid = get_user_id(&msg).unwrap();

    match get_user(conn, usrid) {
        Some(u) => {
            bot.send_message(msg.chat.id, format!("{:?}", u)).await?;
        }
        None => log::info!("user not found"),
    };

    Ok(())
}
