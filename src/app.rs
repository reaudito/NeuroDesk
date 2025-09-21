use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::components::navigation::nav::Nav;
use wasm_bindgen::JsCast;
use crate::router::RouterApp;
use web_sys::{window, HtmlElement};
use crate::components::common::global_state::GlobalState;
use reactive_stores::Store;




#[component]
pub fn App() -> impl IntoView {


    provide_context(Store::new(GlobalState::default()));
    
    let window = window().expect("should have a Window");

    // Check if the user prefers a dark color scheme
    let prefers_dark_mode = match window.match_media("(prefers-color-scheme: dark)") {
        Ok(Some(media_query_list)) => media_query_list.matches(),
        _ => false,
    };

    // Set the class on the <html> tag
    if let Some(document) = window.document() {
        if let Some(html) = document.document_element() {
            let html: HtmlElement = html.unchecked_into();
            if prefers_dark_mode {
                html.set_class_name("dark");
            } else {
                html.set_class_name("");
            }
        }
    }
    view! { <RouterApp /> }
}
