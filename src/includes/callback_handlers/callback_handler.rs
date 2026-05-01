use teloxide::{
    Bot,
    types::CallbackQuery,
    prelude::Requester,
    dispatching::dialogue::GetChatId

};
use sqlx::PgPool;
use crate::includes::select_past_problems::select;
use crate::includes::callback_handlers::send_answer::send_answer;


pub async fn callback_handler(bot: Bot, q: CallbackQuery, pool: PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

  if let Some(ref time) = q.data {

        bot.answer_callback_query(q.id.clone()).await?;

        //Get past problems logs based on the time:
        let logs = match select(pool, time.clone()).await {
            Ok(Some(logs)) => { logs }
            Ok(None) => { 
                send_answer(bot.clone(), q.clone(), "Error! Your time period incorrect! ❌".to_string()).await?;
                return Ok(());
            }
            Err(e) => {
                send_answer(bot.clone(), q.clone(), "Error! In this moment service not available! ❌".to_string()).await?;
                return Err(e);
            }
        };

        let len = logs.iter().len();
        if len > 0 {
            for log in logs {
                let text = format!("{}", log);
                bot.send_message(q.chat_id().unwrap(), text).await?;
            }
        }else{
            send_answer(bot.clone(), q.clone(), "No errors have been found recently! ✅".to_string()).await?;
        }
    }

    Ok(())
}
