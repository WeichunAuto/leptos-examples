use crate::dto::users_dto::UsersDto;
use crate::dto::users_dto_sig::UsersDtoSig;
use leptos::prelude::RwSignal;

#[cfg(feature = "ssr")]
use crate::entity::prelude::*;

#[cfg(feature = "ssr")]
use crate::entity::users;

#[cfg(feature = "ssr")]
use crate::entity::users::ActiveModel;

use leptos::{
    prelude::{expect_context, ServerFnError},
    server,
};

#[cfg(feature = "ssr")]
use sea_orm::{prelude::*, Condition, QueryOrder, Set};

#[cfg(feature = "ssr")]
use crate::state::app_state::AppState;

#[server]
pub async fn get_users_sig() -> Result<Vec<UsersDtoSig>, ServerFnError> {
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

        let users_dto_sig: Vec<_> = users
            .into_iter()
            .map(|user| UsersDtoSig {
                // key: user.id.to_string(),
                id: user.id,
                fullname: RwSignal::new(user.fullname),
                email: RwSignal::new(user.email),
                create_at: Some(
                    user.create_at
                        .unwrap()
                        .to_string()
                        .chars()
                        .take(16)
                        .collect(),
                ),
                ws_id: RwSignal::new(user.ws_id),
            })
            .collect();

        // tracing::info!("query users_dto results: {:?}", users_dto);
        Ok(users_dto_sig)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("get_users should only run on the server");
}

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
                // key: user.id.to_string(),
                id: user.id,
                fullname: user.fullname,
                email: user.email,
                create_at: Some(
                    user.create_at
                        .unwrap()
                        .to_string()
                        .chars()
                        .take(16)
                        .collect(),
                ),
                ws_id: user.ws_id,
            })
            .collect();

        // tracing::info!("query users_dto results: {:?}", users_dto);
        Ok(users_dto)
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("get_users should only run on the server");
}

#[server]
pub async fn delete_users(id: i64) -> Result<bool, ServerFnError> {
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
                    return Ok(true);
                } else {
                    tracing::error!("When delete the user, with id = : {:?} not found", id);
                    // ApiResponse::error(format!("User with id = : {:?} not found", id))
                    return Ok(false);
                }
            }
            Err(e) => {
                tracing::error!("error deleting user: {:?}", e);
                // ApiResponse::error(format!("error deleting user: {:?}", e))
                return Ok(false);
            }
        }
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("delete_users should only run on the server");
}

#[server]
pub async fn add_or_update_users(users_dto: UsersDto) -> Result<UsersDto, ServerFnError> {
    tracing::info!("users_dto: {:?}", users_dto);
    #[cfg(feature = "ssr")]
    {
        let state = expect_context::<AppState>();
        let db = state.db();

        // 更新
        if users_dto.id > -1 {
            let rt = users::Entity::update(users::ActiveModel {
                id: Set(users_dto.id),
                fullname: Set(users_dto.fullname),
                email: Set(users_dto.email),
                ws_id: Set(users_dto.ws_id as i64), 
                ..Default::default()
            })
            .exec(db)
            .await;

            match rt {
                Ok(user) => {
                    tracing::info!(
                        "user updated successfully with id = : {:?}, name = : {:?}",
                        user.id,
                        user.fullname
                    );
                    Ok(UsersDto::new(
                        user.id,
                        user.fullname,
                        user.email,
                        None,
                        user.ws_id,
                    ))
                }
                Err(DbErr::RecordNotUpdated) => {
                    tracing::error!("User id: {} not found", users_dto.id);
                    Ok(UsersDto::default())
                }
                Err(e) => {
                    tracing::error!("error updating user: {:?}", e);
                    Ok(UsersDto::default())
                }
            }
        }
        // 新增
        else {
            let new_user = ActiveModel {
                fullname: Set(users_dto.fullname),
                email: Set(users_dto.email),
                password_hash: Set("test".to_string()),
                ws_id: Set(users_dto.ws_id),
                ..Default::default()
            };

            let rt = new_user.insert(db).await;

            match rt {
                Ok(user) => {
                    tracing::info!(
                        "user created successfully with id = : {:?} and name = : {:?}",
                        user.id,
                        user.fullname
                    );
                    Ok(UsersDto::new(
                        user.id,
                        user.fullname,
                        user.email,
                        Some(user.create_at.unwrap().to_string()),
                        user.ws_id,
                    ))
                }
                Err(e) => {
                    tracing::error!("error creating user: {:?}", e);
                    // ApiResponse::error(format!("error creating user: {:?}", e))
                    Ok(UsersDto::default())
                }
            }
        }
    }

    #[cfg(not(feature = "ssr"))]
    unreachable!("delete_users should only run on the server");
}
