use std::fmt::format;

use sqlx::{sqlite, Executor, Pool, Sqlite};

pub async fn connect_db() -> Result<Pool<Sqlite>, &'static str> {
    let opt = sqlite::SqliteConnectOptions::new().filename("data.db").create_if_missing(true);
    let connection = sqlite::SqlitePool::connect_with(opt).await;
    match connection {
        Ok(some) => Ok(some),
        Err(_) => Err("Couldn't connect to database.")
    }
}

pub async fn create_user_table() -> Result<(), &'static str> {
    let conn = match connect_db().await {
        Ok(s) => s,
        Err(s) => return Err(s)
    };

    let sql = "CREATE TABLE IF NOT EXISTS users (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        username TEXT NOT NULL
                    )";
    let _ = match conn.execute(sql).await {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Couldn't create users table. ")
    };
}

pub async fn create_api_table() -> Result<(), &'static str> {
    let conn = match connect_db().await {
        Ok(s) => s,
        Err(s) => return Err(s)
    };

    let sql = "CREATE TABLE IF NOT EXISTS api_keys ( id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL, key TEXT NOT NULL, use_count INTEGER NOT NULL )";
    let _ = match conn.execute(sql).await {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Couldn't create api table. ")
    };
}


pub async fn create_log_table() -> Result<(), &'static str> {
    let conn = match connect_db().await {
        Ok(s) => s,
        Err(s) => return Err(s)
    };

    let sql = "CREATE TABLE IF NOT EXISTS log (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        key TEXT NOT NULL,
        use_time INTEGER NOT NULL
    )";
    let _ = match conn.execute(sql).await {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Couldn't create api table. ")
    };
}



pub async fn execute_sql(sql: &str) -> Result<(), &'static str> {
    let conn = match connect_db().await {
        Ok(s) => s,
        Err(s) => return Err(s)
    };

    let _ = match conn.execute(sql).await {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Couldn't execute sql.")
    };
}


pub async fn add_user(username: &str) -> Result<(), &'static str> {
    let sql = format!("INSERT INTO USERS (username) VALUES ('{}')", username);
    let _ = match execute_sql(&sql).await {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Couldn't store user.")
    };
}


pub async fn get_number_of_apis(username: &str) -> Result<i32, &'static str> {
    let conn = match connect_db().await {
        Ok(s) => s,
        Err(s) => return Err(s)
    };
    let query = format!("SELECT COUNT(*) AS count FROM api_keys WHERE username='{}'", username);
    let count: (i32, ) = sqlx::query_as(&query).fetch_one(&conn).await.unwrap();
    Ok(count.0)
}



