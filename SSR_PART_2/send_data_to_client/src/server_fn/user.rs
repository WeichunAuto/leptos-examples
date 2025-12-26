use crate::dto::users_dto::UsersDto;

#[cfg(feature = "ssr")]
use crate::entity::prelude::*;

#[cfg(feature = "ssr")]
use crate::entity::users;
use leptos::{
    prelude::{expect_context, ServerFnError},
    server,
};

#[cfg(feature = "ssr")]
use sea_orm::{prelude::*, Condition, QueryOrder};

#[cfg(feature = "ssr")]
use crate::state::app_state::AppState;

#[server]
pub async fn get_users() -> Result<Vec<UsersDto>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let state = expect_context::<AppState>();
        let db = state.db();

        let conditions = Condition::all();
        let users = Users::find()
            .filter(conditions)
            .order_by_desc(users::Column::CreateAt)
            .all(db)
            .await
            .unwrap();
        // tracing::info!("query users results: {:?}", users);
        // ApiResponse::success("success", Some(users))

        let users_dto: Vec<_> = users
            .into_iter()
            .map(|user| UsersDto {
                id: user.id,
                fullname: user.fullname,
                email: user.email,
                create_at: user
                    .create_at
                    .unwrap()
                    .to_string()
                    .chars()
                    .take(16)
                    .collect(),
                ws_id: user.ws_id,
            })
            .collect();

        tracing::info!("query users_dto results: {:?}", users_dto);
        Ok(users_dto)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("get_users should only run on the server");
}

#[server]
pub async fn delete_users(id: i64) -> Result<bool, ServerFnError>{
    #[cfg(feature = "ssr")]
    {
        let state = expect_context::<AppState>();
        let db = state.db();
        let rt = users::Entity::delete_by_id(id as i64).exec(db).await;

        match rt {
            Ok(deleted_user) => {
                if deleted_user.rows_affected > 0 {
                    tracing::info!("User was deleted successfully with id = : {:?}!", id);
                    // ApiResponse::success("User was deleted successfully!", None)
                    return Ok(true)
                } else {
                    tracing::error!("When delete the user, with id = : {:?} not found", id);
                    // ApiResponse::error(format!("User with id = : {:?} not found", id))
                    return Ok(false)
                }
            }
            Err(e) => {
                tracing::error!("error deleting user: {:?}", e);
                // ApiResponse::error(format!("error deleting user: {:?}", e))
                return Ok(false)
            }
        }
        
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("delete_users should only run on the server");
}