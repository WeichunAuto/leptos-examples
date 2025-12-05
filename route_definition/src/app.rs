use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes, A},
    hooks::use_params_map,
    path,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/route_definition.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <h1>"Contact App"</h1>

            // this <nav> will show on every routes,
            // because it's outside the <Routes/>
            // note: we can just use normal <a> tags
            // and the router will use client-side navigation

            <nav class="top-nav">
                <ul>
                    <li><a href="/">"Home"</a></li>
                    <li><a href="/contacts">"Contacts"</a></li>
                </ul>
            </nav>

            <main>
                <Routes fallback=move || "Not found.">
                    // just has an un-nested "Home"
                    <Route path=path!("/") view= || view! {<h3>"Home"</h3>}/>

                    <ParentRoute path=path!("/contacts") view=ContactList >
                        <Route path=path!("") view=|| view! {
                            <div class="select-user">"Select a User to view contact info."</div>
                        }/>
                        // if no id specified, fall back
                        <ParentRoute path=path!(":id") view=ContactInfo>
                            <Route path=path!("") view=|| view! {
                                <div class="tab">
                                    "(Contact Info)"
                                </div>
                            }/>
                            <Route path=path!("conversations") view=|| view! {
                                <div class="tab">
                                    "(Conversations)"
                                </div>
                            }/>
                        </ParentRoute>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn ContactList() -> impl IntoView {
    view! {
            <div class="contacts">
                // here's our contact list component itself
                <div style="width: 200px; border:solid 1px gray;">
                    <h3>"Contacts"</h3>
                    <div class="contact-list-contacts">
                        <p><A href="alice">"Alice"</A></p>
                        <p><A href="bob">"Bob"</A></p>
                        <p><A href="steve">"Steve"</A></p>
                    </div>
                </div>

                // <Outlet/> will show the nested child route
                // we can position this outlet wherever we want
                // within the layout
                // 子组件占位符
                <Outlet/>
            </div>

    }
}

#[component]
fn ContactInfo() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();

    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };

    view! {
        <div style="width: 250px; border: solid 1px green;">
            <h4>{name}</h4>
            <div class="contact-info">
                <div class="tabs">
                    <A href="" exact=true>"Contact Info"</A>
                    <A href="conversations">"Conversations"</A>
                </div>

                // <Outlet/> here is the tabs that are nested
                // underneath the /contacts/:id route
                <Outlet/>
            </div>
        </div>
    }
}
