use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::path;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::component::{blog_previews::BlogPreviews, edit_post::EditPost};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="dark:bg-gray-800 text-white p-4">
            <div class="container mx-auto flex justify-between items-center">
                <a href="/" class="text-2xl font-bold">Moonbound</a>

                <nav>
                    <h3>"Hey, this is nav."</h3>
                    <ul>
                        <li><a href="/">Blog</a></li>
                        <li><a href="/edit_post/dfdqwr2342asdc">Edit</a></li>
                    </ul>
                </nav>
            </div>
        </div>


    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (count, set_count) = signal(0);

    view! {
        <button on:click = move |_| set_count.set(count.get()+1)>
        "Click me: "{count}
        </button>

        <p>
        "Double Value: " {count.get() * 2}
        </p>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
