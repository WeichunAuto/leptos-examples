use leptos::{ev::SubmitEvent, prelude::*};

use crate::{dto::users_dto::UsersDto, server_fn::user::AddOrUpdateUsers};

#[component]
pub fn UsersForm(users: UsersDto, callback: Callback<UsersDto>) -> impl IntoView {
    let submit = ServerAction::<AddOrUpdateUsers>::new();

    let (updated_user_signal, set_updated_user_signal) = signal(UsersDto::default());
    Effect::new(move || {
        leptos::logging::log!("form effect submit version: {}", submit.version().get());
        // leptos::logging::log!("form effect submit version: ");
        let sub_ret = submit.value().get();

        if let Some(Ok(true)) = sub_ret {
            // let updated_user = updated_user_signal.get();
            callback.run(updated_user_signal.get());
        }
    });

    let on_submit = move |ev: SubmitEvent| {
        let data = AddOrUpdateUsers::from_event(&ev).unwrap();
        leptos::logging::log!("提交之前：{:?}", data);
        set_updated_user_signal.set(UsersDto {
            // key: data.id.unwrap().to_string(),
            id: data.id,
            fullname: data.fullname.clone(),
            email: data.email.clone(),
            create_at: None,
            ws_id: data.ws_id,
        });
        
    };

    view! {
        <ActionForm
            action=submit
            on:submit:capture=on_submit
        >
            <div class="form_div">
            <label>"ID: "
                <input type="number" name="users_dto[id]" value={users.id}/>
            </label>
            </div>
            <div class="form_div">
                <label>
                    "Full Name"
                    <input type="text" name="users_dto[fullname]" value={users.fullname}/>
                </label>
            </div>

            <div class="form_div">
                <label>
                    "Email"
                    <input type="text" name="users_dto[email]" value={users.email} />
                </label>
            </div>

            <div class="form_div">
                <label>
                    "分组ID"
                    <select name="users_dto[ws_id]">
                        <option selected=move || users.ws_id==0>
                            "0"
                        </option>
                        <option  selected=move || users.ws_id==1>
                            "1"
                        </option>
                    </select>
                </label>
            </div>

            <input type="submit"/>
        </ActionForm>
    }
}
