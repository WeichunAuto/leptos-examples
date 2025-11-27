use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet};

#[component]
fn If(
    condition: Signal<bool>,
    // Component slot, should be passed through the <Then slot> syntax
    then: Then,
    #[prop(default=vec![])] else_if: Vec<ElseIf>,
    #[prop(optional)] fallback: Option<Fallback>,
) -> impl IntoView {
    move || {
        if condition.get() {
            (then.children)().into_any()
        } else if let Some(else_if) = else_if.iter().find(|i| i.condition.get()) {
            (else_if.children)().into_any()
        } else if let Some(fallback) = &fallback {
            (fallback.children)().into_any()
        } else {
            ().into_any()
        }
    }
}

// A simple struct annotated with `#[slot]`,
// which expects children
#[slot]
struct Then {
    children: ChildrenFn,
}

#[slot]
struct ElseIf {
    condition: Signal<bool>,
    children: ChildrenFn,
}

#[slot]
struct Fallback {
    children: ChildrenFn,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (value, set_value) = signal(false);
    let (num, set_num) = signal(1);
    let is_four_times = Signal::derive(move || num.get() % 4 == 0);
    let is_three_times = Signal::derive(move || num.get() % 3 == 0);

    view! {
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/passing_children_to_component.css"/>

        <h1>"模拟 <Show> 组件"</h1>
        <MyShow
            condition = value
            fallback = move || {view!{"Here is fallback " {value}}}
        >
            "Here is children " {value}
        </MyShow>
        <button on:click = move |_| set_value.update(|v| *v = !*v)>"改变状态"</button>

        <h1>"Typed Children Slots"</h1>
        <button on:click= move |_| {
            set_num.set(num.get() + 1);
            leptos::logging::log!("num = {:?}", num.get());
        }>"number 递增"</button>

        // 注意：slot 组件不可以直接定义事件！可以在slot组件的子组件中定义事件。
        <If condition=is_four_times>
            // The `If` component always expects a `Then` child for `then_slot`
            <Then slot:then><p>"偶数时渲染组件"</p></Then>
            <ElseIf slot condition=is_three_times><p>"3 的倍数"</p></ElseIf>
            <Fallback slot><p>"进入了fallback分支！"</p></Fallback>
        </If>

    }
}

/// 模拟 <Show> 组件
#[component]
fn MyShow<F, IV>(condition: ReadSignal<bool>, fallback: F, children: ChildrenFn) -> impl IntoView
where
    F: Fn() -> IV + Send + 'static,
    IV: IntoView,
{
    move || {
        if condition.get() {
            {
                children()
            }
        } else {
            {
                fallback().into_any() // 将 IntoView 转换成 AnyView
            }
        }
    }
}
