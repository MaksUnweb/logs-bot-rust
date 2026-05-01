use teloxide::{
    Bot,
    types::CallbackQuery,
    prelude::Requester,
    sugar::bot::BotMessagesExt,
};


//Particular function for output message from callback_handler:
pub async fn send_answer(bot: Bot, q: CallbackQuery, text: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(message) = q.regular_message() {
        bot.edit_text(message, text).await?;
    } else if let Some(ref id) = q.inline_message_id {
        bot.edit_message_text_inline(id, text).await?;
    }

    Ok(())
}
