use leptos::{html::Input, prelude::*};
use leptos_meta::{provide_meta_context, Stylesheet};
use reactive_stores::Store;

// 定义全局状态数据结构类型
#[derive(Clone, Debug, Default, Store)]
struct GlobalState {
    count: i32,
    name: String,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // 在 App 中创建 Store（全局提供）
    provide_context(Store::new(GlobalState::default()));

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/global_state_management.css"/>

        <GlobalStateCounter />

    }
}

#[component]
fn GlobalStateCounter() -> impl IntoView {
    // 从 Context 中取出 Store
    let state = expect_context::<Store<GlobalState>>();

    // #[derive(Store)]它自动生成：
    // (1) GlobalStateStoreFields trait
    // - 给每个字段（count, name）生成 getter
    // - 每个 getter 返回一个可响应的 Subfield
    // (2) Store<GlobalState> 实现 StoreField
    // 让 Store 能被 reactive_stores 识别
    let count = state.count();
    let name = state.name();

    let input_ref = NodeRef::<Input>::new();
    view! {

        <div>
            <button on:click= move |_| {
                // *count、*name 是 对信号（Signal）内部值解引用的写操作方式。
                // 因为 Subfield 通过实现 DerefMut 来暴露字段的可变引用，所以必须解引用才能写入字段值。
                *count.write() += 1;
            }>
                "Increase Global Count"
            </button>

            // count.get() 读取值
            <p>"Count value is : "{move || count.get()} </p>

        </div>

        <div>
            <form
                on:submit = move |ev| {
                    ev.prevent_default();
                    // 获取表单值
                    let input = input_ref.get().expect("failed give input value");
                    *name.write() = input.value();
                }
            >
                <label>
                    "Name: "
                    <input type="text" node_ref=input_ref/>
                </label>
                <button type="submit">"Update Name"</button>
            </form>
            <p>"Name value is : "{move || name.get()} </p>
        </div>
    }
}
