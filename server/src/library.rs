pub struct User {
    pub index: Option<i32>,
    pub username: String
}

pub struct Api_Key {
    pub index: Option<i32>,
    pub owner: String,
    pub key: String,
    pub use_count: i32
}

pub struct Log {
    pub index: Option<i32>,
    pub key: String,
    pub use_time: i64
}

