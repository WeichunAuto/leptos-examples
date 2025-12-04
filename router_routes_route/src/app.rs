use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/router_routes_route.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=path!("/") view=HomePage/>
                    // If I go to /users/3, the path matches <Users/> and <UserProfile/>.
                    // If I go to /users, the path matches <Users/> and <NoUser/>

                    // ParentRoute 用来定义“有布局的嵌套路由”，它的 view 会成为父级布局，子路由的视图会自动插入到该布局中的 <Outlet/>。
                    <ParentRoute path=path!("/users") view=Users>
                        <Route path=path!(":id") view=UserProfile/>
                        <Route path=path!("") view= move || view! {<h2>"Can not match any route."</h2>}/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Users() -> impl IntoView {
    view! {
        <h1>"This is the users page."</h1>

        // 子 Route 要想渲染出来，必须在父组件里放 <Outlet/>，否则永远不会显示。
        // 子路由通过 <Outlet/> 挂载到父布局
        <Outlet/>
    }
}

#[component]
fn UserProfile() -> impl IntoView {
    view! {
        <p>"This is the user profile page."</p>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
