use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
    utils::command::BotCommands,
};

pub mod receive_info;
use crate::receive_info::*;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting nutritionist bot");

    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>().branch(
        case![State::Start]
            .branch(case![Command::Help].endpoint(help))
            .branch(case![Command::Start].endpoint(start))
            .branch(case![Command::Portfolio].endpoint(portfolio)),
    );

    let message_handler = Update::filter_message()
        .branch(command_handler)
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
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query().branch(
        case![State::Final {
            gender,
            age,
            height,
            weight,
            physical_activity_level,
            goal
        }]
        .endpoint(portfolio),
    );

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
