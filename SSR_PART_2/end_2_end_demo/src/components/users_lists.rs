use leptos::prelude::*;
use reactive_stores::{Store, StoreFieldIterator};

use crate::components::users_form::UsersForm;
use crate::dto::users_dto::{UsersDto, UsersDtoDataStoreStoreFields, UsersDtoStoreFields};
use crate::server_fn::user::get_users;
use crate::{dto::users_dto::UsersDtoDataStore, server_fn::user::delete_users};

#[component]
pub fn UsersList() -> impl IntoView {
    let async_users = Resource::new(|| (), |_| get_users());
    let users_dto_store = Store::new(UsersDtoDataStore { rows: vec![] });

    let (selected_line, set_selected_line) = signal::<Option<UsersDto>>(None);

    Effect::new(move || {
        leptos::logging::log!("users lists run.");

        if let Some(Ok(users)) = async_users.get() {
            // 模式解构
            users_dto_store.set(UsersDtoDataStore { rows: users })
        };
    });

    // 删除用户 by id
    let delete_user_action = Action::new(move |id: &i64| {
        let id = *id;
        async move {
            let rt = delete_users(id).await;
            match rt {
                Ok(is_success) => {
                    if is_success {
                        leptos::logging::log!("delete is successful. now update the view");
                        // let current_data = users.get().unwrap_or_default();
                        // let filter_data: Vec<_> = current_data
                        //     .into_iter()
                        //     .filter(|user| user.id != id)
                        //     .collect();
                        // set_users.set(Some(filter_data));
                        leptos::logging::log!("view updated");
                    } else {
                        leptos::logging::log!("delete not successful.");
                    }
                }
                Err(_) => {
                    leptos::logging::log!("error happened");
                }
            }
        }
    });

    // 更新
    let update_user_store = Callback::new(move |user_dto: UsersDto| {
        leptos::logging::log!("start update users store. {:?}", user_dto);

        for row_signal in users_dto_store.rows().iter_unkeyed() {
            // let fullname_signal = row_signal.fullname();
            // let row_id = row_signal.id().get();
            if row_signal.id().get().unwrap() == user_dto.id.unwrap() {
                // *fullname_signal.write() = user_dto.fullname.clone();
                // let rw = row_signal.write();
                let mut row = row_signal.write();
                row.fullname = user_dto.fullname.clone();
                leptos::logging::log!("if inside: user_dto.id.unwrap() = {}", user_dto.id.unwrap());
            }
        }
        // users_dto_store.update(|store| {
        //     if let Some(row) = store
        //         .rows
        //         .iter_mut()
        //         .find(|r| r.id.unwrap() == user_dto.id.unwrap())
        //     {
        //         row.fullname = user_dto.fullname.clone();
        //         row.email = user_dto.email.clone();
        //         row.ws_id = user_dto.ws_id;
        //         leptos::logging::log!("updated!");
        //     }
        // });
    });

    view! {
        <Suspense
                fallback = move || {view! {<p>"Load...."</p>}}
            >
            {
                move || {
                    view! {
                        <div>
                            <table>
                                <thead>
                                    <tr>
                                    <th>"ID"</th>
                                    <th>"fullname"</th>
                                    <th>"email"</th>
                                    <th>"create_at"</th>
                                    <th>"ws_id"</th>
                                    <th>"操作"</th>
                                    </tr>
                                </thead>

                                <tbody>

                                    <ForEnumerate
                                        each = move || users_dto_store.rows()
                                        key = |row_signal| row_signal.read().id
                                        children = move |_, row_signal| {
                                            let users_dto = row_signal.read();
                                            view! {
                                                <tr on:click=move |_| set_selected_line.set(Some(UsersDto {id: users_dto.id, fullname: users_dto.fullname.clone(), email: users_dto.email.clone(), create_at: users_dto.create_at.clone(), ws_id: users_dto.ws_id }))>
                                                    <th>{users_dto.id}</th>
                                                    <th>{users_dto.fullname.clone()}</th>
                                                    <th>{users_dto.email.clone()}</th>
                                                    <th>{users_dto.create_at.clone()}</th>
                                                    <th>{users_dto.ws_id}</th>
                                                    <th>
                                                        <button on:click= move |ev| {
                                                                ev.stop_propagation();
                                                                delete_user_action.dispatch(row_signal.read().id.unwrap());
                                                            }>
                                                                "delete"
                                                        </button>
                                                    </th>
                                                </tr>
                                            }
                                        }
                                    />
                                </tbody>
                            </table>
                        </div>
                    }
                }
            }
        </Suspense>


        <Show
            when= move|| selected_line.get().is_some()
            fallback= move || {view! {<span></span>}}
        >
        <div class="users-form">
            <div class="close"
                on:click=move |_| set_selected_line.set(None)
            >"关闭"</div>
            <div class="form">
                <UsersForm
                    users=selected_line.get().unwrap_or_default()
                    callback = update_user_store

                />
            </div>
        </div>
        </Show>
    }
}
