use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet};

// 这里可以做网络请求，读取数据库等
async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1000).await;
    value * 10
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (count, set_count) = signal(0);

    // Resources are reactive wrappers for asynchronous tasks, which allow you to integrate an asynchronous Future into the synchronous reactive system.
    // They effectively allow you to load some async data, and then reactively access it either synchronously or asynchronously.
    // You can .await a resource like an ordinary Future, and this will track it.

    // Resources come in two primary flavors: Resource and LocalResource.
    // If you’re using server-side rendering, you should default to using Resource
    let async_data = LocalResource::new(move || load_data(count.get()));
    let stable_data = LocalResource::new(move || load_data(1));

    view! {

        <Stylesheet id="leptos" href="/pkg/load_data_with_resource.css"/>
        <button on:click= move |_| *set_count.write() += 1>"Count + 1"</button>

        <p>
            <code>"Stable"</code>": " {move || stable_data.get()}
        </p>

        <p>
            <code>"count"</code>": " {count}
        </p>

        <p>
            <code>"async_value"</code>": "
            {
                move || match async_data.get() {
                    Some(data) => format!("Server returned value: {}", data),
                    None => "Loading....".to_string()
                }
            }
            <br/>
        </p>

    }
}
