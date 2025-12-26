use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsersDto {
    pub id: i64,
    pub fullname: String,
    // pub gender: Option<Gender>,
    pub email: String,
    pub create_at: String,
    pub ws_id: i64,
}
