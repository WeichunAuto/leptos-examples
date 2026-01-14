use leptos::{logging::log, prelude::*};

use crate::{dto::users_dto_sig::UsersDtoSig, server_fn::user::AddOrUpdateUsers};

#[component]
pub fn UsersFormSig(users: UsersDtoSig, callback: Callback<UsersDtoSig>) -> impl IntoView {
    let submit = ServerAction::<AddOrUpdateUsers>::new();

    let (pre_submit_version, set_pre_submit_version) = signal(0);

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
            let submited_value = submit.value().get().unwrap().unwrap();
            // let s = submited_value();

            log!("submited_value = {:?}", submited_value);

            // 创建时间如果为 None 则更新；如果为 Some 则新增
            let created_time = match submited_value.create_at.is_some() {
                true => submited_value.create_at,
                false => None,
            };

            // 不创建新信号，而是更新现有信号的值
            let pass_back_user = UsersDtoSig {
                id: submited_value.id,
                fullname: users.fullname, // 使用传入的 existing signal
                email: users.email,       // 使用传入的 existing signal
                create_at: created_time,
                ws_id: users.ws_id, // 使用传入的 existing signal
            };

            // 更新信号的值
            users.fullname.set(submited_value.fullname);
            users.email.set(submited_value.email);
            users.ws_id.set(submited_value.ws_id);

            callback.run(pass_back_user);
        }
    });

    view! {
        <ActionForm
            action=submit
            // on:submit:capture=on_submit
        >
            <div class="form_div">
            <label>"ID: "
                <input type="number" name="users_dto[id]" readonly value=users.id/>
            </label>
            </div>
            <div class="form_div">
                <label>
                    "Full Name"
                    <input type="text" name="users_dto[fullname]"
                    // bind:value=users.fullname
                    prop:value = move || users.fullname.get()
                    />
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
