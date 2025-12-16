use leptos::prelude::*;
use leptos::Params;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::hooks::use_params;
use leptos_router::hooks::use_query_map;
use leptos_router::params::Params;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes, A},
    path,
};

use reactive_stores::Store;

#[derive(Debug, Clone, Store)]
struct UserStore {
    #[store(key: String = |row| row.id.to_string())]
    pub rows: Vec<User>,
}

impl UserStore {
    pub fn new(rows: Vec<User>) -> Self {
        Self { rows }
    }
}

#[derive(Debug, Clone, Store)]
struct User {
    id: i32,
    name: String,
}
impl User {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Params, PartialEq)]
struct UserParams {
    id: Option<i32>,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let rows = vec![
        User::new(0, "Lucy".to_string()),
        User::new(1, "Bobby".to_string()),
        User::new(2, "Nancy".to_string()),
    ];

    let user_store = Store::new(UserStore::new(rows));

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/route_params_queries.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=path!("/") view=HomePage/>
                    <ParentRoute path=path!("/users_list") view= move || {view! {<UserList user_store/>}}>
                        <Route path=path!("") view= || {view! {<p>"没有具体的用户信息。"</p>}}/>
                        <Route path=path!(":id") view= move || {view !{<UserInfoById user_store />}}/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <A href="/users_list">"users list"</A>
    }
}

/// 404 - Not Found
#[component]
fn UserList(user_store: Store<UserStore>) -> impl IntoView {
    view! {
        <div>
            <p>
                <A href="/">"返回首页"</A>
            </p>
            <div style="border:solid 1px green;">
                <h1>"User List page"</h1>
                <div class="grid-table">
                  <div>"ID"</div>
                  <div>"Name"</div>
                  <div>"操作"</div>

                  <ForEnumerate
                    each= move || user_store.rows()
                    key = |row_signal| row_signal.read().id.clone()
                    children = move |_, row_signal| {
                        view! {
                            <div>{row_signal.read().id}</div>
                            <div>{row_signal.read().name.clone()}</div>
                            <div>
                                <A href={row_signal.read().id.to_string()}>"详细"</A> " | "
                                <A href= move || {
                                    format!("{}?name={}", row_signal.read().id.to_string(), row_signal.read().name.to_string())
                                }>"名称详细"</A>
                            </div>
                        }
                    }
                  />
                </div>
            </div>
            <Outlet />
        </div>

    }
}

#[component]
fn UserInfoById(user_store: Store<UserStore>) -> impl IntoView {
    // let params = use_params_map();
    // let id = move || params.read().get("id").unwrap_or_default();
    // 以上方式提取 params id 也可以。

    // 推荐使用自定义struct方式 提取 parames 和 queries 中参数。
    let params = use_params::<UserParams>();
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .unwrap_or_default()
    };

    let queries = use_query_map();
    let query_name = move || queries.read().get("name");

    let name = {
        move || {
            let rows = user_store.rows().read();
            let name = rows
                .iter()
                .find(|user| user.id == id())
                .map(|user| user.name.clone())
                .unwrap_or_default();
            name
        }
    };
    view! {
        <p>"用户信息详情: "<strong>{move || id()}</strong>", "<strong>{move || name()}</strong></p>

        <Show
            when= move || {query_name().is_some()}
            fallback = || () // fallback 什么都不显示
        >
            <p>"queryName = "{move || query_name()}</p>
        </Show>

    }
}
