use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (count, set_count) = signal(0);
    let progress_max = 50;

    let double_count = move || {
        leptos::logging::log!("double_count 闭包被调用了");
        count.get() * 2
    };

    let injected_html = "<p>this is injected html</p>";

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/moonbound-tutorial.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // 动态样式
        <div class="mt">
            <button on:click = move |_| {
                *set_count.write() += 1
            }
            class=("red", move || count.get() %2 == 1)
            // class:red = move || count.get() %2 == 1
            >
                "click me please"
            </button>
        </div>


        // 内样式属性
        <div class="mt"
            style="background-color:green; width: 500px; height:50px;"
        >
            <button class=(["red", "lg_font"], move || count.get()%2 == 1)>"multiple class"</button>
        </div>

        // 进度条
        <div class="mt">
            <progress max=progress_max value=count />
            <button on:click = move |_| set_count.set(1)>"Reset"</button>
        </div>
        // <div class="mt">
        //     <progress max=progress_max value=double_count/>
        //     <span>{double_count}</span>
        // </div>

        // html 注入
        <div class="mt">
            <div inner_html=injected_html/>

        </div>

        <p>"double_count 值："{double_count}</p>
    }
}
