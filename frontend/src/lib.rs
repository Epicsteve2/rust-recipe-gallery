pub mod api;
pub mod app;
mod components;

#[cfg(feature = "ssr")]
pub mod fallback;

pub mod models;

// cfg_if! { if #[cfg(feature = "hydrate")] {
//     use leptos::*;
//     use wasm_bindgen::prelude::wasm_bindgen;
//     use crate::app::*;

//     #[wasm_bindgen]
//     pub fn hydrate() {
//         // initializes logging using the `log` crate
//         _ = console_log::init_with_level(log::Level::Debug);
//         console_error_panic_hook::set_once();

//         leptos::mount_to_body(move || {
//             view! {  <App/> }
//         });
//     }
// }}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;

    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
