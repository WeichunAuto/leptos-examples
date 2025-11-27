use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/components_props.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <button on:click = move |_| set_count.set(count.get()+1)>"正常进度条"</button>
        // <ProgressBar progress= count/>
        <ProgressBar progress=Signal::derive(double_count)/>
        // <ProgressBar progress=double_count/>
    }
}

#[component]
pub fn ProgressBar(
    #[prop(default = 50)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView
{
    view! {
        <div class="mt">
            <progress max=max value=progress />
        </div>
    }
}
