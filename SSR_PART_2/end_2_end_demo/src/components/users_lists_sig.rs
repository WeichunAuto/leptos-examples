use leptos::logging::log;
use leptos::prelude::*;
use reactive_stores::{Store, StoreFieldIterator};

use crate::components::users_form::UsersForm;
use crate::components::users_form_sig::UsersFormSig;
use crate::dto::users_dto::{UsersDto, UsersDtoDataStoreStoreFields, UsersDtoStoreFields};
use crate::dto::users_dto_sig::UsersDtoSig;
use crate::server_fn::user::get_users_sig;
use crate::{dto::users_dto::UsersDtoDataStore, server_fn::user::delete_users};

#[component]
pub fn UsersListSig() -> impl IntoView {
    let async_users = Resource::new(|| (), |_| get_users_sig());

    let (users_list, set_users_list) = signal(vec![]);
    let (selected_line, set_selected_line) = signal::<Option<UsersDtoSig>>(None);

    // 初始化数据集
    Effect::watch(
        move || async_users.get(),
        move |new_value, pre_value, _| {
            if let Some(Ok(users_date)) = new_value {
                let pre_users = pre_value.and_then(|user_or| {
                    if let Some(pre_data_vec) = user_or {
                        pre_data_vec.as_ref().ok()
                    } else {
                        None
                    }
                });

                // 只有当数据不同时才更新
                if Some(users_date) != pre_users {
                    if !users_date.is_empty() {
                        set_users_list.set(users_date.to_owned());
                    }
                }
            }
        },
        true,
    );
    // let users_list = move || async_users.get();

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
    let update_user = Callback::new(move |user_dto: UsersDtoSig| {
        log!("start updating....");
        let target_id = user_dto.id.unwrap();
        log!("target id is : {}", target_id);

        // for row_signal in users_dto_store.rows().iter_unkeyed() {
        //     let name_signal = row_signal.fullname();
        //     let current_name = name_signal.get();

        //     if row_signal.id().get().unwrap() == target_id {
        //         name_signal.set(user_dto.fullname.clone());
        //         log!("updated.");
        //         // break;
        //     } else {
        //         name_signal.set(current_name);
        //     }
        // }
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
                                        each = move || users_list.get()
                                        key = |user| user.id
                                        children = move |_, user| {
                                            let date = user.create_at.clone();
                                            view! {
                                                <tr on:click=move |_| {

                                                    // log!("current user id: {}, current user name: {:?}", current_user.id.unwrap(), current_user.fullname);

                                                    set_selected_line.set(Some(UsersDtoSig {
                                                        id: user.id,
                                                        fullname: user.fullname,
                                                        email: user.email,
                                                        create_at: date.clone(),
                                                        ws_id: user.ws_id,
                                                    }))
                                                }>
                                                                <th>{move || user.id.unwrap_or_default()}</th>
                                                                <th>{move || user.fullname.get()}</th>
                                                                <th>{move || user.email.get()}</th>
                                                                <th>{move || user.create_at.clone().unwrap_or_default().clone()}</th>
                                                                <th>{move || user.ws_id.get()}</th>
                                                                <th>
                                                                    <button on:click= move |ev| {
                                                                        ev.stop_propagation();
                                                                        delete_user_action.dispatch(user.id.unwrap_or_default());
                                                                    }>"删除"</button>
                                                                </th>
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
                <UsersFormSig
                    users=selected_line.get().unwrap_or_default()
                    callback = update_user

                />
            </div>
        </div>
        </Show>
    }
}
