use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    StaticSegment, components::{Route, Router, Routes}, path
};
use reactive_stores::{Store, StoreFieldIterator};

use crate::{components::users_lists::UsersList, dto::users_dto::{UsersDto, UsersDtoDataStore, UsersDtoStoreFields}};
use crate::{dto::users_dto::UsersDtoDataStoreStoreFields, server_fn::user::get_users};

/// The top level of a store always needs to be a struct, so we’ll create a Data wrapper with a single rows field.
#[derive(Debug, Clone, Store)]
pub struct DataStore {
    #[store(key: String = |row| row.key.clone())]
    pub rows: Vec<Student>,
}

#[derive(Debug, Clone, Store)]
pub struct Student {
    pub key: String,
    pub name: String,
    pub height: u16,
}
impl Student {
    pub fn new(key: String, name: String, height: u16) -> Self {
        Self { key, name, height }
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    

    view! {
        // <Stylesheet id="leptos" href="/pkg/iterate_complex_data.css"/>

        <h1>"Complex data iteration"</h1>

        

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage/>
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
            <Suspense
                fallback = move || {view! {<p>"Load...."</p>}}
            >
                <UsersList />
            </Suspense>
    }

}
