use reactive_stores::Store;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Store)]
pub struct UsersDtoDataStore {
    #[store(key: String = |row| row.id.unwrap().to_string())]
    pub rows: Vec<UsersDto>,
}

#[derive(Debug, Store, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsersDto {
    // pub key: String,
    pub id: Option<i64>,
    pub fullname: String,
    // pub gender: Option<Gender>,
    pub email: String,
    pub create_at: Option<String>,
    pub ws_id: i64,
}

impl UsersDto {
    pub fn new(
        // key: String,
        id: Option<i64>,
        fullname: String,
        email: String,
        create_at: Option<String>,
        ws_id: i64,
    ) -> Self {
        Self {
            // key,
            id,
            fullname,
            email,
            create_at,
            ws_id,
        }
    }
}

impl Default for UsersDto {
    fn default() -> Self {
        Self {
            // key: String::from("-1"),
            id: Some(-1),
            fullname: Default::default(),
            email: Default::default(),
            create_at: Default::default(),
            ws_id: Default::default(),
        }
    }
}
