use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet};

// 这里可以做网络请求，读取数据库等
async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1000).await;
    value * 10
}

async fn load_name(name: String) -> String {
    TimeoutFuture::new(1000).await;
    format!("Mr {}", name)
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (count, set_count) = signal(0);
    let (name, set_name) = signal("".to_string());
    let async_count = LocalResource::new(move || load_data(count.get()));
    let async_name = LocalResource::new(move || load_name(name.get()));

    let (pending, set_pending) = signal(false);

    view! {
        <Stylesheet id="leptos" href="/pkg/transition.css"/>

        <p>
        {
            move || if pending.get(){
                "Hang on..."
            } else {
                "Ready...."
            }
        }
        </p>

        <Transition
            fallback = move || view! {<p>"It's loading....."</p>}
            // this will be set to `true` whenever the transition is ongoing
            // 可通过这个 signal 得知当前是否正在 transition 中。
            set_pending
        >
            // Resource 1
            {
                move || async_count.get().map(|data| view! {<p>"Server returned count: "{data}</p>})
            }

            // Resource 2
            {
                move || async_name.get().map(|name| view! {<p>"Server returned name: "{name}</p>})
            }
        </Transition>

        <button on:click= move |_| *set_count.write() += 1>"Count + 1"</button>
        <button on:click = move |_| set_name.update(|name| *name = format!("{}s", *name))>"Set name"</button>

    }
}
