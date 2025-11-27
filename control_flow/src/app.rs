use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (value, set_value) = signal(0);
    let is_odd = move || value.get() % 2 != 0;

    let message = move || {
        if is_odd() {
            "这是奇数"
        } else {
            "这是偶数"
        }
    };

    let opt_message = move || {
        if is_odd() {
            Some("Ding Ding 奇数")
        } else {
            None
        }
    };

    // We can make it a little shorter if we’d like, using bool::then().
    let then_message = move || is_odd().then(|| "Ding Ding Ding");

    let match_message = move || match value.get() {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ if is_odd() => "奇数",
        _ => "偶数",
    };

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/control_flow.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>
        <button on:click = move |_| {
            set_value.set(value.get()+1);
            leptos::logging::log!("value = {}", value.get())
        }>
            "获取随机数"
        </button>

        <button on:click = move |_| { set_value.set(1)}>"重置为1"</button>

        <p>"if message: " {message}</p>
        <p>"option message: " {opt_message}</p>
        <p>"bool then message: " {then_message}</p>
        <p>"match message: " {match_message}</p>

        <h1>"<Show> 组件"</h1>
        // <Show/> memoizes the when condition, so it only renders its <Small/> once,
        // continuing to show the same component until value is greater than five;
        // then it renders <Big/> once, continuing to show it indefinitely or until value goes below five and then renders <Small/> again.
        <Show
          when=move || { value.get() > 5 }
          fallback=|| view! { <Small/> }
          >
          <Big/>
        </Show>

    }
}

#[component]
fn Small() -> impl IntoView {
    leptos::logging::log!("small 组件渲染了！");
    view! {
        <p>"这是一个 Small 组件"</p>
    }
}

#[component]
fn Big() -> impl IntoView {
    leptos::logging::log!("big 组件渲染了！");
    view! {
        <p>"这是一个 Big 组件"</p>
    }
}
