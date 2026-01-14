use leptos::logging::log;
use leptos::prelude::*;

use crate::components::users_form_sig::UsersFormSig;
use crate::dto::users_dto_sig::UsersDtoSig;
use crate::server_fn::user::delete_users;
use crate::server_fn::user::get_users_sig;

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

    // 删除用户 by id
    let delete_user_action = Action::new(move |id: &i64| {
        let id = *id;
        async move {
            let rt = delete_users(id).await;
            match rt {
                Ok(is_success) => {
                    if is_success {
                        leptos::logging::log!("delete is successful. now update the view");
                        set_users_list.update(|list| {
                            list.retain(|user| user.id != id);
                        });

                        let remain_users: Vec<_> = users_list
                            .get()
                            .into_iter()
                            .filter(|user| user.id != id)
                            .collect();
                        set_users_list.set(remain_users);

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

    // 更新 或 新增
    let update_or_add_user = Callback::new(move |user_dto: UsersDtoSig| {
        // 新增
        if let Some(created_time) = user_dto.create_at {
            let new_user = UsersDtoSig::new(
                user_dto.id,
                user_dto.fullname,
                user_dto.email,
                Some(created_time.chars().take(16).collect()),
                user_dto.ws_id,
            );

            log!("new_user = {:?}", new_user.fullname.get());
            set_users_list.write().push(new_user);

            set_selected_line.set(None);
        }
        // 更新
        else {
            log!("start updating.... :{}", user_dto.fullname.get());
            let target_id = user_dto.id;

            let update_user_opt = users_list
                .get()
                .into_iter()
                .find(|user| user.id == user_dto.id);

            if let Some(update_user) = update_user_opt {
                update_user.fullname.set(user_dto.fullname.get());
                log!(
                    "updated: {}, target id is : {}",
                    user_dto.fullname.get(),
                    target_id
                );
            }
        }
    });

    view! {
        <Suspense
                fallback = move || {view! {
                    <div>

                        <p>"加载中..."</p>
                    </div>
                }}
            >
                        <div>
                            <div>
                                <button on:click= move |_| set_selected_line.set(Some(UsersDtoSig::default()))>"添加"</button>
                            </div>

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
                                                    set_selected_line.set(Some(UsersDtoSig {
                                                        id: user.id,
                                                        fullname: user.fullname,
                                                        email: user.email,
                                                        create_at: date.clone(),
                                                        ws_id: user.ws_id,
                                                    }))
                                                }>
                                                                <th>{move || user.id}</th>
                                                                <th>{move || user.fullname.get_untracked()}</th>
                                                                <th>{move || user.email.get()}</th>
                                                                <th>{move || user.create_at.clone().unwrap_or_default().clone()}</th>
                                                                <th>{move || user.ws_id.get()}</th>
                                                                <th>
                                                                    <button on:click= move |ev| {
                                                                        ev.stop_propagation();
                                                                        delete_user_action.dispatch(user.id);
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
                >
                "关闭"
                </div>
                <div class="form">
                    <UsersFormSig
                        users=selected_line.get().unwrap_or_default()
                        callback = update_or_add_user

                    />
                </div>
            </div>
        </Show>
    }
}
