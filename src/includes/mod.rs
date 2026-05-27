pub mod bot_errors;
pub mod callback_handlers;
pub mod chech_allow_user;
pub mod commands_handler;
pub mod select_all_problems;
pub mod select_past_problems;

use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use teloxide::{
    Bot,
    dispatching::dialogue::InMemStorage,
    dispatching::{HandlerExt, UpdateFilterExt},
    dptree,
    prelude::{Dispatcher, Message, ResponseResult, Update},
};

use crate::includes::callback_handlers::callback_handler::callback_handler;
use crate::includes::callback_handlers::past_problems::inline_handler;
use crate::includes::select_all_problems::{State, start_dialogue};

type HandlerResult = Result<(), Box<dyn std::error::Error + Sync + Send>>;

pub async fn start_bot(pool: PgPool) -> ResponseResult<()> {
    dotenv().ok();

    let token = env::var("TELOXIDE_TOKEN").expect("Error taking token!");
    let bot = Bot::new(token);
    let dialogue_storage = InMemStorage::<State>::new();

    let callback_handler_branch =
        dptree::entry().branch(Update::filter_callback_query().endpoint(callback_handler));

    let inline_branch_handler =
        dptree::entry().branch(Update::filter_inline_query().endpoint(inline_handler));

    let dialogue_handler = dptree::entry().branch(
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start])
            .endpoint(start_dialogue),
    );

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<commands_handler::Commands>()
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .endpoint(commands_handler::commands_handler),
        )
        .branch(dialogue_handler)
        .branch(callback_handler_branch)
        .branch(inline_branch_handler);

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![dialogue_storage, pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
