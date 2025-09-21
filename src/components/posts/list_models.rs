use crate::components::navigation::nav::Nav;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelData {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

#[component]
pub fn ModelList() -> impl IntoView {
    // Holds the list of models
    let (models, set_models) = signal::<Vec<ModelData>>(vec![]);

    // Load models when component mounts
    spawn_local(async move {
        let res = invoke_without_args("list_models").await;
        if let Ok(list) = from_value::<Vec<ModelData>>(res) {
            set_models.set(list);
        }
    });

    view! {
        <>
        <Nav />
        <div class="p-4 dark:bg-gray-900 dark:text-white">
            <h2 class="text-xl font-bold mb-2">"Available Models"</h2>
            <ul class="space-y-2">
                <For
                    each=move || models.get()
                    key=|m| m.name.clone()
                    children=move |model: ModelData| {
                        view! {
                            <li class="p-2 rounded border shadow-sm">
                                <p class="font-semibold">{model.name.clone()}</p>
                                <p class="text-sm">
                                    {format!("Modified: {}", model.modified_at)}
                                </p>
                                <p class="text-sm">
                                    {format!("Size: {} MB", model.size / 1024 / 1024)}
                                </p>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
        </>
    }
}
