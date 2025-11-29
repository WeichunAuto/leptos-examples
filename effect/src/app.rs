use leptos::{logging, prelude::*};
use leptos_meta::{provide_meta_context, Stylesheet};

/// Effects are intended to synchronize the reactive system with the non-reactive world outside,
/// not to synchronize between different reactive values.
/// In other words: using an effect to read a value from one signal and set it in another is always sub-optimal.

/// If you need to define a signal that depends on the value of other signals,
/// use a derived signal or a Memo. Writing to a signal inside an effect isn’t the end of the world,
/// and it won’t cause your computer to light on fire,
/// but a derived signal or memo is always better—not only because the dataflow is clear, but because the performance is better.

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (a, set_a) = signal(0);
    let (b, set_b) = signal(1);

    // Effect 只会自动追踪在闭包内部被读取的信号。
    Effect::new(move |_| {
        // immediately prints "Value: 0" and subscribes to `a`
        logging::log!("a Value: {}", a.read());

        // 外部 ‘b’ 的变化不会出发 Effect 重新执行。
        set_b.update(|v| *v = *v * 2);

        // 在 Effect 中执行这一句，会引起死循环！
        // 因为，b.get()会使 Effect 订阅追踪 b 的变化进而重新执行 Effect, 而 set_b.set()使 b 产生了变化。
        // set_b.set(b.get() * 2);
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/effect.css"/>

        <h1>"Leptos Effect::new()"</h1>

        // <button on:click = move|_| set_a.set(*(a.read()) + 1)>"a 自增 +1"</button>
        <button on:click = move|_| set_a.update(|v| *v = *v+1)>"a 自增 +1"</button>

        <button on:click = move|_| set_b.update(|v| *v = *v+1)>"b 自增 +1"</button>
        <p>"b 的值为："{b}</p>

        <h1>"Leptos Effect::watch()"</h1>
        <EffectWatchComp />
    }
}

/// watch() 可以比
#[component]
fn EffectWatchComp() -> impl IntoView {
    let (price, set_price) = signal(1000);

    let effect_watch = Effect::watch(
        // 指定 依赖项。如果有多个依赖项，则使用元组，如 `move || (price.get(), name.get())`
        // 这个闭包中会被频繁调用，以监控dependency的变化，应避免 expensive 的计算
        move || price.get(),
        move |new_price, pre_price, _| {
            if let Some(old) = pre_price {
                let change = new_price - old;
                leptos::logging::log!("价格变化了：{}", change);
            } else {
                leptos::logging::log!("初识价格为：{}", new_price);
            }
        },
        true, // true: 表示组件挂载时立即执行一次handler. false: 表示仅在 dependency 变化时执行handler.
    );

    view! {
        <button on:click=move |_| set_price.update(|p| *p += 10)>
            "涨价: " {price}
        </button>

        <button on:click= move|_| effect_watch.stop()>"停止 Watch "</button>
    }
}
