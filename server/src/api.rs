use chrono::Utc;
use warp::filters::log::log;
use crate::{library::{addition, Log, ReturnMessage, User}, sql::{connect_db, execute_sql, get_number_of_apis}};
use rand::{distr::Alphanumeric, Rng};

pub async fn validate_user(username: &str) -> Result<ReturnMessage, ReturnMessage>{
    let conn = match connect_db().await {
        Ok(s) => s,
        Err(s) => return Err(ReturnMessage{status_code: 501,
            message: s.to_owned(),
            data: None
            })
    };
    let sql = format!("SELECT * FROM users WHERE username = '{}'", username);
    let rows:Vec<User> = sqlx::query_as(&sql).fetch_all(&conn).await.unwrap();
    
    match rows.is_empty() {
        true => return Err(ReturnMessage {
            status_code: 1,
            message: "Couldn't find user.".to_owned(),
            data: None
        }),
        false => return Ok(ReturnMessage {
            status_code: 100,
            message: "Found a user.".to_owned(),
            data: Some(username.to_owned())
        })
    }
}

fn generate_api_key() -> String {
    let mut rng = rand::thread_rng();
    let api_key: String = (0..32)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    api_key
}

pub async fn create_api_key(username: &str) -> Result<ReturnMessage, ReturnMessage> {
    let _ = match validate_user(username).await {
        Ok(_) => (),
        Err(e) => return Err(e)
    };

    match  get_number_of_apis(username).await {
        Ok(s) => {
            if s > 5 {
                return Err(ReturnMessage {
                    status_code: 102, 
                    message: "Generation exceeds count of 5".to_owned(),
                    data: None
                });
            } 
        },
        Err(s) => return Err(ReturnMessage {
            status_code: 103, 
            message: s.to_owned(),
            data: None
        })
    }

    let new_key = generate_api_key();
    println!("API KEY: {:?}", &new_key);
    let sql = format!("INSERT INTO api_keys (username, key, use_count) VALUES ('{}', '{}', 0)", username, new_key);

    let _ = match execute_sql(&sql).await  {
        Ok(_) => return Ok(ReturnMessage {
            status_code: 100, 
            message: "API key created".to_owned(),
            data: Some(new_key.clone())
        }), 
        Err(_) => return Err(ReturnMessage {
            status_code: 101, 
            message: "Couldn't create API key.".to_owned(),
            data: None
        })
    };
}


pub async fn delete_api_key(username: &str, api_key: &str) -> Result<ReturnMessage, ReturnMessage> {
    let sql = format!("DELETE FROM api_keys WHERE username = '{}' AND key = '{}'", username, api_key);
    let _ = match execute_sql(&sql).await {
        Ok(_) => return  Ok(ReturnMessage {
            status_code: 200,
            message: "API Key Deleted".to_owned(),
            data: None
        }),
        Err(_) => return Err(ReturnMessage {
            status_code: 201,
            message: "Couldn't delete API key.".to_owned(),
            data: None
        })
    };
}


pub async fn perform_api_fn(a: i32, b: i32, username: &str, api_key: &str) -> Result<ReturnMessage, ReturnMessage> {

    let limit: i32 = 50;
    let conn = match connect_db().await {
        Ok(s) => s,
        Err(s) => return Err(ReturnMessage{status_code: 501,
            message: s.to_owned(),
            data: None
            })
    };

    let now = Utc::now();
    let utc_seconds = now.timestamp();

    let count = format!("SELECT COUNT(*) AS log_count FROM log WHERE key = '{}' AND use_time >= {}", api_key, utc_seconds - 3600);
    let log_count: (i64, ) =  sqlx::query_as(&count).fetch_one(&conn).await.unwrap();

    if log_count.0 >= limit as i64 {
        return Err(ReturnMessage { status_code: 1001, message: "RATE LIMIT REACHED.".to_owned(), data: None }) ;
    }

    let result = format!("{}", addition(a, b));

    //updation 
    let update_api_table = format!("UPDATE api_keys SET use_count = use_count + 1 WHERE username = '{}'  AND key = '{}'", username, api_key);
    let update_log_table = format!("INSERT into log (key, use_time) VALUES ('{}', {})", api_key, utc_seconds);

    let _ = execute_sql(&update_api_table).await;
    let _ = execute_sql(&update_log_table).await;

    Ok(ReturnMessage {
        status_code: 1000,
        message: "API Called.".to_owned(),
        data: Some(result)
    })
}