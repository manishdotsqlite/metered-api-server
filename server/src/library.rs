use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use warp::{reject::{Reject, Rejection}, reply::Reply};
use warp::reply::json;
use warp::http::StatusCode;


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
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if let Some(return_message) = err.find::<ReturnMessage>() {
        // Handle custom ReturnMessage error
        Ok(warp::reply::with_status(json(return_message), StatusCode::BAD_REQUEST))
    } else if err.is_not_found() {
        let msg = ReturnMessage {
            status_code: StatusCode::NOT_FOUND.as_u16(),
            message: "Not Found".to_string(),
            data: None,
        };
        Ok(warp::reply::with_status(json(&msg), StatusCode::NOT_FOUND))
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        let msg = ReturnMessage {
            status_code: StatusCode::METHOD_NOT_ALLOWED.as_u16(),
            message: "Method Not Allowed".to_string(),
            data: None,
        };
        Ok(warp::reply::with_status(json(&msg), StatusCode::METHOD_NOT_ALLOWED))
    } else {
        let msg = ReturnMessage {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "Internal Server Error".to_string(),
            data: None,
        };
        Ok(warp::reply::with_status(json(&msg), StatusCode::INTERNAL_SERVER_ERROR))
    }
}



// API FUNCTION
pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}




