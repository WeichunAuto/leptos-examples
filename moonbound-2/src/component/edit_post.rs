use leptos::{prelude::Read, *};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Params, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct EditPostParams {
    post_id: Option<String>,
}

#[component]
pub fn EditPost() -> impl IntoView {
    let params = use_params::<EditPostParams>();

    // let display_params = move || match params.get() {
    //     Ok(EditPostParams { post_id: Some(s) }) => {
    //         s
    //     }
    //     _ => "error".to_string(),
    // };

    let display_params = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.post_id.clone())
            .unwrap_or("error here.".to_string())
    };

    view! {
        {display_params}
    }
}
