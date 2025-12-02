use gloo_timers::future::TimeoutFuture;
use leptos::{html::Input, prelude::*};
use leptos_meta::{provide_meta_context, Stylesheet};
use uuid::Uuid;

//Actions and resources seem similar, but they represent fundamentally different things.
// If you’re trying to load data by running an async function, either once or when some other value changes,
// you probably want to use a resource.
// If you’re trying to occasionally run an async function in response to something like a user clicking a button,
// you probably want to use an Action.

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Think of it as a mutation: some imperative async action you run,
// whereas a resource would be some async data you load
async fn add_todo(text: &str) -> Uuid {
    leptos::logging::log!("the input value is: {}", text);
    _ = text;

    // fake a one-second delay
    // SendWrapper allows us to use this !Send browser API; don't worry about it
    send_wrapper::SendWrapper::new(TimeoutFuture::new(1_000)).await;

    // pretend this is a post ID or something
    Uuid::new_v4()
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // an action takes an async function with single argument
    // it can be a simple type, a struct, or ()
    // 一个入参，如果需要传入多个参数，则可以将参数包装成一个结构体或元组
    let todo = Action::new(|input: &String| {
        // the input is a reference, but we need the Future to own it
        // this is important: we need to clone and move into the Future
        // so it has a 'static lifetime
        // 传入 Future 的参数，必须是拥有所有权的
        let input = input.to_owned();
        async move { add_todo(&input).await }
    });

    // actions provide a bunch of synchronous, reactive variables
    // that tell us different things about the state of the action
    let submitted = todo.input();
    let pending = todo.pending();
    let todo_id = todo.value();

    let input_ref = NodeRef::<Input>::new();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/actions.css"/>

        <form
            on:submit = move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("Input to exist.");
                todo.dispatch(input.value());
            }
        >
            <label>
                "what do you need to do?"
                <input type="text" node_ref=input_ref />
            </label>
            <button type="submit">"Add Todo"</button>
        </form>

        // 可以获取 action 的当前pending状态，为true时，可展示提示UI.
        <p>"当前 pending 状态：" {move || pending.get()}</p>
        <p>{move || pending.get().then_some("Loading....")}</p>
        <p>{move || pending.get().then(|| view! {<strong>"加载中。。。"</strong>})}</p>

        // 当前pending状态为true时，通过get()方法获取入参值
        <p>"Submitted: " {move || submitted.get()}</p>

        <p>
            "Pending: "
            <code>{move || format!("{:#?}", pending.get())}</code>
        </p>

        // 获取每一个异步请求的ID，异步方法 add_todo(text: &str) 中生成并返回。
        <p>
            "Todo ID: "
            <code>{move || format!("{:#?}", todo_id.get())}</code>
        </p>
    }
}
