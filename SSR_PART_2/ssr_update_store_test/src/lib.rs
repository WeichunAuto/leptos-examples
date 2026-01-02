pub mod app;

#[cfg(feature = "ssr")] // 整个 backend 模块只在server使用
pub mod entity;

#[cfg(feature = "ssr")] // 整个 backend 模块只在server使用
pub mod config;

#[cfg(feature = "ssr")] // 整个 backend 模块只在server使用
pub mod state;

pub mod dto;
pub mod server_fn;
pub mod components;



#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
