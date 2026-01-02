use leptos::logging::log;
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
        let target_id = user_dto.id.unwrap();

        for row_signal in users_dto_store.rows().iter_unkeyed() {
            let name_signal = row_signal.fullname();
            let current_name = name_signal.get();

            if row_signal.id().get().unwrap() == target_id {
                name_signal.set(user_dto.fullname.clone());
                log!("updated.");
            } else {
                name_signal.set(current_name);
            }
        }
    });

    view! {
        <Suspense
                fallback = move || {view! {<p>"Load...."</p>}}
            >

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
                                        key = |row_signal| row_signal.read().id.unwrap()
                                        children = move |_, row_signal| {

                                            // 为点击事件克隆整个 row_signal
                                            let row_clone = row_signal.clone();

                                            view! {
                                                <tr on:click=move |_| {
                                                    // 当前行对象
                                                    let current_user = row_clone.read();

                                                    set_selected_line.set(Some(UsersDto {
                                                        id: current_user.id,
                                                        fullname: current_user.fullname.clone(),
                                                        email: current_user.email.clone(),
                                                        create_at: current_user.create_at.clone(),
                                                        ws_id: current_user.ws_id,
                                                    }))
                                                }>
                                                    {
                                                        move || {
                                                            // 这里如果.read() 则需要.clone()
                                                            // 如果.get() 则不需要.clone(), 因为.get()内部会自动克隆
                                                            // let users = row_signal.read().clone();
                                                            let users = row_signal.get();
                                                            view! {
                                                                <th>{move || users.id.unwrap_or_default()}</th>
                                                                <th>{move || users.fullname.clone()}</th>
                                                                <th>{move || users.email.clone()}</th>
                                                                <th>{move || users.create_at.clone()}</th>
                                                                <th>{move || users.ws_id}</th>
                                                                <th>
                                                                    <button on:click= move |ev| {
                                                                        ev.stop_propagation();
                                                                        delete_user_action.dispatch(users.id.unwrap_or_default());
                                                                    }>"删除"</button>
                                                                </th>
                                                            }
                                                        }
                                                    }
                                                </tr>
                                            }
                                        }
                                    />
                                </tbody>
                            </table>
                        </div>
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
