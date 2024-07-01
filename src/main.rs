use chrono::{self, Timelike};
use core::time;
use dotenv::dotenv;
use lazy_static::lazy_static;
use model::usecases::get_set_userid_by_time;
use std::{env, time::Duration};
use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
    utils::command::BotCommands,
};

use diesel::r2d2::{Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use diesel::{r2d2::ConnectionManager, Connection};

use crate::{
    main_functions::{
        diet::{
            diet, diet_constructor_parser, diet_create_parser, diet_edit_name_parser,
            diet_edit_parser, diet_parser, diet_remove_parser,
        },
        main_functions_parser,
        notifications::{notifications, notifications_choose_diet_parser, notifications_parser},
        pfc::pfc,
        pfcfood::{pfcfood, pfcfood_parser},
        profile::{
            profile, profile_parser, receive_age, receive_gender, receive_goal, receive_height,
            receive_physical_activity_level, receive_weight,
        },
        start,
    },
    state::{Command, State},
    utils::{test_func1, test_func2, test_func3},
};

pub mod domain;
pub mod main_functions;
pub mod model;
pub mod schema;
pub mod state;
pub mod utils;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type DbConnection = SqliteConnection;

lazy_static! {
    static ref POOL: Pool<ConnectionManager<DbConnection>> = get_connection_pool();
}

pub fn get_connection_pool() -> Pool<ConnectionManager<DbConnection>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<DbConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn create_notification_thread(bot: Bot) {
    tokio::spawn(async move {
        let conn = &mut get_db_connection();
        loop {
            let t = chrono::offset::Local::now();
            let t = format!("{}:{}", t.hour(), t.minute());
            let v = get_set_userid_by_time(conn, &t);
            for userid in v {
                let _ = bot
                    .send_message(ChatId(userid.parse::<i64>().unwrap()), "Время приема пищи")
                    .await;
            }

            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting nutritionist bot");
    let bot = Bot::from_env();

    let _ = create_notification_thread(bot.clone());

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

pub fn get_db_connection() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    POOL.get().unwrap()
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Start].endpoint(start))
        .branch(case![Command::Profile].endpoint(profile))
        .branch(case![Command::Diet].endpoint(diet))
        .branch(case![Command::PFC].endpoint(pfc))
        .branch(case![Command::PFCFood].endpoint(pfcfood))
        .branch(case![Command::Notifications].endpoint(notifications))
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Test1].endpoint(test_func1))
        .branch(case![Command::Test2].endpoint(test_func2))
        .branch(case![Command::Test3].endpoint(test_func3));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::Start].endpoint(main_functions_parser))
        //
        .branch(case![State::Profile].endpoint(profile_parser))
        .branch(case![State::ReceiveGender].endpoint(receive_gender))
        .branch(case![State::ReceiveAge { gender }].endpoint(receive_age))
        .branch(case![State::ReceiveHeight { gender, age }].endpoint(receive_height))
        .branch(
            case![State::ReceiveWeight {
                gender,
                age,
                height
            }]
            .endpoint(receive_weight),
        )
        .branch(
            case![State::ReceivePhysicalActivityLevel {
                gender,
                age,
                height,
                weight
            }]
            .endpoint(receive_physical_activity_level),
        )
        .branch(
            case![State::ReceiveGoal {
                gender,
                age,
                height,
                weight,
                physical_activity_level
            }]
            .endpoint(receive_goal),
        )
        //
        .branch(case![State::Diet].endpoint(diet_parser))
        .branch(case![State::DietCreate].endpoint(diet_create_parser))
        .branch(case![State::DietEdit].endpoint(diet_edit_parser))
        .branch(case![State::DietEditName { name, userid }].endpoint(diet_edit_name_parser))
        .branch(case![State::DietRemove].endpoint(diet_remove_parser))
        .branch(case![State::DietConstructor].endpoint(diet_constructor_parser))
        //
        .branch(case![State::PFCFood].endpoint(pfcfood_parser))
        //
        .branch(case![State::Notifications].endpoint(notifications_parser))
        .branch(case![State::NotificationsChooseDiet].endpoint(notifications_choose_diet_parser))
        //
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query();

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see the usage.",
    )
    .await?;
    Ok(())
}
