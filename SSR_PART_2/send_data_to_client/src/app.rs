use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::{
    dto::users_dto::UsersDto,
    server_fn::user::{delete_users, get_users},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        <Stylesheet id="leptos" href="/pkg/send_data_to_client.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (users, set_users) = signal::<Option<Vec<UsersDto>>>(None);

    let async_users = Resource::new(move || users.get(), |_| get_users());
    Effect::new(move |_| {
        if users.get().is_none() {
            let data = async_users.get().and_then(|res| res.ok());
            set_users.set(data);
            leptos::logging::log!("size: {}", users.get().unwrap().len());
        }
    });

    let delete_user_action = Action::new(move |id: &i64| {
        let id = *id;
        // let id = id.to_owned();
        async move {
            let rt = delete_users(id).await;
            match rt {
                Ok(is_success) => {
                    if is_success {
                        leptos::logging::log!("delete is successful. now update the view");
                        let current_data = users.get().unwrap_or_default();
                        let filter_data: Vec<_> = current_data
                            .into_iter()
                            .filter(|user| user.id != id)
                            .collect();
                        set_users.set(Some(filter_data));
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

    view! {
        <h1>"Welcome to Leptos!"</h1>
            <Suspense
                fallback = move || {view! {<p>"Load...."</p>}}
            >
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
                            // Resource 在资源加载完成之前.get()一定是 None, 所以不能直接unwrape(), 否则 unwrape()一个option值一定panic.
                            // 通过.and_then() 将数据结构 Option<Result<Vec<UserDto>>> 剥掉里面的 Result
                            // 然后通过 unwrap_or_default 来处理 None
                            // each = move|| async_users.get().and_then(|res|res.ok()).unwrap_or_default()
                            each = move || users.get().unwrap_or_default()
                            key = |user| user.id
                            children = move |_, user| {
                                view! {
                                        <tr>
                                            <th>{user.id}</th>
                                            <th>{user.fullname}</th>
                                            <th>{user.email}</th>
                                            <th>{user.create_at}</th>
                                            <th>{user.ws_id}</th>
                                            <th><button on:click= move |_| {delete_user_action.dispatch(user.id);}>"delete"</button></th>
                                        </tr>
                                    }
                            }
                        />
                    </tbody>
                </table>
            </Suspense>
    }
}
