use leptos::prelude::*;
use reactive_stores::{Store, StoreFieldIterator};

use crate::components::users_form::UsersForm;
use crate::dto::users_dto::{UsersDto, UsersDtoDataStoreStoreFields, UsersDtoStoreFields};
use crate::server_fn::user::get_users;
use crate::{dto::users_dto::UsersDtoDataStore, server_fn::user::delete_users};

#[component]
pub fn UsersList() -> impl IntoView {
    let async_users = Resource::new(|| (), |_| get_users());
    let data_store = Store::new(UsersDtoDataStore { rows: vec![] });

    let (selected_line, set_selected_line) = signal::<Option<UsersDto>>(None);

    Effect::new(move || {
        leptos::logging::log!("users lists run.");

        if let Some(Ok(users)) = async_users.get() {
            // 模式解构
            data_store.set(UsersDtoDataStore { rows: users })
        };
    });

    // 更新
    let update_data_store = Callback::new(move |latest_fullname: String| {
        // leptos::logging::log!("start update users store. {:?}", latest_fullname);

        let special_name = "Bob";
        for row_signal in data_store.rows().iter_unkeyed() {
            let name_signal = row_signal.fullname();
            let current_name = name_signal.get();
            let new_name = format!("{} SUPER", current_name);

            if current_name.eq(special_name) {
                *name_signal.write() = format!("Mr {}", new_name);
            } else {
                *name_signal.write() = latest_fullname.clone();
            }
        }
    });

    view! {

        <button on:click= move |_| {
            let special_name = "Bob";
            for row_signal in data_store.rows().iter_unkeyed() {
                let name_signal = row_signal.fullname();
                let current_name = name_signal.get();
                let new_name = format!("{} SUPER", current_name);

                if current_name.eq(special_name) {
                    *name_signal.write() = format!("Mr {}", new_name);
                } else {
                    *name_signal.write() = new_name;
                }
            }
        }>"更新姓名"</button>


        <ul>
            <ForEnumerate
                each = move || data_store.rows() // .rows() 是个getter 方法
                key = |row_signal| row_signal.read().id.unwrap()
                children = move |index, row_signal| {
                    let users_dto = row_signal.read().clone();
                    view! {
                        <li
                             on:click=move |_| set_selected_line.set(Some(UsersDto {id: users_dto.id, fullname: users_dto.fullname.clone(), email: users_dto.email.clone(), create_at: users_dto.create_at.clone(), ws_id: users_dto.ws_id }))
                        >{move || {
                            let student = row_signal.read(); // 一次性提取 student 对象
                            format!("index: {}, key: {}, name: {}, height: {} ", index.get(), student.id.unwrap(), student.fullname, student.email)

                        }}

                        </li>
                    }
                }
            >

            </ForEnumerate>

        </ul>

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
                        // users=selected_line.get().unwrap_or_default()
                        callback = update_data_store

                    />
                </div>
            </div>
        </Show>


    }
}
