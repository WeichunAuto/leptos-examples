use leptos::{ev::SubmitEvent, prelude::*};

use crate::{dto::users_dto::UsersDto, server_fn::user::AddOrUpdateUsers};

#[component]
pub fn UsersForm(callback: Callback<String>) -> impl IntoView {
    let submit = ServerAction::<AddOrUpdateUsers>::new();

    let (pre_submit_version, set_pre_submit_version) = signal(0);

    let (updated_user_signal, set_updated_user_signal) = signal(UsersDto::default());
    // let mut latest_fullname = String::from("");

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

    Effect::new(move || {
        // leptos::logging::log!("form effect submit version: ");
        let current_submit_version = submit.version();

        leptos::logging::log!(
            "current_submit_version is : {:?}",
            current_submit_version.get()
        );

        if current_submit_version.get() > pre_submit_version.get() {
            // let updated_user = updated_user_signal.get();
            set_pre_submit_version.set(current_submit_version.get());
            // leptos::logging::log!("latest fullname is : {}", latest_fullname);
            callback.run(updated_user_signal.get().fullname);
        }
    });

    view! {
        <div>
            <ActionForm
            action=submit
            on:submit:capture=on_submit
        >
            <div class="form_div">
            <label>"ID: "
                <input type="number" name="users_dto[id]" value=""/>
            </label>
            </div>
            <div class="form_div">
                <label>
                    "Full Name"
                    <input type="text" name="users_dto[fullname]" value=""/>
                </label>
            </div>

            <div class="form_div">
                <label>
                    "Email"
                    <input type="text" name="users_dto[email]" value="" />
                </label>
            </div>

            <div class="form_div">
                <label>
                    "分组ID"
                    <select name="users_dto[ws_id]">
                        <option selected=true>
                            "0"
                        </option>
                        <option >
                            "1"
                        </option>
                    </select>
                </label>
            </div>

            <input type="submit"/>
        </ActionForm>
            <div>
                // <button on:click= move |_| callback.run()>"更新"</button>
            </div>
        </div>

    }
}
