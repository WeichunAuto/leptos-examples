use leptos::{ev::MouseEvent, prelude::*};
use leptos_meta::{provide_meta_context, Stylesheet};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (toggle, set_toggle) = signal(false);

    // 把这个闭包作为参数传递给子组件，在闭包中修改 父组件中的值
    let update_toggle = move |_| set_toggle.update(|value| *value = !*value);

    // 把 set_toggle 共享给所有层级的子组件，包括多层级子组件
    provide_context(set_toggle);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/parent_child_communication.css"/>
        <h1>"父组件与子组件 沟通"</h1>

        <p>"Toggled ? "
            <Show
                when = move || {toggle.get()}
                fallback = move || view! {<strong>{toggle}</strong>}
            >
                <strong style="color: green; font-size: 25px;">{toggle}</strong>
            </Show>
        </p>

        <h2>"通过 props 传值"</h2>
        <PropsButton setter= set_toggle/>

        <h2>"通过 a closure Callback"</h2>
        <CallbackButton on_click= update_toggle/>

        <h2>"通过 直接在组件上监听点击事件"</h2>
        <EventListenerButton on:click = move |_| set_toggle.update(|value| *value = !*value)/>

        <h2>"The Context API"</h2>
        <ContextApi />


    }
}

#[component]
fn PropsButton(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button on:click= move |_| setter.update(|value| *value = !*value) style="padding: 10px;">"Props更新状态"</button>
    }
}

#[component]
fn CallbackButton(on_click: impl FnMut(MouseEvent) + 'static) -> impl IntoView {
    view! {
        <button on:click= on_click style="padding: 10px;">"Callback更新状态"</button>
    }
}

#[component]
fn EventListenerButton() -> impl IntoView {
    view! {
        <button>"Event Listener更新状态"</button>
    }
}

#[component]
pub fn ContextApi() -> impl IntoView {
    view! {
        <header>
            <h1>"My Page"</h1>
        </header>
        <main>
            <Content />
        </main>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ButtonD />
        </div>
    }
}


/// this can be one of the most effective techniques for global state management in Leptos: 
/// simply provide the state at the highest level you’ll need it, and use it wherever you need it lower down.
/// 
/// Note that there are no performance downsides to this approach. 
/// Because you are passing a fine-grained reactive signal, 
/// nothing happens in the intervening components (<Layout/> and <Content/>) when you update it. 
/// You are communicating directly between <ButtonD/> and <App/>. 
#[component]
pub fn ButtonD() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>().expect("请在 顶层组件 内使用 provide_context");

    view! {
        <button on:click= move |_| setter.update(|value| *value = !*value) style="padding: 10px;">"Context API 更新状态"</button>
    }
}
