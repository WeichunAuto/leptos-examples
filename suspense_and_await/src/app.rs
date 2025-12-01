use std::future::{Pending, Ready};

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet};

/// Suspense 的特点：必须和 Resource 一起使用. Suspense = 为 Resource 设计的加载边界    
/// - Resource “正在加载” → Suspense 显示 fallback
/// - Resource 加载完成 → Suspense 自动切换到实际Children内容
/// - Resource 再次刷新 → Suspense 再次挂起
/// 典型用于页面加载数据：多个 API 并行加载

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

async fn fetch_monkeys(monkey: i32) -> i32 {
    // maybe this didn't need to be async
    monkey * 2
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (count, set_count) = signal(0);
    let (name, set_name) = signal("".to_string());
    let async_count = LocalResource::new(move || load_data(count.get()));
    let async_name = LocalResource::new(move || load_name(name.get()));

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/suspense_and_await.css"/>

        <h1>"以下为 Suspense 组件示例"</h1>

        // <Suspense> Component will waiting for resources to load and shows the fallback.
        // When all resources are loaded, it shows the children.
        <Suspense
            fallback = move || view! {<p>"It's loading....."</p>}
        >
            // Resource 1
            {
                move || async_count.get().map(|data| view! {<p>"Server returned count: "{data}</p>})
            }

            // Resource 2
            {
                move || async_name.get().map(|name| view! {<p>"Server returned name: "{name}</p>})
            }
        </Suspense>


        <Suspense
            fallback = move || view! {<p>"Suspend way, It's loading....."</p>}
        >
            <p>
                {
                    // 效果与上一个 Suspense 组件相同
                    // Suspend allows you use to an async block in the view.
                    // Suspend allows us to avoid null-checking each resource, and removes some additional complexity from the code.
                    move || Suspend::new(async move {
                        async_count.await
                    })
                }
            </p>
        </Suspense>

        <button on:click= move |_| *set_count.write() += 1>"Count + 1"</button>
        <button on:click = move |_| set_name.update(|name| *name = format!("{}s", *name))>"Set name"</button>


        <h1>"以下为 Await 组件示例"</h1>

        // Await 组件 trying to wait for some Future to resolve before rendering,
        // - It only polls the Future once, and does not respond to any reactive changes.
        // - It does not render anything until the Future resolves.
        // - After the Future resolves, it binds its data to whatever variable name you choose and then renders its children with that variable in scope.
        <Await
            future=fetch_monkeys(3)
            let:data
        >
            <p>{*data} " little monkeys, jumping on the bed."</p>
        </Await>
    }
}
