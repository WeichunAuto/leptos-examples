use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::{
    components::{ Route, Router, Routes},
    path,
};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
}

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

#[server]
pub async fn get_user(user_info: UserInfo) -> Result<UserInfo, ServerFnError> {
    tracing::info!("fetch user： {}", user_info.name);

    use tokio::time::sleep;
    use std::time::Duration;

    sleep(Duration::from_secs(2)).await;

    Ok(user_info)
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <h1>"Welcome to Leptos!"</h1>

        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=path!("/") view=HomePage/>
                </Routes>
            </main>
        </Router>

    }
}

#[component]
fn HomePage() -> impl IntoView {

    let action = Action::new(move |input: &UserInfo| {
        let input = input.to_owned();
        async move { get_user(input).await }

    });
    
    let input_params = UserInfo{id: 0, name: String::from("Lucy John")};

    view! {
        <button on:click= move |_| {action.dispatch(input_params.clone());}>
            {move || {
                if action.pending().get() { 
                    // 通过 action.input() 获取传进 action.dispatch 的参数值
                    leptos::logging::log!("action input = {}", action.input().get().unwrap().name);
                    "Loading...".to_string() 
                } else if let Some(input) = action.value().get(){ // 通过 action.value() 获取 server fn get_user() 的返回数据
                    input.unwrap().name
                } else {
                    "点击触发Action".to_string() 
                }
            }}
        </button>

    }
}
