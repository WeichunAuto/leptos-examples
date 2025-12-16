use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::{
    components::{ Route, Router, Routes},
    path,
};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;


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

#[derive(Params, PartialEq)]
struct UserParams {
    id: Option<String>,
}

#[server]
pub async fn get_user(user_id: String) -> Result<String, ServerFnError> {
    tracing::info!("fetch user： {}", user_id);

    use tokio::time::sleep;
    use std::time::Duration;

    sleep(Duration::from_secs(2)).await;

    Ok(format!("user_id = {}", user_id))
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
    let params = use_params::<UserParams>();

    let user_id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
            .unwrap_or_default()
    };

    let action = Action::new(move |input: &String| {
        let input = input.to_owned();
        async move { get_user(input).await }

    });
    
    let input_params = "前端参数";

    view! {
        <button on:click= move |_| {action.dispatch(input_params.to_string());}>
            {move || if action.pending().get() { "Loading...".to_string() } else { "点击触发Action".to_string() }}
        </button>

        <p>"params 值是："{move || user_id()}</p>
    }
}
