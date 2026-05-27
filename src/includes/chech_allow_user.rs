use sqlx::{Executor, PgPool, Postgres};
use teloxide::types::UserId;
use crate::includes::bot_errors::BotErrors;


//function for check matching user with some ID  from allow list
pub async fn check(current_id: UserId, pool: PgPool) -> bool {
  match check_in_db(current_id, &pool).await {
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
async fn check_in_db<'e, E>(uid: UserId, executor: E) -> Result<(), sqlx::error::Error>
where E: Executor<'e, Database = Postgres>
{
    let uid = uid.0 as i64;
  let _ = sqlx::query("SELECT 1 FROM telegram_admins WHERE uid = $1")
      .bind(uid)
      .fetch_one(executor)
      .await?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use sqlx::{Executor, Postgres, postgres::PgPoolOptions};
use teloxide::types::UserId;

use crate::includes::chech_allow_user::check_in_db;


    #[tokio::test]
    async fn test_check_in_db() {
       let pool = PgPoolOptions::new() 
           .max_connections(5) 
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect("postgres://Admin:1111@127.0.0.1/logs_db?connect_timeout=3").await.unwrap();
       let mut transaction = pool.begin().await.expect("Error make transaction!");
       let test_uid = 11111;
       let user_id = UserId(test_uid);
       let test_login = "TEST_LOGIN";
       insert_test_data(&mut *transaction, test_uid as i64, test_login).await.expect("Error Inserting test data!");
       
       //Check output:
       match check_in_db(user_id, &mut *transaction).await {
           Ok(_) => {
               
           }
           Err(sqlx::Error::RowNotFound) => {
               panic!("Error! Test not completed! Row Not Found!");
           }
           Err(e) => {
               panic!("{}", e);
           }
       }

       transaction.rollback().await.expect("Error! transaction couldt rollback!");
    }
    
    async fn insert_test_data<'e, E>(executor: E, uid: i64, login: &str) -> Result<(), sqlx::error::Error> 
    where E: Executor<'e, Database = Postgres>
    {
        let _sql = sqlx::query("INSERT INTO telegram_admins (uid, login) VALUES ($1, $2)") 
            .bind(&uid)
            .bind(login)
            .execute(executor)
            .await?;
        Ok(())
    }
}
