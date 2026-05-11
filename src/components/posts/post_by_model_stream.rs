use crate::components::common::spinner::LoadingSpinner;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Event;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, callback: &js_sys::Function) -> JsValue;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelData {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize)]
struct StreamAiArgs {
    content: String,
    model: String,
}

#[component]
pub fn StreamAiModelView() -> impl IntoView {
    let (post, set_post) = signal(String::new());
    let (response, set_response) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (models, set_models) = signal::<Vec<ModelData>>(vec![]);
    let (selected_model, set_selected_model) = signal(String::new());

    spawn_local(async move {
        let res = invoke_without_args("list_models").await;
        if let Ok(list) = from_value::<Vec<ModelData>>(res) {
            if let Some(first) = list.first() {
                set_selected_model.set(first.name.clone());
            }
            set_models.set(list);
        }
    });

    spawn_local({
        let set_response = set_response.clone();
        async move {
            let callback = Closure::<dyn FnMut(JsValue)>::new(move |event| {
                if let Some(data) = js_sys::Reflect::get(&event, &JsValue::from_str("payload"))
                    .ok()
                    .and_then(|v| v.as_string())
                {
                    set_response.update(|r| r.push_str(&data));
                }
            });
            let _ = listen("ai-stream", callback.as_ref().unchecked_ref()).await;
            callback.forget();
        }
    });

    // Listen for cancellation event to reset loading state
    spawn_local({
        async move {
            let callback = Closure::<dyn FnMut(JsValue)>::new(move |_| {
                set_is_loading.set(false);
            });
            let _ = listen("ai-stream-cancelled", callback.as_ref().unchecked_ref()).await;
            callback.forget();
        }
    });

    let stream_ai = move |_| {
        let content = post.get();
        let model = selected_model.get();
        if content.is_empty() || model.is_empty() {
            return;
        }
        set_response.set(String::new());
        set_is_loading.set(true);

        spawn_local(async move {
            let args = to_value(&StreamAiArgs { content, model }).unwrap();
            let _ = invoke("stream_ai_model", args).await;
            set_is_loading.set(false);
        });
    };

    // 👇 Stop button handler
    let stop_stream = move |_| {
        spawn_local(async move {
            let _ = invoke_without_args("stop_stream").await;
        });
    };

    let clear = move |_| {
        set_post.set(String::new());
        set_response.set(String::new());
    };

    view! {
        <div class="p-4 dark:bg-gray-900 dark:text-white">
            <h2 class="text-xl font-bold mb-2">"AI Streaming Assistant"</h2>

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
                        view! { <option value=m.name.clone()>{m.name.clone()}</option> }
                    }
                />
            </select>

            <textarea
                class="w-full h-48 p-2 border rounded dark:bg-gray-800"
                placeholder="Write your prompt here..."
                prop:value=move || post.get()
                on:input=move |e| set_post.set(event_target_value(&e))
            />

            <div class="flex space-x-4 mt-2">
                // Start button — hidden while streaming
                <button
                    class="px-4 py-2 bg-blue-500 text-white rounded
                           disabled:opacity-50 disabled:cursor-not-allowed"
                    on:click=stream_ai
                    disabled=move || is_loading.get()
                >
                    "Start Stream"
                </button>

                // 👇 Stop button — only visible while streaming
                {move || is_loading.get().then(|| view! {
                    <button
                        class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded"
                        on:click=stop_stream
                    >
                        "Stop"
                    </button>
                })}

                <button
                    class="px-4 py-2 bg-gray-500 text-white rounded"
                    on:click=clear
                >
                    "Clear"
                </button>
            </div>

            <div class="mt-4 p-4 border rounded bg-gray-50 dark:bg-gray-800 text-sm">
                {move || {
                    let html = response
                        .get()
                        .replace("\n", "<br>")
                        .replace("<think>", r#"<think><span class="italic text-sm">"#)
                        .replace("</think>", "</span></think>");
                    if is_loading.get() {
                        view! {
                            <>
                                <LoadingSpinner />
                                <div inner_html=html></div>
                            </>
                        }.into_any()
                    } else {
                        view! { <div inner_html=html></div> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
