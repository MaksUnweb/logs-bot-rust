use sqlx::PgPool;
use teloxide::{
  dispatching::dialogue::{InMemStorage}, payloads::SendMessageSetters, prelude::{Bot, Dialogue, Message, Requester }, types::UserId, utils::command::BotCommands
};

use crate::includes::HandlerResult;
use crate::includes::chech_allow_user::check;
use crate::includes::callback_handlers::past_problems::make_keyboard_time_periods;
use crate::includes::select_all_problems::{State};


//Доступные команды для бота:
#[derive(BotCommands, Clone)]
  #[command(rename_rule = "snake_case", description = "Available Commands:")]
pub enum Commands {
    #[command(description = "Start command")]
    Start,
   #[command(description = "Output of all commands")] 
   Help,
   #[command(description = "Who is?")]
   GetUser,
   #[command(description = "Get past problems")]
   GetPastProblems,
   #[command(description = "Get all problems with pagination")]
   AllProblems
}

pub async fn commands_handler(bot: Bot, msg: Message, cmd: Commands, pool: PgPool, dialogue: Dialogue<State, InMemStorage<State>>) -> HandlerResult {
  let user_id: UserId;
  if let Some(user) = &msg.from {
    user_id = user.id;
  }else{
    bot.send_message(msg.chat.id, "Error retrieving data! Try again later!").await?;
    user_id = UserId(0);
  }
  
  //Проверяем является ли пользователь разрешённым:
  if !check(user_id, pool.clone()).await {
    bot.send_message(msg.chat.id, "Access denied!").await?;
    return Ok(());
  }
  
  match cmd {
      Commands::Start => {
          let message = format!("Welcome to the log tracking bot 😊\n This bot is an add-on to the web interface and log collection program.\n{}
              ", Commands::descriptions().to_string());
        bot.send_message(msg.chat.id, message).await?;
      }
    Commands::Help => {
      bot.send_message(msg.chat.id, Commands::descriptions().to_string()).await?;
    }
    Commands::GetUser => {
      bot.send_message(msg.chat.id, format!("This your data: {:?}", msg.from)).await?;
    }
    Commands::GetPastProblems => {
        let keyboard = make_keyboard_time_periods();
        bot.send_message(msg.chat.id, "Time periods: 📅").reply_markup(keyboard).await?;
    }
    Commands::AllProblems=> {
        bot.send_message(msg.chat.id, "Here is a list of problematic logs").await?;
        bot.send_message(msg.chat.id, "If you want to continue, type \"Ok\"").await?;
        dialogue.update(State::Start).await?;
    }
  }
  Ok(())
}
