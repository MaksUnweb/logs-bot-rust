use teloxide::{
    Bot, dispatching::dialogue::InMemStorage, 
    prelude::{Dialogue, Requester},
    types::Message
};
use sqlx::{PgPool, Row};

#[derive(Clone)]
struct Paginate(i64);

#[derive(Clone, Default)]
pub enum State {
   #[default] 
    Start
}

//Function for start dialogue:
pub async fn start_dialogue(bot: Bot, msg: Message, dialogue: Dialogue<State, InMemStorage<State>>, pool: PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(user_input) = msg.text() {

        //Make "offset" from pagination:
       let mut offset = Paginate(0);
       match user_input {
           "Ok" => {
                 bot.send_message(msg.chat.id, "Selecting logs...").await?;
                    let logs: Option<Vec<String>> = output_from_database(pool.clone(), offset.clone()).await?;
                    if let Some(logs) = logs {
                    for log in logs {
                        let format_data = format!("{} ❗", log);
                        bot.send_message(msg.chat.id, format_data).await?;
                    }
                    } else {
                        bot.send_message(msg.chat.id, "There are no errors in the logs ✅").await?;
                    }
                    bot.send_message(msg.chat.id, "If you want to see more, type \"More\", if you want to exit, type \"Exit\"").await?;
                 }
            "More" => {
                 offset.0 += 5;
                let logs: Option<Vec<String>> = output_from_database(pool.clone(), offset.clone()).await?;
                if let Some(logs) = logs {
                for log in logs {
                    bot.send_message(msg.chat.id, log).await?;
                }
                } else {
                    bot.send_message(msg.chat.id, "There are no errors in the logs").await?;
                }
                bot.send_message(msg.chat.id, "If you need to see more logs, write \"More\"").await?;

            }
            "Exit" => {
                 bot.send_message(msg.chat.id, "The conclusion is complete, goodbye ✋").await?;
                 dialogue.exit().await?;
            }
            _ =>  {}
       }
    }

    Ok(())
}


//Function for outputs problems with offset: 
async fn output_from_database(pool: PgPool, offset: Paginate) -> Result<Option<Vec<String>>, sqlx::error::Error> {
    let rows = sqlx::query(r#"
            SELECT text_error FROM logs ORDER BY time_data DESC LIMIT $1 OFFSET $2
        "#)    
        .bind(5)
        .bind(offset.0)
        .fetch_all(&pool)
        .await?;
    
    let mut vec_data: Vec<String> = Vec::new();

    if rows.iter().len() > 0 {
        rows.iter().for_each(|row| { 
            vec_data.push(row.get("text_error"));
        });
        return Ok(Some(vec_data))
    }else{
        return Ok(None)
    }
}

