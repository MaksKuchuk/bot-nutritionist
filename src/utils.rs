use std::iter::zip;

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
    get_db_connection,
    model::{
        usecases::{create_update_user, get_random_example_diet, get_user},
        DietExample, Food, NewUser, User, UserDiet,
    },
    HandlerResult, MyDialogue,
};

use itertools::izip;

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

pub fn get_string_diets(diets: Vec<UserDiet>) -> String {
    let mut str = String::from("Рационы питания: \n\n");

    if diets.is_empty() {
        str += "\t\t\tСписок пуст";
    }

    for d in diets {
        str += &format!("\t{}: \n{}\n\n\n", d.name, d.diet);
    }

    str
}

pub fn get_string_diets_pfc(diets: Vec<UserDiet>) -> String {
    let mut s = String::new();

    for d in diets {
        s += &d.name;
        s += ":\n";
        let d = &d.diet;
        let meals = d.trim().split("\n\n");

        let (mut p, mut f, mut c) = (0f64, 0f64, 0f64);

        for meal in meals {
            let mealvec: Vec<&str> = meal.trim().split("\n").collect();

            for food in &mealvec[1..] {
                let v: Vec<&str> = food.trim().split(",").map(|item| item.trim()).collect();

                let weight = v[1].parse::<f64>().unwrap() / 100f64;

                let pfc: Vec<&str> = v[2].trim().split(" ").collect();
                let (tp, tf, tc) = (
                    &pfc[0][1..].parse::<f64>().unwrap(),
                    pfc[1].parse::<f64>().unwrap(),
                    &pfc[2][..(pfc[2].len() - 1)].parse::<f64>().unwrap(),
                );
                p += tp;
                f += tf;
                c += tc;
            }
        }

        let k = p * 4f64 + f * 9f64 + c * 4f64;
        s += &format!(
            "\t Ккал: {:.1} \n\t Белки: {:.1}г \n\t Жиры: {:.1}г \n\t Углеводы: {:.1}г\n\n",
            k, p, f, c
        );
    }

    s
}

pub fn is_diet_right(diet: &str) -> Result<(), String> {
    let meals = diet.trim().split("\n\n");
    for meal in meals {
        let mealvec: Vec<&str> = meal.trim().split("\n").collect();
        if mealvec.is_empty() {
            return Err("Ошибка форматирования".to_string());
        }

        let time = mealvec[0];
        let timevec: Vec<&str> = time.trim().split(":").collect();
        if timevec.len() != 2 {
            return Err("Ошибка форматирования времени".to_string());
        }
        match (timevec[0].parse::<u32>(), timevec[1].parse::<u32>()) {
            (Ok(h), Ok(m)) => {
                if h > 24 || m > 59 {
                    return Err("Ошибка форматирования времени".to_string());
                }
            }
            _ => return Err("Ошибка форматирования времени".to_string()),
        }

        for food in &mealvec[1..] {
            let v: Vec<&str> = food.trim().split(",").map(|item| item.trim()).collect();
            if v.len() != 3 {
                return Err("Ошибка форматирования продукта".to_string());
            }

            if let Err(_) = v[1].parse::<u32>() {
                return Err("Ошибка форматирования продукта (неправильно указан вес)".to_string());
            }

            let pfc: Vec<&str> = v[2].trim().split(" ").collect();
            if pfc.len() != 3 {
                return Err("Ошибка форматирования продукта (неправильно указан БЖУ)".to_string());
            }

            if !pfc[0].starts_with("(") || !pfc[2].ends_with(")") {
                return Err("Ошибка форматирования продукта (неправильно указан БЖУ)".to_string());
            }

            let pfc = vec![&pfc[0][1..], pfc[1], &pfc[2][..(pfc[2].len() - 1)]];
            for item in pfc {
                if let Err(_) = item.parse::<f32>() {
                    return Err(
                        "Ошибка форматирования продукта (неправильно указан БЖУ)".to_string()
                    );
                }
            }
        }
    }
    Ok(())
}

pub fn create_example_diet_string(diet: DietExample, kcpfc: KcPFC) -> String {
    let products: Vec<&str> = diet.products.split("\\").collect();
    let weights: Vec<&str> = diet.weights.split("\\").collect();
    let time = vec!["8:00", "13:00", "19:00"];

    let mut s = String::new();
    s += &format!(
        "Ориентир КБЖУ: \n\t Ккал: {} \n\t Белки: {}г \n\t Жиры: {}г \n\t Углеводы: {}г\n\n\n",
        kcpfc.kcal, kcpfc.proteins, kcpfc.fats, kcpfc.carbohydrates
    );
    s += "Пример рациона для вашей дневной нормы КБЖУ:\n\n";

    for (p, w, t) in izip!(&products, &weights, time) {
        s += &format!("{}\n", t);
        for (pname, wnum) in izip!(p.split(","), w.split(",")) {
            s += &format!(
                "\t\t {} {}гр\n",
                pname.trim(),
                (wnum.trim().parse::<f64>().unwrap() * (kcpfc.kcal as f64) / 1000f64) as u32
            );
        }
        s += "\n";
    }

    s
}

pub async fn test_func1(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let _ = is_diet_right("8:20\nрис, 150, (2.9 25.2 0.4)\nтреска отварная, 50, (17.8 0 0.7)\nбелый хлеб, 50, (11 48 4)\n\n13:00\nгречка на воде, 150, (3.38 19.94 0.62)\nяйцо куриное вареное, 30, (13 1.12 10.61)\n\n19:10\nяблоко красное, 100, (0.4 17 0)\nбанан, 100, (1.2 22 0.2)");
    Ok(())
}

pub async fn test_func2(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let conn = &mut get_db_connection();

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
        Ok(_) => log::info!("user added"),
        Err(_) => log::info!("user wasn't add"),
    }

    bot.send_message(msg.chat.id, "Все ок").await?;

    Ok(())
}

pub async fn test_func3(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let conn = &mut get_db_connection();

    dbg!(get_random_example_diet(conn));

    Ok(())
}
