use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/for_component.css"/>

        <h1>"For Component Example"</h1>
        <DynamicForComponent intial_length=5/>

        <h1 class="mt">"ForEnumerate Component Example"</h1>
        <DynamicForEnumerateComponent initial_length = 6 /> 

    }
}

/// For Component Example
#[component]
fn DynamicForComponent(intial_length: u16) -> impl IntoView {
    let initial_counters = (0..intial_length)
        .map(|i| (i, ArcRwSignal::new(i)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = signal(initial_counters);

    let next_count_id = RwSignal::new(intial_length); // 这里需要使用 Signal 来包装，因为在视图渲染中的For组件中使用到这个变量。

    let add_counter = move |_| {
        let next_id = next_count_id.get();
        let new_counter = ArcRwSignal::new(next_id);
        set_counters.update(move |counter_vec| {
            counter_vec.push((next_id, new_counter));
            // next_count_id += 1;
            next_count_id.set(next_id + 1);
        });
    };

    view! {
        <div class="mt">
            <button on:click=add_counter>"Add Counter"</button>
        </div>
        <p class="mt">"迭代列表如下："</p>
        <ul>
            <For
                each = move || counters.get()
                key = |item| item.0
                children = move |(id, counter)| {
                    let count = RwSignal::from(counter);
                    view! {
                        <li>
                            <button on:click=move |_| *count.write() += 1>
                                {count}
                            </button>
                            <button on:click=move |_| {
                                set_counters.write().retain(|(counter_id, _)| {
                                    *counter_id != id
                                })
                            }>
                                "Remove"
                            </button>
                        </li>
                    }
                }
            />
        </ul>
    }
}

/// ForEnumerate Component Example
#[component]
fn DynamicForEnumerateComponent(initial_length: u16) -> impl IntoView {
    let initial_counters = (0..initial_length)
        .map(|i| RwSignal::new(i + 2))
        .collect::<Vec<_>>();

    let (counters, set_counters) = signal(initial_counters);

    let next_count_id = RwSignal::new(initial_length + 2); // 这里需要使用 Signal 来包装，因为在视图渲染中的For组件中使用到这个变量。

    let add_counter = move |_| {
        let next_id = next_count_id.get();
        let new_count = RwSignal::new(next_id);
        set_counters.update(|counter_vec| {
            counter_vec.push(new_count);
        });
        next_count_id.set(next_id + 1);
    };

    view! {
        <div class="mt">
            <button on:click=add_counter>"Add Counter"</button>
        </div>
        <p class="mt">"迭代列表如下："</p>
        <ul>
            <ForEnumerate
                each = move || counters.get()
                key = |item| item.get()
                children = move |index, item| {
                    view! {
                        <li>
                            <button on:click = move |_| *item.write() += 1>"Index: " {index}</button>
                            <button on:click = move |_| {
                                set_counters.write().retain(|counter_id| {
                                    counter_id.get() != index.get() as u16
                                })
                            }>"Remove Item: " {item}</button>
                        </li>
                    }
                }

            >

            </ForEnumerate>
        </ul>
    }
}
