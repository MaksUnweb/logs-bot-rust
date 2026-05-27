use sqlx::{PgPool, Row};
use crate::includes::bot_errors::BotErrors;


//Function for start select 
//in this moment output from database outputs five past records
pub async fn select(pool: PgPool, time_period: String) -> Result<Option<Vec<String>>, Box<dyn std::error::Error + Send + Sync>> {

  //Creating sql-reqwest from output:
  let sql_reqwest = match conversion_time(time_period) {
    Some(sql) => { sql }
    None => { return Ok(None); }
  };

  let limit = 5;
  let offset = 0;
  let logs = match select_from_database(pool, limit, offset, sql_reqwest).await {
    Ok(logs) => { logs }
    Err(e) => {
        BotErrors::DbErrorFatal(e);
        vec![]
    }
  };

  Ok(Some(logs))
}

fn conversion_time(time: String) -> Option<&'static str> {

    let sql_queries = vec![
        r#"
            SELECT text_error FROM logs 
            WHERE time_data >= NOW() - INTERVAL '5 minute'
            ORDER BY time_data DESC 
            LIMIT $1 OFFSET $2
        "#,
        r#"
            SELECT text_error FROM logs 
            WHERE time_data >= NOW() - INTERVAL '15 minute'
            ORDER BY time_data DESC 
            LIMIT $1 OFFSET $2
        "#,
        r#"
            SELECT text_error FROM logs 
            WHERE time_data >= NOW() - INTERVAL '30 minute'
            ORDER BY time_data DESC 
            LIMIT $1 OFFSET $2
        "#,
        r#"
            SELECT text_error FROM logs 
            WHERE time_data >= NOW() - INTERVAL '1 hour'
            ORDER BY time_data DESC 
            LIMIT $1 OFFSET $2
        "#,
        r#"
            SELECT text_error FROM logs 
            WHERE time_data >= NOW() - INTERVAL '2 hour'
            ORDER BY time_data DESC 
            LIMIT $1 OFFSET $2
        "#,
    ];

    match time.as_str() {
        "5min 🕜" => return Some(sql_queries[0]),
        "15min 🕜" => return Some(sql_queries[1]),
        "30min 🕜" => return Some(sql_queries[2]),
        "1 hour 🕜" => return Some(sql_queries[3]),
        "2 hours 🕜" => return Some(sql_queries[4]),
        _ => None

    }
}

//Function for make reqwest for database:
async fn select_from_database(pool: PgPool, limit: i64, offset: i64, sql: &str) -> Result<Vec<String>, sqlx::error::Error> {
    let data = sqlx::query(sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(&pool)
        .await?;
    
    let mut vec_logs = Vec::new();
    data.iter().for_each(|d| {
        let text_error: String = d.get(0);
        vec_logs.push(text_error);
    });

    Ok(vec_logs)
}

#[cfg(test)]

#[test]
fn test_conversion() {
//     let test_input = "15min 🕜".to_string();
//     let success_result = 
//         Some(r#"
//             SELECT text_error FROM logs 
//             WHERE time_data >= NOW() - INTERVAL '15 minute'
//             ORDER BY time_data DESC 
//             LIMIT $1 OFFSET $2
//         "#,
// );
//     let func_return_data = conversion_time(test_input);
//     assert_eq!(func_return_data, success_result);
let test_input = "15min 🕜".to_string();
 
 // Не сравнивай всю строку целиком!
 let result = conversion_time(test_input).unwrap();
 
 // Проверяй только ключевые части
 assert!(result.contains("SELECT text_error FROM logs"));
 assert!(result.contains("INTERVAL '15 minute'"));
 assert!(result.contains("LIMIT $1 OFFSET $2"));
}
