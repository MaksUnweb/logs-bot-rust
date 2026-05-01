use sqlx::PgPool;
use teloxide::types::UserId;
use crate::includes::bot_errors::BotErrors;


//function for check matching user with some ID  from allow list
pub async fn check(current_id: UserId, pool: PgPool) -> bool {
  match check_in_db(current_id, pool).await {
    Ok(_) => {
        return  true;
    }
    Err(sqlx::Error::RowNotFound) => {
        return false;
    }
    Err(e) => {
        BotErrors::DbErrorFatal(e);
        return false
    }
  }
  
}

//Reqwest for check is there user with this uid:
async fn check_in_db(uid: UserId, pool: PgPool) -> Result<(), sqlx::error::Error> {
    let uid = uid.0 as i64;
  let _ = sqlx::query("SELECT 1 FROM telegram_admins WHERE uid = $1")
      .bind(uid)
      .fetch_one(&pool)
      .await?;

    Ok(())
}
