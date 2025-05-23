use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use warp::{reject::{Reject, Rejection}, reply::Reply};


#[derive(Debug, FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub username: String
}

#[derive(Debug, FromRow)]
pub struct Api_Key {
    pub id: Option<i32>,
    pub owner: String,
    pub key: String,
    pub use_count: i32
}

#[derive(Debug, FromRow)]
pub struct Log {
    pub id: Option<i32>,
    pub key: String,
    pub use_time: i64
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnMessage{
    pub status_code: u16,
    pub message: String,
    pub data: Option<String>,
}

impl Reject for ReturnMessage {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let return_message = if let Some(custom_error) = err.find::<ReturnMessage>() {
        custom_error
    } else {
        & ReturnMessage {
            status_code: 500,
            message: "Internal server error.".to_owned(),
            data: None
        }
    };

    Ok(warp::reply::json(&return_message))
}


// API FUNCTION
pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}




