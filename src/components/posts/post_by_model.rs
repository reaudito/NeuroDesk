use crate::components::common::spinner::LoadingSpinner;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelData {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize)]
struct AskAiArgs {
    content: String,
    model: String,
}

#[component]
pub fn CreatePostWithModels() -> impl IntoView {
    // State
    let (post, set_post) = signal(String::new());
    let (response, set_response) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (models, set_models) = signal::<Vec<ModelData>>(vec![]);
    let (selected_model, set_selected_model) = signal(String::new());

    // Load models on mount
    spawn_local(async move {
        let res = invoke_without_args("list_models").await;
        if let Ok(list) = from_value::<Vec<ModelData>>(res) {
            if let Some(first) = list.first() {
                set_selected_model.set(first.name.clone());
            }
            set_models.set(list);
        }
    });

    // Ask AI
    let ask_ai = move |_| {
        let content = post.get();
        let model = selected_model.get();
        if content.is_empty() || model.is_empty() {
            return;
        }

        set_is_loading.set(true);
        spawn_local(async move {
            let args = to_value(&AskAiArgs { content, model }).unwrap();
            let res = invoke("ask_ai_model", args).await;
            if let Some(text) = res.as_string() {
                set_response.set(text);
            }
            set_is_loading.set(false);
        });
    };

    // Clear
    let clear = move |_| {
        set_post.set(String::new());
        set_response.set(String::new());
    };

    view! {
        <>
        <div class="p-4 dark:bg-gray-900 dark:text-white">
            <h2 class="text-xl font-bold mb-2">"Ask AI Model"</h2>

            // Select model
            <label class="block mb-2">"Choose a model:"</label>

            <select
                class="appearance-none mb-4 p-2 border rounded
                       bg-white text-black
                       dark:bg-gray-800 dark:text-white
                       dark:border-gray-600"
                on:change=move |ev| set_selected_model.set(event_target_value(&ev))
            >
                <For
                    each=move || models.get()
                    key=|m| m.name.clone()
                    children=move |m: ModelData| {
                        view! {
                            <option value=m.name.clone()>{m.name.clone()}</option>
                        }
                    }
                />
            </select>


            // Input content
            <textarea
                class="w-full h-48 p-2 border rounded dark:bg-gray-800"
                placeholder="Write your prompt here..."
                on:input=move |e| set_post.set(event_target_value(&e))
            />

            // Buttons
            <div class="flex space-x-4 mt-2">
                <button
                    class="px-4 py-2 bg-blue-500 text-white rounded"
                    on:click=ask_ai
                    disabled=move || is_loading.get()
                >
                    {move || if is_loading.get() { "Loading..." } else { "Ask AI" }}
                </button>

                <button
                    class="px-4 py-2 bg-gray-500 text-white rounded"
                    on:click=clear
                >
                    "Clear"
                </button>
            </div>

            // Response
            <div class="mt-4 p-4 border rounded hover:bg-gray-100 dark:hover:bg-gray-700">
                {move || {
                    if is_loading.get() {
                        view! { <LoadingSpinner /> }.into_any()
                    } else {
                        let html_content = response
                            .get()
                            .replace("\n", "<br>")
                            .replace("<think>", r#"<think><span class="italic text-sm">"#)
                            .replace("</think>", "</span></think>");
                        view! { <div inner_html=html_content></div> }.into_any()
                    }
                }}
            </div>
        </div>
        </>
    }
}
