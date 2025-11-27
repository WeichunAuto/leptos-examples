use leptos::{logging, prelude::*};
use leptos_meta::{provide_meta_context, Stylesheet, Title};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let values = vec![1, 2, 3, 4];

    let length = 5;
    let counters = RwSignal::new((1..=length).map(|i| RwSignal::new(i)).collect::<Vec<_>>());

    let counter_buttons = move || {
        counters
            .get()
            .iter() //引用迭代，不移动所有权
            .map(|&item| {
                // 注意，这里 &item 是模式匹配解构，不可以使用 *item
                view! {
                    <li>
                        <button on:click = move |_| item.set(item.get() +1)>
                            // {format!("按钮: {}", item)}
                            {item}
                        </button>
                    </li>
                }
            })
            .collect_view()
    };

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/iteration.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <ul>
            {
                values.into_iter()
                    .map(|item| view! {
                        <li>{format!("第{}个", item)}</li>
                    })
                    // .collect::<Vec<_>>()
                    .collect_view()
            }
        </ul>
        <br />

        <button on:click = move |_| {
            counters.update(|items| {
                items.push(RwSignal::new(10));
            });
                logging::log!("按钮被点击了，当前计数器数量: {}", counters.get().len());


        }>添加counter</button>
        <ul>
            {move || counter_buttons} // 在view中，这边必须要是个闭包，当count_buttons变化时，页面才会reactive
        //     { move ||
        //         counters
        //     .get()
        //     .iter()
        //     .map(|&item| {
        //         // 注意，这里 &item 是模式匹配解构，不可以使用 *item
        //         view! {
        //             <li>
        //                 <button on:click = move |_| item.set(item.get() +1)>
        //                     // {format!("按钮: {}", item)}
        //                     {item}
        //                 </button>
        //             </li>
        //         }
        //     })
        //     .collect_view()
        // }
        </ul>

        <p>"当前计数器数量: " {move || counters.get().len()}</p>


    }
}
