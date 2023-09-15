#![feature(result_flattening)]

cfg_if::cfg_if! {if #[cfg(feature = "ssr")] {
    pub mod server;
}}

pub mod app;
pub mod auth;
pub mod err;
pub mod form;
pub mod home;
pub mod image;
pub mod layout;
pub mod post;
pub mod settings;
pub mod upload;
pub mod util;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    use crate::auth::User;
    use leptos::*;
    use wasm_bindgen::JsValue;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let user: JsValue =
        js_sys::Reflect::get(&web_sys::window().unwrap(), &JsValue::from_str("USER"))
            .unwrap_or(JsValue::NULL);
    let user: Option<User> = serde_wasm_bindgen::from_value(user).ok();
    logging::log!("USER: {:?}", user);

    let settings: JsValue =
        js_sys::Reflect::get(&web_sys::window().unwrap(), &JsValue::from_str("SETTINGS"))
            .unwrap_or(JsValue::NULL);
    let settings: settings::SettingsCx =
        serde_wasm_bindgen::from_value(settings).unwrap_or_default();
    logging::log!("SETTINGS: {:?}", settings);

    leptos::mount_to_body(move || {
        view! { <App user=user.clone() settings=settings.clone()/> }
    });
}
