use crate::domain::profile_domain::{Gender, PhysicalActivityLevel};
use teloxide::utils::command::BotCommands;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,

    Profile,
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

    Diet,
    DietCreate,
    DietEdit,
    DietRemove,
    BuySubscription,
    DietConstructor,
    DietExample,

    PFCFood,
    // PFCFoodMeat,
    // PFCFoodFish,
    // PFCFoodMilk,
    // PFCFoodBread,
    // PFCFoodCerealPasta,
    // PFCFoodFruitVegetableBerry,
    // PFCFoodJuice,
    Notifications,
    NotificationsChooseDiet,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Поддерживаемые команды:")]
pub enum Command {
    #[command(description = "вывести этот текст.")]
    Help,
    #[command(description = "начать чат с ботом.")]
    Start,
    #[command(description = "получить информацио о пользователе.")]
    Profile,
    #[command(description = "рацион питания.")]
    Diet,
    #[command(description = "рассчет дневной нормы КБЖУ")]
    PFC,
    #[command(description = "пример продуктов с текущей нормой КБЖУ")]
    PFCFood,
    #[command(description = "уведомления о приеме пищи.")]
    Notifications,
    #[command(description = "debug command.")]
    Test1,
    Test2,
    Test3,
}
