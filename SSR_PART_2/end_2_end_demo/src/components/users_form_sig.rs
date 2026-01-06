use leptos::{ev::SubmitEvent, logging::log, prelude::*};

use crate::{dto::{users_dto::UsersDto, users_dto_sig::UsersDtoSig}, server_fn::user::AddOrUpdateUsers};

#[component]
pub fn UsersFormSig(users: UsersDtoSig, callback: Callback<UsersDtoSig>) -> impl IntoView {
    let submit = ServerAction::<AddOrUpdateUsers>::new();

    let (pre_submit_version, set_pre_submit_version) = signal(0);

    let (id, set_id) = signal(String::from("-1"));
    let (fullname, set_fullname) = signal(String::from(""));
    let (email, set_email) = signal(String::from(""));
    let (ws_id, set_ws_id) = signal(String::from("-1"));

    log!("users dto in form: {:?}", users);

    // 初始化 form 表单字段
    // Effect::watch(
    //     move || users.id.unwrap_or_default(),
    //     move |_, _, _| {
    //         set_id.set(users.id.unwrap_or_default().to_string());
    //         set_fullname.set(users.fullname.clone());
    //         set_email.set(users.email.clone());
    //         set_ws_id.set(users.ws_id.to_string());

    //         log!("form initialized.");
    //     },
    //     true,
    // );

    // form 提交成功后，将最新 user 回传，用于更新 store 和页面展示.
    Effect::new(move || {
        let current_submit_version = submit.version();
        leptos::logging::log!(
            "current_submit_version is : {:?}, and pre_submit_version is : {:?}",
            current_submit_version.get(),
            pre_submit_version.get()
        );

        if current_submit_version.get() > pre_submit_version.get() {
            set_pre_submit_version.set(current_submit_version.get());
            let pass_back_user = UsersDtoSig::new(
                users.id,
                users.fullname,
                users.email,
                None,
                users.ws_id,
            );
            // let pass_back_user = users.clone();

            callback.run(pass_back_user);
        }
    });

    // let on_submit = move |ev: SubmitEvent| {
    //     let data = AddOrUpdateUsers::from_event(&ev).unwrap();
    //     leptos::logging::log!("提交之前：{:?}", data);
    //     set_updated_user_signal.set(UsersDto {
    //         // key: data.id.unwrap().to_string(),
    //         id: data.id,
    //         fullname: data.fullname.clone(),
    //         email: data.email.clone(),
    //         create_at: None,
    //         ws_id: data.ws_id,
    //     });
    // };

    view! {
        <ActionForm
            action=submit
            // on:submit:capture=on_submit
        >
            <div class="form_div">
            <label>"ID: "
                <input type="number" name="users_dto[id]" value=users.id.unwrap_or_default()/>
            </label>
            </div>
            <div class="form_div">
                <label>
                    "Full Name"
                    <input type="text" name="users_dto[fullname]" bind:value=users.fullname/>
                </label>
            </div>

            <div class="form_div">
                <label>
                    "Email"
                    <input type="text" name="users_dto[email]" bind:value=users.email />
                </label>
            </div>

            <div class="form_div">
                <label>
                    "分组ID"
                    <select name="users_dto[ws_id]"
                        on:change:target=move |ev| {
                            // set_ws_id.set(ev.target().value().parse().unwrap());
                            users.ws_id.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=move || users.ws_id
                    >
                        <option>
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
    }
}
