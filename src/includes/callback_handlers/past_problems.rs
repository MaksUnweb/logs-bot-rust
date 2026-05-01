use teloxide::{
    prelude::Requester,
    Bot,
    types::{InlineQuery, InlineKeyboardMarkup, InlineKeyboardButton, InlineQueryResultArticle, InputMessageContent, InputMessageContentText}
};



pub fn make_keyboard_time_periods() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let time_periods = ["5min 🕜", "15min 🕜", "30min 🕜", "1 hour 🕜", "2 hours 🕜"];

    for time in time_periods.chunks(3) {
        let row = time
            .iter()
            .map(|&time| InlineKeyboardButton::callback(time.to_owned(), time.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}


pub async fn inline_handler(bot: Bot, q: InlineQuery) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let choose_time_period = InlineQueryResultArticle::new(
        "0",
        "Chose a time period: 📅",
        InputMessageContent::Text(InputMessageContentText::new("Time periods:")),
    )
    .reply_markup(make_keyboard_time_periods());

    bot.answer_inline_query(q.id, vec![choose_time_period.into()]).await?;

    Ok(())
}
