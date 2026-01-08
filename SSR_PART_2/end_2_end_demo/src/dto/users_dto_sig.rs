use leptos::prelude::RwSignal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsersDtoSig {
    // pub key: String,
    pub id: i64,
    pub fullname: RwSignal<String>,
    // pub gender: Option<Gender>,
    pub email: RwSignal<String>,
    pub create_at: Option<String>,
    pub ws_id: RwSignal<i64>,
}

impl UsersDtoSig {
    pub fn new(
        // key: String,
        id: i64,
        fullname: RwSignal<String>,
        email: RwSignal<String>,
        create_at: Option<String>,
        ws_id: RwSignal<i64>,
    ) -> Self {
        Self {
            id,
            fullname,
            email,
            create_at,
            ws_id,
        }
    }
}   

impl Default for UsersDtoSig {
    fn default() -> Self {
        Self {
            // key: String::from("-1"),
            id: -1,
            fullname: Default::default(),
            email: Default::default(),
            create_at: Default::default(),
            ws_id: Default::default(),
        }
    }
}
