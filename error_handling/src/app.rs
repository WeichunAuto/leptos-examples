use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/error_handling.css"/>

        // sets the document title
        <NumericInput />
        <p style="margin-top: 20px;">"*****************************"</p>
        <NumericInputWithErrorHandling />
    }
}

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = signal(Ok(0));
    view! {
        <label>
            "请输入一个数字："
            <input
                type="number"
                on:input:target = move |ev| {
                    set_value.set(ev.target().value().parse::<i32>());
                }/>
        </label>
        <p>"你输入的数字是："<strong>{value}</strong></p>
    }
}

#[component]
fn NumericInputWithErrorHandling() -> impl IntoView {
    let (value, set_value) = signal(Ok(0));
    view! {
        <label>
            "请输入一个数字（带错误处理）："
            <input
                type="number"
                on:input:target = move |ev| {
                    set_value.set(ev.target().value().parse::<i32>());
                }/>
        </label>

        // An <ErrorBoundary/> is a little like the <Show/> component we saw in the last chapter.
        // If everything’s okay—which is to say, if everything is Ok(_)—it renders its children.
        // But if there’s an Err(_) rendered among those children, it will trigger the <ErrorBoundary/>’s fallback.
        // value 在signal中是个Result类型，如果Ok 则渲染子元素，如果Err，则渲染fallback.
        <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        // <p>"Not a number! Errors: "</p>
                        // we can render a list of errors
                        // as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            >
                <p>
                    "你输入的数字是： "
                    // because `value` is `Result<i32, _>`,
                    // it will render the `i32` if it is `Ok`,
                    // and render nothing and trigger the error boundary
                    // if it is `Err`. It's a signal, so this will dynamically
                    // update when `value` changes
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>
    }
}
